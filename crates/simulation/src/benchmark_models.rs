//! Deterministic model workloads for regression tests and external benchmarks.

use crate::component::{ComponentTypeId, InvalidComponentTypeId};
use crate::document::{
    CanvasPosition, ComponentInstance, ComponentReference, Composition, Connection, DocumentHeader,
    LoggingPolicy, MODEL_SCHEMA_VERSION, ModelDocument, PortEndpoint, ProbeDefinition,
    SimulationSettings,
};
use crate::identity::{ComponentId, ConnectionId, DocumentId, ProbeId, SystemId};
use crate::timing::FixedStepSemantics;
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Builds a mixed math, logic, routing, lookup, and assertion workload.
///
/// # Errors
///
/// Returns an error only if a hard-coded built-in identity violates the ID contract.
pub fn mixed_signal_benchmark() -> Result<ModelDocument, InvalidComponentTypeId> {
    let components = vec![
        instance(1, "signal.constant", "two", &[("value", "2.0")])?,
        instance(2, "signal.constant", "three", &[("value", "3.0")])?,
        instance(3, "signal.subtract", "difference", &[])?,
        instance(4, "signal.divide", "ratio", &[])?,
        instance(
            5,
            "signal.clamp",
            "limited",
            &[("minimum", "-1.0"), ("maximum", "1.0")],
        )?,
        instance(6, "signal.greater_than", "comparison", &[])?,
        instance(7, "signal.boolean_not", "invert", &[])?,
        instance(8, "signal.switch", "route", &[])?,
        instance(
            9,
            "signal.lookup",
            "calibration",
            &[("x0", "0.0"), ("y0", "0.0"), ("x1", "3.0"), ("y1", "30.0")],
        )?,
        instance(
            10,
            "signal.assertion",
            "range check",
            &[("minimum", "0.0"), ("maximum", "30.0")],
        )?,
    ];
    let connections = vec![
        connection(1, 1, "out", 3, "a"),
        connection(2, 2, "out", 3, "b"),
        connection(3, 3, "out", 4, "a"),
        connection(4, 2, "out", 4, "b"),
        connection(5, 4, "out", 5, "in"),
        connection(6, 1, "out", 6, "a"),
        connection(7, 2, "out", 6, "b"),
        connection(8, 6, "out", 7, "in"),
        connection(9, 7, "out", 8, "select"),
        connection(10, 1, "out", 8, "false"),
        connection(11, 2, "out", 8, "true"),
        connection(12, 8, "out", 9, "in"),
        connection(13, 9, "out", 10, "in"),
    ];
    Ok(model(700, components, connections, 10, 0.1, 0.01))
}

/// Builds a stateful fixed-step control workload.
///
/// # Errors
///
/// Returns an error only if a hard-coded built-in identity violates the ID contract.
pub fn control_benchmark() -> Result<ModelDocument, InvalidComponentTypeId> {
    let components = vec![
        instance(
            1,
            "signal.step",
            "command",
            &[
                ("initial_value", "0.0"),
                ("final_value", "1.0"),
                ("step_time", "0.0"),
            ],
        )?,
        instance(2, "signal.gain", "gain", &[("gain", "2.0")])?,
        instance(
            3,
            "signal.first_order_transfer",
            "plant",
            &[
                ("gain", "1.0"),
                ("time_constant", "0.5"),
                ("initial_value", "0.0"),
            ],
        )?,
        instance(
            4,
            "signal.integrator",
            "accumulator",
            &[("initial_value", "0.0")],
        )?,
    ];
    let connections = vec![
        connection(1, 1, "out", 2, "in"),
        connection(2, 2, "out", 3, "in"),
        connection(3, 3, "out", 4, "in"),
    ];
    Ok(model(701, components, connections, 4, 1.0, 0.1))
}

/// Creates one built-in source instance.
fn instance(
    id: u128,
    type_id: &str,
    name: &str,
    overrides: &[(&str, &str)],
) -> Result<ComponentInstance, InvalidComponentTypeId> {
    Ok(ComponentInstance {
        id: ComponentId::from_raw(id),
        name: name.into(),
        component: ComponentReference::BuiltIn {
            type_id: ComponentTypeId::new(type_id)?,
        },
        parameter_overrides: overrides
            .iter()
            .map(|(key, value)| ((*key).into(), (*value).into()))
            .collect(),
        enabled: true,
        position: CanvasPosition { x: 0.0, y: 0.0 },
    })
}

/// Creates one root-system connection.
fn connection(
    id: u128,
    source: u128,
    source_port: &str,
    target: u128,
    target_port: &str,
) -> Connection {
    Connection {
        id: ConnectionId::from_raw(id),
        source: endpoint(source, source_port),
        target: endpoint(target, target_port),
        label: None,
        route: Vec::new(),
    }
}

/// Creates one endpoint in a benchmark root system.
fn endpoint(component_id: u128, port_key: &str) -> PortEndpoint {
    PortEndpoint {
        component_id: ComponentId::from_raw(component_id),
        port_key: port_key.into(),
    }
}

/// Assembles shared deterministic model metadata.
fn model(
    document_id: u128,
    components: Vec<ComponentInstance>,
    connections: Vec<Connection>,
    probe_component: u128,
    stop_time: f64,
    timestep: f64,
) -> ModelDocument {
    ModelDocument {
        header: DocumentHeader {
            schema_version: MODEL_SCHEMA_VERSION,
            document_id: DocumentId::from_raw(document_id),
            title: "Signal library benchmark".into(),
            description: "Deterministic workload for regression and measurement.".into(),
            author: "simulation".into(),
            created_at: "2026-08-24T00:00:00Z".into(),
            updated_at: "2026-08-24T00:00:00Z".into(),
            migrations: Vec::new(),
        },
        root: Composition {
            system_id: SystemId::from_raw(1),
            components,
            connections,
            annotations: BTreeMap::new(),
        },
        simulation: SimulationSettings {
            start_time: 0.0,
            stop_time,
            timestep,
            maximum_steps: 100_000,
            random_seed: 7,
            logging: LoggingPolicy::EveryStep,
            semantics: FixedStepSemantics::default(),
        },
        probes: vec![ProbeDefinition {
            id: ProbeId::from_raw(1),
            target: endpoint(probe_component, "out"),
            display_name: "benchmark output".into(),
            plot_group: Some(ShareableString::from("benchmark")),
        }],
        dependencies: Vec::new(),
    }
}
