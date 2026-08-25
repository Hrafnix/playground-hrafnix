//! Deterministic one-dimensional translational mechanics.
//!
//! This execution path is separate from directed signal scheduling because
//! physical connections are bidirectional conservative nodes.

#![allow(
    clippy::arithmetic_side_effects,
    clippy::float_arithmetic,
    reason = "Validated finite physical quantities require explicit constitutive and integration arithmetic."
)]

use crate::identity::{ComponentId, NodeId};
use std::collections::{BTreeMap, BTreeSet};

/// A lumped mass that owns the dynamic state of one free node.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mass {
    /// Stable component identity.
    pub id: ComponentId,
    /// Physical node carrying position, velocity, and force.
    pub node: NodeId,
    /// Positive mass in kilograms.
    pub mass: f64,
    /// Position at initialization in meters.
    pub initial_position: f64,
    /// Velocity at initialization in meters per second.
    pub initial_velocity: f64,
}

impl Mass {
    /// Creates a validated lumped mass.
    ///
    /// # Errors
    ///
    /// Returns an error when mass is not finite and positive or an initial
    /// condition is not finite.
    pub fn new(
        id: ComponentId,
        node: NodeId,
        mass: f64,
        initial_position: f64,
        initial_velocity: f64,
    ) -> Result<Self, TranslationalError> {
        if !mass.is_finite() || mass <= 0.0 {
            return Err(TranslationalError::InvalidParameter { id, key: "mass" });
        }
        if !initial_position.is_finite() {
            return Err(TranslationalError::InvalidParameter {
                id,
                key: "initial_position",
            });
        }
        if !initial_velocity.is_finite() {
            return Err(TranslationalError::InvalidParameter {
                id,
                key: "initial_velocity",
            });
        }
        Ok(Self {
            id,
            node,
            mass,
            initial_position,
            initial_velocity,
        })
    }
}

/// An ideal linear spring between two physical nodes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spring {
    /// Stable component identity.
    pub id: ComponentId,
    /// Positive-force reference endpoint.
    pub node_a: NodeId,
    /// Opposite endpoint.
    pub node_b: NodeId,
    /// Positive stiffness in newtons per meter.
    pub stiffness: f64,
    /// Unloaded signed displacement `position_a - position_b` in meters.
    pub free_length: f64,
}

impl Spring {
    /// Creates a validated ideal spring.
    ///
    /// # Errors
    ///
    /// Returns an error for coincident endpoints, nonpositive stiffness, or a
    /// nonfinite free length.
    pub fn new(
        id: ComponentId,
        node_a: NodeId,
        node_b: NodeId,
        stiffness: f64,
        free_length: f64,
    ) -> Result<Self, TranslationalError> {
        if node_a == node_b {
            return Err(TranslationalError::CoincidentEndpoints { id });
        }
        if !stiffness.is_finite() || stiffness <= 0.0 {
            return Err(TranslationalError::InvalidParameter {
                id,
                key: "stiffness",
            });
        }
        if !free_length.is_finite() {
            return Err(TranslationalError::InvalidParameter {
                id,
                key: "free_length",
            });
        }
        Ok(Self {
            id,
            node_a,
            node_b,
            stiffness,
            free_length,
        })
    }
}

/// An ideal viscous damper between two physical nodes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Damper {
    /// Stable component identity.
    pub id: ComponentId,
    /// Positive-force reference endpoint.
    pub node_a: NodeId,
    /// Opposite endpoint.
    pub node_b: NodeId,
    /// Positive damping coefficient in newton-seconds per meter.
    pub damping: f64,
}

impl Damper {
    /// Creates a validated ideal viscous damper.
    ///
    /// # Errors
    ///
    /// Returns an error for coincident endpoints or nonpositive damping.
    pub fn new(
        id: ComponentId,
        node_a: NodeId,
        node_b: NodeId,
        damping: f64,
    ) -> Result<Self, TranslationalError> {
        if node_a == node_b {
            return Err(TranslationalError::CoincidentEndpoints { id });
        }
        if !damping.is_finite() || damping <= 0.0 {
            return Err(TranslationalError::InvalidParameter { id, key: "damping" });
        }
        Ok(Self {
            id,
            node_a,
            node_b,
            damping,
        })
    }
}

/// A constant external force applied in the positive coordinate direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ForceSource {
    /// Stable component identity.
    pub id: ComponentId,
    /// Node receiving the force.
    pub node: NodeId,
    /// Applied force in newtons.
    pub force: f64,
}

impl ForceSource {
    /// Creates a validated constant force source.
    ///
    /// # Errors
    ///
    /// Returns an error when force is not finite.
    pub const fn new(
        id: ComponentId,
        node: NodeId,
        force: f64,
    ) -> Result<Self, TranslationalError> {
        if !force.is_finite() {
            return Err(TranslationalError::InvalidParameter { id, key: "force" });
        }
        Ok(Self { id, node, force })
    }
}

/// A prescribed stationary node.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FixedBoundary {
    /// Stable component identity.
    pub id: ComponentId,
    /// Constrained node.
    pub node: NodeId,
    /// Fixed position in meters.
    pub position: f64,
}

impl FixedBoundary {
    /// Creates a stationary boundary.
    ///
    /// # Errors
    ///
    /// Returns an error when position is not finite.
    pub const fn new(
        id: ComponentId,
        node: NodeId,
        position: f64,
    ) -> Result<Self, TranslationalError> {
        if !position.is_finite() {
            return Err(TranslationalError::InvalidParameter {
                id,
                key: "position",
            });
        }
        Ok(Self { id, node, position })
    }
}

/// A node following a prescribed constant-acceleration trajectory.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotionSource {
    /// Stable component identity.
    pub id: ComponentId,
    /// Constrained node.
    pub node: NodeId,
    /// Position at time zero in meters.
    pub initial_position: f64,
    /// Velocity at time zero in meters per second.
    pub initial_velocity: f64,
    /// Constant acceleration in meters per second squared.
    pub acceleration: f64,
}

impl MotionSource {
    /// Creates a validated prescribed motion source.
    ///
    /// # Errors
    ///
    /// Returns an error when any trajectory coefficient is not finite.
    pub fn new(
        id: ComponentId,
        node: NodeId,
        initial_position: f64,
        initial_velocity: f64,
        acceleration: f64,
    ) -> Result<Self, TranslationalError> {
        for (key, value) in [
            ("initial_position", initial_position),
            ("initial_velocity", initial_velocity),
            ("acceleration", acceleration),
        ] {
            if !value.is_finite() {
                return Err(TranslationalError::InvalidParameter { id, key });
            }
        }
        Ok(Self {
            id,
            node,
            initial_position,
            initial_velocity,
            acceleration,
        })
    }

    /// Evaluates the prescribed position and velocity at one time.
    #[allow(
        clippy::arithmetic_side_effects,
        clippy::float_arithmetic,
        reason = "Validated finite polynomial coefficients are checked after trajectory evaluation."
    )]
    fn state_at(self, time: f64) -> Result<NodeState, TranslationalError> {
        let position = self.initial_position
            + self.initial_velocity * time
            + 0.5 * self.acceleration * time * time;
        let velocity = self.initial_velocity + self.acceleration * time;
        let state = NodeState {
            position,
            velocity,
            acceleration: self.acceleration,
            force: 0.0,
            reaction_force: 0.0,
        };
        if !state_is_finite(state) {
            return Err(TranslationalError::NonFiniteState);
        }
        Ok(state)
    }
}

/// A nonintrusive position measurement attached to one node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositionSensor {
    /// Stable component identity.
    pub id: ComponentId,
    /// Measured node.
    pub node: NodeId,
}

impl PositionSensor {
    /// Creates a position sensor.
    #[must_use]
    pub const fn new(id: ComponentId, node: NodeId) -> Self {
        Self { id, node }
    }
}

/// Failure to define or execute a translational model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranslationalError {
    /// A numeric component parameter violated its contract.
    InvalidParameter {
        /// Component containing the parameter.
        id: ComponentId,
        /// Stable parameter key.
        key: &'static str,
    },
    /// A two-node component connected both ports to one node.
    CoincidentEndpoints {
        /// Invalid component.
        id: ComponentId,
    },
    /// A component identity was used more than once.
    DuplicateComponent {
        /// Repeated identity.
        id: ComponentId,
    },
    /// A component references a node without a state owner or boundary.
    UnboundNode {
        /// Missing node.
        node: NodeId,
    },
    /// More than one state owner or boundary controls a node.
    ConflictingNodeOwner {
        /// Multiply-owned node.
        node: NodeId,
    },
    /// Fixed-step settings were invalid.
    InvalidTiming,
    /// Arithmetic produced a nonfinite physical value.
    NonFiniteState,
}

/// Validated collection of translational mechanics components.
#[derive(Debug, Clone, Default)]
pub struct TranslationalModel {
    /// Dynamic state owners.
    masses: Vec<Mass>,
    /// Elastic force contributors.
    springs: Vec<Spring>,
    /// Dissipative force contributors.
    dampers: Vec<Damper>,
    /// External effort sources.
    force_sources: Vec<ForceSource>,
    /// Prescribed stationary nodes.
    fixed_boundaries: Vec<FixedBoundary>,
    /// Prescribed moving nodes.
    motion_sources: Vec<MotionSource>,
    /// Nonintrusive position measurements.
    position_sensors: Vec<PositionSensor>,
    /// All stable component identities.
    component_ids: BTreeSet<ComponentId>,
}

impl TranslationalModel {
    /// Creates an empty translational model.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            masses: Vec::new(),
            springs: Vec::new(),
            dampers: Vec::new(),
            force_sources: Vec::new(),
            fixed_boundaries: Vec::new(),
            motion_sources: Vec::new(),
            position_sensors: Vec::new(),
            component_ids: BTreeSet::new(),
        }
    }

    /// Adds a lumped mass.
    ///
    /// # Errors
    ///
    /// Returns an error when its component ID is already present.
    pub fn add_mass(&mut self, mass: Mass) -> Result<(), TranslationalError> {
        self.reserve_id(mass.id)?;
        self.masses.push(mass);
        Ok(())
    }

    /// Adds an ideal spring.
    ///
    /// # Errors
    ///
    /// Returns an error when its component ID is already present.
    pub fn add_spring(&mut self, spring: Spring) -> Result<(), TranslationalError> {
        self.reserve_id(spring.id)?;
        self.springs.push(spring);
        Ok(())
    }

    /// Adds an ideal viscous damper.
    ///
    /// # Errors
    ///
    /// Returns an error when its component ID is already present.
    pub fn add_damper(&mut self, damper: Damper) -> Result<(), TranslationalError> {
        self.reserve_id(damper.id)?;
        self.dampers.push(damper);
        Ok(())
    }

    /// Adds a constant external force source.
    ///
    /// # Errors
    ///
    /// Returns an error when its component ID is already present.
    pub fn add_force_source(&mut self, source: ForceSource) -> Result<(), TranslationalError> {
        self.reserve_id(source.id)?;
        self.force_sources.push(source);
        Ok(())
    }

    /// Adds a stationary boundary.
    ///
    /// # Errors
    ///
    /// Returns an error when its component ID is already present.
    pub fn add_fixed_boundary(
        &mut self,
        boundary: FixedBoundary,
    ) -> Result<(), TranslationalError> {
        self.reserve_id(boundary.id)?;
        self.fixed_boundaries.push(boundary);
        Ok(())
    }

    /// Adds a prescribed moving boundary.
    ///
    /// # Errors
    ///
    /// Returns an error when its component ID is already present.
    pub fn add_motion_source(&mut self, source: MotionSource) -> Result<(), TranslationalError> {
        self.reserve_id(source.id)?;
        self.motion_sources.push(source);
        Ok(())
    }

    /// Adds a nonintrusive position sensor.
    ///
    /// # Errors
    ///
    /// Returns an error when its component ID is already present.
    pub fn add_position_sensor(
        &mut self,
        sensor: PositionSensor,
    ) -> Result<(), TranslationalError> {
        self.reserve_id(sensor.id)?;
        self.position_sensors.push(sensor);
        Ok(())
    }

    /// Reserves one stable component identity.
    fn reserve_id(&mut self, id: ComponentId) -> Result<(), TranslationalError> {
        if !self.component_ids.insert(id) {
            return Err(TranslationalError::DuplicateComponent { id });
        }
        Ok(())
    }

    /// Validates node ownership and references.
    fn validate(&self) -> Result<(), TranslationalError> {
        let mut owners = BTreeSet::new();
        for node in self
            .masses
            .iter()
            .map(|mass| mass.node)
            .chain(self.fixed_boundaries.iter().map(|boundary| boundary.node))
            .chain(self.motion_sources.iter().map(|source| source.node))
        {
            if !owners.insert(node) {
                return Err(TranslationalError::ConflictingNodeOwner { node });
            }
        }
        for node in self
            .springs
            .iter()
            .flat_map(|spring| [spring.node_a, spring.node_b])
            .chain(
                self.dampers
                    .iter()
                    .flat_map(|damper| [damper.node_a, damper.node_b]),
            )
            .chain(self.force_sources.iter().map(|source| source.node))
            .chain(self.position_sensors.iter().map(|sensor| sensor.node))
        {
            if !owners.contains(&node) {
                return Err(TranslationalError::UnboundNode { node });
            }
        }
        Ok(())
    }
}

/// Fixed-step execution settings for a translational run.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TranslationalRunSettings {
    /// Positive constant timestep in seconds.
    pub timestep: f64,
    /// Number of state transitions after the initial sample.
    pub step_count: u64,
}

impl TranslationalRunSettings {
    /// Creates validated fixed-step settings.
    ///
    /// # Errors
    ///
    /// Returns an error for a nonfinite or nonpositive timestep.
    pub fn new(timestep: f64, step_count: u64) -> Result<Self, TranslationalError> {
        if !timestep.is_finite() || timestep <= 0.0 {
            return Err(TranslationalError::InvalidTiming);
        }
        Ok(Self {
            timestep,
            step_count,
        })
    }
}

/// State of one translational node at a sample time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeState {
    /// Position in meters.
    pub position: f64,
    /// Velocity in meters per second.
    pub velocity: f64,
    /// Acceleration in meters per second squared.
    pub acceleration: f64,
    /// Net constitutive force before a boundary reaction, in newtons.
    pub force: f64,
    /// Constraint force applied by a fixed or moving boundary, in newtons.
    pub reaction_force: f64,
}

/// Energy accounting for one physical sample.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnergyDiagnostic {
    /// Kinetic energy in joules.
    pub kinetic: f64,
    /// Elastic potential energy in joules.
    pub elastic: f64,
    /// Damping energy dissipated since initialization, in joules.
    pub dissipated: f64,
    /// Work supplied by force and motion sources since initialization, in joules.
    pub external_work: f64,
    /// `stored + dissipated - external_work - initial_stored`, in joules.
    pub balance_residual: f64,
}

impl EnergyDiagnostic {
    /// Returns total stored kinetic plus elastic energy.
    #[must_use]
    pub fn stored(self) -> f64 {
        self.kinetic + self.elastic
    }
}

/// One atomically committed physical sample.
#[derive(Debug, Clone, PartialEq)]
pub struct TranslationalSample {
    /// Simulation time in seconds.
    pub time: f64,
    /// Node states ordered by stable node identity.
    pub nodes: BTreeMap<NodeId, NodeState>,
    /// Signed force reported by each force-producing component.
    pub component_forces: BTreeMap<ComponentId, f64>,
    /// Energy accounting at this sample.
    pub energy: EnergyDiagnostic,
}

/// Completed deterministic translational run.
#[derive(Debug, Clone, PartialEq)]
pub struct TranslationalRun {
    /// Initial sample followed by one sample per transition.
    pub samples: Vec<TranslationalSample>,
    /// Position samples keyed by position-sensor component ID.
    pub position_series: BTreeMap<ComponentId, Vec<f64>>,
    /// Largest absolute energy residual observed in the run.
    pub maximum_energy_residual: f64,
}

impl TranslationalRun {
    /// Returns position samples for one sensor.
    #[must_use]
    pub fn sensor_positions(&self, sensor_id: ComponentId) -> Option<&[f64]> {
        self.position_series.get(&sensor_id).map(Vec::as_slice)
    }
}

/// Executes one validated translational model using velocity Verlet.
#[derive(Debug, Clone)]
pub struct TranslationalRuntime {
    /// Immutable component graph.
    model: TranslationalModel,
}

impl TranslationalRuntime {
    /// Builds a runtime from a validated immutable model snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error for missing or conflicting node owners.
    pub fn new(model: &TranslationalModel) -> Result<Self, TranslationalError> {
        model.validate()?;
        Ok(Self {
            model: model.clone(),
        })
    }

    /// Runs from component initial conditions and captures the initial sample.
    ///
    /// # Errors
    ///
    /// Returns an error if arithmetic produces a nonfinite state.
    #[allow(
        clippy::as_conversions,
        clippy::cast_precision_loss,
        reason = "Validated bounded step indices are converted to compute nonaccumulating sample times."
    )]
    pub fn run(
        &self,
        settings: TranslationalRunSettings,
    ) -> Result<TranslationalRun, TranslationalError> {
        let mut nodes = self.initial_nodes()?;
        self.update_forces_and_accelerations(&mut nodes)?;
        let (initial_kinetic, initial_elastic) = self.stored_energy(&nodes);
        let initial_energy = initial_kinetic + initial_elastic;
        let mut external_work = 0.0_f64;
        let mut dissipated = 0.0_f64;
        let mut previous_external_power = self.external_power(&nodes)?;
        let mut previous_dissipation_power = self.dissipation_power(&nodes)?;
        let mut samples =
            vec![self.sample(0.0, &nodes, initial_energy, external_work, dissipated)?];
        let mut position_series = self.initial_sensor_series(&nodes)?;
        let mut maximum_energy_residual = 0.0_f64;

        for step_index in 1..=settings.step_count {
            let previous_accelerations = self
                .model
                .masses
                .iter()
                .map(|mass| {
                    nodes
                        .get(&mass.node)
                        .map(|state| (mass.node, state.acceleration))
                        .ok_or(TranslationalError::UnboundNode { node: mass.node })
                })
                .collect::<Result<BTreeMap<_, _>, _>>()?;
            for mass in &self.model.masses {
                let state = nodes
                    .get_mut(&mass.node)
                    .ok_or(TranslationalError::UnboundNode { node: mass.node })?;
                state.position += state.velocity * settings.timestep
                    + 0.5 * state.acceleration * settings.timestep * settings.timestep;
                state.velocity += state.acceleration * settings.timestep;
            }
            let time = settings.timestep * step_index as f64;
            self.update_motion_sources(time, &mut nodes)?;
            self.update_forces_and_accelerations(&mut nodes)?;
            for mass in &self.model.masses {
                let previous = previous_accelerations
                    .get(&mass.node)
                    .copied()
                    .ok_or(TranslationalError::UnboundNode { node: mass.node })?;
                let state = nodes
                    .get_mut(&mass.node)
                    .ok_or(TranslationalError::UnboundNode { node: mass.node })?;
                state.velocity += 0.5 * (state.acceleration - previous) * settings.timestep;
            }
            self.update_forces_and_accelerations(&mut nodes)?;

            let external_power = self.external_power(&nodes)?;
            let dissipation_power = self.dissipation_power(&nodes)?;
            external_work +=
                f64::midpoint(previous_external_power, external_power) * settings.timestep;
            dissipated +=
                f64::midpoint(previous_dissipation_power, dissipation_power) * settings.timestep;
            previous_external_power = external_power;
            previous_dissipation_power = dissipation_power;
            let sample = self.sample(time, &nodes, initial_energy, external_work, dissipated)?;
            maximum_energy_residual =
                maximum_energy_residual.max(sample.energy.balance_residual.abs());
            self.capture_sensor_positions(&nodes, &mut position_series)?;
            samples.push(sample);
        }

        Ok(TranslationalRun {
            samples,
            position_series,
            maximum_energy_residual,
        })
    }

    /// Creates initialized node state from masses and boundaries.
    fn initial_nodes(&self) -> Result<BTreeMap<NodeId, NodeState>, TranslationalError> {
        let mut nodes = BTreeMap::new();
        for mass in &self.model.masses {
            nodes.insert(
                mass.node,
                NodeState {
                    position: mass.initial_position,
                    velocity: mass.initial_velocity,
                    acceleration: 0.0,
                    force: 0.0,
                    reaction_force: 0.0,
                },
            );
        }
        for boundary in &self.model.fixed_boundaries {
            nodes.insert(
                boundary.node,
                NodeState {
                    position: boundary.position,
                    velocity: 0.0,
                    acceleration: 0.0,
                    force: 0.0,
                    reaction_force: 0.0,
                },
            );
        }
        for source in &self.model.motion_sources {
            nodes.insert(source.node, source.state_at(0.0)?);
        }
        if nodes.values().any(|state| !state_is_finite(*state)) {
            return Err(TranslationalError::NonFiniteState);
        }
        Ok(nodes)
    }

    /// Reassembles constitutive forces, accelerations, and reactions.
    fn update_forces_and_accelerations(
        &self,
        nodes: &mut BTreeMap<NodeId, NodeState>,
    ) -> Result<(), TranslationalError> {
        for state in nodes.values_mut() {
            state.force = 0.0;
            state.reaction_force = 0.0;
        }
        for spring in &self.model.springs {
            let position_a = node_state(nodes, spring.node_a)?.position;
            let position_b = node_state(nodes, spring.node_b)?.position;
            let force_on_a = -spring.stiffness * (position_a - position_b - spring.free_length);
            add_force(nodes, spring.node_a, force_on_a)?;
            add_force(nodes, spring.node_b, -force_on_a)?;
        }
        for damper in &self.model.dampers {
            let velocity_a = node_state(nodes, damper.node_a)?.velocity;
            let velocity_b = node_state(nodes, damper.node_b)?.velocity;
            let force_on_a = -damper.damping * (velocity_a - velocity_b);
            add_force(nodes, damper.node_a, force_on_a)?;
            add_force(nodes, damper.node_b, -force_on_a)?;
        }
        for source in &self.model.force_sources {
            add_force(nodes, source.node, source.force)?;
        }
        for mass in &self.model.masses {
            let state = nodes
                .get_mut(&mass.node)
                .ok_or(TranslationalError::UnboundNode { node: mass.node })?;
            state.acceleration = state.force / mass.mass;
        }
        for boundary in &self.model.fixed_boundaries {
            let state = nodes
                .get_mut(&boundary.node)
                .ok_or(TranslationalError::UnboundNode {
                    node: boundary.node,
                })?;
            state.reaction_force = -state.force;
        }
        for source in &self.model.motion_sources {
            let state = nodes
                .get_mut(&source.node)
                .ok_or(TranslationalError::UnboundNode { node: source.node })?;
            state.reaction_force = -state.force;
        }
        if nodes.values().any(|state| !state_is_finite(*state)) {
            return Err(TranslationalError::NonFiniteState);
        }
        Ok(())
    }

    /// Computes kinetic and elastic potential energy.
    fn stored_energy(&self, nodes: &BTreeMap<NodeId, NodeState>) -> (f64, f64) {
        let kinetic = self
            .model
            .masses
            .iter()
            .map(|mass| {
                let velocity = nodes.get(&mass.node).map_or(0.0, |state| state.velocity);
                0.5 * mass.mass * velocity * velocity
            })
            .sum::<f64>();
        let elastic = self
            .model
            .springs
            .iter()
            .map(|spring| {
                let position_a = nodes
                    .get(&spring.node_a)
                    .map_or(0.0, |state| state.position);
                let position_b = nodes
                    .get(&spring.node_b)
                    .map_or(0.0, |state| state.position);
                let extension = position_a - position_b - spring.free_length;
                0.5 * spring.stiffness * extension * extension
            })
            .sum::<f64>();
        (kinetic, elastic)
    }

    /// Updates every prescribed motion source at one sample time.
    fn update_motion_sources(
        &self,
        time: f64,
        nodes: &mut BTreeMap<NodeId, NodeState>,
    ) -> Result<(), TranslationalError> {
        for source in &self.model.motion_sources {
            nodes.insert(source.node, source.state_at(time)?);
        }
        Ok(())
    }

    /// Computes instantaneous power supplied by external sources.
    fn external_power(
        &self,
        nodes: &BTreeMap<NodeId, NodeState>,
    ) -> Result<f64, TranslationalError> {
        let force_source_power = self
            .model
            .force_sources
            .iter()
            .map(|source| node_state(nodes, source.node).map(|state| source.force * state.velocity))
            .sum::<Result<f64, _>>()?;
        let motion_source_power = self
            .model
            .motion_sources
            .iter()
            .map(|source| {
                node_state(nodes, source.node).map(|state| state.reaction_force * state.velocity)
            })
            .sum::<Result<f64, _>>()?;
        Ok(force_source_power + motion_source_power)
    }

    /// Computes instantaneous viscous dissipation power.
    fn dissipation_power(
        &self,
        nodes: &BTreeMap<NodeId, NodeState>,
    ) -> Result<f64, TranslationalError> {
        self.model
            .dampers
            .iter()
            .map(|damper| {
                let relative_velocity = node_state(nodes, damper.node_a)?.velocity
                    - node_state(nodes, damper.node_b)?.velocity;
                Ok(damper.damping * relative_velocity * relative_velocity)
            })
            .sum()
    }

    /// Reports the signed force at each force-producing component's first port.
    fn component_forces(
        &self,
        nodes: &BTreeMap<NodeId, NodeState>,
    ) -> Result<BTreeMap<ComponentId, f64>, TranslationalError> {
        let mut forces = BTreeMap::new();
        for mass in &self.model.masses {
            forces.insert(mass.id, node_state(nodes, mass.node)?.force);
        }
        for spring in &self.model.springs {
            let state_a = node_state(nodes, spring.node_a)?;
            let state_b = node_state(nodes, spring.node_b)?;
            forces.insert(
                spring.id,
                -spring.stiffness * (state_a.position - state_b.position - spring.free_length),
            );
        }
        for damper in &self.model.dampers {
            let state_a = node_state(nodes, damper.node_a)?;
            let state_b = node_state(nodes, damper.node_b)?;
            forces.insert(
                damper.id,
                -damper.damping * (state_a.velocity - state_b.velocity),
            );
        }
        for source in &self.model.force_sources {
            forces.insert(source.id, source.force);
        }
        for boundary in &self.model.fixed_boundaries {
            forces.insert(
                boundary.id,
                node_state(nodes, boundary.node)?.reaction_force,
            );
        }
        for source in &self.model.motion_sources {
            forces.insert(source.id, node_state(nodes, source.node)?.reaction_force);
        }
        Ok(forces)
    }

    /// Creates sensor series containing the initial sample.
    fn initial_sensor_series(
        &self,
        nodes: &BTreeMap<NodeId, NodeState>,
    ) -> Result<BTreeMap<ComponentId, Vec<f64>>, TranslationalError> {
        self.model
            .position_sensors
            .iter()
            .map(|sensor| {
                node_state(nodes, sensor.node).map(|state| (sensor.id, vec![state.position]))
            })
            .collect()
    }

    /// Captures one position value for every sensor.
    fn capture_sensor_positions(
        &self,
        nodes: &BTreeMap<NodeId, NodeState>,
        series: &mut BTreeMap<ComponentId, Vec<f64>>,
    ) -> Result<(), TranslationalError> {
        for sensor in &self.model.position_sensors {
            let position = node_state(nodes, sensor.node)?.position;
            series
                .get_mut(&sensor.id)
                .ok_or(TranslationalError::DuplicateComponent { id: sensor.id })?
                .push(position);
        }
        Ok(())
    }

    /// Captures one validated immutable sample.
    fn sample(
        &self,
        time: f64,
        nodes: &BTreeMap<NodeId, NodeState>,
        initial_energy: f64,
        external_work: f64,
        dissipated: f64,
    ) -> Result<TranslationalSample, TranslationalError> {
        let (kinetic, elastic) = self.stored_energy(nodes);
        let balance_residual = kinetic + elastic + dissipated - external_work - initial_energy;
        let energy = EnergyDiagnostic {
            kinetic,
            elastic,
            dissipated,
            external_work,
            balance_residual,
        };
        if !time.is_finite()
            || !energy.stored().is_finite()
            || !dissipated.is_finite()
            || !external_work.is_finite()
            || !balance_residual.is_finite()
        {
            return Err(TranslationalError::NonFiniteState);
        }
        Ok(TranslationalSample {
            time,
            nodes: nodes.clone(),
            component_forces: self.component_forces(nodes)?,
            energy,
        })
    }
}

/// Returns one node state or a stable missing-node error.
fn node_state(
    nodes: &BTreeMap<NodeId, NodeState>,
    node: NodeId,
) -> Result<NodeState, TranslationalError> {
    nodes
        .get(&node)
        .copied()
        .ok_or(TranslationalError::UnboundNode { node })
}

/// Adds one constitutive force to a node.
fn add_force(
    nodes: &mut BTreeMap<NodeId, NodeState>,
    node: NodeId,
    force: f64,
) -> Result<(), TranslationalError> {
    let state = nodes
        .get_mut(&node)
        .ok_or(TranslationalError::UnboundNode { node })?;
    state.force += force;
    Ok(())
}

/// Checks every scalar carried by one node state.
const fn state_is_finite(state: NodeState) -> bool {
    state.position.is_finite()
        && state.velocity.is_finite()
        && state.acceleration.is_finite()
        && state.force.is_finite()
        && state.reaction_force.is_finite()
}

/// Stable translational models used for regression and solver comparison.
pub mod reference_models {
    use super::{
        Damper, FixedBoundary, ForceSource, Mass, MotionSource, PositionSensor, Spring,
        TranslationalError, TranslationalModel, TranslationalRunSettings,
    };
    use crate::identity::{ComponentId, NodeId};

    /// A reference model with one observed position and analytical final value.
    #[derive(Debug, Clone)]
    pub struct TranslationalReferenceCase {
        /// Executable component model.
        pub model: TranslationalModel,
        /// Fixed-step run settings.
        pub settings: TranslationalRunSettings,
        /// Position sensor used for comparison.
        pub sensor_id: ComponentId,
        /// Analytical position at the final sample.
        pub expected_final_position: f64,
        /// Maximum accepted absolute final-position error.
        pub position_tolerance: f64,
    }

    /// Returns the unit undamped oscillator with `position(0) = 1`.
    ///
    /// # Errors
    ///
    /// Returns an error only if a built-in reference component violates its contract.
    #[allow(
        clippy::arithmetic_side_effects,
        clippy::float_arithmetic,
        reason = "The analytical reference evaluates the configured unit oscillator at one second."
    )]
    pub fn unit_oscillator() -> Result<TranslationalReferenceCase, TranslationalError> {
        let moving = NodeId::from_raw(1);
        let ground = NodeId::from_raw(2);
        let sensor_id = ComponentId::from_raw(4);
        let mut model = TranslationalModel::new();
        model.add_mass(Mass::new(ComponentId::from_raw(1), moving, 1.0, 1.0, 0.0)?)?;
        model.add_fixed_boundary(FixedBoundary::new(ComponentId::from_raw(2), ground, 0.0)?)?;
        model.add_spring(Spring::new(
            ComponentId::from_raw(3),
            moving,
            ground,
            1.0,
            0.0,
        )?)?;
        model.add_position_sensor(PositionSensor::new(sensor_id, moving))?;
        Ok(TranslationalReferenceCase {
            model,
            settings: TranslationalRunSettings::new(0.0125, 80)?,
            sensor_id,
            expected_final_position: 1.0_f64.cos(),
            position_tolerance: 2.0e-5,
        })
    }

    /// Returns a damped oscillator used to verify dissipation accounting.
    ///
    /// # Errors
    ///
    /// Returns an error only if a built-in reference component violates its contract.
    pub fn damped_oscillator() -> Result<TranslationalReferenceCase, TranslationalError> {
        let moving = NodeId::from_raw(11);
        let ground = NodeId::from_raw(12);
        let sensor_id = ComponentId::from_raw(15);
        let mut model = TranslationalModel::new();
        model.add_mass(Mass::new(ComponentId::from_raw(11), moving, 1.0, 1.0, 0.0)?)?;
        model.add_fixed_boundary(FixedBoundary::new(ComponentId::from_raw(12), ground, 0.0)?)?;
        model.add_spring(Spring::new(
            ComponentId::from_raw(13),
            moving,
            ground,
            1.0,
            0.0,
        )?)?;
        model.add_damper(Damper::new(ComponentId::from_raw(14), moving, ground, 0.2)?)?;
        model.add_position_sensor(PositionSensor::new(sensor_id, moving))?;
        Ok(TranslationalReferenceCase {
            model,
            settings: TranslationalRunSettings::new(0.001, 1_000)?,
            sensor_id,
            expected_final_position: 0.568_971_890_946_099_8,
            position_tolerance: 2.0e-4,
        })
    }

    /// Returns a forced mass and prescribed-motion spring system exercising all
    /// source and boundary contracts.
    ///
    /// # Errors
    ///
    /// Returns an error only if a built-in reference component violates its contract.
    pub fn driven_system() -> Result<TranslationalModel, TranslationalError> {
        let moving = NodeId::from_raw(21);
        let prescribed = NodeId::from_raw(22);
        let mut model = TranslationalModel::new();
        model.add_mass(Mass::new(ComponentId::from_raw(21), moving, 2.0, 0.0, 0.0)?)?;
        model.add_motion_source(MotionSource::new(
            ComponentId::from_raw(22),
            prescribed,
            0.0,
            0.25,
            0.0,
        )?)?;
        model.add_spring(Spring::new(
            ComponentId::from_raw(23),
            moving,
            prescribed,
            3.0,
            0.0,
        )?)?;
        model.add_damper(Damper::new(
            ComponentId::from_raw(24),
            moving,
            prescribed,
            0.5,
        )?)?;
        model.add_force_source(ForceSource::new(ComponentId::from_raw(25), moving, 1.0)?)?;
        model.add_position_sensor(PositionSensor::new(ComponentId::from_raw(26), moving))?;
        model.add_position_sensor(PositionSensor::new(ComponentId::from_raw(27), prescribed))?;
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Damper, FixedBoundary, ForceSource, Mass, MotionSource, PositionSensor, Spring,
        TranslationalError, TranslationalModel, TranslationalRunSettings, TranslationalRuntime,
        reference_models,
    };
    use crate::identity::{ComponentId, NodeId};

    #[test]
    fn unit_mass_spring_matches_harmonic_oscillator_reference() {
        let reference = reference_models::unit_oscillator().unwrap();
        let run = TranslationalRuntime::new(&reference.model)
            .unwrap()
            .run(reference.settings)
            .unwrap();
        let final_position = run
            .sensor_positions(reference.sensor_id)
            .unwrap()
            .last()
            .copied()
            .unwrap();

        assert!(
            (final_position - reference.expected_final_position).abs()
                <= reference.position_tolerance
        );
        assert!(run.maximum_energy_residual <= 2.0e-5);
    }

    #[test]
    fn constant_force_has_exact_constant_acceleration_and_work_balance() {
        let node = NodeId::from_raw(31);
        let sensor = ComponentId::from_raw(33);
        let mut model = TranslationalModel::new();
        model
            .add_mass(Mass::new(ComponentId::from_raw(31), node, 2.0, 0.0, 0.0).unwrap())
            .unwrap();
        model
            .add_force_source(ForceSource::new(ComponentId::from_raw(32), node, 4.0).unwrap())
            .unwrap();
        model
            .add_position_sensor(PositionSensor::new(sensor, node))
            .unwrap();

        let runtime = TranslationalRuntime::new(&model).unwrap();
        let run = runtime
            .run(TranslationalRunSettings::new(0.1, 10).unwrap())
            .unwrap();
        let final_sample = run.samples.last().unwrap();
        let final_state = final_sample.nodes.get(&node).unwrap();

        assert!((final_state.position - 1.0).abs() <= 1.0e-12);
        assert!((final_state.velocity - 2.0).abs() <= 1.0e-12);
        assert_eq!(run.sensor_positions(sensor).unwrap().len(), 11);
        assert!(final_sample.energy.balance_residual.abs() <= 1.0e-12);
    }

    #[test]
    fn damper_dissipates_energy_and_matches_damped_reference() {
        let reference = reference_models::damped_oscillator().unwrap();
        let run = TranslationalRuntime::new(&reference.model)
            .unwrap()
            .run(reference.settings)
            .unwrap();
        let final_position = run
            .sensor_positions(reference.sensor_id)
            .unwrap()
            .last()
            .copied()
            .unwrap();
        let final_energy = run.samples.last().unwrap().energy;

        assert!(
            (final_position - reference.expected_final_position).abs()
                <= reference.position_tolerance,
            "position: {final_position}"
        );
        assert!(final_energy.dissipated > 0.0);
        assert!(final_energy.stored() < run.samples.first().unwrap().energy.stored());
        assert!(final_energy.balance_residual.abs() <= 2.0e-4);
    }

    #[test]
    fn motion_source_tracks_exactly_and_reports_reaction_work() {
        let model = reference_models::driven_system().unwrap();
        let settings = TranslationalRunSettings::new(0.01, 100).unwrap();
        let run = TranslationalRuntime::new(&model)
            .unwrap()
            .run(settings)
            .unwrap();
        let prescribed_sensor = ComponentId::from_raw(27);
        let prescribed_positions = run.sensor_positions(prescribed_sensor).unwrap();
        let final_sample = run.samples.last().unwrap();
        let prescribed_state = final_sample.nodes.get(&NodeId::from_raw(22)).unwrap();

        assert_eq!(prescribed_positions.first(), Some(&0.0));
        assert!((prescribed_positions.last().unwrap() - 0.25).abs() <= 1.0e-12);
        assert_eq!(prescribed_state.velocity, 0.25);
        assert!(prescribed_state.reaction_force.is_finite());
        assert!(final_sample.energy.external_work.is_finite());
        assert!(final_sample.energy.balance_residual.abs() <= 2.0e-3);
    }

    #[test]
    fn fixed_boundary_reaction_balances_spring_force() {
        let reference = reference_models::unit_oscillator().unwrap();
        let run = TranslationalRuntime::new(&reference.model)
            .unwrap()
            .run(TranslationalRunSettings::new(0.01, 0).unwrap())
            .unwrap();
        let initial = run.samples.first().unwrap();
        let ground = initial.nodes.get(&NodeId::from_raw(2)).unwrap();

        assert_eq!(ground.force, 1.0);
        assert_eq!(ground.reaction_force, -1.0);
        assert_eq!(
            initial.component_forces.get(&ComponentId::from_raw(2)),
            Some(&-1.0)
        );
    }

    #[test]
    fn invalid_parameters_and_topology_are_rejected() {
        let node = NodeId::from_raw(41);
        assert!(matches!(
            Mass::new(ComponentId::from_raw(41), node, 0.0, 0.0, 0.0),
            Err(TranslationalError::InvalidParameter { key: "mass", .. })
        ));
        assert!(Damper::new(ComponentId::from_raw(42), node, node, 1.0).is_err());

        let mut unbound = TranslationalModel::new();
        unbound
            .add_position_sensor(PositionSensor::new(ComponentId::from_raw(43), node))
            .unwrap();
        assert!(matches!(
            TranslationalRuntime::new(&unbound),
            Err(TranslationalError::UnboundNode { .. })
        ));

        let mut conflict = TranslationalModel::new();
        conflict
            .add_mass(Mass::new(ComponentId::from_raw(44), node, 1.0, 0.0, 0.0).unwrap())
            .unwrap();
        conflict
            .add_motion_source(
                MotionSource::new(ComponentId::from_raw(45), node, 0.0, 0.0, 0.0).unwrap(),
            )
            .unwrap();
        assert!(matches!(
            TranslationalRuntime::new(&conflict),
            Err(TranslationalError::ConflictingNodeOwner { .. })
        ));
    }

    #[test]
    fn repeated_runs_reset_exactly() {
        let reference = reference_models::unit_oscillator().unwrap();
        let runtime = TranslationalRuntime::new(&reference.model).unwrap();

        assert_eq!(
            runtime.run(reference.settings).unwrap(),
            runtime.run(reference.settings).unwrap()
        );
    }

    #[test]
    fn constructors_define_all_phase_nine_components() {
        let first = NodeId::from_raw(51);
        let second = NodeId::from_raw(52);

        assert!(Spring::new(ComponentId::from_raw(51), first, second, 1.0, 0.0).is_ok());
        assert!(Damper::new(ComponentId::from_raw(52), first, second, 1.0).is_ok());
        assert!(ForceSource::new(ComponentId::from_raw(53), first, 1.0).is_ok());
        assert!(FixedBoundary::new(ComponentId::from_raw(54), first, 0.0).is_ok());
        assert!(MotionSource::new(ComponentId::from_raw(55), first, 0.0, 0.0, 0.0).is_ok());
        assert_eq!(
            PositionSensor::new(ComponentId::from_raw(56), first).node,
            first
        );
    }
}
