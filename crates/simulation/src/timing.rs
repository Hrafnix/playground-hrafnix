use serde::{Deserialize, Serialize};

/// Policy for samples at the beginning of a run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InitialSamplePolicy {
    /// Capture initialized outputs at `start_time` before the first state update.
    CaptureInitializedOutputs,
}

/// Contract for stateful component output and state commit ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateUpdatePolicy {
    /// Compute outputs from current state, then atomically commit next state.
    OutputsBeforeStateCommit,
}

/// Fixed-step semantics used by the first runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixedStepSemantics {
    /// Initial output sampling policy.
    pub initial_sample: InitialSamplePolicy,
    /// Stateful component commit policy.
    pub state_update: StateUpdatePolicy,
}

impl Default for FixedStepSemantics {
    fn default() -> Self {
        Self {
            initial_sample: InitialSamplePolicy::CaptureInitializedOutputs,
            state_update: StateUpdatePolicy::OutputsBeforeStateCommit,
        }
    }
}

/// Validated fixed-step time grid.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FixedStepPlan {
    /// First sample time.
    start_time: f64,
    /// Constant simulation timestep.
    timestep: f64,
    /// Number of state transitions that do not pass the stop time.
    step_count: u64,
}

/// Invalid fixed-step timing configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixedStepPlanError {
    /// At least one input was NaN or infinite.
    NonFinite,
    /// Timestep was zero or negative.
    NonPositiveTimestep,
    /// Stop time preceded start time.
    ReversedInterval,
    /// Requested interval contains more steps than the runtime can index.
    TooManySteps,
}

impl FixedStepPlan {
    /// Creates a grid whose last step is the greatest `start + n * timestep`
    /// that does not exceed `stop_time`.
    ///
    /// # Errors
    ///
    /// Returns an error for nonfinite values, nonpositive timesteps, reversed
    /// intervals, or a step count larger than `u64`.
    #[allow(
        clippy::arithmetic_side_effects,
        clippy::as_conversions,
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::float_arithmetic,
        reason = "Validated finite nonnegative timing values are explicitly bounded before conversion."
    )]
    pub fn new(start_time: f64, stop_time: f64, timestep: f64) -> Result<Self, FixedStepPlanError> {
        if !start_time.is_finite() || !stop_time.is_finite() || !timestep.is_finite() {
            return Err(FixedStepPlanError::NonFinite);
        }
        if timestep <= 0.0 {
            return Err(FixedStepPlanError::NonPositiveTimestep);
        }
        if stop_time < start_time {
            return Err(FixedStepPlanError::ReversedInterval);
        }

        let ratio = (stop_time - start_time) / timestep;
        let tolerance = f64::EPSILON * ratio.abs().max(1.0) * 8.0;
        let steps = (ratio + tolerance).floor();
        if steps > u64::MAX as f64 {
            return Err(FixedStepPlanError::TooManySteps);
        }

        Ok(Self {
            start_time,
            timestep,
            step_count: steps as u64,
        })
    }

    /// Returns the number of state transitions in the plan.
    #[must_use]
    pub const fn step_count(self) -> u64 {
        self.step_count
    }

    /// Returns the number of samples when initialized outputs are captured.
    #[must_use]
    pub fn sample_count(self) -> u128 {
        u128::from(self.step_count) + 1
    }

    /// Returns the time at `sample_index`, including the initial sample at zero.
    #[must_use]
    #[allow(
        clippy::arithmetic_side_effects,
        clippy::as_conversions,
        clippy::cast_precision_loss,
        clippy::float_arithmetic,
        reason = "The fixed-step contract computes sample time directly from its validated integer index."
    )]
    pub fn sample_time(self, sample_index: u64) -> Option<f64> {
        if sample_index > self.step_count {
            return None;
        }

        Some(self.start_time + self.timestep * sample_index as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::{FixedStepPlan, FixedStepSemantics, InitialSamplePolicy, StateUpdatePolicy};

    #[test]
    fn includes_stop_time_when_exactly_on_grid() {
        let plan = FixedStepPlan::new(0.0, 1.0, 0.25).unwrap();

        assert_eq!(plan.step_count(), 4);
        assert_eq!(plan.sample_count(), 5);
        assert_eq!(plan.sample_time(4), Some(1.0));
    }

    #[test]
    fn ends_at_last_grid_point_before_off_grid_stop() {
        let plan = FixedStepPlan::new(0.0, 1.0, 0.3).unwrap();

        assert_eq!(plan.step_count(), 3);
        assert_eq!(plan.sample_time(3), Some(0.899_999_999_999_999_9));
        assert_eq!(plan.sample_time(4), None);
    }

    #[test]
    fn state_contract_samples_initialized_output_before_committing_next_state() {
        let semantics = FixedStepSemantics::default();

        assert_eq!(
            semantics.initial_sample,
            InitialSamplePolicy::CaptureInitializedOutputs
        );
        assert_eq!(
            semantics.state_update,
            StateUpdatePolicy::OutputsBeforeStateCommit
        );
    }
}
