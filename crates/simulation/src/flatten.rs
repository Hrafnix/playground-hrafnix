//! Deterministic adaptation from resolved hierarchy to an executable flat graph.

use crate::diagnostic::{Diagnostic, DiagnosticCategory, DiagnosticSeverity};
use crate::document::{Connection, PortEndpoint};
use crate::identity::{ComponentId, ConnectionId};
use crate::resolve::{ResolvedComponent, ResolvedComponentSource, ResolvedModel, ResolvedSystem};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Flattens all custom-component boundaries while retaining source provenance.
///
/// # Errors
///
/// Returns a stable diagnostic when a public mapping cannot be resolved or the
/// deterministic executable identity space is exhausted.
pub fn flatten_model(model: &ResolvedModel) -> Result<ResolvedModel, Diagnostic> {
    let mut builder = FlatBuilder::default();
    let root = builder.flatten_system(&model.root, "", true)?;
    let probes = model
        .probes
        .iter()
        .map(|probe| {
            let mut flattened = probe.clone();
            flattened.target = root.endpoint(&probe.target)?;
            Ok(flattened)
        })
        .collect::<Result<Vec<_>, Diagnostic>>()?;

    Ok(ResolvedModel {
        document_id: model.document_id,
        root: ResolvedSystem {
            id: model.root.id,
            document_id: model.root.document_id,
            components: builder.components,
            connections: builder.connections,
        },
        simulation: model.simulation,
        probes,
    })
}

/// Flattened endpoints addressable within one source system.
struct FlatSystem {
    /// Source component-port endpoints mapped to executable leaf endpoints.
    endpoints: BTreeMap<(ComponentId, ShareableString), PortEndpoint>,
}

impl FlatSystem {
    /// Resolves one source-system endpoint to its executable leaf endpoint.
    fn endpoint(&self, endpoint: &PortEndpoint) -> Result<PortEndpoint, Diagnostic> {
        self.endpoints
            .get(&(endpoint.component_id, endpoint.port_key.clone()))
            .cloned()
            .ok_or_else(flatten_mapping_diagnostic)
    }
}

/// Stateful deterministic traversal builder used only during runtime construction.
#[derive(Default)]
struct FlatBuilder {
    /// Next executable component identity.
    next_component_id: u128,
    /// Next executable connection identity.
    next_connection_id: u128,
    /// Flattened built-in leaves in source traversal order.
    components: Vec<ResolvedComponent>,
    /// Flattened and rewired connections in source traversal order.
    connections: Vec<Connection>,
}

impl FlatBuilder {
    /// Recursively appends one system and returns its addressable leaf endpoints.
    fn flatten_system(
        &mut self,
        system: &ResolvedSystem,
        parent_path: &str,
        ancestor_enabled: bool,
    ) -> Result<FlatSystem, Diagnostic> {
        let mut endpoints = BTreeMap::new();
        for component in &system.components {
            let path = scoped_name(parent_path, component.name.as_str());
            match &component.source {
                ResolvedComponentSource::BuiltIn { .. } => {
                    let executable_id = self.component_id()?;
                    let mut flattened = component.clone();
                    flattened.id = executable_id;
                    flattened.name = path.into();
                    flattened.enabled = ancestor_enabled && component.enabled;
                    for port in &component.ports {
                        endpoints.insert(
                            (component.id, port.key.clone()),
                            PortEndpoint {
                                component_id: executable_id,
                                port_key: port.key.clone(),
                            },
                        );
                    }
                    self.components.push(flattened);
                }
                ResolvedComponentSource::Custom {
                    port_mappings,
                    parameter_mappings,
                    implementation,
                    ..
                } => {
                    let mut configured_implementation = implementation.as_ref().clone();
                    apply_parameter_mappings(
                        component,
                        parameter_mappings,
                        &mut configured_implementation,
                    )?;
                    let nested = self.flatten_system(
                        &configured_implementation,
                        path.as_str(),
                        ancestor_enabled && component.enabled,
                    )?;
                    for mapping in port_mappings {
                        let public_key = component
                            .public_port_ids
                            .get(&mapping.public_port_id)
                            .ok_or_else(flatten_mapping_diagnostic)?;
                        endpoints.insert(
                            (component.id, public_key.clone()),
                            nested.endpoint(&mapping.internal)?,
                        );
                    }
                }
                ResolvedComponentSource::Unresolved { diagnostic, .. } => {
                    return Err(diagnostic.clone());
                }
            }
        }

        let flattened_system = FlatSystem { endpoints };
        for connection in &system.connections {
            let mut flattened = connection.clone();
            flattened.id = self.connection_id()?;
            flattened.source = flattened_system.endpoint(&connection.source)?;
            flattened.target = flattened_system.endpoint(&connection.target)?;
            self.connections.push(flattened);
        }
        Ok(flattened_system)
    }

    /// Allocates the next deterministic executable component identity.
    fn component_id(&mut self) -> Result<ComponentId, Diagnostic> {
        self.next_component_id = self
            .next_component_id
            .checked_add(1)
            .ok_or_else(flatten_identity_diagnostic)?;
        Ok(ComponentId::from_raw(self.next_component_id))
    }

    /// Allocates the next deterministic executable connection identity.
    fn connection_id(&mut self) -> Result<ConnectionId, Diagnostic> {
        self.next_connection_id = self
            .next_connection_id
            .checked_add(1)
            .ok_or_else(flatten_identity_diagnostic)?;
        Ok(ConnectionId::from_raw(self.next_connection_id))
    }
}

/// Applies one custom instance's public expressions to its immediate private graph.
fn apply_parameter_mappings(
    component: &ResolvedComponent,
    mappings: &[crate::document::PublicParameterMapping],
    implementation: &mut ResolvedSystem,
) -> Result<(), Diagnostic> {
    for mapping in mappings {
        let public = component
            .parameters
            .iter()
            .find(|parameter| parameter.key == mapping.public_parameter_key)
            .ok_or_else(flatten_parameter_mapping_diagnostic)?;
        let expression = component
            .parameter_overrides
            .get(&public.key)
            .unwrap_or(&public.default_expression)
            .clone();
        let internal = implementation
            .components
            .iter_mut()
            .find(|candidate| candidate.id == mapping.internal.component_id)
            .ok_or_else(flatten_parameter_mapping_diagnostic)?;
        if !internal
            .parameters
            .iter()
            .any(|parameter| parameter.key == mapping.internal.parameter_key)
        {
            return Err(flatten_parameter_mapping_diagnostic());
        }
        internal
            .parameter_overrides
            .insert(mapping.internal.parameter_key.clone(), expression);
    }
    Ok(())
}

/// Creates one deterministic scoped leaf name.
fn scoped_name(parent: &str, name: &str) -> String {
    if parent.is_empty() {
        name.to_owned()
    } else {
        format!("{parent}/{name}")
    }
}

/// Reports an invalid or incomplete public interface mapping.
fn flatten_mapping_diagnostic() -> Diagnostic {
    Diagnostic::new(
        DiagnosticSeverity::Error,
        DiagnosticCategory::Validation,
        None,
        Some("port_mappings".into()),
        "simulation_flatten_invalid_public_mapping",
    )
}

/// Reports an invalid public-to-private parameter mapping.
fn flatten_parameter_mapping_diagnostic() -> Diagnostic {
    Diagnostic::new(
        DiagnosticSeverity::Error,
        DiagnosticCategory::Validation,
        None,
        Some("parameter_mappings".into()),
        "simulation_flatten_invalid_parameter_mapping",
    )
}

/// Reports deterministic executable identity exhaustion.
fn flatten_identity_diagnostic() -> Diagnostic {
    Diagnostic::new(
        DiagnosticSeverity::Error,
        DiagnosticCategory::Runtime,
        None,
        Some("identity".into()),
        "simulation_flatten_identity_exhausted",
    )
}
