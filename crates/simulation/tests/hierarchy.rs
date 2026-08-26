//! End-to-end hierarchy, scoping, and reproducibility fixtures.

use shareable_string::ShareableString;
use simulation::builtins::register_signal_builtins;
use simulation::component::{ComponentTypeId, ParameterDefinition, PortDefinition, PortDirection};
use simulation::custom_tests::{ComponentTestOutcome, run_custom_component_tests};
use simulation::diagnostic::{Diagnostic, DiagnosticCategory, DiagnosticSeverity};
use simulation::document::{
    ArtifactRevision, COMPONENT_SCHEMA_VERSION, CanvasPosition, ComponentInstance,
    ComponentReference, ComponentTestCase, Composition, Connection, CustomComponentDocument,
    DependencyLock, DocumentHeader, ExpectedOutputSeries, LoggingPolicy, MODEL_SCHEMA_VERSION,
    ModelDocument, ParameterEndpoint, PortEndpoint, ProbeDefinition, PublicParameterMapping,
    PublicPortDefinition, PublicPortMapping, SimulationSettings,
};
use simulation::identity::{
    ComponentId, ConnectionId, DocumentId, PortId, ProbeId, RunId, SystemId,
};
use simulation::parameter::ParameterValueType;
use simulation::registry::ComponentRegistry;
use simulation::resolve::{CustomComponentLoader, LoadedCustomComponent, resolve_model};
use simulation::results::RunStatus;
use simulation::runtime::SimulationRuntime;
use simulation::timing::FixedStepSemantics;
use simulation::value::RuntimeValue;
use std::collections::BTreeMap;

const COMPONENT_SOURCE: &str = "components/gain-wrapper.json";
const OUTER_SOURCE: &str = "components/outer-wrapper.json";

struct MemoryLoader {
    components: BTreeMap<ShareableString, LoadedCustomComponent>,
}

impl CustomComponentLoader for MemoryLoader {
    fn load(&self, source: &str) -> Result<LoadedCustomComponent, Diagnostic> {
        self.components.get(source).cloned().ok_or_else(|| {
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

fn header(document_id: u128, component: bool) -> DocumentHeader {
    DocumentHeader {
        schema_version: if component {
            COMPONENT_SCHEMA_VERSION
        } else {
            MODEL_SCHEMA_VERSION
        },
        document_id: DocumentId::from_raw(document_id),
        title: "Hierarchy fixture".into(),
        description: "".into(),
        author: "tests".into(),
        created_at: "2026-08-24T00:00:00Z".into(),
        updated_at: "2026-08-24T00:00:00Z".into(),
        migrations: vec![],
    }
}

fn scalar_port(key: &str, direction: PortDirection, required: bool) -> PortDefinition {
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

fn builtin(
    id: u128,
    type_id: &str,
    overrides: &[(&str, &str)],
) -> Result<ComponentInstance, String> {
    Ok(ComponentInstance {
        id: ComponentId::from_raw(id),
        name: format!("component-{id}").into(),
        component: ComponentReference::BuiltIn {
            type_id: ComponentTypeId::new(type_id).map_err(|error| format!("{error:?}"))?,
            version: None,
        },
        parameter_overrides: overrides
            .iter()
            .map(|(key, value)| ((*key).into(), (*value).into()))
            .collect(),
        enabled: true,
        position: CanvasPosition { x: 0.0, y: 0.0 },
    })
}

fn connection(id: u128, source: u128, target: u128) -> Connection {
    Connection {
        id: ConnectionId::from_raw(id),
        source: PortEndpoint {
            component_id: ComponentId::from_raw(source),
            port_key: "out".into(),
        },
        target: PortEndpoint {
            component_id: ComponentId::from_raw(target),
            port_key: "in".into(),
        },
        label: None,
        route: vec![],
    }
}

fn gain_wrapper() -> Result<CustomComponentDocument, String> {
    let input_id = PortId::from_raw(1);
    let output_id = PortId::from_raw(2);
    Ok(CustomComponentDocument {
        header: header(50, true),
        revision: ArtifactRevision("1.0.0".into()),
        appearance: Default::default(),
        public_parameters: vec![ParameterDefinition {
            key: "gain".into(),
            display_name: "Gain".into(),
            description: "".into(),
            value_type: ParameterValueType::Scalar,
            default_expression: "1.0".into(),
        }],
        public_ports: vec![
            PublicPortDefinition {
                id: input_id,
                definition: scalar_port("in", PortDirection::Input, true),
            },
            PublicPortDefinition {
                id: output_id,
                definition: scalar_port("out", PortDirection::Output, false),
            },
        ],
        implementation: Composition {
            system_id: SystemId::from_raw(1),
            components: vec![builtin(1, "signal.gain", &[])?],
            connections: vec![],
            annotations: BTreeMap::new(),
        },
        port_mappings: vec![
            PublicPortMapping {
                public_port_id: input_id,
                internal: PortEndpoint {
                    component_id: ComponentId::from_raw(1),
                    port_key: "in".into(),
                },
            },
            PublicPortMapping {
                public_port_id: output_id,
                internal: PortEndpoint {
                    component_id: ComponentId::from_raw(1),
                    port_key: "out".into(),
                },
            },
        ],
        parameter_mappings: vec![PublicParameterMapping {
            public_parameter_key: "gain".into(),
            internal: ParameterEndpoint {
                component_id: ComponentId::from_raw(1),
                parameter_key: "gain".into(),
            },
        }],
        state: vec![],
        test_cases: vec![ComponentTestCase {
            name: "constant gain".into(),
            parameter_overrides: BTreeMap::from([("gain".into(), "3.0".into())]),
            simulation: Some(settings()),
            inputs: BTreeMap::from([("in".into(), "2.0".into())]),
            expected_outputs: vec![ExpectedOutputSeries {
                port_key: "out".into(),
                timestamps: vec![0.0, 0.25, 0.5],
                values: vec![RuntimeValue::Scalar(6.0); 3],
            }],
            expected_behavior: "Output is input multiplied by gain".into(),
        }],
        dependencies: vec![],
        documentation: "Mapped gain wrapper".into(),
        replacement: None,
    })
}

fn outer_wrapper(inner: &CustomComponentDocument) -> CustomComponentDocument {
    let input_id = PortId::from_raw(11);
    let output_id = PortId::from_raw(12);
    CustomComponentDocument {
        header: header(60, true),
        revision: ArtifactRevision("1.0.0".into()),
        appearance: Default::default(),
        public_parameters: inner.public_parameters.clone(),
        public_ports: vec![
            PublicPortDefinition {
                id: input_id,
                definition: scalar_port("in", PortDirection::Input, true),
            },
            PublicPortDefinition {
                id: output_id,
                definition: scalar_port("out", PortDirection::Output, false),
            },
        ],
        implementation: Composition {
            system_id: SystemId::from_raw(1),
            components: vec![ComponentInstance {
                id: ComponentId::from_raw(1),
                name: "inner".into(),
                component: ComponentReference::Custom {
                    document_id: inner.header.document_id,
                    revision: inner.revision.clone(),
                    source: COMPONENT_SOURCE.into(),
                },
                parameter_overrides: BTreeMap::new(),
                enabled: true,
                position: CanvasPosition { x: 0.0, y: 0.0 },
            }],
            connections: vec![],
            annotations: BTreeMap::new(),
        },
        port_mappings: vec![
            PublicPortMapping {
                public_port_id: input_id,
                internal: PortEndpoint {
                    component_id: ComponentId::from_raw(1),
                    port_key: "in".into(),
                },
            },
            PublicPortMapping {
                public_port_id: output_id,
                internal: PortEndpoint {
                    component_id: ComponentId::from_raw(1),
                    port_key: "out".into(),
                },
            },
        ],
        parameter_mappings: vec![PublicParameterMapping {
            public_parameter_key: "gain".into(),
            internal: ParameterEndpoint {
                component_id: ComponentId::from_raw(1),
                parameter_key: "gain".into(),
            },
        }],
        state: vec![],
        test_cases: inner.test_cases.clone(),
        dependencies: vec![DependencyLock {
            document_id: inner.header.document_id,
            revision: inner.revision.clone(),
            checksum: "gain-wrapper-checksum".into(),
            source: COMPONENT_SOURCE.into(),
        }],
        documentation: "Two-level mapped gain wrapper".into(),
        replacement: None,
    }
}

fn settings() -> SimulationSettings {
    SimulationSettings {
        start_time: 0.0,
        stop_time: 0.5,
        timestep: 0.25,
        maximum_steps: 2,
        random_seed: 17,
        logging: LoggingPolicy::EveryStep,
        semantics: FixedStepSemantics::default(),
    }
}

fn probe(component_id: u128) -> ProbeDefinition {
    ProbeDefinition {
        id: ProbeId::from_raw(20),
        target: PortEndpoint {
            component_id: ComponentId::from_raw(component_id),
            port_key: "out".into(),
        },
        display_name: "output".into(),
        plot_group: None,
    }
}

fn nested_model() -> Result<ModelDocument, String> {
    let custom_id = ComponentId::from_raw(2);
    Ok(ModelDocument {
        header: header(1, false),
        root: Composition {
            system_id: SystemId::from_raw(10),
            components: vec![
                builtin(1, "signal.constant", &[("value", "2.0")])?,
                ComponentInstance {
                    id: custom_id,
                    name: "wrapper".into(),
                    component: ComponentReference::Custom {
                        document_id: DocumentId::from_raw(60),
                        revision: ArtifactRevision("1.0.0".into()),
                        source: OUTER_SOURCE.into(),
                    },
                    parameter_overrides: BTreeMap::from([("gain".into(), "3.0".into())]),
                    enabled: true,
                    position: CanvasPosition { x: 0.0, y: 0.0 },
                },
            ],
            connections: vec![connection(1, 1, 2)],
            annotations: BTreeMap::new(),
        },
        simulation: settings(),
        probes: vec![probe(2)],
        dependencies: vec![DependencyLock {
            document_id: DocumentId::from_raw(60),
            revision: ArtifactRevision("1.0.0".into()),
            checksum: "outer-wrapper-checksum".into(),
            source: OUTER_SOURCE.into(),
        }],
    })
}

fn flat_model() -> Result<ModelDocument, String> {
    Ok(ModelDocument {
        header: header(2, false),
        root: Composition {
            system_id: SystemId::from_raw(20),
            components: vec![
                builtin(1, "signal.constant", &[("value", "2.0")])?,
                builtin(2, "signal.gain", &[("gain", "3.0")])?,
            ],
            connections: vec![connection(1, 1, 2)],
            annotations: BTreeMap::new(),
        },
        simulation: settings(),
        probes: vec![probe(2)],
        dependencies: vec![],
    })
}

fn run(
    model: &ModelDocument,
    registry: &ComponentRegistry,
    loader: &MemoryLoader,
    run_id: u128,
) -> Result<simulation::results::SimulationRun, String> {
    let resolved = resolve_model(model, registry, loader).map_err(|error| format!("{error:?}"))?;
    let mut runtime =
        SimulationRuntime::new(&resolved, registry).map_err(|error| format!("{error:?}"))?;
    Ok(runtime.run(RunId::from_raw(run_id)))
}

fn verify_nested_and_flattened_models() -> Result<(), String> {
    let mut registry = ComponentRegistry::new();
    register_signal_builtins(&mut registry).map_err(|error| format!("{error:?}"))?;
    let inner = gain_wrapper()?;
    let outer = outer_wrapper(&inner);
    let loader = MemoryLoader {
        components: BTreeMap::from([
            (
                COMPONENT_SOURCE.into(),
                LoadedCustomComponent {
                    document: inner,
                    checksum: "gain-wrapper-checksum".into(),
                },
            ),
            (
                OUTER_SOURCE.into(),
                LoadedCustomComponent {
                    document: outer.clone(),
                    checksum: "outer-wrapper-checksum".into(),
                },
            ),
        ]),
    };

    let first_nested = run(&nested_model()?, &registry, &loader, 1)?;
    let second_nested = run(&nested_model()?, &registry, &loader, 2)?;
    let flattened = run(&flat_model()?, &registry, &loader, 3)?;

    if first_nested.status != RunStatus::Completed {
        return Err(format!(
            "nested run did not complete: {:?}",
            first_nested.status
        ));
    }
    if first_nested.series != second_nested.series {
        return Err("repeated lock-pinned runs produced different series".into());
    }
    if first_nested.series != flattened.series {
        return Err("nested and flattened runs produced different series".into());
    }
    let expected_values = vec![RuntimeValue::Scalar(6.0); 3];
    if first_nested.series.first().map(|series| &series.values) != Some(&expected_values) {
        return Err("nested run produced unexpected values".into());
    }

    let test_results = run_custom_component_tests(
        &outer,
        OUTER_SOURCE,
        "outer-wrapper-checksum",
        &registry,
        &loader,
    );
    if test_results.len() != 1
        || test_results.first().map(|result| &result.outcome) != Some(&ComponentTestOutcome::Passed)
    {
        return Err(format!("custom component tests failed: {test_results:?}"));
    }
    Ok(())
}

#[test]
fn nested_and_flattened_models_are_equivalent_and_lock_pinned() {
    assert_eq!(verify_nested_and_flattened_models(), Ok(()));
}
