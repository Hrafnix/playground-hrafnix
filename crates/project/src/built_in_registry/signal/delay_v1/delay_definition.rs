use crate::built_in_registry::signal::delay_v1::delay_component::DelayV1Computed;
use crate::definitions::built_in_component_definition::built_in_component_definition;
use crate::definitions::icon_definition::icon_definition;
use crate::definitions::port_definition::port_definition;
use crate::{
    BuiltInComponentDefinition, BuiltInComponentTrait, Component, ComponentComputeError, Computed,
    PortKind, Rotation,
};
use datastore::{item_compile_time, number_compile_time};
use expression_engine::prelude::{ComputedItem, ExpressionEngine, ParameterObjectInputData};

/// Version 1 definition of the one-step delay component.
pub static DELAY_V1: BuiltInComponentDefinition = built_in_component_definition!(
    "delay",
    1,
    "Delay",
    datastore::parameter_object_compile_time!(
        "Parameters",
        [(
            "p_initial_value",
            item_compile_time!(number = number_compile_time!("Initial value", default = "0.0")),
        ),]
    ),
    datastore::variable_object_compile_time!(
        "Variables",
        [(
            "v_value",
            item_compile_time!(number = number_compile_time!("Value", default = "0.0")),
        ),]
    ),
    icon_definition!(include_str!("delay.svg"), (32, 32)),
    [
        port_definition!(
            "input",
            "Input",
            PortKind::SignalInput,
            (0, 16),
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

/// Delay component version 1.
#[derive(Debug)]
pub struct DelayV1;

impl BuiltInComponentTrait for DelayV1 {
    fn definition(&self) -> &'static BuiltInComponentDefinition {
        &DELAY_V1
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
            .get("p_initial_value")
            .ok_or(ComponentComputeError::MissingParameter("p_initial_value"))?;
        let ComputedItem::Float(value) = value else {
            return Err(ComponentComputeError::InvalidParameterType(
                "p_initial_value",
            ));
        };
        Ok(Box::new(DelayV1Computed::new(*value)))
    }
}
