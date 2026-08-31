#![allow(
    clippy::arithmetic_side_effects,
    clippy::float_arithmetic,
    reason = "Validated physical quantities require integration arithmetic."
)]

use crate::{
    ComponentInstanceId, ComponentQ, Computed, MechanicalNodeId, MechanicalNodeState,
    TranslationalError, TranslationalQPhase,
};
use std::any::Any;
use std::collections::BTreeMap;

/// Evaluated parameters for a lumped mass.
#[derive(Debug, PartialEq)]
pub struct MassV1Computed {
    /// Positive mass in kilograms.
    mass: f64,
    /// Initial position in meters.
    initial_position: f64,
    /// Initial velocity in meters per second.
    initial_velocity: f64,
}

impl MassV1Computed {
    /// Creates evaluated mass parameters.
    #[must_use]
    pub const fn new(mass: f64, initial_position: f64, initial_velocity: f64) -> Self {
        Self {
            mass,
            initial_position,
            initial_velocity,
        }
    }
}

impl Computed for MassV1Computed {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_component_q(&mut self) -> Option<&mut dyn ComponentQ> {
        Some(self)
    }
}

impl ComponentQ for MassV1Computed {
    fn initialize_q(
        &mut self,
        component: ComponentInstanceId,
        nodes: &[MechanicalNodeId],
        state: &mut BTreeMap<MechanicalNodeId, MechanicalNodeState>,
    ) -> Result<(), TranslationalError> {
        let node = one_node(nodes)?;
        if !self.mass.is_finite() || self.mass <= 0.0 {
            return Err(TranslationalError::InvalidParameter {
                id: component,
                key: "mass",
            });
        }
        if !self.initial_position.is_finite() {
            return Err(TranslationalError::InvalidParameter {
                id: component,
                key: "initial_position",
            });
        }
        if !self.initial_velocity.is_finite() {
            return Err(TranslationalError::InvalidParameter {
                id: component,
                key: "initial_velocity",
            });
        }
        state.insert(
            node,
            MechanicalNodeState {
                position: self.initial_position,
                velocity: self.initial_velocity,
                acceleration: 0.0,
                force: 0.0,
                reaction_force: 0.0,
                wave_variable: 0.0,
                characteristic_impedance: 0.0,
                equivalent_mass: self.mass,
            },
        );
        Ok(())
    }

    fn simulate_q(
        &mut self,
        phase: TranslationalQPhase,
        timestep: f64,
        nodes: &[MechanicalNodeId],
        state: &mut BTreeMap<MechanicalNodeId, MechanicalNodeState>,
    ) -> Result<(), TranslationalError> {
        let node = one_node(nodes)?;
        let node_state = state
            .get_mut(&node)
            .ok_or(TranslationalError::UnboundNode(node))?;
        match phase {
            TranslationalQPhase::Predict => {
                node_state.position += node_state.velocity * timestep
                    + 0.5 * node_state.acceleration * timestep * timestep;
                node_state.velocity += 0.5 * node_state.acceleration * timestep;
            }
            TranslationalQPhase::Respond => {
                node_state.force = node_state.wave_variable
                    + node_state.characteristic_impedance * node_state.velocity;
                node_state.acceleration = node_state.force / self.mass;
                node_state.equivalent_mass = self.mass;
                node_state.reaction_force = 0.0;
            }
            TranslationalQPhase::Correct => {
                node_state.velocity += 0.5 * node_state.acceleration * timestep;
            }
        }
        Ok(())
    }

    fn energy_q(
        &self,
        nodes: &[MechanicalNodeId],
        state: &BTreeMap<MechanicalNodeId, MechanicalNodeState>,
    ) -> Result<f64, TranslationalError> {
        let node = one_node(nodes)?;
        let velocity = state
            .get(&node)
            .ok_or(TranslationalError::UnboundNode(node))?
            .velocity;
        Ok(0.5 * self.mass * velocity * velocity)
    }

    fn finalize_q(&mut self) {}
}

/// Extracts the mass's single bound node.
const fn one_node(nodes: &[MechanicalNodeId]) -> Result<MechanicalNodeId, TranslationalError> {
    let [node] = nodes else {
        return Err(TranslationalError::InvalidNodeCount {
            expected: 1,
            actual: nodes.len(),
        });
    };
    Ok(*node)
}
