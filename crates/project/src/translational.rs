//! Deterministic one-dimensional spring-mass mechanics.

#![allow(
    clippy::arithmetic_side_effects,
    clippy::float_arithmetic,
    reason = "Validated physical quantities require constitutive and integration arithmetic."
)]

use crate::{ComponentInstanceId, Computed, SimulationSettings, TranslationalQPhase};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Stable identity of a translational node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MechanicalNodeId(u64);

impl MechanicalNodeId {
    /// Creates a node identity from deterministic source data.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying deterministic value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// A computed translational component bound to model node topology.
#[derive(Debug)]
pub struct TranslationalComponentInstance {
    /// Stable component identity.
    id: ComponentInstanceId,
    /// Mechanical nodes bound in component port order.
    nodes: Vec<MechanicalNodeId>,
    /// Version-specific computed implementation.
    computed: Box<dyn Computed>,
}

impl TranslationalComponentInstance {
    /// Binds an evaluated component to identity and mechanical nodes.
    #[must_use]
    pub const fn new(
        id: ComponentInstanceId,
        nodes: Vec<MechanicalNodeId>,
        computed: Box<dyn Computed>,
    ) -> Self {
        Self {
            id,
            nodes,
            computed,
        }
    }

    /// Returns this instance's stable identity.
    #[must_use]
    pub const fn id(&self) -> ComponentInstanceId {
        self.id
    }
}

/// Failure to define or execute a translational model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranslationalError {
    /// A component parameter violated its contract.
    InvalidParameter {
        /// Component containing the parameter.
        id: ComponentInstanceId,
        /// Stable parameter name.
        key: &'static str,
    },
    /// A spring connects a node to itself.
    CoincidentEndpoints {
        /// Invalid spring component.
        id: ComponentInstanceId,
    },
    /// A bound component received the wrong number of mechanical nodes.
    InvalidNodeCount {
        /// Number of nodes required by the component.
        expected: usize,
        /// Number of nodes supplied by the model.
        actual: usize,
    },
    /// A component binds the same mechanical node to multiple ports.
    CoincidentNodes(MechanicalNodeId),
    /// A computed component does not expose translational behavior.
    NonTranslationalComponent(ComponentInstanceId),
    /// A computed component exposes both C-type and Q-type behavior.
    AmbiguousTranslationalComponent(ComponentInstanceId),
    /// A component identity was repeated.
    DuplicateComponent(ComponentInstanceId),
    /// A spring references a node without a mass or boundary.
    UnboundNode(MechanicalNodeId),
    /// More than one mass or boundary owns a node.
    ConflictingNodeOwner(MechanicalNodeId),
    /// Fixed-step timing is invalid.
    InvalidTiming,
    /// Arithmetic produced a nonfinite state.
    NonFiniteState,
}

impl Display for TranslationalError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "translational simulation failed: {self:?}")
    }
}

impl Error for TranslationalError {}

/// Validated input model for a spring-mass runtime.
#[derive(Debug, Default)]
pub struct TranslationalModel {
    /// Computed components with bound mechanical ports.
    components: Vec<TranslationalComponentInstance>,
    /// Reserved component identities.
    component_ids: BTreeSet<ComponentInstanceId>,
}

impl TranslationalModel {
    /// Creates an empty model.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            components: Vec::new(),
            component_ids: BTreeSet::new(),
        }
    }

    /// Adds a computed component with bound mechanical ports.
    ///
    /// # Errors
    /// Returns an error when its component identity is already used.
    pub fn add_component(
        &mut self,
        component: TranslationalComponentInstance,
    ) -> Result<(), TranslationalError> {
        self.reserve(component.id)?;
        self.components.push(component);
        Ok(())
    }

    /// Reserves one component identity.
    fn reserve(&mut self, id: ComponentInstanceId) -> Result<(), TranslationalError> {
        if self.component_ids.insert(id) {
            Ok(())
        } else {
            Err(TranslationalError::DuplicateComponent(id))
        }
    }
}

/// State of one mechanical node at a sample time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MechanicalNodeState {
    /// Position in meters.
    pub position: f64,
    /// Velocity in meters per second.
    pub velocity: f64,
    /// Acceleration in meters per second squared.
    pub acceleration: f64,
    /// Net spring force before boundary reaction, in newtons.
    pub force: f64,
    /// Constraint reaction force, in newtons.
    pub reaction_force: f64,
    /// TLM wave variable, in newtons.
    pub wave_variable: f64,
    /// TLM characteristic impedance, in newton-seconds per meter.
    pub characteristic_impedance: f64,
    /// Equivalent mass published by the Q-type component, in kilograms.
    pub equivalent_mass: f64,
}

/// One atomically captured mechanical sample.
#[derive(Debug, Clone, PartialEq)]
pub struct TranslationalSample {
    /// Simulation time in seconds.
    pub time: f64,
    /// Node states ordered by stable identity.
    pub nodes: BTreeMap<MechanicalNodeId, MechanicalNodeState>,
    /// Total kinetic plus elastic energy in joules.
    pub stored_energy: f64,
}

/// Completed deterministic spring-mass run.
#[derive(Debug, Clone, PartialEq)]
pub struct TranslationalRun {
    /// Initial sample followed by one sample per transition.
    pub samples: Vec<TranslationalSample>,
    /// Largest absolute stored-energy drift.
    pub maximum_energy_residual: f64,
}

impl TranslationalRun {
    /// Returns position samples for one node.
    #[must_use]
    pub fn node_positions(&self, node: MechanicalNodeId) -> Option<Vec<f64>> {
        self.samples
            .iter()
            .map(|sample| sample.nodes.get(&node).map(|state| state.position))
            .collect()
    }
}

/// Hopsan-style C/Q runtime with velocity-Verlet state integration.
#[derive(Debug)]
pub struct TranslationalRuntime {
    /// Components indexed by stable identity.
    components: BTreeMap<ComponentInstanceId, TranslationalComponentInstance>,
    /// Stable C-type execution order.
    c_schedule: Vec<ComponentInstanceId>,
    /// Stable Q-type execution order.
    q_schedule: Vec<ComponentInstanceId>,
    /// Shared mechanical node storage.
    nodes: BTreeMap<MechanicalNodeId, MechanicalNodeState>,
}

impl TranslationalRuntime {
    /// Creates a runtime from a validated model.
    ///
    /// # Errors
    /// Returns an error for conflicting owners or unbound spring nodes.
    pub fn new(model: TranslationalModel) -> Result<Self, TranslationalError> {
        let mut components: BTreeMap<_, _> = model
            .components
            .into_iter()
            .map(|component| (component.id, component))
            .collect();
        let mut c_schedule = Vec::new();
        let mut q_schedule = Vec::new();
        let mut owners = BTreeSet::new();
        for (id, component) in &mut components {
            let is_c = component.computed.as_component_c().is_some();
            let is_q = component.computed.as_component_q().is_some();
            match (is_c, is_q) {
                (true, false) => c_schedule.push(*id),
                (false, true) => {
                    for node in &component.nodes {
                        if !owners.insert(*node) {
                            return Err(TranslationalError::ConflictingNodeOwner(*node));
                        }
                    }
                    q_schedule.push(*id);
                }
                (false, false) => {
                    return Err(TranslationalError::NonTranslationalComponent(*id));
                }
                (true, true) => {
                    return Err(TranslationalError::AmbiguousTranslationalComponent(*id));
                }
            }
        }
        for id in &c_schedule {
            let component = components
                .get(id)
                .ok_or(TranslationalError::NonTranslationalComponent(*id))?;
            for node in &component.nodes {
                if !owners.contains(node) {
                    return Err(TranslationalError::UnboundNode(*node));
                }
            }
        }
        Ok(Self {
            components,
            c_schedule,
            q_schedule,
            nodes: BTreeMap::new(),
        })
    }

    /// Executes C-type springs before Q-type masses and boundaries each timestep.
    ///
    /// # Errors
    /// Returns an error for invalid timing or nonfinite state.
    #[allow(
        clippy::as_conversions,
        clippy::cast_precision_loss,
        reason = "Validated step indices are converted for deterministic sample times."
    )]
    pub fn run(
        &mut self,
        settings: SimulationSettings,
    ) -> Result<TranslationalRun, TranslationalError> {
        if !settings.timestep.is_finite() || settings.timestep <= 0.0 {
            return Err(TranslationalError::InvalidTiming);
        }
        self.initialize()?;
        self.simulate_c_components(settings.timestep)?;
        self.simulate_q_components(TranslationalQPhase::Respond, settings.timestep)?;
        let initial_energy = self.energy()?;
        let mut samples = vec![self.sample(0.0)?];
        let mut maximum_energy_residual = 0.0_f64;

        for step in 1..=settings.steps {
            self.simulate_q_components(TranslationalQPhase::Predict, settings.timestep)?;
            self.simulate_c_components(settings.timestep)?;
            self.simulate_q_components(TranslationalQPhase::Respond, settings.timestep)?;
            self.simulate_q_components(TranslationalQPhase::Correct, settings.timestep)?;
            self.simulate_c_components(settings.timestep)?;
            self.simulate_q_components(TranslationalQPhase::Respond, settings.timestep)?;
            ensure_finite(&self.nodes)?;
            let sample = self.sample(settings.timestep * step as f64)?;
            maximum_energy_residual =
                maximum_energy_residual.max((sample.stored_energy - initial_energy).abs());
            samples.push(sample);
        }
        self.finalize()?;
        Ok(TranslationalRun {
            samples,
            maximum_energy_residual,
        })
    }

    /// Restores component and node state in Q-before-C initialization order.
    fn initialize(&mut self) -> Result<(), TranslationalError> {
        self.nodes.clear();
        for id in self.q_schedule.clone() {
            let component = self
                .components
                .get_mut(&id)
                .ok_or(TranslationalError::NonTranslationalComponent(id))?;
            let bound_nodes = component.nodes.clone();
            component
                .computed
                .as_component_q()
                .ok_or(TranslationalError::NonTranslationalComponent(id))?
                .initialize_q(id, &bound_nodes, &mut self.nodes)?;
        }
        for id in self.c_schedule.clone() {
            let component = self
                .components
                .get_mut(&id)
                .ok_or(TranslationalError::NonTranslationalComponent(id))?;
            let bound_nodes = component.nodes.clone();
            component
                .computed
                .as_component_c()
                .ok_or(TranslationalError::NonTranslationalComponent(id))?
                .initialize_c(id, &bound_nodes, &mut self.nodes)?;
        }
        ensure_finite(&self.nodes)
    }

    /// Simulates C-type springs and publishes wave variables and impedances.
    fn simulate_c_components(&mut self, timestep: f64) -> Result<(), TranslationalError> {
        for state in self.nodes.values_mut() {
            state.wave_variable = 0.0;
            state.characteristic_impedance = 0.0;
        }
        for id in self.c_schedule.clone() {
            let component = self
                .components
                .get_mut(&id)
                .ok_or(TranslationalError::NonTranslationalComponent(id))?;
            let bound_nodes = component.nodes.clone();
            component
                .computed
                .as_component_c()
                .ok_or(TranslationalError::NonTranslationalComponent(id))?
                .simulate_c(timestep, &bound_nodes, &mut self.nodes)?;
        }
        ensure_finite(&self.nodes)
    }

    /// Simulates one velocity-Verlet phase for all Q-type components.
    fn simulate_q_components(
        &mut self,
        phase: TranslationalQPhase,
        timestep: f64,
    ) -> Result<(), TranslationalError> {
        for id in self.q_schedule.clone() {
            let component = self
                .components
                .get_mut(&id)
                .ok_or(TranslationalError::NonTranslationalComponent(id))?;
            let bound_nodes = component.nodes.clone();
            component
                .computed
                .as_component_q()
                .ok_or(TranslationalError::NonTranslationalComponent(id))?
                .simulate_q(phase, timestep, &bound_nodes, &mut self.nodes)?;
        }
        ensure_finite(&self.nodes)
    }

    /// Computes total stored kinetic and elastic energy.
    fn energy(&mut self) -> Result<f64, TranslationalError> {
        let mut energy = 0.0;
        for id in self.c_schedule.clone() {
            let component = self
                .components
                .get_mut(&id)
                .ok_or(TranslationalError::NonTranslationalComponent(id))?;
            let bound_nodes = component.nodes.clone();
            energy += component
                .computed
                .as_component_c()
                .ok_or(TranslationalError::NonTranslationalComponent(id))?
                .energy_c(&bound_nodes, &self.nodes)?;
        }
        for id in self.q_schedule.clone() {
            let component = self
                .components
                .get_mut(&id)
                .ok_or(TranslationalError::NonTranslationalComponent(id))?;
            let bound_nodes = component.nodes.clone();
            energy += component
                .computed
                .as_component_q()
                .ok_or(TranslationalError::NonTranslationalComponent(id))?
                .energy_q(&bound_nodes, &self.nodes)?;
        }
        if energy.is_finite() {
            Ok(energy)
        } else {
            Err(TranslationalError::NonFiniteState)
        }
    }

    /// Captures one immutable sample.
    fn sample(&mut self, time: f64) -> Result<TranslationalSample, TranslationalError> {
        Ok(TranslationalSample {
            time,
            nodes: self.nodes.clone(),
            stored_energy: self.energy()?,
        })
    }

    /// Finalizes all computed components after a successful run.
    fn finalize(&mut self) -> Result<(), TranslationalError> {
        for id in self.c_schedule.clone() {
            let component = self
                .components
                .get_mut(&id)
                .ok_or(TranslationalError::NonTranslationalComponent(id))?;
            component
                .computed
                .as_component_c()
                .ok_or(TranslationalError::NonTranslationalComponent(id))?
                .finalize_c();
        }
        for id in self.q_schedule.clone() {
            let component = self
                .components
                .get_mut(&id)
                .ok_or(TranslationalError::NonTranslationalComponent(id))?;
            component
                .computed
                .as_component_q()
                .ok_or(TranslationalError::NonTranslationalComponent(id))?
                .finalize_q();
        }
        Ok(())
    }
}

/// Rejects any nonfinite mechanical state.
fn ensure_finite(
    nodes: &BTreeMap<MechanicalNodeId, MechanicalNodeState>,
) -> Result<(), TranslationalError> {
    if nodes.values().all(|state| {
        state.position.is_finite()
            && state.velocity.is_finite()
            && state.acceleration.is_finite()
            && state.force.is_finite()
            && state.reaction_force.is_finite()
            && state.wave_variable.is_finite()
            && state.characteristic_impedance.is_finite()
            && state.equivalent_mass.is_finite()
    }) {
        Ok(())
    } else {
        Err(TranslationalError::NonFiniteState)
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Test fixtures unwrap constructor inputs that are literals satisfying their contracts."
)]
mod tests {
    use super::*;
    use crate::built_in_registry::translational::fixed_boundary_v1::fixed_boundary_component::FixedBoundaryV1Computed;
    use crate::built_in_registry::translational::mass_v1::mass_component::MassV1Computed;
    use crate::built_in_registry::translational::spring_v1::spring_component::SpringV1Computed;

    fn oscillator() -> TranslationalModel {
        let moving = MechanicalNodeId::new(1);
        let ground = MechanicalNodeId::new(2);
        let mut model = TranslationalModel::new();
        assert!(
            model
                .add_component(TranslationalComponentInstance::new(
                    ComponentInstanceId::new(1),
                    vec![moving],
                    Box::new(MassV1Computed::new(1.0, 1.0, 0.0)),
                ))
                .is_ok()
        );
        assert!(
            model
                .add_component(TranslationalComponentInstance::new(
                    ComponentInstanceId::new(2),
                    vec![ground],
                    Box::new(FixedBoundaryV1Computed::new(0.0)),
                ))
                .is_ok()
        );
        assert!(
            model
                .add_component(TranslationalComponentInstance::new(
                    ComponentInstanceId::new(3),
                    vec![moving, ground],
                    Box::new(SpringV1Computed::new(1.0, 0.0)),
                ))
                .is_ok()
        );
        model
    }

    #[test]
    fn spring_mass_oscillates_and_resets_exactly() {
        let Ok(mut runtime) = TranslationalRuntime::new(oscillator()) else {
            panic!("oscillator must validate");
        };
        let settings = SimulationSettings {
            timestep: 0.01,
            steps: 628,
        };
        let Ok(first) = runtime.run(settings) else {
            panic!("oscillator must run");
        };
        let Ok(second) = runtime.run(settings) else {
            panic!("oscillator must reset");
        };

        assert_eq!(first, second);
        let Some(positions) = first.node_positions(MechanicalNodeId::new(1)) else {
            panic!("moving node must be sampled");
        };
        assert!(positions.last().is_some_and(|position| *position > 0.999));
        assert!(first.maximum_energy_residual < 0.000_02);
        let Some(initial) = first.samples.first().map(|sample| &sample.nodes) else {
            panic!("initial sample must be recorded");
        };
        assert_eq!(
            initial
                .get(&MechanicalNodeId::new(1))
                .map(|state| state.characteristic_impedance),
            Some(0.01)
        );
        assert_eq!(
            initial
                .get(&MechanicalNodeId::new(1))
                .map(|state| state.equivalent_mass),
            Some(1.0)
        );
        assert!(
            first
                .samples
                .iter()
                .all(|sample| sample.nodes.values().all(|state| (state.force
                    - state.wave_variable
                    - state.characteristic_impedance * state.velocity)
                    .abs()
                    < f64::EPSILON))
        );
    }

    #[test]
    fn unbound_spring_node_is_rejected() {
        let owned = MechanicalNodeId::new(1);
        let unbound = MechanicalNodeId::new(2);
        let mut model = TranslationalModel::new();
        assert!(
            model
                .add_component(TranslationalComponentInstance::new(
                    ComponentInstanceId::new(1),
                    vec![owned],
                    Box::new(MassV1Computed::new(1.0, 0.0, 0.0)),
                ))
                .is_ok()
        );
        assert!(
            model
                .add_component(TranslationalComponentInstance::new(
                    ComponentInstanceId::new(2),
                    vec![owned, unbound],
                    Box::new(SpringV1Computed::new(1.0, 0.0)),
                ))
                .is_ok()
        );
        assert!(matches!(
            TranslationalRuntime::new(model),
            Err(TranslationalError::UnboundNode(node)) if node == MechanicalNodeId::new(2)
        ));
    }
}
