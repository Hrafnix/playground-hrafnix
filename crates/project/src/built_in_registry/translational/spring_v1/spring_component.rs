#![allow(
    clippy::arithmetic_side_effects,
    clippy::float_arithmetic,
    reason = "Validated physical quantities require constitutive arithmetic."
)]

use crate::{
    ComponentC, ComponentInstanceId, Computed, MechanicalNodeId, MechanicalNodeState,
    TranslationalError,
};
use std::any::Any;
use std::collections::BTreeMap;

/// Evaluated parameters for an ideal spring.
#[derive(Debug, PartialEq)]
pub struct SpringV1Computed {
    /// Stiffness in newtons per meter.
    stiffness: f64,
    /// Unloaded signed displacement in meters.
    free_length: f64,
}

impl SpringV1Computed {
    /// Creates evaluated spring parameters.
    #[must_use]
    pub const fn new(stiffness: f64, free_length: f64) -> Self {
        Self {
            stiffness,
            free_length,
        }
    }
}

impl Computed for SpringV1Computed {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_component_c(&mut self) -> Option<&mut dyn ComponentC> {
        Some(self)
    }
}

impl ComponentC for SpringV1Computed {
    fn initialize_c(
        &mut self,
        component: ComponentInstanceId,
        nodes: &[MechanicalNodeId],
        _state: &mut BTreeMap<MechanicalNodeId, MechanicalNodeState>,
    ) -> Result<(), TranslationalError> {
        validate_nodes(nodes)?;
        if !self.stiffness.is_finite() || self.stiffness <= 0.0 {
            return Err(TranslationalError::InvalidParameter {
                id: component,
                key: "stiffness",
            });
        }
        if !self.free_length.is_finite() {
            return Err(TranslationalError::InvalidParameter {
                id: component,
                key: "free_length",
            });
        }
        Ok(())
    }

    fn simulate_c(
        &mut self,
        timestep: f64,
        nodes: &[MechanicalNodeId],
        state: &mut BTreeMap<MechanicalNodeId, MechanicalNodeState>,
    ) -> Result<(), TranslationalError> {
        let (node_a, node_b) = validate_nodes(nodes)?;
        let position_a = state
            .get(&node_a)
            .ok_or(TranslationalError::UnboundNode(node_a))?
            .position;
        let position_b = state
            .get(&node_b)
            .ok_or(TranslationalError::UnboundNode(node_b))?
            .position;
        let force = -self.stiffness * (position_a - position_b - self.free_length);
        let impedance = self.stiffness * timestep;
        publish(state, node_a, force, impedance)?;
        publish(state, node_b, -force, impedance)
    }

    fn energy_c(
        &self,
        nodes: &[MechanicalNodeId],
        state: &BTreeMap<MechanicalNodeId, MechanicalNodeState>,
    ) -> Result<f64, TranslationalError> {
        let (node_a, node_b) = validate_nodes(nodes)?;
        let position_a = state
            .get(&node_a)
            .ok_or(TranslationalError::UnboundNode(node_a))?
            .position;
        let position_b = state
            .get(&node_b)
            .ok_or(TranslationalError::UnboundNode(node_b))?
            .position;
        let extension = position_a - position_b - self.free_length;
        Ok(0.5 * self.stiffness * extension * extension)
    }

    fn finalize_c(&mut self) {}
}

/// Validates and extracts the spring's two bound nodes.
fn validate_nodes(
    nodes: &[MechanicalNodeId],
) -> Result<(MechanicalNodeId, MechanicalNodeId), TranslationalError> {
    let &[node_a, node_b] = nodes else {
        return Err(TranslationalError::InvalidNodeCount {
            expected: 2,
            actual: nodes.len(),
        });
    };
    if node_a == node_b {
        Err(TranslationalError::CoincidentNodes(node_a))
    } else {
        Ok((node_a, node_b))
    }
}

/// Publishes one spring port's wave variable and characteristic impedance.
fn publish(
    state: &mut BTreeMap<MechanicalNodeId, MechanicalNodeState>,
    node: MechanicalNodeId,
    force: f64,
    impedance: f64,
) -> Result<(), TranslationalError> {
    let node_state = state
        .get_mut(&node)
        .ok_or(TranslationalError::UnboundNode(node))?;
    node_state.wave_variable += force - impedance * node_state.velocity;
    node_state.characteristic_impedance += impedance;
    Ok(())
}
