//! Headless execution of reusable custom-component test cases.

use crate::component::{ComponentTypeId, PortDirection};
use crate::diagnostic::{Diagnostic, DiagnosticCategory, DiagnosticSeverity};
use crate::document::{
    ArtifactRevision, CanvasPosition, ComponentInstance, ComponentReference, Composition,
    Connection, CustomComponentDocument, DependencyLock, DocumentHeader, MODEL_SCHEMA_VERSION,
    ModelDocument, PortEndpoint, ProbeDefinition,
};
use crate::identity::{ComponentId, ConnectionId, DocumentId, ProbeId, RunId, SystemId};
use crate::registry::ComponentRegistry;
use crate::resolve::{CustomComponentLoader, resolve_model};
use crate::results::{RunStatus, SimulationRun};
use crate::runtime::SimulationRuntime;
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Terminal outcome of one persisted custom-component test case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentTestOutcome {
    /// Exact timestamps and values matched every expectation.
    Passed,
    /// Resolution, runtime construction, execution, or comparison failed.
    Failed,
    /// The legacy descriptive case has no executable settings or expectations.
    Descriptive,
}

/// Result of executing one persisted custom-component test case.
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentTestResult {
    /// Stable case name from the source artifact.
    pub name: ShareableString,
    /// Terminal comparison outcome.
    pub outcome: ComponentTestOutcome,
    /// Completed or failed simulation run when execution started.
    pub run: Option<SimulationRun>,
    /// Test-harness diagnostics outside the simulation run.
    pub diagnostics: Vec<Diagnostic>,
}

/// Executes every test case declared by one custom-component artifact.
#[must_use]
pub fn run_custom_component_tests(
    document: &CustomComponentDocument,
    source: &str,
    checksum: &str,
    registry: &ComponentRegistry,
    loader: &impl CustomComponentLoader,
) -> Vec<ComponentTestResult> {
    document
        .test_cases
        .iter()
        .enumerate()
        .map(|(index, test_case)| {
            let Some(settings) = test_case.simulation else {
                return descriptive_result(test_case.name.clone());
            };
            if test_case.expected_outputs.is_empty() {
                return descriptive_result(test_case.name.clone());
            }
            let model = match test_model(document, source, checksum, test_case, settings, index) {
                Ok(model) => model,
                Err(diagnostic) => {
                    return failed_result(test_case.name.clone(), vec![diagnostic]);
                }
            };
            let resolved = match resolve_model(&model, registry, loader) {
                Ok(resolved) => resolved,
                Err(failure) => {
                    return failed_result(test_case.name.clone(), vec![failure.diagnostic]);
                }
            };
            let mut runtime = match SimulationRuntime::new(&resolved, registry) {
                Ok(runtime) => runtime,
                Err(failure) => {
                    return failed_result(test_case.name.clone(), failure.diagnostics);
                }
            };
            let Some(run_number) = u128::try_from(index)
                .ok()
                .and_then(|value| value.checked_add(1))
            else {
                return failed_result(
                    test_case.name.clone(),
                    vec![test_diagnostic("simulation_custom_test_identity_exhausted")],
                );
            };
            let run = runtime.run(RunId::from_raw(run_number));
            let matches = run.status == RunStatus::Completed
                && run.series.len() == test_case.expected_outputs.len()
                && run
                    .series
                    .iter()
                    .zip(&test_case.expected_outputs)
                    .all(|(actual, expected)| {
                        actual.timestamps == expected.timestamps && actual.values == expected.values
                    });
            if matches {
                ComponentTestResult {
                    name: test_case.name.clone(),
                    outcome: ComponentTestOutcome::Passed,
                    run: Some(run),
                    diagnostics: vec![],
                }
            } else {
                ComponentTestResult {
                    name: test_case.name.clone(),
                    outcome: ComponentTestOutcome::Failed,
                    run: Some(run),
                    diagnostics: vec![test_diagnostic("simulation_custom_test_output_mismatch")],
                }
            }
        })
        .collect()
}

/// Builds a temporary model that drives public inputs from constant sources.
fn test_model(
    document: &CustomComponentDocument,
    source: &str,
    checksum: &str,
    test_case: &crate::document::ComponentTestCase,
    settings: crate::document::SimulationSettings,
    index: usize,
) -> Result<ModelDocument, Diagnostic> {
    let constant_type = ComponentTypeId::new("signal.constant")
        .map_err(|_| test_diagnostic("simulation_custom_test_invalid_builtin_id"))?;
    let index = u128::try_from(index)
        .map_err(|_| test_diagnostic("simulation_custom_test_identity_exhausted"))?;
    let custom_id = ComponentId::from_raw(1);
    let mut components = vec![ComponentInstance {
        id: custom_id,
        name: "component_under_test".into(),
        component: ComponentReference::Custom {
            document_id: document.header.document_id,
            revision: document.revision.clone(),
            source: source.into(),
        },
        parameter_overrides: test_case.parameter_overrides.clone(),
        enabled: true,
        position: CanvasPosition { x: 0.0, y: 0.0 },
    }];
    let mut connections = Vec::new();
    let mut next_component = 2_u128;
    let mut next_connection = 1_u128;
    for port in document
        .public_ports
        .iter()
        .filter(|port| port.definition.direction == PortDirection::Input)
    {
        let Some(expression) = test_case.inputs.get(&port.definition.key) else {
            continue;
        };
        let source_id = ComponentId::from_raw(next_component);
        next_component = next_component.saturating_add(1);
        components.push(ComponentInstance {
            id: source_id,
            name: format!("input_{}", port.definition.key).into(),
            component: ComponentReference::BuiltIn {
                type_id: constant_type.clone(),
            },
            parameter_overrides: BTreeMap::from([("value".into(), expression.clone())]),
            enabled: true,
            position: CanvasPosition { x: 0.0, y: 0.0 },
        });
        connections.push(Connection {
            id: ConnectionId::from_raw(next_connection),
            source: PortEndpoint {
                component_id: source_id,
                port_key: "out".into(),
            },
            target: PortEndpoint {
                component_id: custom_id,
                port_key: port.definition.key.clone(),
            },
            label: None,
            route: vec![],
        });
        next_connection = next_connection.saturating_add(1);
    }
    let probes = test_case
        .expected_outputs
        .iter()
        .enumerate()
        .map(|(probe_index, expected)| {
            u128::try_from(probe_index)
                .ok()
                .and_then(|value| value.checked_add(1))
                .map(|probe_id| ProbeDefinition {
                    id: ProbeId::from_raw(probe_id),
                    target: PortEndpoint {
                        component_id: custom_id,
                        port_key: expected.port_key.clone(),
                    },
                    display_name: expected.port_key.clone(),
                    plot_group: None,
                })
        })
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| test_diagnostic("simulation_custom_test_identity_exhausted"))?;
    Ok(ModelDocument {
        header: DocumentHeader {
            schema_version: MODEL_SCHEMA_VERSION,
            document_id: DocumentId::from_raw(u128::MAX.saturating_sub(index)),
            title: format!("{} test", document.header.title).into(),
            description: "Generated custom-component test model".into(),
            author: "simulation".into(),
            created_at: "".into(),
            updated_at: "".into(),
            migrations: vec![],
        },
        root: Composition {
            system_id: SystemId::from_raw(1),
            components,
            connections,
            annotations: BTreeMap::new(),
        },
        simulation: settings,
        probes,
        dependencies: vec![DependencyLock {
            document_id: document.header.document_id,
            revision: ArtifactRevision(document.revision.0.clone()),
            checksum: checksum.into(),
            source: source.into(),
        }],
    })
}

/// Creates a result for a legacy human-readable test case.
const fn descriptive_result(name: ShareableString) -> ComponentTestResult {
    ComponentTestResult {
        name,
        outcome: ComponentTestOutcome::Descriptive,
        run: None,
        diagnostics: vec![],
    }
}

/// Creates a failed result before simulation execution.
const fn failed_result(name: ShareableString, diagnostics: Vec<Diagnostic>) -> ComponentTestResult {
    ComponentTestResult {
        name,
        outcome: ComponentTestOutcome::Failed,
        run: None,
        diagnostics,
    }
}

/// Creates a stable custom-test harness diagnostic.
fn test_diagnostic(message_key: &'static str) -> Diagnostic {
    Diagnostic::new(
        DiagnosticSeverity::Error,
        DiagnosticCategory::Runtime,
        None,
        Some("test_case".into()),
        message_key,
    )
}
