use super::fixed_boundary_component::FixedBoundaryV1Computed;
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

/// Version 1 definition of a fixed boundary.
pub static FIXED_BOUNDARY_V1: BuiltInComponentDefinition = built_in_component_definition!(
    "translational_fixed_boundary",
    1,
    "Fixed Boundary",
    datastore::parameter_object_compile_time!(
        "Parameters",
        [(
            "p_position",
            item_compile_time!(
                number_with_units = number_with_units_compile_time!(
                    "Position",
                    UnitId::Length_Meter,
                    default = "0.0"
                )
            )
        ),]
    ),
    datastore::variable_object_compile_time!("Variables", []),
    icon_definition!(include_str!("fixed_boundary.svg"), (40, 32)),
    [port_definition!(
        "mechanical",
        "Mechanical",
        PortKind::Translational,
        (40, 16),
        (40, 32),
        Rotation::Degrees0,
        true
    )],
);

/// Fixed-boundary component version 1.
#[derive(Debug)]
pub struct FixedBoundaryV1;

impl BuiltInComponentTrait for FixedBoundaryV1 {
    fn definition(&self) -> &'static BuiltInComponentDefinition {
        &FIXED_BOUNDARY_V1
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
        Ok(Box::new(FixedBoundaryV1Computed::new(
            crate::computed::computed_number(&values, "p_position")?,
        )))
    }
}
