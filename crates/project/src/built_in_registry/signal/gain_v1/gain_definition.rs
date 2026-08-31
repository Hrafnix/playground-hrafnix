use crate::built_in_registry::signal::gain_v1::gain_component::GainV1Computed;
use crate::definitions::built_in_component_definition::built_in_component_definition;
use crate::definitions::icon_definition::icon_definition;
use crate::definitions::port_definition::port_definition;
use crate::{
    BuiltInComponentDefinition, BuiltInComponentTrait, Component, ComponentComputeError, Computed,
    PortKind, Rotation,
};
use datastore::{item_compile_time, number_compile_time};
use expression_engine::prelude::{ComputedItem, ExpressionEngine, ParameterObjectInputData};

/// Version 1 definition of the gain component.
pub static GAIN_V1: BuiltInComponentDefinition = built_in_component_definition!(
    "gain",
    1,
    "Gain",
    datastore::parameter_object_compile_time!(
        "Parameters",
        [(
            "p_gain",
            item_compile_time!(number = number_compile_time!("Gain", default = "1.0")),
        ),]
    ),
    datastore::variable_object_compile_time!("Variables", []),
    icon_definition!(include_str!("gain.svg"), (32, 32)),
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

/// Gain component version 1.
#[derive(Debug)]
pub struct GainV1;

impl BuiltInComponentTrait for GainV1 {
    fn definition(&self) -> &'static BuiltInComponentDefinition {
        &GAIN_V1
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
        let gain = parameters
            .get("p_gain")
            .ok_or(ComponentComputeError::MissingParameter("p_gain"))?;
        let ComputedItem::Float(gain) = gain else {
            return Err(ComponentComputeError::InvalidParameterType("p_gain"));
        };

        Ok(Box::new(GainV1Computed::new(*gain)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gain_definition_instantiates_and_computes() {
        let implementation = GainV1;
        let component = implementation.instantiate();
        let Ok(computed) = implementation.compute(&component, &ExpressionEngine::new()) else {
            panic!("default gain should compute");
        };
        let Some(gain) = computed.as_any().downcast_ref::<GainV1Computed>() else {
            panic!("gain implementation should return GainV1Computed");
        };

        assert_eq!(gain.gain().to_bits(), 1.0_f64.to_bits());
        assert_eq!(gain.apply(2.5).to_bits(), 2.5_f64.to_bits());
    }
}
