use crate::built_in_registry::signal::add_v1::add_component::AddV1Computed;
use crate::definitions::built_in_component_definition::built_in_component_definition;
use crate::definitions::icon_definition::icon_definition;
use crate::definitions::port_definition::port_definition;
use crate::{
    BuiltInComponentDefinition, BuiltInComponentTrait, Component, ComponentComputeError, Computed,
    PortKind, Rotation,
};
use expression_engine::prelude::ExpressionEngine;

/// Version 1 definition of the add component.
pub static ADD_V1: BuiltInComponentDefinition = built_in_component_definition!(
    "add",
    1,
    "Add",
    datastore::parameter_object_compile_time!("Parameters", &[]),
    datastore::variable_object_compile_time!("Variables", []),
    icon_definition!(include_str!("add.svg"), (32, 32)),
    [
        port_definition!(
            "a",
            "Input A",
            PortKind::SignalInput,
            (0, 10),
            (32, 32),
            Rotation::Degrees0,
            true
        ),
        port_definition!(
            "b",
            "Input B",
            PortKind::SignalInput,
            (0, 22),
            (32, 32),
            Rotation::Degrees0,
            true
        ),
        port_definition!(
            "output",
            "Output",
            PortKind::SignalOutput,
            (32, 16),
            (32, 32),
            Rotation::Degrees0,
            false
        ),
    ],
);

/// Add component version 1.
#[derive(Debug)]
pub struct AddV1;

impl BuiltInComponentTrait for AddV1 {
    fn definition(&self) -> &'static BuiltInComponentDefinition {
        &ADD_V1
    }

    fn compute(
        &self,
        component: &Component,
        _engine: &ExpressionEngine,
    ) -> Result<Box<dyn Computed>, ComponentComputeError> {
        if component.id() != self.definition().id()
            || component.version() != self.definition().version()
        {
            return Err(ComponentComputeError::DefinitionMismatch);
        }
        Ok(Box::new(AddV1Computed))
    }
}
