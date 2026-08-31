#![allow(
    clippy::print_stdout,
    reason = "This executable demonstrates simulation results in a terminal."
)]

//! Runs a configured unit spring-mass oscillator.

use expression_engine::prelude::ExpressionEngine;
use keys::component_key;
use project::built_in_registry::BUILT_IN_REGISTRY;
use project::{
    Component, ComponentInstanceId, MechanicalNodeId, SimulationRun, SimulationRuntime,
    SimulationSettings, TranslationalComponentInstance, TranslationalModel,
};
use std::error::Error;
use std::io;

/// Converts a datastore edit diagnostic into a standard example error.
#[allow(
    clippy::needless_pass_by_value,
    reason = "Result::map_err supplies the diagnostic by value."
)]
fn edit_error(error: datastore::prelude::Message) -> io::Error {
    io::Error::other(format!("{error:?}"))
}

/// Instantiates a registered component or reports a missing registry entry.
fn component(id: keys::ConstComponentKey) -> Result<Component, io::Error> {
    BUILT_IN_REGISTRY
        .get(id)
        .map(|item| item.current().instantiate())
        .ok_or_else(|| io::Error::other(format!("component `{id}` is not registered")))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mass_definition = BUILT_IN_REGISTRY
        .get(component_key!("translational_mass"))
        .ok_or_else(|| io::Error::other("mass is not registered"))?
        .current();
    let spring_definition = BUILT_IN_REGISTRY
        .get(component_key!("translational_spring"))
        .ok_or_else(|| io::Error::other("spring is not registered"))?
        .current();
    let boundary_definition = BUILT_IN_REGISTRY
        .get(component_key!("translational_fixed_boundary"))
        .ok_or_else(|| io::Error::other("fixed boundary is not registered"))?
        .current();

    let mut mass = component(component_key!("translational_mass"))?;
    let mut spring = component(component_key!("translational_spring"))?;
    let boundary = component(component_key!("translational_fixed_boundary"))?;
    mass.set_parameter_expression("p_mass", "1.0")
        .map_err(edit_error)?;
    mass.set_parameter_expression("p_initial_position", "1.0")
        .map_err(edit_error)?;
    spring
        .set_parameter_expression("p_stiffness", "1.0")
        .map_err(edit_error)?;

    let engine = ExpressionEngine::new();
    let mass = mass_definition.compute(&mass, &engine)?;
    let spring = spring_definition.compute(&spring, &engine)?;
    let boundary = boundary_definition.compute(&boundary, &engine)?;

    let moving = MechanicalNodeId::new(1);
    let ground = MechanicalNodeId::new(2);
    let mut model = TranslationalModel::new();
    model.add_component(TranslationalComponentInstance::new(
        ComponentInstanceId::new(1),
        vec![moving],
        mass,
    ))?;
    model.add_component(TranslationalComponentInstance::new(
        ComponentInstanceId::new(2),
        vec![ground],
        boundary,
    ))?;
    model.add_component(TranslationalComponentInstance::new(
        ComponentInstanceId::new(3),
        vec![moving, ground],
        spring,
    ))?;

    let mut runtime = SimulationRuntime::new(model)?;
    let SimulationRun::Translational(run) = runtime.run(SimulationSettings {
        timestep: 0.01,
        steps: 628,
    })?
    else {
        return Err(
            io::Error::other("translational model returned a non-translational run").into(),
        );
    };
    let positions = run
        .node_positions(moving)
        .ok_or_else(|| io::Error::other("moving node was not sampled"))?;

    println!("samples: {}", run.samples.len());
    println!(
        "final position: {:.6}",
        positions.last().copied().unwrap_or_default()
    );
    println!(
        "maximum energy residual: {:.8}",
        run.maximum_energy_residual
    );
    Ok(())
}
