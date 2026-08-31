use expression_engine::prelude::Message;
use keys::ConstPortKey;
use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Failure to turn an editable component into a computed component.
#[derive(Debug)]
pub enum ComponentComputeError {
    /// Parameter expression evaluation failed.
    ParameterEvaluation(Vec<Message>),
    /// Variable expression evaluation failed.
    VariableEvaluation(Vec<Message>),
    /// The component does not match the selected built-in implementation.
    DefinitionMismatch,
    /// A required computed parameter was absent.
    MissingParameter(&'static str),
    /// A computed parameter did not have the type required by the component.
    InvalidParameterType(&'static str),
}

impl Display for ComponentComputeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "component computation failed: {self:?}")
    }
}

impl Error for ComponentComputeError {}

/// Scalar values indexed by stable component port identifiers.
pub type SignalValues = BTreeMap<ConstPortKey, f64>;

/// Failure while evaluating a computed signal component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalComputeError {
    /// A required input was not supplied.
    MissingInput(ConstPortKey),
    /// An input value was not finite.
    NonFiniteInput(ConstPortKey),
    /// An output value was not finite.
    NonFiniteOutput(ConstPortKey),
}

impl Display for SignalComputeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "signal computation failed: {self:?}")
    }
}

impl Error for SignalComputeError {}

/// Executable scalar-signal behavior exposed by a computed component.
pub trait SignalComputed: Debug + Send {
    /// Returns whether current outputs depend directly on current inputs.
    fn is_direct_feedthrough(&self) -> bool;

    /// Restores deterministic initial state and returns initial outputs.
    ///
    /// # Errors
    /// Returns an error when initialization produces an invalid output.
    fn initialize_signal(&mut self) -> Result<SignalValues, SignalComputeError>;

    /// Evaluates one step without committing component-owned state.
    ///
    /// # Errors
    /// Returns an error when a required input is absent or a value is nonfinite.
    fn evaluate_signal(
        &mut self,
        timestep: f64,
        inputs: &SignalValues,
    ) -> Result<SignalValues, SignalComputeError>;

    /// Atomically commits state proposed by the latest successful evaluation.
    fn commit_signal(&mut self);

    /// Finalizes component-owned signal state.
    fn finalize_signal(&mut self) {}
}

/// Integration phase presented to Q-type translational components.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranslationalQPhase {
    /// Advances position and the first half of velocity.
    Predict,
    /// Computes force and acceleration from current wave values.
    Respond,
    /// Completes the second half of the velocity update.
    Correct,
}

/// C-type component that publishes wave variables and characteristic impedances.
pub trait ComponentC: Debug + Send {
    /// Restores deterministic initial node state.
    ///
    /// # Errors
    /// Returns an error when the binding or initialized state is invalid.
    fn initialize_c(
        &mut self,
        component: crate::ComponentInstanceId,
        nodes: &[crate::MechanicalNodeId],
        state: &mut BTreeMap<crate::MechanicalNodeId, crate::MechanicalNodeState>,
    ) -> Result<(), crate::TranslationalError>;

    /// Publishes wave variables and characteristic impedances for one timestep.
    ///
    /// # Errors
    /// Returns an error when the binding or resulting node state is invalid.
    fn simulate_c(
        &mut self,
        timestep: f64,
        nodes: &[crate::MechanicalNodeId],
        state: &mut BTreeMap<crate::MechanicalNodeId, crate::MechanicalNodeState>,
    ) -> Result<(), crate::TranslationalError>;

    /// Returns this component's stored-energy contribution.
    ///
    /// # Errors
    /// Returns an error when a bound node is unavailable or energy is invalid.
    fn energy_c(
        &self,
        nodes: &[crate::MechanicalNodeId],
        state: &BTreeMap<crate::MechanicalNodeId, crate::MechanicalNodeState>,
    ) -> Result<f64, crate::TranslationalError>;

    /// Finalizes component-owned runtime state.
    fn finalize_c(&mut self);
}

/// Q-type component that consumes waves and writes physical node state.
pub trait ComponentQ: Debug + Send {
    /// Restores deterministic initial node state.
    ///
    /// # Errors
    /// Returns an error when the binding or initialized state is invalid.
    fn initialize_q(
        &mut self,
        component: crate::ComponentInstanceId,
        nodes: &[crate::MechanicalNodeId],
        state: &mut BTreeMap<crate::MechanicalNodeId, crate::MechanicalNodeState>,
    ) -> Result<(), crate::TranslationalError>;

    /// Executes one integration phase for the Q-type component.
    ///
    /// # Errors
    /// Returns an error when the binding or resulting node state is invalid.
    fn simulate_q(
        &mut self,
        phase: TranslationalQPhase,
        timestep: f64,
        nodes: &[crate::MechanicalNodeId],
        state: &mut BTreeMap<crate::MechanicalNodeId, crate::MechanicalNodeState>,
    ) -> Result<(), crate::TranslationalError>;

    /// Returns this component's stored-energy contribution.
    ///
    /// # Errors
    /// Returns an error when a bound node is unavailable or energy is invalid.
    fn energy_q(
        &self,
        nodes: &[crate::MechanicalNodeId],
        state: &BTreeMap<crate::MechanicalNodeId, crate::MechanicalNodeState>,
    ) -> Result<f64, crate::TranslationalError>;

    /// Finalizes component-owned runtime state.
    fn finalize_q(&mut self);
}

/// Runtime behavior produced from an evaluated component.
pub trait Computed: Any + Debug + Send {
    /// Returns this value as [`Any`] for concrete runtime inspection.
    fn as_any(&self) -> &dyn Any;

    /// Returns this runtime as a signal component when supported.
    fn as_signal(&mut self) -> Option<&mut dyn SignalComputed> {
        None
    }

    /// Returns this runtime as a C-type component when supported.
    fn as_component_c(&mut self) -> Option<&mut dyn ComponentC> {
        None
    }

    /// Returns this runtime as a Q-type component when supported.
    fn as_component_q(&mut self) -> Option<&mut dyn ComponentQ> {
        None
    }
}

/// Reads and validates one required signal input.
pub(crate) fn signal_input(
    inputs: &SignalValues,
    key: ConstPortKey,
) -> Result<f64, SignalComputeError> {
    let value = inputs
        .get(&key)
        .copied()
        .ok_or(SignalComputeError::MissingInput(key))?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(SignalComputeError::NonFiniteInput(key))
    }
}

/// Creates and validates the conventional scalar output map.
pub(crate) fn signal_output(value: f64) -> Result<SignalValues, SignalComputeError> {
    let output = keys::port_key!("output");
    if value.is_finite() {
        Ok(SignalValues::from([(output, value)]))
    } else {
        Err(SignalComputeError::NonFiniteOutput(output))
    }
}

/// Extracts a scalar from a computed numeric parameter, with or without units.
pub(crate) fn computed_number(
    parameters: &expression_engine::prelude::ParameterObjectComputedData,
    key: &'static str,
) -> Result<f64, ComponentComputeError> {
    let value = parameters
        .get(key)
        .ok_or(ComponentComputeError::MissingParameter(key))?;
    match value {
        expression_engine::prelude::ComputedItem::Float(value)
        | expression_engine::prelude::ComputedItem::FloatWithUnit { value, .. } => Ok(*value),
        expression_engine::prelude::ComputedItem::Boolean(_)
        | expression_engine::prelude::ComputedItem::Integer(_)
        | expression_engine::prelude::ComputedItem::String(_)
        | expression_engine::prelude::ComputedItem::Identifier(_)
        | expression_engine::prelude::ComputedItem::Path(_)
        | expression_engine::prelude::ComputedItem::Table(_)
        | expression_engine::prelude::ComputedItem::TableWithUnits(_)
        | expression_engine::prelude::ComputedItem::Unit(_) => {
            Err(ComponentComputeError::InvalidParameterType(key))
        }
    }
}
