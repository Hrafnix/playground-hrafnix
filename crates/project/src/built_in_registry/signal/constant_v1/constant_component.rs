use crate::{Computed, SignalComputeError, SignalComputed, SignalValues};
use std::any::Any;

/// Evaluated runtime for constant component version 1.
#[derive(Debug, PartialEq)]
pub struct ConstantV1Computed {
    /// Evaluated constant output value.
    value: f64,
}

impl ConstantV1Computed {
    /// Creates a computed constant source.
    #[must_use]
    pub const fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Computed for ConstantV1Computed {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_signal(&mut self) -> Option<&mut dyn SignalComputed> {
        Some(self)
    }
}

impl SignalComputed for ConstantV1Computed {
    fn is_direct_feedthrough(&self) -> bool {
        false
    }

    fn initialize_signal(&mut self) -> Result<SignalValues, SignalComputeError> {
        crate::computed::signal_output(self.value)
    }

    fn evaluate_signal(
        &mut self,
        _timestep: f64,
        _inputs: &SignalValues,
    ) -> Result<SignalValues, SignalComputeError> {
        crate::computed::signal_output(self.value)
    }

    fn commit_signal(&mut self) {}
}
