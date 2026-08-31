use crate::{Computed, SignalComputeError, SignalComputed, SignalValues};
use keys::port_key;
use std::any::Any;

/// Evaluated runtime state for delay component version 1.
#[derive(Debug, PartialEq)]
pub struct DelayV1Computed {
    /// Output restored at initialization.
    initial_value: f64,
    /// Current committed output state.
    value: f64,
    /// Input proposed for the next atomic commit.
    pending_value: Option<f64>,
}

impl DelayV1Computed {
    /// Creates a computed one-step delay.
    #[must_use]
    pub const fn new(initial_value: f64) -> Self {
        Self {
            initial_value,
            value: initial_value,
            pending_value: None,
        }
    }
}

impl Computed for DelayV1Computed {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_signal(&mut self) -> Option<&mut dyn SignalComputed> {
        Some(self)
    }
}

impl SignalComputed for DelayV1Computed {
    fn is_direct_feedthrough(&self) -> bool {
        false
    }

    fn initialize_signal(&mut self) -> Result<SignalValues, SignalComputeError> {
        self.value = self.initial_value;
        self.pending_value = None;
        crate::computed::signal_output(self.value)
    }

    fn evaluate_signal(
        &mut self,
        _timestep: f64,
        inputs: &SignalValues,
    ) -> Result<SignalValues, SignalComputeError> {
        let next_value = crate::computed::signal_input(inputs, port_key!("input"))?;
        self.pending_value = Some(next_value);
        crate::computed::signal_output(next_value)
    }

    fn commit_signal(&mut self) {
        if let Some(value) = self.pending_value.take() {
            self.value = value;
        }
    }

    fn finalize_signal(&mut self) {
        self.pending_value = None;
    }
}
