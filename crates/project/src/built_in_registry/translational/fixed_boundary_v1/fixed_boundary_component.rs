#![allow(
    clippy::float_arithmetic,
    reason = "A boundary reaction reverses a validated physical force."
)]

use crate::{
    ComponentInstanceId, ComponentQ, Computed, MechanicalNodeId, MechanicalNodeState,
    TranslationalError, TranslationalQPhase,
};
use std::any::Any;
use std::collections::BTreeMap;

/// Evaluated parameters for a fixed boundary.
#[derive(Debug, PartialEq)]
pub struct FixedBoundaryV1Computed {
    /// Prescribed position in meters.
    position: f64,
}

impl FixedBoundaryV1Computed {
    /// Creates evaluated boundary parameters.
    #[must_use]
    pub const fn new(position: f64) -> Self {
        Self { position }
    }
}

impl Computed for FixedBoundaryV1Computed {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_component_q(&mut self) -> Option<&mut dyn ComponentQ> {
        Some(self)
    }
}

impl ComponentQ for FixedBoundaryV1Computed {
    fn initialize_q(
        &mut self,
        component: ComponentInstanceId,
        nodes: &[MechanicalNodeId],
        state: &mut BTreeMap<MechanicalNodeId, MechanicalNodeState>,
    ) -> Result<(), TranslationalError> {
        let node = one_node(nodes)?;
        if !self.position.is_finite() {
            return Err(TranslationalError::InvalidParameter {
                id: component,
                key: "position",
            });
        }
        state.insert(
            node,
            MechanicalNodeState {
                position: self.position,
                velocity: 0.0,
                acceleration: 0.0,
                force: 0.0,
                reaction_force: 0.0,
                wave_variable: 0.0,
                characteristic_impedance: 0.0,
                equivalent_mass: 1.0,
            },
        );
        Ok(())
    }

    fn simulate_q(
        &mut self,
        phase: TranslationalQPhase,
        _timestep: f64,
        nodes: &[MechanicalNodeId],
        state: &mut BTreeMap<MechanicalNodeId, MechanicalNodeState>,
    ) -> Result<(), TranslationalError> {
        let node = one_node(nodes)?;
        let node_state = state
            .get_mut(&node)
            .ok_or(TranslationalError::UnboundNode(node))?;
        if phase == TranslationalQPhase::Respond {
            node_state.position = self.position;
            node_state.velocity = 0.0;
            node_state.force = node_state.wave_variable;
            node_state.acceleration = 0.0;
            node_state.reaction_force = -node_state.force;
        }
        Ok(())
    }

    fn energy_q(
        &self,
        nodes: &[MechanicalNodeId],
        _state: &BTreeMap<MechanicalNodeId, MechanicalNodeState>,
    ) -> Result<f64, TranslationalError> {
        one_node(nodes)?;
        Ok(0.0)
    }

    fn finalize_q(&mut self) {}
}

/// Extracts the boundary's single bound node.
const fn one_node(nodes: &[MechanicalNodeId]) -> Result<MechanicalNodeId, TranslationalError> {
    let [node] = nodes else {
        return Err(TranslationalError::InvalidNodeCount {
            expected: 1,
            actual: nodes.len(),
        });
    };
    Ok(*node)
}
