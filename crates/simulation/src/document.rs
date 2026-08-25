use crate::component::{ComponentTypeId, ParameterDefinition, PortDefinition};
use crate::identity::{ComponentId, ConnectionId, DocumentId, PortId, ProbeId, SystemId};
use crate::parameter::ParameterValueType;
use crate::timing::FixedStepSemantics;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Current schema accepted for model documents.
pub const MODEL_SCHEMA_VERSION: SchemaVersion = SchemaVersion { major: 1, minor: 0 };
/// Current schema accepted for custom-component documents.
pub const COMPONENT_SCHEMA_VERSION: SchemaVersion = SchemaVersion { major: 1, minor: 0 };

/// Version of a persisted document schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaVersion {
    /// Breaking schema version.
    pub major: u16,
    /// Backward-compatible schema version.
    pub minor: u16,
}

/// Shared descriptive and compatibility metadata for persisted documents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentHeader {
    /// Schema version used to encode this document.
    pub schema_version: SchemaVersion,
    /// Stable document identity.
    pub document_id: DocumentId,
    /// User-facing title.
    pub title: ShareableString,
    /// User-facing description.
    pub description: ShareableString,
    /// Author or owning team.
    pub author: ShareableString,
    /// Creation timestamp supplied by the application.
    pub created_at: ShareableString,
    /// Last-update timestamp supplied by the application.
    pub updated_at: ShareableString,
    /// Migrations already applied to this artifact.
    pub migrations: Vec<MigrationRecord>,
}

/// Audit record for one persisted document migration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRecord {
    /// Schema version before migration.
    pub from: SchemaVersion,
    /// Schema version after migration.
    pub to: SchemaVersion,
    /// Stable migration identifier.
    pub migration_id: ShareableString,
}

/// Revision of an independently versioned custom-component artifact.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ArtifactRevision(pub ShareableString);

/// Reference to either a registry built-in or a custom-component document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ComponentReference {
    /// Registry-provided primitive.
    BuiltIn {
        /// Stable registry type ID.
        type_id: ComponentTypeId,
    },
    /// Independently persisted reusable component.
    Custom {
        /// Expected custom-component document identity.
        document_id: DocumentId,
        /// Requested artifact revision.
        revision: ArtifactRevision,
        /// Application-resolved path or URI.
        source: ShareableString,
    },
}

/// Reproducibility pin for a custom-component dependency.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyLock {
    /// Expected custom-component document identity.
    pub document_id: DocumentId,
    /// Resolved revision.
    pub revision: ArtifactRevision,
    /// BLAKE3 checksum encoded as lowercase hexadecimal text.
    pub checksum: ShareableString,
    /// Resolved path or URI.
    pub source: ShareableString,
}

/// Persisted canvas position.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CanvasPosition {
    /// Horizontal canvas coordinate.
    pub x: f64,
    /// Vertical canvas coordinate.
    pub y: f64,
}

/// One component instance in a source composition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentInstance {
    /// Stable instance identity.
    pub id: ComponentId,
    /// Scoped user-facing name.
    pub name: ShareableString,
    /// Built-in or custom-component source.
    pub component: ComponentReference,
    /// Calculator expressions keyed by public parameter key.
    pub parameter_overrides: BTreeMap<ShareableString, ShareableString>,
    /// Whether the instance participates in execution.
    pub enabled: bool,
    /// Persisted editor position.
    pub position: CanvasPosition,
}

/// Reference to one component port in a source composition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortEndpoint {
    /// Component instance identity.
    pub component_id: ComponentId,
    /// Public port key on that instance.
    pub port_key: ShareableString,
}

/// Persisted connection between two component ports.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Connection {
    /// Stable connection identity.
    pub id: ConnectionId,
    /// Output endpoint.
    pub source: PortEndpoint,
    /// Input endpoint.
    pub target: PortEndpoint,
    /// Optional user-facing label.
    pub label: Option<ShareableString>,
    /// Persisted visual routing points.
    pub route: Vec<CanvasPosition>,
}

/// Persisted output probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbeDefinition {
    /// Stable probe identity.
    pub id: ProbeId,
    /// Port sampled by the probe.
    pub target: PortEndpoint,
    /// User-facing series name.
    pub display_name: ShareableString,
    /// Optional plot-group key.
    pub plot_group: Option<ShareableString>,
}

/// Root editable composition shared by models and custom components.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Composition {
    /// Stable root-system identity.
    pub system_id: SystemId,
    /// Component instances in stable display order.
    pub components: Vec<ComponentInstance>,
    /// Connections in stable display order.
    pub connections: Vec<Connection>,
    /// Persisted editor annotations.
    pub annotations: BTreeMap<ShareableString, ShareableString>,
}

/// Initial logging modes supported by model documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoggingPolicy {
    /// Capture every fixed-step sample.
    EveryStep,
    /// Capture every `interval` grid samples and always capture the final sample.
    EveryNthStep {
        /// Positive sample-index interval.
        interval: u64,
    },
}

impl LoggingPolicy {
    /// Returns whether one sample index should be retained.
    #[must_use]
    pub fn captures(self, sample_index: u64, final_index: u64) -> bool {
        match self {
            Self::EveryStep => true,
            Self::EveryNthStep { interval } => {
                sample_index.checked_rem(interval) == Some(0) || sample_index == final_index
            }
        }
    }
}

/// Persisted fixed-step simulation settings.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SimulationSettings {
    /// First simulation time.
    pub start_time: f64,
    /// Requested stop time.
    pub stop_time: f64,
    /// Fixed timestep.
    pub timestep: f64,
    /// Hard execution limit.
    pub maximum_steps: u64,
    /// Deterministic random seed.
    pub random_seed: u64,
    /// Output sampling policy.
    pub logging: LoggingPolicy,
    /// Fixed-step endpoint and state behavior.
    pub semantics: FixedStepSemantics,
}

/// Top-level persisted simulation model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelDocument {
    /// Shared identity and compatibility metadata.
    pub header: DocumentHeader,
    /// Root editable composition.
    pub root: Composition,
    /// Simulation and logging settings.
    pub simulation: SimulationSettings,
    /// Requested output probes.
    pub probes: Vec<ProbeDefinition>,
    /// Locked custom-component dependencies.
    pub dependencies: Vec<DependencyLock>,
}

/// Public port with a persisted identity for mapping across artifact boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicPortDefinition {
    /// Stable public-port identity.
    pub id: PortId,
    /// Interface metadata.
    pub definition: PortDefinition,
}

/// Mapping from a public custom-component port into its private graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicPortMapping {
    /// Public custom-component port.
    pub public_port_id: PortId,
    /// Internal component port implementing it.
    pub internal: PortEndpoint,
}

/// Private state declaration owned by a custom component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateDeclaration {
    /// Stable state key within the component.
    pub key: ShareableString,
    /// State value shape.
    pub value_type: ParameterValueType,
    /// Calculator expression evaluated during initialization.
    pub initial_expression: ShareableString,
}

/// Persisted executable example for a custom component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentTestCase {
    /// User-facing test name.
    pub name: ShareableString,
    /// Public parameter overrides used by the case.
    pub parameter_overrides: BTreeMap<ShareableString, ShareableString>,
    /// Human-readable expected behavior until executable assertions are introduced.
    pub expected_behavior: ShareableString,
}

/// Independently versioned reusable custom-component artifact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomComponentDocument {
    /// Shared identity and compatibility metadata.
    pub header: DocumentHeader,
    /// Independent artifact revision.
    pub revision: ArtifactRevision,
    /// Public component parameters.
    pub public_parameters: Vec<ParameterDefinition>,
    /// Public component ports.
    pub public_ports: Vec<PublicPortDefinition>,
    /// Private implementation graph.
    pub implementation: Composition,
    /// Public-to-private port mappings.
    pub port_mappings: Vec<PublicPortMapping>,
    /// Private state declarations.
    pub state: Vec<StateDeclaration>,
    /// Persisted component examples/tests.
    pub test_cases: Vec<ComponentTestCase>,
    /// Locked transitive custom-component dependencies.
    pub dependencies: Vec<DependencyLock>,
    /// Long-form component documentation.
    pub documentation: ShareableString,
    /// Optional replacement artifact identity when deprecated.
    pub replacement: Option<DocumentId>,
}
