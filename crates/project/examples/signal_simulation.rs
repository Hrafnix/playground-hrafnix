#![allow(
    clippy::print_stdout,
    reason = "This executable demonstrates simulation results in a terminal."
)]

//! Runs a configured constant-to-gain-to-delay signal model.

use expression_engine::prelude::ExpressionEngine;
use keys::{component_key, port_key};
use project::built_in_registry::BUILT_IN_REGISTRY;
use project::{
    ComponentInstanceId, SignalComponentInstance, SignalConnection, SignalEndpoint, SignalModel,
    SimulationRun, SimulationRuntime, SimulationSettings,
};
use std::error::Error;
use std::io;

/// Creates a stable endpoint for the demonstration model.
const fn endpoint(component: u64, port: keys::ConstPortKey) -> SignalEndpoint {
    SignalEndpoint {
        component: ComponentInstanceId::new(component),
        port,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let constant_definition = BUILT_IN_REGISTRY
        .get(component_key!("constant"))
        .ok_or_else(|| io::Error::other("constant is not registered"))?
        .current();
    let gain_definition = BUILT_IN_REGISTRY
        .get(component_key!("gain"))
        .ok_or_else(|| io::Error::other("gain is not registered"))?
        .current();
    let delay_definition = BUILT_IN_REGISTRY
        .get(component_key!("delay"))
        .ok_or_else(|| io::Error::other("delay is not registered"))?
        .current();

    let mut constant = constant_definition.instantiate();
    let mut gain = gain_definition.instantiate();
    let mut delay = delay_definition.instantiate();
    constant
        .set_parameter_expression("p_value", "2.0")
        .map_err(|error| io::Error::other(format!("{error:?}")))?;
    gain.set_parameter_expression("p_gain", "3.0")
        .map_err(|error| io::Error::other(format!("{error:?}")))?;
    delay
        .set_parameter_expression("p_initial_value", "-1.0")
        .map_err(|error| io::Error::other(format!("{error:?}")))?;

    let engine = ExpressionEngine::new();
    let model = SignalModel {
        components: vec![
            SignalComponentInstance::compute(
                ComponentInstanceId::new(1),
                constant_definition,
                &constant,
                &engine,
            )?,
            SignalComponentInstance::compute(
                ComponentInstanceId::new(2),
                gain_definition,
                &gain,
                &engine,
            )?,
            SignalComponentInstance::compute(
                ComponentInstanceId::new(3),
                delay_definition,
                &delay,
                &engine,
            )?,
        ],
        connections: vec![
            SignalConnection {
                source: endpoint(1, port_key!("output")),
                target: endpoint(2, port_key!("input")),
            },
            SignalConnection {
                source: endpoint(2, port_key!("output")),
                target: endpoint(3, port_key!("input")),
            },
        ],
    };

    let mut runtime = SimulationRuntime::new(model)?;
    let SimulationRun::Signal(run) = runtime.run(SimulationSettings {
        timestep: 0.25,
        steps: 4,
    })?
    else {
        return Err(io::Error::other("signal model returned a non-signal run").into());
    };
    let delayed = run
        .series
        .get(&endpoint(3, port_key!("output")))
        .ok_or_else(|| io::Error::other("delay output was not recorded"))?;

    println!("time:    {:?}", run.times);
    println!("delayed: {delayed:?}");
    Ok(())
}
