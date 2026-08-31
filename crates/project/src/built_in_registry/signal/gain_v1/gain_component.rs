use crate::{Computed, SignalComputeError, SignalComputed, SignalValues};
use keys::port_key;
use std::any::Any;
use std::ops::Mul;

/// Evaluated runtime state for gain component version 1.
#[derive(Debug, PartialEq)]
pub struct GainV1Computed {
    /// Evaluated scalar multiplier.
    gain: f64,
}

impl GainV1Computed {
    /// Creates a computed gain with an evaluated multiplier.
    #[must_use]
    pub const fn new(gain: f64) -> Self {
        Self { gain }
    }

    /// Returns the evaluated gain multiplier.
    #[must_use]
    pub const fn gain(&self) -> f64 {
        self.gain
    }

    /// Applies the gain multiplier to an input sample.
    #[must_use]
    pub fn apply(&self, input: f64) -> f64 {
        input.mul(self.gain)
    }
}

impl Computed for GainV1Computed {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_signal(&mut self) -> Option<&mut dyn SignalComputed> {
        Some(self)
    }
}

impl SignalComputed for GainV1Computed {
    fn is_direct_feedthrough(&self) -> bool {
        true
    }

    fn initialize_signal(&mut self) -> Result<SignalValues, SignalComputeError> {
        crate::computed::signal_output(0.0)
    }

    fn evaluate_signal(
        &mut self,
        _timestep: f64,
        inputs: &SignalValues,
    ) -> Result<SignalValues, SignalComputeError> {
        let input = crate::computed::signal_input(inputs, port_key!("input"))?;
        crate::computed::signal_output(self.apply(input))
    }

    fn commit_signal(&mut self) {}
}
