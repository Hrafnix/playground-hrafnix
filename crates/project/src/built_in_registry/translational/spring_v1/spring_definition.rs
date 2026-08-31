use super::spring_component::SpringV1Computed;
use crate::definitions::built_in_component_definition::built_in_component_definition;
use crate::definitions::icon_definition::icon_definition;
use crate::definitions::port_definition::port_definition;
use crate::{
    BuiltInComponentDefinition, BuiltInComponentTrait, Component, ComponentComputeError, Computed,
    PortKind, Rotation,
};
use datastore::{item_compile_time, number_with_units_compile_time};
use expression_engine::prelude::{ExpressionEngine, ParameterObjectInputData};
use units::UnitId;

/// Version 1 definition of an ideal spring.
pub static SPRING_V1: BuiltInComponentDefinition = built_in_component_definition!(
    "translational_spring",
    1,
    "Spring",
    datastore::parameter_object_compile_time!(
        "Parameters",
        [
            (
                "p_stiffness",
                item_compile_time!(
                    number_with_units =
                        number_with_units_compile_time!("Stiffness", UnitId::None, default = "1.0")
                )
            ),
            (
                "p_free_length",
                item_compile_time!(
                    number_with_units = number_with_units_compile_time!(
                        "Free length",
                        UnitId::Length_Meter,
                        default = "0.0"
                    )
                )
            ),
        ]
    ),
    datastore::variable_object_compile_time!("Variables", []),
    icon_definition!(include_str!("spring.svg"), (64, 32)),
    [
        port_definition!(
            "a",
            "Mechanical A",
            PortKind::Translational,
            (0, 16),
            (64, 32),
            Rotation::Degrees0,
            true
        ),
        port_definition!(
            "b",
            "Mechanical B",
            PortKind::Translational,
            (64, 16),
            (64, 32),
            Rotation::Degrees0,
            true
        ),
    ],
);

/// Spring component version 1.
#[derive(Debug)]
pub struct SpringV1;

impl BuiltInComponentTrait for SpringV1 {
    fn definition(&self) -> &'static BuiltInComponentDefinition {
        &SPRING_V1
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
        let values = engine
            .evaluate_parameters(&ParameterObjectInputData::new(component.parameters()))
            .map_err(ComponentComputeError::ParameterEvaluation)?;
        Ok(Box::new(SpringV1Computed::new(
            crate::computed::computed_number(&values, "p_stiffness")?,
            crate::computed::computed_number(&values, "p_free_length")?,
        )))
    }
}
