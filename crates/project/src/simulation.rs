use crate::{
    BuiltInComponentTrait, Component, ComponentComputeError, Computed, PortDefinition, PortKind,
    SignalComputeError, SignalValues,
};
use expression_engine::prelude::ExpressionEngine;
use keys::ConstPortKey;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Stable identity of a component instance within one model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentInstanceId(u64);

impl ComponentInstanceId {
    /// Creates an identity from deterministic source data.
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

/// One endpoint in a computed signal graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalEndpoint {
    /// Component instance containing the port.
    pub component: ComponentInstanceId,
    /// Stable port identifier.
    pub port: ConstPortKey,
}

/// A runtime component and the static port contract used to validate it.
#[derive(Debug)]
pub struct SignalComponentInstance {
    /// Stable instance identity.
    id: ComponentInstanceId,
    /// Static ports copied from the selected component definition.
    ports: &'static [PortDefinition],
    /// Version-specific computed implementation.
    computed: Box<dyn Computed>,
}

impl SignalComponentInstance {
    /// Creates a runtime instance from an already-computed component.
    #[must_use]
    pub const fn new(
        id: ComponentInstanceId,
        ports: &'static [PortDefinition],
        computed: Box<dyn Computed>,
    ) -> Self {
        Self {
            id,
            ports,
            computed,
        }
    }

    /// Evaluates an editable component with its selected built-in implementation.
    ///
    /// # Errors
    /// Returns an error when expressions fail or the component does not match the implementation.
    pub fn compute(
        id: ComponentInstanceId,
        implementation: &dyn BuiltInComponentTrait,
        component: &Component,
        engine: &ExpressionEngine,
    ) -> Result<Self, ComponentComputeError> {
        let computed = implementation.compute(component, engine)?;
        Ok(Self::new(id, implementation.definition().ports(), computed))
    }

    /// Returns this instance's stable identity.
    #[must_use]
    pub const fn id(&self) -> ComponentInstanceId {
        self.id
    }
}

/// Directed connection from one signal output to one signal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalConnection {
    /// Source output endpoint.
    pub source: SignalEndpoint,
    /// Target input endpoint.
    pub target: SignalEndpoint,
}

/// Owned, immutable-topology input to a simulation runtime.
#[derive(Debug, Default)]
pub struct SignalModel {
    /// Computed component instances.
    pub components: Vec<SignalComponentInstance>,
    /// Directed signal connections.
    pub connections: Vec<SignalConnection>,
}

/// Configuration for a deterministic fixed-step run.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulationSettings {
    /// Time represented by one transition.
    pub timestep: f64,
    /// Number of transitions after the initialized sample.
    pub steps: u64,
}

/// A model supported by the simulation platform.
#[derive(Debug)]
pub enum SimulationModel {
    /// Directed synchronous signal graph.
    Signal(SignalModel),
    /// One-dimensional mechanical network.
    Translational(crate::TranslationalModel),
}

impl From<SignalModel> for SimulationModel {
    fn from(model: SignalModel) -> Self {
        Self::Signal(model)
    }
}

impl From<crate::TranslationalModel> for SimulationModel {
    fn from(model: crate::TranslationalModel) -> Self {
        Self::Translational(model)
    }
}

/// Prepared runtime for a supported simulation domain.
#[derive(Debug)]
pub enum SimulationRuntime {
    /// Signal-domain runtime.
    Signal(SignalRuntime),
    /// Translational-domain runtime.
    Translational(crate::TranslationalRuntime),
}

impl SimulationRuntime {
    /// Validates and prepares a model using its domain solver.
    ///
    /// # Errors
    /// Returns the selected solver's model-validation error.
    pub fn new(model: impl Into<SimulationModel>) -> Result<Self, SimulationError> {
        match model.into() {
            SimulationModel::Signal(model) => SignalRuntime::new(model)
                .map(Self::Signal)
                .map_err(SimulationError::Signal),
            SimulationModel::Translational(model) => crate::TranslationalRuntime::new(model)
                .map(Self::Translational)
                .map_err(SimulationError::Translational),
        }
    }

    /// Executes a complete fixed-step run using the selected solver.
    ///
    /// # Errors
    /// Returns the selected solver's timing or execution error.
    pub fn run(&mut self, settings: SimulationSettings) -> Result<SimulationRun, SimulationError> {
        match self {
            Self::Signal(runtime) => runtime
                .run(settings)
                .map(SimulationRun::Signal)
                .map_err(SimulationError::Signal),
            Self::Translational(runtime) => runtime
                .run(settings)
                .map(SimulationRun::Translational)
                .map_err(SimulationError::Translational),
        }
    }
}

/// Completed run from a supported simulation domain.
#[derive(Debug, Clone, PartialEq)]
pub enum SimulationRun {
    /// Signal samples indexed by endpoint.
    Signal(SignalRun),
    /// Translational node-state samples.
    Translational(crate::TranslationalRun),
}

/// Failure while preparing or executing a supported simulation domain.
#[derive(Debug)]
pub enum SimulationError {
    /// Signal solver failure.
    Signal(SignalSimulationError),
    /// Translational solver failure.
    Translational(crate::TranslationalError),
}

impl Display for SimulationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Signal(error) => Display::fmt(error, formatter),
            Self::Translational(error) => Display::fmt(error, formatter),
        }
    }
}

impl Error for SimulationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Signal(error) => Some(error),
            Self::Translational(error) => Some(error),
        }
    }
}

/// Completed deterministic signal simulation.
#[derive(Debug, Clone, PartialEq)]
pub struct SignalRun {
    /// Sample times, including initialized outputs at time zero.
    pub times: Vec<f64>,
    /// Output values indexed by stable endpoint.
    pub series: BTreeMap<SignalEndpoint, Vec<f64>>,
}

/// Failure while validating or executing a signal model.
#[derive(Debug)]
pub enum SignalSimulationError {
    /// A component could not be evaluated into runtime state.
    ComponentCompute(ComponentComputeError),
    /// A component instance identity was repeated.
    DuplicateComponent(ComponentInstanceId),
    /// A connection referenced an absent component.
    UnknownComponent(ComponentInstanceId),
    /// A connection referenced an absent or incorrectly directed port.
    InvalidPort(SignalEndpoint),
    /// A computed component did not expose signal behavior.
    NonSignalComponent(ComponentInstanceId),
    /// More than one source drives an input.
    MultipleInputSources(SignalEndpoint),
    /// A required input has no source.
    MissingRequiredInput(SignalEndpoint),
    /// Direct-feedthrough components contain a cycle.
    AlgebraicLoop,
    /// Timing is nonfinite or nonpositive.
    InvalidTimestep,
    /// A component rejected signal values during execution.
    SignalCompute(ComponentInstanceId, SignalComputeError),
}

impl Display for SignalSimulationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "simulation failed: {self:?}")
    }
}

impl Error for SignalSimulationError {}

impl From<ComponentComputeError> for SignalSimulationError {
    fn from(error: ComponentComputeError) -> Self {
        Self::ComponentCompute(error)
    }
}

/// Prepared synchronous runtime for one validated signal graph.
#[derive(Debug)]
pub struct SignalRuntime {
    /// Components indexed by stable identity.
    components: BTreeMap<ComponentInstanceId, SignalComponentInstance>,
    /// Connections indexed by target input.
    inputs: BTreeMap<SignalEndpoint, SignalEndpoint>,
    /// Stable execution order for state-owning components and sources.
    stateful_schedule: Vec<ComponentInstanceId>,
    /// Topological execution order for direct-feedthrough components.
    direct_schedule: Vec<ComponentInstanceId>,
    /// Latest committed output values.
    outputs: BTreeMap<SignalEndpoint, f64>,
}

impl SignalRuntime {
    /// Validates and schedules an owned model snapshot.
    ///
    /// # Errors
    /// Returns an error for invalid topology, incompatible components, or algebraic loops.
    pub fn new(model: SignalModel) -> Result<Self, SignalSimulationError> {
        let mut components = BTreeMap::new();
        for component in model.components {
            let id = component.id;
            if components.insert(id, component).is_some() {
                return Err(SignalSimulationError::DuplicateComponent(id));
            }
        }

        let mut direct = BTreeSet::new();
        for (id, component) in &mut components {
            let Some(signal) = component.computed.as_signal() else {
                return Err(SignalSimulationError::NonSignalComponent(*id));
            };
            if signal.is_direct_feedthrough() {
                direct.insert(*id);
            }
        }

        let mut inputs = BTreeMap::new();
        for connection in &model.connections {
            validate_port(&components, connection.source, PortKind::SignalOutput)?;
            validate_port(&components, connection.target, PortKind::SignalInput)?;
            if inputs
                .insert(connection.target, connection.source)
                .is_some()
            {
                return Err(SignalSimulationError::MultipleInputSources(
                    connection.target,
                ));
            }
        }
        validate_required_inputs(&components, &inputs)?;

        let direct_schedule = build_direct_schedule(&direct, &model.connections)?;
        let stateful_schedule = components
            .keys()
            .filter(|id| !direct.contains(id))
            .copied()
            .collect();
        Ok(Self {
            components,
            inputs,
            stateful_schedule,
            direct_schedule,
            outputs: BTreeMap::new(),
        })
    }

    /// Resets and executes a complete fixed-step run.
    ///
    /// # Errors
    /// Returns an error for invalid timing or component evaluation failure.
    #[allow(
        clippy::as_conversions,
        clippy::cast_precision_loss,
        clippy::float_arithmetic,
        reason = "Sample times are derived from a u64 step index and validated fixed-step settings."
    )]
    pub fn run(
        &mut self,
        settings: SimulationSettings,
    ) -> Result<SignalRun, SignalSimulationError> {
        if !settings.timestep.is_finite() || settings.timestep <= 0.0 {
            return Err(SignalSimulationError::InvalidTimestep);
        }

        self.initialize()?;
        let capacity = usize::try_from(settings.steps)
            .ok()
            .and_then(|steps| steps.checked_add(1))
            .unwrap_or(0);
        let mut run = SignalRun {
            times: Vec::with_capacity(capacity),
            series: self
                .outputs
                .keys()
                .map(|endpoint| (*endpoint, Vec::with_capacity(capacity)))
                .collect(),
        };
        self.sample(0.0, &mut run);

        for step in 1..=settings.steps {
            self.transition(settings.timestep)?;
            self.sample(step as f64 * settings.timestep, &mut run);
        }
        for id in self.components.keys().copied().collect::<Vec<_>>() {
            self.signal_mut(id)?.finalize_signal();
        }
        Ok(run)
    }

    /// Restores all component state and propagates initialized direct outputs.
    fn initialize(&mut self) -> Result<(), SignalSimulationError> {
        self.outputs.clear();
        let ids: Vec<_> = self.components.keys().copied().collect();
        for id in ids {
            let signal = self.signal_mut(id)?;
            let values = signal
                .initialize_signal()
                .map_err(|error| SignalSimulationError::SignalCompute(id, error))?;
            self.store_outputs(id, values)?;
        }
        self.evaluate_direct(0.0)
    }

    /// Performs one atomic state transition followed by direct propagation.
    fn transition(&mut self, timestep: f64) -> Result<(), SignalSimulationError> {
        let snapshot = self.outputs.clone();
        let mut proposals = Vec::with_capacity(self.stateful_schedule.len());
        for id in self.stateful_schedule.clone() {
            let inputs = self.inputs_for(id, &snapshot);
            let values = self
                .signal_mut(id)?
                .evaluate_signal(timestep, &inputs)
                .map_err(|error| SignalSimulationError::SignalCompute(id, error))?;
            proposals.push((id, values));
        }
        for id in &self.stateful_schedule.clone() {
            self.signal_mut(*id)?.commit_signal();
        }
        for (id, values) in proposals {
            self.store_outputs(id, values)?;
        }
        self.evaluate_direct(timestep)
    }

    /// Evaluates direct-feedthrough components in topological order.
    fn evaluate_direct(&mut self, timestep: f64) -> Result<(), SignalSimulationError> {
        for id in self.direct_schedule.clone() {
            let inputs = self.inputs_for(id, &self.outputs);
            let values = self
                .signal_mut(id)?
                .evaluate_signal(timestep, &inputs)
                .map_err(|error| SignalSimulationError::SignalCompute(id, error))?;
            self.store_outputs(id, values)?;
        }
        Ok(())
    }

    /// Returns the mutable signal behavior for one validated component.
    fn signal_mut(
        &mut self,
        id: ComponentInstanceId,
    ) -> Result<&mut dyn crate::SignalComputed, SignalSimulationError> {
        self.components
            .get_mut(&id)
            .and_then(|component| component.computed.as_signal())
            .ok_or(SignalSimulationError::NonSignalComponent(id))
    }

    /// Resolves one component's input values from an output snapshot.
    fn inputs_for(
        &self,
        id: ComponentInstanceId,
        outputs: &BTreeMap<SignalEndpoint, f64>,
    ) -> SignalValues {
        self.inputs
            .iter()
            .filter(|(target, _)| target.component == id)
            .filter_map(|(target, source)| outputs.get(source).map(|value| (target.port, *value)))
            .collect()
    }

    /// Validates and stores outputs from one component evaluation.
    fn store_outputs(
        &mut self,
        id: ComponentInstanceId,
        values: SignalValues,
    ) -> Result<(), SignalSimulationError> {
        for (port, value) in values {
            let endpoint = SignalEndpoint {
                component: id,
                port,
            };
            validate_port(&self.components, endpoint, PortKind::SignalOutput)?;
            if !value.is_finite() {
                return Err(SignalSimulationError::SignalCompute(
                    id,
                    SignalComputeError::NonFiniteOutput(port),
                ));
            }
            self.outputs.insert(endpoint, value);
        }
        Ok(())
    }

    /// Appends one aligned sample to the result.
    fn sample(&self, time: f64, run: &mut SignalRun) {
        run.times.push(time);
        for (endpoint, values) in &mut run.series {
            if let Some(value) = self.outputs.get(endpoint) {
                values.push(*value);
            }
        }
    }
}

/// Validates one endpoint against a component's static port contract.
fn validate_port(
    components: &BTreeMap<ComponentInstanceId, SignalComponentInstance>,
    endpoint: SignalEndpoint,
    kind: PortKind,
) -> Result<(), SignalSimulationError> {
    let component = components
        .get(&endpoint.component)
        .ok_or(SignalSimulationError::UnknownComponent(endpoint.component))?;
    if component
        .ports
        .iter()
        .any(|port| port.id() == endpoint.port && port.kind() == kind)
    {
        Ok(())
    } else {
        Err(SignalSimulationError::InvalidPort(endpoint))
    }
}

/// Ensures every required signal input has exactly one source.
fn validate_required_inputs(
    components: &BTreeMap<ComponentInstanceId, SignalComponentInstance>,
    inputs: &BTreeMap<SignalEndpoint, SignalEndpoint>,
) -> Result<(), SignalSimulationError> {
    for (id, component) in components {
        for port in component
            .ports
            .iter()
            .filter(|port| port.kind() == PortKind::SignalInput && port.required())
        {
            let endpoint = SignalEndpoint {
                component: *id,
                port: port.id(),
            };
            if !inputs.contains_key(&endpoint) {
                return Err(SignalSimulationError::MissingRequiredInput(endpoint));
            }
        }
    }
    Ok(())
}

/// Builds a stable topological order for direct-feedthrough components.
fn build_direct_schedule(
    direct: &BTreeSet<ComponentInstanceId>,
    connections: &[SignalConnection],
) -> Result<Vec<ComponentInstanceId>, SignalSimulationError> {
    let mut dependencies: BTreeSet<(ComponentInstanceId, ComponentInstanceId)> = BTreeSet::new();
    for connection in connections {
        if direct.contains(&connection.source.component)
            && direct.contains(&connection.target.component)
        {
            dependencies.insert((connection.source.component, connection.target.component));
        }
    }

    let mut indegree: BTreeMap<_, usize> = direct.iter().map(|id| (*id, 0)).collect();
    let mut outgoing: BTreeMap<ComponentInstanceId, Vec<ComponentInstanceId>> = BTreeMap::new();
    for (source, target) in dependencies {
        if let Some(value) = indegree.get_mut(&target) {
            *value = value.saturating_add(1);
        }
        outgoing.entry(source).or_default().push(target);
    }

    let mut ready: BTreeSet<_> = indegree
        .iter()
        .filter_map(|(id, degree)| (*degree == 0).then_some(*id))
        .collect();
    let mut schedule = Vec::with_capacity(direct.len());
    while let Some(id) = ready.pop_first() {
        schedule.push(id);
        if let Some(targets) = outgoing.get(&id) {
            for target in targets {
                let Some(degree) = indegree.get_mut(target) else {
                    continue;
                };
                *degree = degree.saturating_sub(1);
                if *degree == 0 {
                    ready.insert(*target);
                }
            }
        }
    }
    if schedule.len() == direct.len() {
        Ok(schedule)
    } else {
        Err(SignalSimulationError::AlgebraicLoop)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::built_in_registry::BUILT_IN_REGISTRY;
    use crate::built_in_registry::signal::constant_v1::constant_component::ConstantV1Computed;
    use crate::built_in_registry::signal::constant_v1::constant_definition::CONSTANT_V1;
    use crate::built_in_registry::signal::delay_v1::delay_component::DelayV1Computed;
    use crate::built_in_registry::signal::delay_v1::delay_definition::DELAY_V1;
    use crate::built_in_registry::signal::gain_v1::gain_component::GainV1Computed;
    use crate::built_in_registry::signal::gain_v1::gain_definition::GAIN_V1;
    use keys::port_key;

    fn endpoint(component: u64, port: ConstPortKey) -> SignalEndpoint {
        SignalEndpoint {
            component: ComponentInstanceId::new(component),
            port,
        }
    }

    #[test]
    fn domain_neutral_runtime_dispatches_both_solvers() {
        let settings = SimulationSettings {
            timestep: 0.1,
            steps: 1,
        };
        let Ok(mut signal) = SimulationRuntime::new(SignalModel::default()) else {
            panic!("empty signal model should validate");
        };
        let Ok(mut translational) = SimulationRuntime::new(crate::TranslationalModel::new()) else {
            panic!("empty translational model should validate");
        };

        assert!(matches!(signal.run(settings), Ok(SimulationRun::Signal(_))));
        assert!(matches!(
            translational.run(settings),
            Ok(SimulationRun::Translational(_))
        ));
    }

    #[test]
    fn deterministic_chain_resets_between_runs() {
        let model = SignalModel {
            components: vec![
                SignalComponentInstance::new(
                    ComponentInstanceId::new(1),
                    CONSTANT_V1.ports(),
                    Box::new(ConstantV1Computed::new(2.0)),
                ),
                SignalComponentInstance::new(
                    ComponentInstanceId::new(2),
                    GAIN_V1.ports(),
                    Box::new(GainV1Computed::new(3.0)),
                ),
                SignalComponentInstance::new(
                    ComponentInstanceId::new(3),
                    DELAY_V1.ports(),
                    Box::new(DelayV1Computed::new(-1.0)),
                ),
            ],
            connections: vec![
                SignalConnection {
                    source: endpoint(1, port_key!("output")),
                    target: endpoint(2, port_key!("input")),
                },
                SignalConnection {
                    source: endpoint(2, port_key!("output")),
                    target: endpoint(3, port_key!("input")),
                },
            ],
        };
        let Ok(mut runtime) = SignalRuntime::new(model) else {
            panic!("model should validate");
        };
        let settings = SimulationSettings {
            timestep: 0.25,
            steps: 2,
        };
        let Ok(first) = runtime.run(settings) else {
            panic!("first run should succeed");
        };
        let Ok(second) = runtime.run(settings) else {
            panic!("second run should reset");
        };

        assert_eq!(first, second);
        assert_eq!(first.times, vec![0.0, 0.25, 0.5]);
        assert_eq!(
            first.series.get(&endpoint(2, port_key!("output"))),
            Some(&vec![6.0, 6.0, 6.0])
        );
        assert_eq!(
            first.series.get(&endpoint(3, port_key!("output"))),
            Some(&vec![-1.0, 6.0, 6.0])
        );
    }

    #[test]
    fn direct_cycle_is_rejected() {
        let model = SignalModel {
            components: vec![
                SignalComponentInstance::new(
                    ComponentInstanceId::new(1),
                    GAIN_V1.ports(),
                    Box::new(GainV1Computed::new(1.0)),
                ),
                SignalComponentInstance::new(
                    ComponentInstanceId::new(2),
                    GAIN_V1.ports(),
                    Box::new(GainV1Computed::new(1.0)),
                ),
            ],
            connections: vec![
                SignalConnection {
                    source: endpoint(1, port_key!("output")),
                    target: endpoint(2, port_key!("input")),
                },
                SignalConnection {
                    source: endpoint(2, port_key!("output")),
                    target: endpoint(1, port_key!("input")),
                },
            ],
        };

        assert!(matches!(
            SignalRuntime::new(model),
            Err(SignalSimulationError::AlgebraicLoop)
        ));
    }

    #[test]
    fn registry_components_compute_into_a_runnable_graph() {
        let Some(constant_item) = BUILT_IN_REGISTRY.get(keys::component_key!("constant")) else {
            panic!("constant must be registered");
        };
        let Some(gain_item) = BUILT_IN_REGISTRY.get(keys::component_key!("gain")) else {
            panic!("gain must be registered");
        };
        let constant_definition = constant_item.current();
        let gain_definition = gain_item.current();
        let Some(delay_item) = BUILT_IN_REGISTRY.get(keys::component_key!("delay")) else {
            panic!("delay must be registered");
        };
        let delay_definition = delay_item.current();
        let mut constant = constant_definition.instantiate();
        let mut gain = gain_definition.instantiate();
        let mut delay = delay_definition.instantiate();
        assert!(constant.set_parameter_expression("p_value", "2.0").is_ok());
        assert!(gain.set_parameter_expression("p_gain", "3.0").is_ok());
        assert!(
            delay
                .set_parameter_expression("p_initial_value", "-1.0")
                .is_ok()
        );
        let engine = ExpressionEngine::new();
        let Ok(constant) = SignalComponentInstance::compute(
            ComponentInstanceId::new(1),
            constant_definition,
            &constant,
            &engine,
        ) else {
            panic!("constant defaults must compute");
        };
        let Ok(gain) = SignalComponentInstance::compute(
            ComponentInstanceId::new(2),
            gain_definition,
            &gain,
            &engine,
        ) else {
            panic!("gain defaults must compute");
        };
        let Ok(delay) = SignalComponentInstance::compute(
            ComponentInstanceId::new(3),
            delay_definition,
            &delay,
            &engine,
        ) else {
            panic!("delay parameters must compute");
        };
        let model = SignalModel {
            components: vec![constant, gain, delay],
            connections: vec![
                SignalConnection {
                    source: endpoint(1, port_key!("output")),
                    target: endpoint(2, port_key!("input")),
                },
                SignalConnection {
                    source: endpoint(2, port_key!("output")),
                    target: endpoint(3, port_key!("input")),
                },
            ],
        };
        let Ok(mut runtime) = SignalRuntime::new(model) else {
            panic!("computed defaults must form a valid graph");
        };
        let Ok(run) = runtime.run(SimulationSettings {
            timestep: 0.1,
            steps: 1,
        }) else {
            panic!("computed graph must run");
        };

        assert_eq!(run.times, vec![0.0, 0.1]);
        assert_eq!(
            run.series.get(&endpoint(2, port_key!("output"))),
            Some(&vec![6.0, 6.0])
        );
        assert_eq!(
            run.series.get(&endpoint(3, port_key!("output"))),
            Some(&vec![-1.0, 6.0])
        );
    }
}
