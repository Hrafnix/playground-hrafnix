use crate::component::{
    ComponentCapabilities, ComponentCapability, ComponentDefinition, ComponentTypeId,
    ParameterDefinition, PortDefinition, PortDirection, SemanticVersion,
};
use crate::diagnostic::{Diagnostic, DiagnosticCategory, DiagnosticSeverity, EntityReference};
use crate::document::{
    ArtifactRevision, ComponentReference, Composition, CustomComponentDocument, DependencyLock,
    ModelDocument, ProbeDefinition, PublicParameterMapping, PublicPortMapping, SimulationSettings,
};
use crate::identity::{ComponentId, DocumentId, PortId, SystemId};
use crate::registry::ComponentRegistry;
use shareable_string::ShareableString;
use std::collections::{BTreeMap, BTreeSet};

/// Loaded custom-component artifact and checksum of its persisted bytes.
#[derive(Debug, Clone, PartialEq)]
pub struct LoadedCustomComponent {
    /// Parsed source document.
    pub document: CustomComponentDocument,
    /// Lowercase hexadecimal checksum computed by the loader.
    pub checksum: ShareableString,
}

/// Application-provided custom-component source loader.
pub trait CustomComponentLoader {
    /// Loads and parses one source artifact.
    ///
    /// # Errors
    ///
    /// Returns a resolution diagnostic when the source cannot be loaded.
    fn load(&self, source: &str) -> Result<LoadedCustomComponent, Diagnostic>;
}

/// Source location retained on every resolved component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceProvenance {
    /// Artifact containing the source instance.
    pub document_id: DocumentId,
    /// Source system containing the instance.
    pub system_id: SystemId,
    /// Stable source instance identity.
    pub component_id: ComponentId,
}

/// Fully resolved source of one component instance.
#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedComponentSource {
    /// Installed built-in definition.
    BuiltIn {
        /// Stable registry identity.
        type_id: ComponentTypeId,
        /// Resolved registry version.
        version: SemanticVersion,
    },
    /// Expanded custom-component implementation.
    Custom {
        /// Stable source artifact identity.
        document_id: DocumentId,
        /// Resolved source revision.
        revision: ArtifactRevision,
        /// Public-to-private port mappings.
        port_mappings: Vec<PublicPortMapping>,
        /// Public-to-private parameter mappings.
        parameter_mappings: Vec<PublicParameterMapping>,
        /// Recursively expanded private implementation.
        implementation: Box<ResolvedSystem>,
    },
    /// Read-only placeholder retaining an unavailable source reference.
    Unresolved {
        /// Original persisted source reference.
        reference: ComponentReference,
        /// Stable reason this source could not be resolved.
        diagnostic: Diagnostic,
    },
}

/// One immutable component instance after dependency resolution.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedComponent {
    /// Stable identity within its source system.
    pub id: ComponentId,
    /// Scoped user-facing name.
    pub name: ShareableString,
    /// Resolved public parameter interface.
    pub parameters: Vec<ParameterDefinition>,
    /// Resolved public port interface.
    pub ports: Vec<PortDefinition>,
    /// Public custom-component port identities mapped to their stable keys.
    pub public_port_ids: BTreeMap<PortId, ShareableString>,
    /// Runtime and scheduling traits.
    pub capabilities: ComponentCapabilities,
    /// Persisted instance parameter expressions.
    pub parameter_overrides: BTreeMap<ShareableString, ShareableString>,
    /// Whether the source instance participates in execution.
    pub enabled: bool,
    /// Built-in definition or expanded custom implementation.
    pub source: ResolvedComponentSource,
    /// Stable source provenance.
    pub provenance: SourceProvenance,
}

/// Immutable resolved system preserving one source composition boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedSystem {
    /// Stable source system identity.
    pub id: SystemId,
    /// Artifact containing this system.
    pub document_id: DocumentId,
    /// Components in source order.
    pub components: Vec<ResolvedComponent>,
    /// Connections in source order.
    pub connections: Vec<crate::document::Connection>,
}

/// Fully resolved model ready for graph validation.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedModel {
    /// Stable source model identity.
    pub document_id: DocumentId,
    /// Expanded root system.
    pub root: ResolvedSystem,
    /// Model-owned simulation settings.
    pub simulation: SimulationSettings,
    /// Model-owned output probes.
    pub probes: Vec<ProbeDefinition>,
}

/// Stable classification of a dependency resolution failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionFailureKind {
    /// No matching dependency lock was present.
    MissingLock,
    /// Loaded document identity differs from the reference or lock.
    IdentityMismatch,
    /// Loaded revision differs from the reference or lock.
    RevisionMismatch,
    /// Loaded checksum differs from the lock.
    ChecksumMismatch,
    /// Loaded source differs from the lock.
    SourceMismatch,
    /// Dependency graph contains a direct or transitive file cycle.
    DependencyCycle,
    /// A built-in component is unavailable.
    UnknownBuiltIn,
    /// The application loader failed.
    LoadFailed,
}

/// Resolution failure with a navigable diagnostic and optional cycle path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolutionFailure {
    /// Stable failure classification.
    pub kind: ResolutionFailureKind,
    /// User-facing structured diagnostic.
    pub diagnostic: Diagnostic,
    /// Complete repeated-endpoint path for dependency cycles.
    pub dependency_path: Vec<DocumentId>,
}

/// Resolves and expands all custom components referenced by a model.
///
/// # Errors
///
/// Returns the first deterministic dependency, lock, cycle, or registry failure.
pub fn resolve_model(
    model: &ModelDocument,
    registry: &ComponentRegistry,
    loader: &impl CustomComponentLoader,
) -> Result<ResolvedModel, ResolutionFailure> {
    let root = resolve_system(
        &model.root,
        model.header.document_id,
        &model.dependencies,
        registry,
        loader,
        &mut Vec::new(),
        false,
    )?;

    Ok(ResolvedModel {
        document_id: model.header.document_id,
        root,
        simulation: model.simulation,
        probes: model.probes.clone(),
    })
}

/// Resolves available dependencies and preserves unavailable instances as placeholders.
#[must_use]
pub fn resolve_model_with_placeholders(
    model: &ModelDocument,
    registry: &ComponentRegistry,
    loader: &impl CustomComponentLoader,
) -> ResolvedModel {
    let root = resolve_system(
        &model.root,
        model.header.document_id,
        &model.dependencies,
        registry,
        loader,
        &mut Vec::new(),
        true,
    )
    .unwrap_or_else(|failure| ResolvedSystem {
        id: model.root.system_id,
        document_id: model.header.document_id,
        components: model
            .root
            .components
            .iter()
            .map(|instance| {
                unresolved_component(
                    instance,
                    failure.diagnostic.clone(),
                    SourceProvenance {
                        document_id: model.header.document_id,
                        system_id: model.root.system_id,
                        component_id: instance.id,
                    },
                )
            })
            .collect(),
        connections: model.root.connections.clone(),
    });
    ResolvedModel {
        document_id: model.header.document_id,
        root,
        simulation: model.simulation,
        probes: model.probes.clone(),
    }
}

/// Resolves every component in one source composition while preserving its boundary.
fn resolve_system(
    composition: &Composition,
    document_id: DocumentId,
    locks: &[DependencyLock],
    registry: &ComponentRegistry,
    loader: &impl CustomComponentLoader,
    stack: &mut Vec<DocumentId>,
    preserve_unresolved: bool,
) -> Result<ResolvedSystem, ResolutionFailure> {
    let mut components = Vec::with_capacity(composition.components.len());
    for instance in &composition.components {
        let provenance = SourceProvenance {
            document_id,
            system_id: composition.system_id,
            component_id: instance.id,
        };
        let resolution = match &instance.component {
            ComponentReference::BuiltIn { type_id } => registry
                .require(type_id, instance.id)
                .map(|definition| resolved_builtin(instance, definition, provenance))
                .map_err(|diagnostic| ResolutionFailure {
                    kind: ResolutionFailureKind::UnknownBuiltIn,
                    diagnostic,
                    dependency_path: Vec::new(),
                }),
            ComponentReference::Custom {
                document_id: expected_id,
                revision,
                source,
            } => resolve_custom(
                instance,
                *expected_id,
                revision,
                source,
                locks,
                registry,
                loader,
                stack,
                provenance,
                preserve_unresolved,
            ),
        };
        let resolved = match resolution {
            Ok(component) => component,
            Err(failure) if preserve_unresolved => {
                unresolved_component(instance, failure.diagnostic, provenance)
            }
            Err(failure) => return Err(failure),
        };
        components.push(resolved);
    }

    Ok(ResolvedSystem {
        id: composition.system_id,
        document_id,
        components,
        connections: composition.connections.clone(),
    })
}

/// Copies an installed built-in definition into an immutable resolved instance.
fn resolved_builtin(
    instance: &crate::document::ComponentInstance,
    definition: &ComponentDefinition,
    provenance: SourceProvenance,
) -> ResolvedComponent {
    ResolvedComponent {
        id: instance.id,
        name: instance.name.clone(),
        parameters: definition.parameters.clone(),
        ports: definition.ports.clone(),
        public_port_ids: BTreeMap::new(),
        capabilities: definition.capabilities.clone(),
        parameter_overrides: instance.parameter_overrides.clone(),
        enabled: instance.enabled,
        source: ResolvedComponentSource::BuiltIn {
            type_id: definition.type_id.clone(),
            version: definition.version,
        },
        provenance,
    }
}

#[allow(clippy::too_many_arguments)]
/// Loads, verifies, and recursively expands one custom-component instance.
fn resolve_custom(
    instance: &crate::document::ComponentInstance,
    expected_id: DocumentId,
    revision: &ArtifactRevision,
    source: &ShareableString,
    locks: &[DependencyLock],
    registry: &ComponentRegistry,
    loader: &impl CustomComponentLoader,
    stack: &mut Vec<DocumentId>,
    provenance: SourceProvenance,
    preserve_unresolved: bool,
) -> Result<ResolvedComponent, ResolutionFailure> {
    let Some(lock) = locks.iter().find(|lock| lock.document_id == expected_id) else {
        return Err(failure(
            ResolutionFailureKind::MissingLock,
            instance.id,
            "simulation_resolution_missing_lock",
        ));
    };
    if lock.source != *source {
        return Err(failure(
            ResolutionFailureKind::SourceMismatch,
            instance.id,
            "simulation_resolution_source_mismatch",
        ));
    }

    let artifact = loader
        .load(source.as_str())
        .map_err(|diagnostic| ResolutionFailure {
            kind: ResolutionFailureKind::LoadFailed,
            diagnostic,
            dependency_path: Vec::new(),
        })?;
    if artifact.document.header.document_id != expected_id || lock.document_id != expected_id {
        return Err(failure(
            ResolutionFailureKind::IdentityMismatch,
            instance.id,
            "simulation_resolution_identity_mismatch",
        ));
    }
    if artifact.document.revision != *revision || lock.revision != *revision {
        return Err(failure(
            ResolutionFailureKind::RevisionMismatch,
            instance.id,
            "simulation_resolution_revision_mismatch",
        ));
    }
    if artifact.checksum != lock.checksum {
        return Err(failure(
            ResolutionFailureKind::ChecksumMismatch,
            instance.id,
            "simulation_resolution_checksum_mismatch",
        ));
    }
    if let Some(cycle_start) = stack.iter().position(|id| *id == expected_id) {
        let mut dependency_path = stack.get(cycle_start..).unwrap_or_default().to_vec();
        dependency_path.push(expected_id);
        let mut result = failure(
            ResolutionFailureKind::DependencyCycle,
            instance.id,
            "simulation_resolution_dependency_cycle",
        );
        result.dependency_path = dependency_path;
        return Err(result);
    }

    stack.push(expected_id);
    let implementation = resolve_system(
        &artifact.document.implementation,
        expected_id,
        &artifact.document.dependencies,
        registry,
        loader,
        stack,
        preserve_unresolved,
    );
    stack.pop();
    let implementation = implementation?;
    let capabilities = custom_capabilities(
        &artifact.document.public_ports,
        &artifact.document.port_mappings,
        &implementation,
    );

    Ok(ResolvedComponent {
        id: instance.id,
        name: instance.name.clone(),
        parameters: artifact.document.public_parameters.clone(),
        ports: artifact
            .document
            .public_ports
            .iter()
            .map(|port| port.definition.clone())
            .collect(),
        public_port_ids: artifact
            .document
            .public_ports
            .iter()
            .map(|port| (port.id, port.definition.key.clone()))
            .collect(),
        capabilities,
        parameter_overrides: instance.parameter_overrides.clone(),
        enabled: instance.enabled,
        source: ResolvedComponentSource::Custom {
            document_id: expected_id,
            revision: revision.clone(),
            port_mappings: artifact.document.port_mappings.clone(),
            parameter_mappings: artifact.document.parameter_mappings.clone(),
            implementation: Box::new(implementation),
        },
        provenance,
    })
}

/// Creates a read-only placeholder from one unresolved source instance.
fn unresolved_component(
    instance: &crate::document::ComponentInstance,
    diagnostic: Diagnostic,
    provenance: SourceProvenance,
) -> ResolvedComponent {
    ResolvedComponent {
        id: instance.id,
        name: instance.name.clone(),
        parameters: vec![],
        ports: vec![],
        public_port_ids: BTreeMap::new(),
        capabilities: ComponentCapabilities::default(),
        parameter_overrides: instance.parameter_overrides.clone(),
        enabled: instance.enabled,
        source: ResolvedComponentSource::Unresolved {
            reference: instance.component.clone(),
            diagnostic,
        },
        provenance,
    }
}

/// Derives whether a custom component has any mapped input-to-output direct path.
fn custom_capabilities(
    public_ports: &[crate::document::PublicPortDefinition],
    mappings: &[PublicPortMapping],
    implementation: &ResolvedSystem,
) -> ComponentCapabilities {
    let public_directions: BTreeMap<PortId, PortDirection> = public_ports
        .iter()
        .map(|port| (port.id, port.definition.direction))
        .collect();
    let components: BTreeMap<ComponentId, &ResolvedComponent> = implementation
        .components
        .iter()
        .map(|component| (component.id, component))
        .collect();
    let mut reachable = BTreeSet::new();
    for mapping in mappings {
        if public_directions.get(&mapping.public_port_id) == Some(&PortDirection::Input)
            && components
                .get(&mapping.internal.component_id)
                .is_some_and(|component| {
                    component
                        .capabilities
                        .contains(ComponentCapability::DirectFeedthrough)
                })
        {
            reachable.insert(mapping.internal.component_id);
        }
    }

    loop {
        let previous_len = reachable.len();
        for connection in &implementation.connections {
            if reachable.contains(&connection.source.component_id)
                && components
                    .get(&connection.target.component_id)
                    .is_some_and(|component| {
                        component
                            .capabilities
                            .contains(ComponentCapability::DirectFeedthrough)
                    })
            {
                reachable.insert(connection.target.component_id);
            }
        }
        if reachable.len() == previous_len {
            break;
        }
    }

    let has_direct_path = mappings.iter().any(|mapping| {
        public_directions.get(&mapping.public_port_id) == Some(&PortDirection::Output)
            && reachable.contains(&mapping.internal.component_id)
    });
    if has_direct_path {
        ComponentCapabilities::new([ComponentCapability::DirectFeedthrough])
    } else {
        ComponentCapabilities::default()
    }
}

/// Creates a stable component-scoped resolution failure.
fn failure(
    kind: ResolutionFailureKind,
    component_id: ComponentId,
    message_key: &'static str,
) -> ResolutionFailure {
    ResolutionFailure {
        kind,
        diagnostic: Diagnostic::new(
            DiagnosticSeverity::Error,
            DiagnosticCategory::Resolution,
            Some(EntityReference::Component(component_id)),
            Some("component".into()),
            message_key,
        ),
        dependency_path: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CustomComponentLoader, LoadedCustomComponent, ResolutionFailureKind, ResolvedComponent,
        ResolvedComponentSource, ResolvedSystem, SourceProvenance, custom_capabilities,
        resolve_model, resolve_model_with_placeholders,
    };
    use crate::component::{
        ComponentCapabilities, ComponentCapability, ComponentDefinition, ComponentTypeId,
        PortDefinition, PortDirection, SemanticVersion,
    };
    use crate::diagnostic::{Diagnostic, DiagnosticCategory, DiagnosticSeverity};
    use crate::document::{
        ArtifactRevision, COMPONENT_SCHEMA_VERSION, CanvasPosition, ComponentInstance,
        ComponentReference, Composition, CustomComponentDocument, DependencyLock, DocumentHeader,
        LoggingPolicy, MODEL_SCHEMA_VERSION, ModelDocument, PortEndpoint, PublicPortDefinition,
        PublicPortMapping, SchemaVersion, SimulationSettings,
    };
    use crate::identity::{ComponentId, DocumentId, PortId, SystemId};
    use crate::parameter::ParameterValueType;
    use crate::registry::ComponentRegistry;
    use crate::timing::{FixedStepSemantics, InitialSamplePolicy, StateUpdatePolicy};
    use shareable_string::ShareableString;
    use std::collections::BTreeMap;

    struct MemoryLoader(BTreeMap<ShareableString, LoadedCustomComponent>);

    impl CustomComponentLoader for MemoryLoader {
        fn load(&self, source: &str) -> Result<LoadedCustomComponent, Diagnostic> {
            self.0.get(source).cloned().ok_or_else(|| {
                Diagnostic::new(
                    DiagnosticSeverity::Error,
                    DiagnosticCategory::Resolution,
                    None,
                    Some("source".into()),
                    "simulation_resolution_load_failed",
                )
            })
        }
    }

    fn header(document_id: u128, schema_version: SchemaVersion) -> DocumentHeader {
        DocumentHeader {
            schema_version,
            document_id: DocumentId::from_raw(document_id),
            title: "fixture".into(),
            description: "".into(),
            author: "test".into(),
            created_at: "2026-01-01T00:00:00Z".into(),
            updated_at: "2026-01-01T00:00:00Z".into(),
            migrations: vec![],
        }
    }

    fn instance(id: u128, document_id: u128, source: &str) -> ComponentInstance {
        ComponentInstance {
            id: ComponentId::from_raw(id),
            name: format!("component-{id}").into(),
            component: ComponentReference::Custom {
                document_id: DocumentId::from_raw(document_id),
                revision: ArtifactRevision("1.0.0".into()),
                source: source.into(),
            },
            parameter_overrides: BTreeMap::new(),
            enabled: true,
            position: CanvasPosition { x: 0.0, y: 0.0 },
        }
    }

    fn lock(document_id: u128, source: &str) -> DependencyLock {
        DependencyLock {
            document_id: DocumentId::from_raw(document_id),
            revision: ArtifactRevision("1.0.0".into()),
            checksum: format!("checksum-{document_id}").into(),
            source: source.into(),
        }
    }

    fn custom(
        document_id: u128,
        system_id: u128,
        components: Vec<ComponentInstance>,
        dependencies: Vec<DependencyLock>,
    ) -> CustomComponentDocument {
        CustomComponentDocument {
            header: header(document_id, COMPONENT_SCHEMA_VERSION),
            revision: ArtifactRevision("1.0.0".into()),
            public_parameters: vec![],
            public_ports: vec![],
            implementation: Composition {
                system_id: SystemId::from_raw(system_id),
                components,
                connections: vec![],
                annotations: BTreeMap::new(),
            },
            port_mappings: vec![],
            parameter_mappings: vec![],
            state: vec![],
            test_cases: vec![],
            dependencies,
            documentation: "".into(),
            replacement: None,
        }
    }

    fn model(component: ComponentInstance, dependencies: Vec<DependencyLock>) -> ModelDocument {
        ModelDocument {
            header: header(1, MODEL_SCHEMA_VERSION),
            root: Composition {
                system_id: SystemId::from_raw(10),
                components: vec![component],
                connections: vec![],
                annotations: BTreeMap::new(),
            },
            simulation: SimulationSettings {
                start_time: 0.0,
                stop_time: 1.0,
                timestep: 0.1,
                maximum_steps: 100,
                random_seed: 1,
                logging: LoggingPolicy::EveryStep,
                semantics: FixedStepSemantics {
                    initial_sample: InitialSamplePolicy::CaptureInitializedOutputs,
                    state_update: StateUpdatePolicy::OutputsBeforeStateCommit,
                },
            },
            probes: vec![],
            dependencies,
        }
    }

    fn registry() -> ComponentRegistry {
        let mut registry = ComponentRegistry::new();
        registry
            .register(ComponentDefinition {
                type_id: ComponentTypeId::new("signal.constant").unwrap(),
                version: SemanticVersion {
                    major: 1,
                    minor: 0,
                    patch: 0,
                },
                display_name: "Constant".into(),
                category: "Signal/Sources".into(),
                aliases: vec![],
                tags: vec![],
                documentation: "".into(),
                parameters: vec![],
                ports: vec![],
                capabilities: ComponentCapabilities::default(),
                deprecation: None,
            })
            .unwrap();
        registry
    }

    #[test]
    fn resolves_nested_custom_components_with_provenance() {
        let leaf = custom(3, 30, vec![], vec![]);
        let middle = custom(
            2,
            20,
            vec![instance(201, 3, "leaf.json")],
            vec![lock(3, "leaf.json")],
        );
        let loader = MemoryLoader(BTreeMap::from([
            (
                "middle.json".into(),
                LoadedCustomComponent {
                    document: middle,
                    checksum: "checksum-2".into(),
                },
            ),
            (
                "leaf.json".into(),
                LoadedCustomComponent {
                    document: leaf,
                    checksum: "checksum-3".into(),
                },
            ),
        ]));

        let resolved = resolve_model(
            &model(
                instance(101, 2, "middle.json"),
                vec![lock(2, "middle.json")],
            ),
            &registry(),
            &loader,
        )
        .unwrap();
        let ResolvedComponentSource::Custom { implementation, .. } =
            &resolved.root.components[0].source
        else {
            panic!("expected expanded custom component");
        };
        let ResolvedComponentSource::Custom {
            implementation: leaf,
            ..
        } = &implementation.components[0].source
        else {
            panic!("expected expanded nested component");
        };

        assert_eq!(implementation.document_id, DocumentId::from_raw(2));
        assert_eq!(leaf.document_id, DocumentId::from_raw(3));
        assert_eq!(
            implementation.components[0].provenance.document_id,
            DocumentId::from_raw(2)
        );
    }

    #[test]
    fn reports_complete_transitive_dependency_cycle_path() {
        let first = custom(
            2,
            20,
            vec![instance(201, 3, "second.json")],
            vec![lock(3, "second.json")],
        );
        let second = custom(
            3,
            30,
            vec![instance(301, 2, "first.json")],
            vec![lock(2, "first.json")],
        );
        let loader = MemoryLoader(BTreeMap::from([
            (
                "first.json".into(),
                LoadedCustomComponent {
                    document: first,
                    checksum: "checksum-2".into(),
                },
            ),
            (
                "second.json".into(),
                LoadedCustomComponent {
                    document: second,
                    checksum: "checksum-3".into(),
                },
            ),
        ]));

        let failure = resolve_model(
            &model(instance(101, 2, "first.json"), vec![lock(2, "first.json")]),
            &registry(),
            &loader,
        )
        .unwrap_err();

        assert_eq!(failure.kind, ResolutionFailureKind::DependencyCycle);
        assert_eq!(
            failure.dependency_path,
            vec![
                DocumentId::from_raw(2),
                DocumentId::from_raw(3),
                DocumentId::from_raw(2)
            ]
        );
    }

    #[test]
    fn permissive_resolution_preserves_unknown_builtin_placeholder() {
        let unknown_type = ComponentTypeId::new("missing.signal").unwrap();
        let instance = ComponentInstance {
            id: ComponentId::from_raw(77),
            name: "missing".into(),
            component: ComponentReference::BuiltIn {
                type_id: unknown_type.clone(),
            },
            parameter_overrides: BTreeMap::new(),
            enabled: true,
            position: CanvasPosition { x: 0.0, y: 0.0 },
        };
        let source = model(instance, vec![]);
        let resolved = resolve_model_with_placeholders(
            &source,
            &ComponentRegistry::new(),
            &MemoryLoader(BTreeMap::new()),
        );

        let ResolvedComponentSource::Unresolved {
            reference,
            diagnostic,
        } = &resolved.root.components[0].source
        else {
            panic!("expected unresolved placeholder");
        };
        assert_eq!(
            reference,
            &ComponentReference::BuiltIn {
                type_id: unknown_type,
            }
        );
        assert_eq!(
            diagnostic.message_key().as_str(),
            "simulation_registry_unknown_builtin"
        );
        assert_eq!(
            resolved.root.components[0].provenance.component_id,
            ComponentId::from_raw(77)
        );
        assert!(
            crate::validation::validate_model(
                &resolved,
                crate::validation::ValidationLimits::default()
            )
            .iter()
            .any(|value| value.message_key() == diagnostic.message_key())
        );
    }

    #[test]
    fn rejects_checksum_that_differs_from_dependency_lock() {
        let loader = MemoryLoader(BTreeMap::from([(
            "component.json".into(),
            LoadedCustomComponent {
                document: custom(2, 20, vec![], vec![]),
                checksum: "different-checksum".into(),
            },
        )]));

        let failure = resolve_model(
            &model(
                instance(101, 2, "component.json"),
                vec![lock(2, "component.json")],
            ),
            &registry(),
            &loader,
        )
        .unwrap_err();

        assert_eq!(failure.kind, ResolutionFailureKind::ChecksumMismatch);
        assert_eq!(
            failure.diagnostic.message_key().as_str(),
            "simulation_resolution_checksum_mismatch"
        );
    }

    #[test]
    fn derives_custom_feedthrough_across_mapped_private_paths() {
        let input_id = PortId::from_raw(11);
        let output_id = PortId::from_raw(12);
        let public_ports = vec![
            public_port(input_id, "in", PortDirection::Input),
            public_port(output_id, "out", PortDirection::Output),
        ];
        let mappings = vec![mapping(input_id, "in"), mapping(output_id, "out")];
        let mut implementation = ResolvedSystem {
            id: SystemId::from_raw(20),
            document_id: DocumentId::from_raw(2),
            components: vec![private_component(ComponentCapabilities::default())],
            connections: vec![],
        };

        assert!(
            !custom_capabilities(&public_ports, &mappings, &implementation)
                .contains(ComponentCapability::DirectFeedthrough)
        );

        implementation.components[0].capabilities =
            ComponentCapabilities::new([ComponentCapability::DirectFeedthrough]);
        assert!(
            custom_capabilities(&public_ports, &mappings, &implementation)
                .contains(ComponentCapability::DirectFeedthrough)
        );
    }

    fn public_port(id: PortId, key: &str, direction: PortDirection) -> PublicPortDefinition {
        PublicPortDefinition {
            id,
            definition: PortDefinition {
                key: key.into(),
                display_name: key.into(),
                description: "".into(),
                direction,
                value_type: ParameterValueType::Scalar,
                unit: None,
                required: direction == PortDirection::Input,
            },
        }
    }

    fn mapping(public_port_id: PortId, port_key: &str) -> PublicPortMapping {
        PublicPortMapping {
            public_port_id,
            internal: PortEndpoint {
                component_id: ComponentId::from_raw(5),
                port_key: port_key.into(),
            },
        }
    }

    fn private_component(capabilities: ComponentCapabilities) -> ResolvedComponent {
        let id = ComponentId::from_raw(5);
        ResolvedComponent {
            id,
            name: "private".into(),
            parameters: vec![],
            ports: vec![
                public_port(PortId::from_raw(51), "in", PortDirection::Input).definition,
                public_port(PortId::from_raw(52), "out", PortDirection::Output).definition,
            ],
            public_port_ids: BTreeMap::new(),
            capabilities,
            parameter_overrides: BTreeMap::new(),
            enabled: true,
            source: ResolvedComponentSource::BuiltIn {
                type_id: ComponentTypeId::new("test.private").unwrap(),
                version: SemanticVersion {
                    major: 1,
                    minor: 0,
                    patch: 0,
                },
            },
            provenance: SourceProvenance {
                document_id: DocumentId::from_raw(2),
                system_id: SystemId::from_raw(20),
                component_id: id,
            },
        }
    }
}
