//! Command-driven model editing, persistence, validation, and execution.

use shareable_string::ShareableString;
use simulation::builtins::register_signal_builtins;
use simulation::component::ComponentTypeId;
use simulation::diagnostic::{Diagnostic, DiagnosticCategory, DiagnosticSeverity};
use simulation::document::{
    CanvasPosition, ComponentInstance, ComponentReference, Composition, Connection, DocumentHeader,
    LoggingPolicy, MODEL_SCHEMA_VERSION, ModelDocument, PortEndpoint, ProbeDefinition,
    SimulationSettings,
};
use simulation::identity::{ComponentId, ConnectionId, DocumentId, ProbeId, RunId, SystemId};
use simulation::persistence::{load_custom_component_json, load_model_json, save_model_json};
use simulation::registry::ComponentRegistry;
use simulation::resolve::{
    CustomComponentLoader, LoadedCustomComponent, resolve_model, resolve_model_with_placeholders,
};
use simulation::results::SimulationRun;
use simulation::runtime::SimulationRuntime;
use simulation::timing::FixedStepSemantics;
use simulation::validation::{ValidationLimits, validate_model};
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// One persistent model mutation accepted by [`DocumentController`].
#[derive(Debug, Clone, PartialEq)]
pub enum DocumentCommand {
    /// Adds one registry built-in to the root system.
    AddBuiltIn {
        /// Stable built-in type identity.
        type_id: ComponentTypeId,
        /// User-facing instance name.
        name: ShareableString,
        /// Initial canvas location.
        position: CanvasPosition,
    },
    /// Changes one component's canvas location.
    MoveComponent {
        /// Component to move.
        component_id: ComponentId,
        /// New canvas location.
        position: CanvasPosition,
    },
    /// Sets or replaces one public parameter expression.
    SetParameter {
        /// Component to configure.
        component_id: ComponentId,
        /// Public parameter key.
        key: ShareableString,
        /// Source expression.
        expression: ShareableString,
    },
    /// Connects one output to one input.
    Connect {
        /// Source endpoint.
        source: PortEndpoint,
        /// Target endpoint.
        target: PortEndpoint,
    },
    /// Adds a persisted output probe.
    AddProbe {
        /// Endpoint to sample.
        target: PortEndpoint,
        /// User-facing result label.
        display_name: ShareableString,
    },
    /// Removes a component and all links and probes that reference it.
    RemoveComponent {
        /// Component to remove.
        component_id: ComponentId,
    },
    /// Replaces model timing and logging settings.
    SetSimulation(SimulationSettings),
}

/// Identity created by one command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandOutcome {
    /// No new entity was created.
    Updated,
    /// A component was created.
    Component(ComponentId),
    /// A connection was created.
    Connection(ConnectionId),
    /// A probe was created.
    Probe(ProbeId),
}

/// Application-layer operation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorError(ShareableString);

impl EditorError {
    /// Creates an error suitable for display in the application shell.
    #[must_use]
    pub fn new(message: impl Into<ShareableString>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for EditorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl std::error::Error for EditorError {}

/// Filesystem-backed custom-component loader rooted beside the model file.
#[derive(Debug, Clone)]
pub struct FilesystemComponentLoader {
    /// Directory used to resolve relative artifact sources.
    base_directory: PathBuf,
}

impl FilesystemComponentLoader {
    /// Creates a loader rooted at `base_directory`.
    #[must_use]
    pub fn new(base_directory: impl Into<PathBuf>) -> Self {
        Self {
            base_directory: base_directory.into(),
        }
    }
}

impl CustomComponentLoader for FilesystemComponentLoader {
    fn load(&self, source: &str) -> Result<LoadedCustomComponent, Diagnostic> {
        let path = self.base_directory.join(source);
        let bytes = fs::read(path).map_err(|_error| load_diagnostic())?;
        let json = std::str::from_utf8(&bytes).map_err(|_error| load_diagnostic())?;
        let document = load_custom_component_json(json)?;
        Ok(LoadedCustomComponent {
            document,
            checksum: blake3::hash(&bytes).to_hex().as_str().into(),
        })
    }
}

/// Owns one open model and all application-level mutation history.
#[derive(Debug)]
pub struct DocumentController {
    /// Installed executable built-ins.
    registry: ComponentRegistry,
    /// Open editable source document.
    document: ModelDocument,
    /// Current native file location.
    path: Option<PathBuf>,
    /// Snapshots preceding successfully applied commands.
    undo: Vec<ModelDocument>,
    /// Snapshots removed by undo.
    redo: Vec<ModelDocument>,
    /// Whether the source differs from its last save/open state.
    dirty: bool,
    /// Most recent validation diagnostics.
    diagnostics: Vec<Diagnostic>,
    /// Most recent synchronous simulation run.
    last_run: Option<SimulationRun>,
    /// Monotonic application-local run identity.
    next_run_id: u128,
}

impl DocumentController {
    /// Creates a controller around a new empty model.
    ///
    /// # Errors
    ///
    /// Returns an error if the standard built-in registry cannot be installed.
    pub fn new(document_id: DocumentId, timestamp: &str) -> Result<Self, EditorError> {
        Self::from_document(empty_model(document_id, timestamp), None)
    }

    /// Opens a native model file.
    ///
    /// # Errors
    ///
    /// Returns an error for unreadable, malformed, or unsupported documents.
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, EditorError> {
        let path = path.into();
        let source = fs::read_to_string(&path).map_err(display_error)?;
        let document = load_model_json(&source).map_err(debug_error)?;
        Self::from_document(document, Some(path))
    }

    /// Restores a model from its autosave sidecar while retaining the native target path.
    ///
    /// # Errors
    ///
    /// Returns an error when the sidecar is unreadable or invalid.
    pub fn recover(path: impl Into<PathBuf>) -> Result<Self, EditorError> {
        let path = path.into();
        let source = fs::read_to_string(recovery_path(&path)).map_err(display_error)?;
        let document = load_model_json(&source).map_err(debug_error)?;
        let mut controller = Self::from_document(document, Some(path))?;
        controller.dirty = true;
        Ok(controller)
    }

    /// Returns the editable source document.
    #[must_use]
    pub const fn document(&self) -> &ModelDocument {
        &self.document
    }

    /// Returns the current native file location.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Returns whether unsaved command mutations exist.
    #[must_use]
    pub const fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Returns installed built-in metadata in stable identity order.
    pub fn palette(&self) -> impl Iterator<Item = &simulation::component::ComponentDefinition> {
        self.registry.iter()
    }

    /// Returns installed metadata for one built-in type.
    #[must_use]
    pub fn definition(
        &self,
        type_id: &ComponentTypeId,
    ) -> Option<&simulation::component::ComponentDefinition> {
        self.registry.get(type_id)
    }

    /// Returns diagnostics from the most recent validate or run action.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Returns the most recent run.
    #[must_use]
    pub const fn last_run(&self) -> Option<&SimulationRun> {
        self.last_run.as_ref()
    }

    /// Applies one persistent source mutation and records an undo snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error when a referenced entity does not exist or identity space is exhausted.
    pub fn execute(&mut self, command: DocumentCommand) -> Result<CommandOutcome, EditorError> {
        let before = self.document.clone();
        let outcome = apply_command(&mut self.document, command)?;
        self.undo.push(before);
        self.redo.clear();
        self.dirty = true;
        self.diagnostics.clear();
        self.last_run = None;
        Ok(outcome)
    }

    /// Restores the source snapshot preceding the latest command.
    #[must_use]
    pub fn undo(&mut self) -> bool {
        let Some(previous) = self.undo.pop() else {
            return false;
        };
        self.redo
            .push(std::mem::replace(&mut self.document, previous));
        self.dirty = true;
        self.last_run = None;
        true
    }

    /// Reapplies the most recently undone source snapshot.
    #[must_use]
    pub fn redo(&mut self) -> bool {
        let Some(next) = self.redo.pop() else {
            return false;
        };
        self.undo.push(std::mem::replace(&mut self.document, next));
        self.dirty = true;
        self.last_run = None;
        true
    }

    /// Resolves permissively and refreshes deterministic diagnostics.
    pub fn validate(&mut self) {
        let loader = self.loader();
        let resolved = resolve_model_with_placeholders(&self.document, &self.registry, &loader);
        self.diagnostics = validate_model(&resolved, ValidationLimits::default());
    }

    /// Strictly resolves and executes the open model.
    ///
    /// # Errors
    ///
    /// Returns an error when resolution, validation, runtime construction, or run identity fails.
    pub fn run(&mut self) -> Result<&SimulationRun, EditorError> {
        let loader = self.loader();
        let resolved =
            resolve_model(&self.document, &self.registry, &loader).map_err(debug_error)?;
        self.diagnostics = validate_model(&resolved, ValidationLimits::default());
        if self
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity() == DiagnosticSeverity::Error)
        {
            return Err(EditorError::new("model validation failed"));
        }
        let mut runtime = SimulationRuntime::new(&resolved, &self.registry).map_err(debug_error)?;
        let run_id = RunId::from_raw(self.next_run_id);
        self.next_run_id = self
            .next_run_id
            .checked_add(1)
            .ok_or_else(|| EditorError::new("run identity space exhausted"))?;
        self.last_run = Some(runtime.run(run_id));
        self.last_run
            .as_ref()
            .ok_or_else(|| EditorError::new("runtime did not retain its result"))
    }

    /// Atomically saves to the current native path.
    ///
    /// # Errors
    ///
    /// Returns an error if no path is known or serialization/replacement fails.
    pub fn save(&mut self) -> Result<(), EditorError> {
        let path = self
            .path
            .clone()
            .ok_or_else(|| EditorError::new("choose a model path before saving"))?;
        self.save_as(path)
    }

    /// Atomically saves to a new native path and makes it current.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization or same-directory replacement fails.
    pub fn save_as(&mut self, path: impl Into<PathBuf>) -> Result<(), EditorError> {
        let path = path.into();
        let source = save_model_json(&self.document).map_err(display_error)?;
        let temporary = temporary_path(&path);
        fs::write(&temporary, source).map_err(display_error)?;
        fs::rename(&temporary, &path).map_err(display_error)?;
        remove_if_present(&recovery_path(&path))?;
        self.path = Some(path);
        self.dirty = false;
        Ok(())
    }

    /// Writes a recoverable sidecar without changing dirty or native-path state.
    ///
    /// # Errors
    ///
    /// Returns an error when serialization or sidecar replacement fails.
    pub fn autosave(&self, path: impl AsRef<Path>) -> Result<(), EditorError> {
        let path = recovery_path(path.as_ref());
        let source = save_model_json(&self.document).map_err(display_error)?;
        let temporary = temporary_path(&path);
        fs::write(&temporary, source).map_err(display_error)?;
        fs::rename(temporary, path).map_err(display_error)
    }

    /// Creates a controller with the standard executable registry.
    fn from_document(document: ModelDocument, path: Option<PathBuf>) -> Result<Self, EditorError> {
        let mut registry = ComponentRegistry::new();
        register_signal_builtins(&mut registry).map_err(debug_error)?;
        Ok(Self {
            registry,
            document,
            path,
            undo: Vec::new(),
            redo: Vec::new(),
            dirty: false,
            diagnostics: Vec::new(),
            last_run: None,
            next_run_id: 1,
        })
    }

    /// Creates a filesystem loader relative to the current model path.
    fn loader(&self) -> FilesystemComponentLoader {
        let base = self
            .path
            .as_deref()
            .and_then(Path::parent)
            .unwrap_or_else(|| Path::new("."));
        FilesystemComponentLoader::new(base)
    }
}

/// Applies one command without touching controller history.
fn apply_command(
    document: &mut ModelDocument,
    command: DocumentCommand,
) -> Result<CommandOutcome, EditorError> {
    match command {
        DocumentCommand::AddBuiltIn {
            type_id,
            name,
            position,
        } => {
            let id = ComponentId::from_raw(next_component_id(document)?);
            document.root.components.push(ComponentInstance {
                id,
                name,
                component: ComponentReference::BuiltIn { type_id },
                parameter_overrides: BTreeMap::new(),
                enabled: true,
                position,
            });
            Ok(CommandOutcome::Component(id))
        }
        DocumentCommand::MoveComponent {
            component_id,
            position,
        } => {
            component_mut(document, component_id)?.position = position;
            Ok(CommandOutcome::Updated)
        }
        DocumentCommand::SetParameter {
            component_id,
            key,
            expression,
        } => {
            component_mut(document, component_id)?
                .parameter_overrides
                .insert(key, expression);
            Ok(CommandOutcome::Updated)
        }
        DocumentCommand::Connect { source, target } => {
            require_component(document, source.component_id)?;
            require_component(document, target.component_id)?;
            let id = ConnectionId::from_raw(next_connection_id(document)?);
            document.root.connections.push(Connection {
                id,
                source,
                target,
                label: None,
                route: Vec::new(),
            });
            Ok(CommandOutcome::Connection(id))
        }
        DocumentCommand::AddProbe {
            target,
            display_name,
        } => {
            require_component(document, target.component_id)?;
            let id = ProbeId::from_raw(next_probe_id(document)?);
            document.probes.push(ProbeDefinition {
                id,
                target,
                display_name,
                plot_group: None,
            });
            Ok(CommandOutcome::Probe(id))
        }
        DocumentCommand::RemoveComponent { component_id } => {
            let original_length = document.root.components.len();
            document
                .root
                .components
                .retain(|component| component.id != component_id);
            if document.root.components.len() == original_length {
                return Err(EditorError::new("component does not exist"));
            }
            document.root.connections.retain(|connection| {
                connection.source.component_id != component_id
                    && connection.target.component_id != component_id
            });
            document
                .probes
                .retain(|probe| probe.target.component_id != component_id);
            Ok(CommandOutcome::Updated)
        }
        DocumentCommand::SetSimulation(settings) => {
            document.simulation = settings;
            Ok(CommandOutcome::Updated)
        }
    }
}

/// Creates a valid empty source model.
fn empty_model(document_id: DocumentId, timestamp: &str) -> ModelDocument {
    ModelDocument {
        header: DocumentHeader {
            schema_version: MODEL_SCHEMA_VERSION,
            document_id,
            title: "Untitled signal model".into(),
            description: "".into(),
            author: "".into(),
            created_at: timestamp.into(),
            updated_at: timestamp.into(),
            migrations: Vec::new(),
        },
        root: Composition {
            system_id: SystemId::from_raw(1),
            components: Vec::new(),
            connections: Vec::new(),
            annotations: BTreeMap::new(),
        },
        simulation: SimulationSettings {
            start_time: 0.0,
            stop_time: 1.0,
            timestep: 0.01,
            maximum_steps: 100_000,
            random_seed: 1,
            logging: LoggingPolicy::EveryStep,
            semantics: FixedStepSemantics::default(),
        },
        probes: Vec::new(),
        dependencies: Vec::new(),
    }
}

/// Returns a mutable component or a stable application error.
fn component_mut(
    document: &mut ModelDocument,
    component_id: ComponentId,
) -> Result<&mut ComponentInstance, EditorError> {
    document
        .root
        .components
        .iter_mut()
        .find(|component| component.id == component_id)
        .ok_or_else(|| EditorError::new("component does not exist"))
}

/// Checks that one component is present.
fn require_component(
    document: &ModelDocument,
    component_id: ComponentId,
) -> Result<(), EditorError> {
    document
        .root
        .components
        .iter()
        .any(|component| component.id == component_id)
        .then_some(())
        .ok_or_else(|| EditorError::new("component does not exist"))
}

/// Allocates the next component identity.
fn next_component_id(document: &ModelDocument) -> Result<u128, EditorError> {
    next_id(
        document
            .root
            .components
            .iter()
            .map(|component| component.id.as_raw()),
    )
}

/// Allocates the next connection identity.
fn next_connection_id(document: &ModelDocument) -> Result<u128, EditorError> {
    next_id(
        document
            .root
            .connections
            .iter()
            .map(|connection| connection.id.as_raw()),
    )
}

/// Allocates the next probe identity.
fn next_probe_id(document: &ModelDocument) -> Result<u128, EditorError> {
    next_id(document.probes.iter().map(|probe| probe.id.as_raw()))
}

/// Allocates one greater than the largest existing identity.
fn next_id(ids: impl Iterator<Item = u128>) -> Result<u128, EditorError> {
    ids.max()
        .unwrap_or(0)
        .checked_add(1)
        .ok_or_else(|| EditorError::new("entity identity space exhausted"))
}

/// Builds a same-directory temporary path for atomic replacement.
fn temporary_path(path: &Path) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(".tmp");
    PathBuf::from(name)
}

/// Builds the autosave sidecar path for one native model.
#[must_use]
pub fn recovery_path(path: &Path) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(".autosave");
    PathBuf::from(name)
}

/// Removes a sidecar when present.
fn remove_if_present(path: &Path) -> Result<(), EditorError> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(display_error(error)),
    }
}

/// Creates a stable dependency-load diagnostic.
fn load_diagnostic() -> Diagnostic {
    Diagnostic::new(
        DiagnosticSeverity::Error,
        DiagnosticCategory::Resolution,
        None,
        Some("source".into()),
        "simulation_editor_component_load_failed",
    )
}

/// Converts displayable errors into application errors.
fn display_error(error: impl fmt::Display) -> EditorError {
    EditorError::new(error.to_string())
}

/// Converts structured lower-layer errors into application errors.
fn debug_error(error: impl fmt::Debug) -> EditorError {
    EditorError::new(format!("{error:?}"))
}
