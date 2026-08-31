use super::mass_component::MassV1Computed;
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

/// Version 1 definition of a lumped mass.
pub static MASS_V1: BuiltInComponentDefinition = built_in_component_definition!(
    "translational_mass",
    1,
    "Mass",
    datastore::parameter_object_compile_time!(
        "Parameters",
        [
            (
                "p_mass",
                item_compile_time!(
                    number_with_units = number_with_units_compile_time!(
                        "Mass",
                        UnitId::Mass_Kilogram,
                        default = "1.0"
                    )
                )
            ),
            (
                "p_initial_position",
                item_compile_time!(
                    number_with_units = number_with_units_compile_time!(
                        "Initial position",
                        UnitId::Length_Meter,
                        default = "0.0"
                    )
                )
            ),
            (
                "p_initial_velocity",
                item_compile_time!(
                    number_with_units = number_with_units_compile_time!(
                        "Initial velocity",
                        UnitId::None,
                        default = "0.0"
                    )
                )
            ),
        ]
    ),
    datastore::variable_object_compile_time!("Variables", []),
    icon_definition!(include_str!("mass.svg"), (48, 32)),
    [port_definition!(
        "mechanical",
        "Mechanical",
        PortKind::Translational,
        (48, 16),
        (48, 32),
        Rotation::Degrees0,
        true
    )],
);

/// Mass component version 1.
#[derive(Debug)]
pub struct MassV1;

impl BuiltInComponentTrait for MassV1 {
    fn definition(&self) -> &'static BuiltInComponentDefinition {
        &MASS_V1
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
        Ok(Box::new(MassV1Computed::new(
            crate::computed::computed_number(&values, "p_mass")?,
            crate::computed::computed_number(&values, "p_initial_position")?,
            crate::computed::computed_number(&values, "p_initial_velocity")?,
        )))
    }
}
