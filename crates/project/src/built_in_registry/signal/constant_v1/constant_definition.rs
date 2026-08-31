use crate::built_in_registry::signal::constant_v1::constant_component::ConstantV1Computed;
use crate::definitions::built_in_component_definition::built_in_component_definition;
use crate::definitions::icon_definition::icon_definition;
use crate::definitions::port_definition::port_definition;
use crate::{
    BuiltInComponentDefinition, BuiltInComponentTrait, Component, ComponentComputeError, Computed,
    PortKind, Rotation,
};
use datastore::{item_compile_time, number_compile_time};
use expression_engine::prelude::{ComputedItem, ExpressionEngine, ParameterObjectInputData};

/// Version 1 definition of the constant component.
pub static CONSTANT_V1: BuiltInComponentDefinition = built_in_component_definition!(
    "constant",
    1,
    "Constant",
    datastore::parameter_object_compile_time!(
        "Parameters",
        [(
            "p_value",
            item_compile_time!(number = number_compile_time!("Value", default = "0.0")),
        ),]
    ),
    datastore::variable_object_compile_time!("Variables", []),
    icon_definition!(include_str!("constant.svg"), (32, 32)),
    [port_definition!(
        "output",
        "Output",
        PortKind::SignalOutput,
        (32, 16),
        (32, 32),
        Rotation::Degrees0,
        false
    ),],
);

/// Constant component version 1.
#[derive(Debug)]
pub struct ConstantV1;

impl BuiltInComponentTrait for ConstantV1 {
    fn definition(&self) -> &'static BuiltInComponentDefinition {
        &CONSTANT_V1
    }

    fn compute(
        &self,
        component: &Component,
        engine: &ExpressionEngine,
    ) -> Result<Box<dyn Computed>, ComponentComputeError> {
        if component.id() != self.definition().id()
            || component.version() != self.definition().version()
        {
            return Err(ComponentComputeError::DefinitionMismatch);
        }
        let parameters = engine
            .evaluate_parameters(&ParameterObjectInputData::new(component.parameters()))
            .map_err(ComponentComputeError::ParameterEvaluation)?;
        let value = parameters
            .get("p_value")
            .ok_or(ComponentComputeError::MissingParameter("p_value"))?;
        let ComputedItem::Float(value) = value else {
            return Err(ComponentComputeError::InvalidParameterType("p_value"));
        };
        Ok(Box::new(ConstantV1Computed::new(*value)))
    }
}
