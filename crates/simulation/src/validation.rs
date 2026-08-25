use crate::component::{PortDefinition, PortDirection};
use crate::diagnostic::{Diagnostic, DiagnosticCategory, DiagnosticSeverity, EntityReference};
use crate::document::{PortEndpoint, PublicPortMapping};
use crate::identity::{ComponentId, ConnectionId};
use crate::parameter::ParameterValueType;
use crate::resolve::{ResolvedComponent, ResolvedComponentSource, ResolvedModel, ResolvedSystem};
use crate::timing::FixedStepPlan;
use std::collections::{BTreeMap, BTreeSet};

/// Resource limits enforced before runtime construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationLimits {
    /// Maximum components across all expanded systems.
    pub maximum_components: usize,
    /// Maximum connections across all expanded systems.
    pub maximum_connections: usize,
    /// Maximum model probes.
    pub maximum_probes: usize,
}

impl Default for ValidationLimits {
    fn default() -> Self {
        Self {
            maximum_components: 100_000,
            maximum_connections: 200_000,
            maximum_probes: 100_000,
        }
    }
}

/// Validates a fully resolved model and returns all deterministic diagnostics.
#[must_use]
pub fn validate_model(model: &ResolvedModel, limits: ValidationLimits) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut counts = GraphCounts::default();
    validate_system(&model.root, &mut counts, &mut diagnostics);

    if counts.components > limits.maximum_components {
        diagnostics.push(model_diagnostic(
            model,
            "components",
            "simulation_validation_component_limit",
        ));
    }
    if counts.connections > limits.maximum_connections {
        diagnostics.push(model_diagnostic(
            model,
            "connections",
            "simulation_validation_connection_limit",
        ));
    }
    if model.probes.len() > limits.maximum_probes {
        diagnostics.push(model_diagnostic(
            model,
            "probes",
            "simulation_validation_probe_limit",
        ));
    }

    match FixedStepPlan::new(
        model.simulation.start_time,
        model.simulation.stop_time,
        model.simulation.timestep,
    ) {
        Ok(plan) if plan.step_count() <= model.simulation.maximum_steps => {}
        Ok(_) => diagnostics.push(model_diagnostic(
            model,
            "maximum_steps",
            "simulation_validation_step_limit",
        )),
        Err(_) => diagnostics.push(model_diagnostic(
            model,
            "simulation",
            "simulation_validation_invalid_timing",
        )),
    }

    let root_components = component_index(&model.root);
    for probe in &model.probes {
        if endpoint_port(&root_components, &probe.target).is_none() {
            diagnostics.push(Diagnostic::new(
                DiagnosticSeverity::Error,
                DiagnosticCategory::Validation,
                Some(EntityReference::Probe(probe.id)),
                Some(probe.target.port_key.clone()),
                "simulation_validation_dangling_probe",
            ));
        }
    }

    diagnostics
}

/// Saturating expanded-graph entity counts.
#[derive(Default)]
struct GraphCounts {
    /// Number of expanded component instances.
    components: usize,
    /// Number of expanded connections.
    connections: usize,
}

/// Validates one system and recursively visits private custom implementations.
fn validate_system(
    system: &ResolvedSystem,
    counts: &mut GraphCounts,
    diagnostics: &mut Vec<Diagnostic>,
) {
    counts.components = counts.components.saturating_add(system.components.len());
    counts.connections = counts.connections.saturating_add(system.connections.len());

    let mut component_ids = BTreeSet::new();
    let mut component_names = BTreeSet::new();
    for component in &system.components {
        if !component_ids.insert(component.id) {
            diagnostics.push(component_diagnostic(
                component.id,
                "id",
                "simulation_validation_duplicate_component_id",
            ));
        }
        if !component_names.insert(component.name.clone()) {
            diagnostics.push(component_diagnostic(
                component.id,
                "name",
                "simulation_validation_duplicate_component_name",
            ));
        }
        validate_component_interface(component, diagnostics);
        if let ResolvedComponentSource::Custom {
            port_mappings,
            implementation,
            ..
        } = &component.source
        {
            validate_public_mappings(component, port_mappings, implementation, diagnostics);
            validate_system(implementation, counts, diagnostics);
        }
    }

    let components = component_index(system);
    let mut connection_ids = BTreeSet::new();
    let mut connected_inputs = BTreeMap::<(ComponentId, &str), ConnectionId>::new();
    for connection in &system.connections {
        if !connection_ids.insert(connection.id) {
            diagnostics.push(connection_diagnostic(
                connection.id,
                "id",
                "simulation_validation_duplicate_connection_id",
            ));
        }
        let source_port = endpoint_port(&components, &connection.source);
        let target_port = endpoint_port(&components, &connection.target);
        if source_port.is_none() || target_port.is_none() {
            diagnostics.push(connection_diagnostic(
                connection.id,
                "endpoint",
                "simulation_validation_dangling_connection",
            ));
            continue;
        }
        let (Some(source_port), Some(target_port)) = (source_port, target_port) else {
            continue;
        };
        if source_port.direction != PortDirection::Output
            || target_port.direction != PortDirection::Input
        {
            diagnostics.push(connection_diagnostic(
                connection.id,
                "direction",
                "simulation_validation_port_direction",
            ));
        }
        if !types_compatible(&source_port.value_type, &target_port.value_type) {
            diagnostics.push(connection_diagnostic(
                connection.id,
                "value_type",
                "simulation_validation_port_type",
            ));
        }
        if !units_compatible(source_port, target_port) {
            diagnostics.push(connection_diagnostic(
                connection.id,
                "unit",
                "simulation_validation_port_unit",
            ));
        }
        let input_key = (
            connection.target.component_id,
            connection.target.port_key.as_str(),
        );
        if connected_inputs.insert(input_key, connection.id).is_some() {
            diagnostics.push(connection_diagnostic(
                connection.id,
                "target",
                "simulation_validation_input_cardinality",
            ));
        }
    }

    for component in &system.components {
        for port in &component.ports {
            if port.direction == PortDirection::Input
                && port.required
                && !connected_inputs.contains_key(&(component.id, port.key.as_str()))
            {
                diagnostics.push(component_diagnostic(
                    component.id,
                    port.key.as_str(),
                    "simulation_validation_required_input",
                ));
            }
        }
    }
}

/// Validates unique interface keys and instance parameter overrides.
fn validate_component_interface(component: &ResolvedComponent, diagnostics: &mut Vec<Diagnostic>) {
    validate_unique_keys(
        component.id,
        &component.parameters,
        |parameter| parameter.key.as_str(),
        "simulation_validation_duplicate_parameter_key",
        diagnostics,
    );
    validate_unique_keys(
        component.id,
        &component.ports,
        |port| port.key.as_str(),
        "simulation_validation_duplicate_port_key",
        diagnostics,
    );

    let parameter_keys: BTreeSet<&str> = component
        .parameters
        .iter()
        .map(|parameter| parameter.key.as_str())
        .collect();
    for key in component.parameter_overrides.keys() {
        if !parameter_keys.contains(key.as_str()) {
            diagnostics.push(component_diagnostic(
                component.id,
                key.as_str(),
                "simulation_validation_unknown_parameter",
            ));
        }
    }
}

/// Reports duplicate stable keys in one interface collection.
fn validate_unique_keys<T>(
    component_id: ComponentId,
    values: &[T],
    key: impl Fn(&T) -> &str,
    message_key: &'static str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut keys = BTreeSet::new();
    for value in values {
        let value_key = key(value);
        if !keys.insert(value_key) {
            diagnostics.push(component_diagnostic(component_id, value_key, message_key));
        }
    }
}

/// Checks that custom public-port mappings are unique and target private ports.
fn validate_public_mappings(
    component: &ResolvedComponent,
    mappings: &[PublicPortMapping],
    implementation: &ResolvedSystem,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let components = component_index(implementation);
    let mut mapped_public_ports = BTreeSet::new();
    for mapping in mappings {
        let public_port = component
            .public_port_ids
            .get(&mapping.public_port_id)
            .and_then(|key| component.ports.iter().find(|port| port.key == *key));
        let internal_port = endpoint_port(&components, &mapping.internal);
        let compatible = public_port
            .zip(internal_port)
            .is_some_and(|(public, internal)| {
                public.direction == internal.direction
                    && types_compatible(&public.value_type, &internal.value_type)
                    && units_compatible(public, internal)
            });
        if !compatible || !mapped_public_ports.insert(mapping.public_port_id) {
            diagnostics.push(component_diagnostic(
                component.id,
                "port_mappings",
                "simulation_validation_invalid_public_port_mapping",
            ));
        }
    }
    for public_port_id in component.public_port_ids.keys() {
        if !mapped_public_ports.contains(public_port_id) {
            diagnostics.push(component_diagnostic(
                component.id,
                "port_mappings",
                "simulation_validation_missing_public_port_mapping",
            ));
        }
    }
}

/// Indexes components by stable identity within one system.
fn component_index(system: &ResolvedSystem) -> BTreeMap<ComponentId, &ResolvedComponent> {
    system
        .components
        .iter()
        .map(|component| (component.id, component))
        .collect()
}

/// Resolves one endpoint against a system-local component index.
fn endpoint_port<'a>(
    components: &BTreeMap<ComponentId, &'a ResolvedComponent>,
    endpoint: &PortEndpoint,
) -> Option<&'a PortDefinition> {
    components
        .get(&endpoint.component_id)?
        .ports
        .iter()
        .find(|port| port.key == endpoint.port_key)
}

/// Checks runtime value-shape compatibility between connected ports.
fn types_compatible(source: &ParameterValueType, target: &ParameterValueType) -> bool {
    source == target || scalar_type(source) && scalar_type(target)
}

/// Returns whether a value shape is a scalar, with or without unit metadata.
const fn scalar_type(value_type: &ParameterValueType) -> bool {
    matches!(
        value_type,
        ParameterValueType::Scalar | ParameterValueType::ScalarWithUnit(_)
    )
}

/// Checks canonical unit-family compatibility between connected ports.
fn units_compatible(source: &PortDefinition, target: &PortDefinition) -> bool {
    match (source.unit, target.unit) {
        (None, None) => true,
        (Some(source), Some(target)) => source.family_id() == target.family_id(),
        _ => false,
    }
}

/// Creates a stable component-scoped validation diagnostic.
fn component_diagnostic(
    component_id: ComponentId,
    field: &str,
    message_key: &'static str,
) -> Diagnostic {
    Diagnostic::new(
        DiagnosticSeverity::Error,
        DiagnosticCategory::Validation,
        Some(EntityReference::Component(component_id)),
        Some(field.into()),
        message_key,
    )
}

/// Creates a stable connection-scoped validation diagnostic.
fn connection_diagnostic(
    connection_id: ConnectionId,
    field: &str,
    message_key: &'static str,
) -> Diagnostic {
    Diagnostic::new(
        DiagnosticSeverity::Error,
        DiagnosticCategory::Validation,
        Some(EntityReference::Connection(connection_id)),
        Some(field.into()),
        message_key,
    )
}

/// Creates a stable model-scoped validation diagnostic.
fn model_diagnostic(model: &ResolvedModel, field: &str, message_key: &'static str) -> Diagnostic {
    Diagnostic::new(
        DiagnosticSeverity::Error,
        DiagnosticCategory::Validation,
        Some(EntityReference::Document(model.document_id)),
        Some(field.into()),
        message_key,
    )
}

#[cfg(test)]
mod tests {
    use super::{ValidationLimits, validate_model};
    use crate::component::{ComponentCapabilities, PortDefinition, PortDirection};
    use crate::document::{Connection, PortEndpoint, SimulationSettings};
    use crate::identity::{ComponentId, ConnectionId, DocumentId, SystemId};
    use crate::parameter::ParameterValueType;
    use crate::resolve::{
        ResolvedComponent, ResolvedComponentSource, ResolvedModel, ResolvedSystem, SourceProvenance,
    };
    use crate::timing::FixedStepSemantics;
    use std::collections::BTreeMap;

    fn component(id: u128, ports: Vec<PortDefinition>) -> ResolvedComponent {
        let id = ComponentId::from_raw(id);
        ResolvedComponent {
            id,
            name: format!("component-{id}").into(),
            parameters: vec![],
            ports,
            public_port_ids: BTreeMap::new(),
            capabilities: ComponentCapabilities::default(),
            parameter_overrides: BTreeMap::new(),
            enabled: true,
            source: ResolvedComponentSource::BuiltIn {
                type_id: crate::component::ComponentTypeId::new("test.component").unwrap(),
                version: crate::component::SemanticVersion {
                    major: 1,
                    minor: 0,
                    patch: 0,
                },
            },
            provenance: SourceProvenance {
                document_id: DocumentId::from_raw(1),
                system_id: SystemId::from_raw(10),
                component_id: id,
            },
        }
    }

    fn port(key: &str, direction: PortDirection, required: bool) -> PortDefinition {
        PortDefinition {
            key: key.into(),
            display_name: key.into(),
            description: "".into(),
            direction,
            value_type: ParameterValueType::Scalar,
            unit: None,
            required,
        }
    }

    fn model(connections: Vec<Connection>) -> ResolvedModel {
        ResolvedModel {
            document_id: DocumentId::from_raw(1),
            root: ResolvedSystem {
                id: SystemId::from_raw(10),
                document_id: DocumentId::from_raw(1),
                components: vec![
                    component(1, vec![port("out", PortDirection::Output, false)]),
                    component(2, vec![port("in", PortDirection::Input, true)]),
                    component(3, vec![port("in", PortDirection::Input, true)]),
                ],
                connections,
            },
            simulation: SimulationSettings {
                start_time: 0.0,
                stop_time: 1.0,
                timestep: 0.1,
                maximum_steps: 10,
                random_seed: 1,
                logging: crate::document::LoggingPolicy::EveryStep,
                semantics: FixedStepSemantics::default(),
            },
            probes: vec![],
        }
    }

    fn connection(id: u128, target_component: u128, target_port: &str) -> Connection {
        Connection {
            id: ConnectionId::from_raw(id),
            source: PortEndpoint {
                component_id: ComponentId::from_raw(1),
                port_key: "out".into(),
            },
            target: PortEndpoint {
                component_id: ComponentId::from_raw(target_component),
                port_key: target_port.into(),
            },
            label: None,
            route: vec![],
        }
    }

    #[test]
    fn permits_output_fan_out_to_distinct_inputs() {
        let diagnostics = validate_model(
            &model(vec![connection(1, 2, "in"), connection(2, 3, "in")]),
            ValidationLimits::default(),
        );

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn reports_dangling_link_and_unconnected_required_inputs() {
        let diagnostics = validate_model(
            &model(vec![connection(1, 99, "missing")]),
            ValidationLimits::default(),
        );
        let keys: Vec<&str> = diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message_key().as_str())
            .collect();

        assert!(keys.contains(&"simulation_validation_dangling_connection"));
        assert_eq!(
            keys.iter()
                .filter(|key| **key == "simulation_validation_required_input")
                .count(),
            2
        );
    }
}
