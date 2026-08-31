use crate::{Computed, SignalComputeError, SignalComputed, SignalValues};
use keys::port_key;
use std::any::Any;
use std::ops::Add;

/// Evaluated runtime for add component version 1.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct AddV1Computed;

impl Computed for AddV1Computed {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_signal(&mut self) -> Option<&mut dyn SignalComputed> {
        Some(self)
    }
}

impl SignalComputed for AddV1Computed {
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
        let left = crate::computed::signal_input(inputs, port_key!("a"))?;
        let right = crate::computed::signal_input(inputs, port_key!("b"))?;
        crate::computed::signal_output(left.add(right))
    }

    fn commit_signal(&mut self) {}
}
