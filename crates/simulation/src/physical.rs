//! Contracts shared by future conservative physical-domain implementations.
//!
//! This module defines mathematics and solver boundaries only. Physical
//! component definitions and runtime integration belong to Phase 9.

use units::Dimension;

/// A conservative physical domain supported by a node schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhysicalDomain {
    /// One-dimensional translational mechanics.
    TranslationalMechanics,
}

/// Semantic role of one physical node variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeVariableRole {
    /// Across variable shared by every port at a junction.
    Effort,
    /// Through variable whose signed sum is zero at a junction.
    Flow,
    /// Time integral of the flow variable used for physical state.
    Potential,
}

/// Definition of one variable carried by a physical node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeVariableDefinition {
    /// Stable variable key.
    pub key: &'static str,
    /// Equation role.
    pub role: NodeVariableRole,
    /// Physical dimension.
    pub dimension: Dimension,
}

/// Complete node-variable schema for one physical domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalDomainDefinition {
    /// Domain identity.
    pub domain: PhysicalDomain,
    /// Across variable.
    pub effort: NodeVariableDefinition,
    /// Through variable.
    pub flow: NodeVariableDefinition,
    /// Integrated flow variable.
    pub potential: NodeVariableDefinition,
}

impl PhysicalDomain {
    /// Returns the immutable node schema for this domain.
    #[must_use]
    pub const fn definition(self) -> PhysicalDomainDefinition {
        match self {
            Self::TranslationalMechanics => PhysicalDomainDefinition {
                domain: self,
                effort: NodeVariableDefinition {
                    key: "force",
                    role: NodeVariableRole::Effort,
                    dimension: Dimension::FORCE,
                },
                flow: NodeVariableDefinition {
                    key: "velocity",
                    role: NodeVariableRole::Flow,
                    dimension: Dimension::VELOCITY,
                },
                potential: NodeVariableDefinition {
                    key: "position",
                    role: NodeVariableRole::Potential,
                    dimension: Dimension::LENGTH,
                },
            },
        }
    }
}

/// Variable imposed by a component when assembling physical equations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortCausality {
    /// The component supplies effort and receives solved flow.
    EffortOut,
    /// The component supplies flow and receives solved effort.
    FlowOut,
    /// The component contributes implicit equations without fixed causality.
    Implicit,
}

/// Sign convention applied to every physical through variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlowSignConvention {
    /// Positive flow enters the owning component through the port.
    PositiveIntoComponent,
}

/// Required sign convention for physical ports.
pub const FLOW_SIGN_CONVENTION: FlowSignConvention = FlowSignConvention::PositiveIntoComponent;

/// Failure while assembling junction equations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JunctionError {
    /// A junction must contain at least two connected ports.
    TooFewPorts,
    /// Effort and flow arrays describe different port counts.
    PortCountMismatch,
    /// A node variable was NaN or infinite.
    NonFiniteVariable,
}

/// Evaluates effort-equality equations followed by the flow-conservation equation.
///
/// For `n` ports, residuals are `effort[i] - effort[0]` for all ports after
/// the first, followed by the sum of all flows. Positive flow is into each
/// connected component.
///
/// # Errors
///
/// Returns an error for fewer than two ports, mismatched arrays, or nonfinite values.
#[allow(
    clippy::arithmetic_side_effects,
    clippy::float_arithmetic,
    reason = "Junction residuals are checked for finite physical inputs before arithmetic."
)]
pub fn junction_residuals(efforts: &[f64], flows: &[f64]) -> Result<Vec<f64>, JunctionError> {
    if efforts.len() < 2 {
        return Err(JunctionError::TooFewPorts);
    }
    if efforts.len() != flows.len() {
        return Err(JunctionError::PortCountMismatch);
    }
    if efforts.iter().chain(flows).any(|value| !value.is_finite()) {
        return Err(JunctionError::NonFiniteVariable);
    }

    let reference = efforts.first().copied().ok_or(JunctionError::TooFewPorts)?;
    let mut residuals = efforts
        .iter()
        .skip(1)
        .map(|effort| *effort - reference)
        .collect::<Vec<_>>();
    residuals.push(flows.iter().sum());
    Ok(residuals)
}

/// Policy for constructing a consistent physical initial condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitializationPolicy {
    /// Treatment of explicitly fixed state values.
    pub fixed_states: FixedStatePolicy,
    /// Treatment of algebraic node variables.
    pub algebraic_variables: AlgebraicInitializationPolicy,
    /// Treatment of inconsistent equation structure.
    pub inconsistent_constraints: InconsistentConstraintPolicy,
}

/// Initialization treatment of explicitly fixed states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixedStatePolicy {
    /// Preserve fixed values exactly during initialization.
    Preserve,
}

/// Initialization treatment of algebraic physical variables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgebraicInitializationPolicy {
    /// Solve all algebraic equations before the initial sample.
    SolveBeforeInitialSample,
}

/// Initialization treatment of overconstrained or underconstrained systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InconsistentConstraintPolicy {
    /// Reject inconsistent structure without relaxing constraints.
    Reject,
}

impl Default for InitializationPolicy {
    fn default() -> Self {
        Self {
            fixed_states: FixedStatePolicy::Preserve,
            algebraic_variables: AlgebraicInitializationPolicy::SolveBeforeInitialSample,
            inconsistent_constraints: InconsistentConstraintPolicy::Reject,
        }
    }
}

/// Absolute and relative residual tolerances.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SolverTolerance {
    /// Residual accepted independently of scale.
    pub absolute: f64,
    /// Residual accepted in proportion to the equation scale.
    pub relative: f64,
}

/// Invalid solver-policy configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolverPolicyError {
    /// A tolerance was nonfinite or negative, or both tolerances were zero.
    InvalidTolerance,
    /// The iteration limit was zero.
    ZeroIterationLimit,
}

impl SolverTolerance {
    /// Creates validated residual tolerances.
    ///
    /// # Errors
    ///
    /// Returns an error unless tolerances are finite, nonnegative, and not both zero.
    pub fn new(absolute: f64, relative: f64) -> Result<Self, SolverPolicyError> {
        if !absolute.is_finite()
            || !relative.is_finite()
            || absolute < 0.0
            || relative < 0.0
            || (absolute == 0.0 && relative == 0.0)
        {
            return Err(SolverPolicyError::InvalidTolerance);
        }
        Ok(Self { absolute, relative })
    }

    /// Returns whether one residual satisfies the mixed tolerance criterion.
    #[must_use]
    #[allow(
        clippy::arithmetic_side_effects,
        clippy::float_arithmetic,
        reason = "Validated finite solver values use the standard absolute-plus-relative test."
    )]
    pub fn accepts(self, residual: f64, equation_scale: f64) -> bool {
        residual.is_finite()
            && equation_scale.is_finite()
            && equation_scale >= 0.0
            && residual.abs() <= self.absolute + self.relative * equation_scale
    }
}

/// Deterministic nonlinear solve policy supplied by the simulation host.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonlinearSolverPolicy {
    /// Mixed residual tolerance.
    pub residual_tolerance: SolverTolerance,
    /// Maximum Newton iterations, including the initial residual evaluation.
    pub maximum_iterations: u32,
}

impl NonlinearSolverPolicy {
    /// Creates a validated nonlinear solve policy.
    ///
    /// # Errors
    ///
    /// Returns an error when the iteration limit is zero.
    pub const fn new(
        residual_tolerance: SolverTolerance,
        maximum_iterations: u32,
    ) -> Result<Self, SolverPolicyError> {
        if maximum_iterations == 0 {
            return Err(SolverPolicyError::ZeroIterationLimit);
        }
        Ok(Self {
            residual_tolerance,
            maximum_iterations,
        })
    }
}

/// Deterministic reason that a nonlinear solve did not produce an accepted state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonlinearSolveFailure {
    /// Problem dimensions or buffers were inconsistent.
    InvalidProblem,
    /// A residual, Jacobian entry, or iterate was nonfinite.
    NonFiniteValue,
    /// The linearized system was singular.
    SingularJacobian,
    /// The configured iteration limit was reached.
    IterationLimit,
}

impl NonlinearSolveFailure {
    /// Returns the stable diagnostic key for this failure.
    #[must_use]
    pub const fn diagnostic_key(self) -> &'static str {
        match self {
            Self::InvalidProblem => "simulation_physical_solver_invalid_problem",
            Self::NonFiniteValue => "simulation_physical_solver_non_finite_value",
            Self::SingularJacobian => "simulation_physical_solver_singular_jacobian",
            Self::IterationLimit => "simulation_physical_solver_iteration_limit",
        }
    }
}

/// Implicit equation system passed to a host-provided nonlinear solver.
pub trait NonlinearProblem {
    /// Number of unknowns and residual equations.
    fn dimension(&self) -> usize;

    /// Writes the residual vector for one iterate.
    ///
    /// # Errors
    ///
    /// Returns a deterministic failure for invalid buffers or nonfinite evaluation.
    fn residual(
        &self,
        unknowns: &[f64],
        residuals: &mut [f64],
    ) -> Result<(), NonlinearSolveFailure>;

    /// Writes the row-major Jacobian for one iterate.
    ///
    /// # Errors
    ///
    /// Returns a deterministic failure for invalid buffers or nonfinite evaluation.
    fn jacobian(
        &self,
        unknowns: &[f64],
        row_major_jacobian: &mut [f64],
    ) -> Result<(), NonlinearSolveFailure>;
}

/// Accepted nonlinear solution metadata.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonlinearSolveReport {
    /// Number of residual evaluations used.
    pub iterations: u32,
    /// Infinity norm of the accepted residual.
    pub residual_norm: f64,
}

/// Solver service injected into physical initialization and timestep assembly.
pub trait NonlinearSolver {
    /// Solves in place from the supplied deterministic initial guess.
    ///
    /// # Errors
    ///
    /// Returns a stable failure reason without committing a partial physical state.
    fn solve(
        &self,
        problem: &dyn NonlinearProblem,
        unknowns: &mut [f64],
        policy: NonlinearSolverPolicy,
    ) -> Result<NonlinearSolveReport, NonlinearSolveFailure>;
}

/// Acceptance criteria for fixed-step refinement studies.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimestepConvergenceCriteria {
    /// Minimum observed order for both successive refinements.
    pub minimum_order: f64,
    /// Maximum accepted error on the finest grid.
    pub maximum_fine_error: f64,
}

/// Measured result of a three-grid refinement study.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimestepConvergenceAssessment {
    /// Order estimated from coarse and medium grid errors.
    pub coarse_order: f64,
    /// Order estimated from medium and fine grid errors.
    pub fine_order: f64,
    /// Whether both order and fine-error criteria passed.
    pub accepted: bool,
}

/// Invalid convergence-study data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidConvergenceStudy;

impl TimestepConvergenceCriteria {
    /// Assesses errors from three grids separated by one constant refinement ratio.
    ///
    /// # Errors
    ///
    /// Returns an error for nonfinite/nonpositive errors, invalid criteria, or a
    /// refinement ratio not greater than one.
    #[allow(
        clippy::arithmetic_side_effects,
        clippy::float_arithmetic,
        reason = "Convergence analysis validates positive finite inputs before logarithmic arithmetic."
    )]
    pub fn assess(
        self,
        coarse_error: f64,
        medium_error: f64,
        fine_error: f64,
        refinement_ratio: f64,
    ) -> Result<TimestepConvergenceAssessment, InvalidConvergenceStudy> {
        if !self.minimum_order.is_finite()
            || self.minimum_order <= 0.0
            || !self.maximum_fine_error.is_finite()
            || self.maximum_fine_error <= 0.0
            || !coarse_error.is_finite()
            || coarse_error <= 0.0
            || !medium_error.is_finite()
            || medium_error <= 0.0
            || !fine_error.is_finite()
            || fine_error <= 0.0
            || !refinement_ratio.is_finite()
            || refinement_ratio <= 1.0
        {
            return Err(InvalidConvergenceStudy);
        }

        let ratio_log = refinement_ratio.ln();
        let coarse_order = (coarse_error / medium_error).ln() / ratio_log;
        let fine_order = (medium_error / fine_error).ln() / ratio_log;
        Ok(TimestepConvergenceAssessment {
            coarse_order,
            fine_order,
            accepted: coarse_order >= self.minimum_order
                && fine_order >= self.minimum_order
                && fine_error <= self.maximum_fine_error,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AlgebraicInitializationPolicy, FLOW_SIGN_CONVENTION, FixedStatePolicy, FlowSignConvention,
        InconsistentConstraintPolicy, InitializationPolicy, JunctionError, NonlinearSolveFailure,
        PhysicalDomain, PortCausality, SolverTolerance, TimestepConvergenceCriteria,
        junction_residuals,
    };
    use units::Dimension;

    #[test]
    fn translational_schema_has_power_conjugate_effort_and_flow() {
        let definition = PhysicalDomain::TranslationalMechanics.definition();

        assert_eq!(definition.effort.dimension, Dimension::FORCE);
        assert_eq!(definition.flow.dimension, Dimension::VELOCITY);
        assert_eq!(definition.potential.dimension, Dimension::LENGTH);
        assert_eq!(
            definition
                .effort
                .dimension
                .checked_mul(definition.flow.dimension),
            Ok(Dimension::POWER)
        );
        assert_eq!(
            FLOW_SIGN_CONVENTION,
            FlowSignConvention::PositiveIntoComponent
        );
        assert_ne!(PortCausality::EffortOut, PortCausality::FlowOut);
    }

    #[test]
    fn junction_enforces_effort_equality_and_flow_conservation() {
        assert_eq!(
            junction_residuals(&[12.0, 12.0, 12.0], &[3.0, -1.0, -2.0]),
            Ok(vec![0.0, 0.0, 0.0])
        );
        assert_eq!(
            junction_residuals(&[12.0, 11.5], &[1.0, -0.75]),
            Ok(vec![-0.5, 0.25])
        );
        assert_eq!(
            junction_residuals(&[1.0], &[0.0]),
            Err(JunctionError::TooFewPorts)
        );
    }

    #[test]
    fn initialization_and_solver_failures_are_strict_and_stable() {
        let policy = InitializationPolicy::default();
        assert_eq!(policy.fixed_states, FixedStatePolicy::Preserve);
        assert_eq!(
            policy.algebraic_variables,
            AlgebraicInitializationPolicy::SolveBeforeInitialSample
        );
        assert_eq!(
            policy.inconsistent_constraints,
            InconsistentConstraintPolicy::Reject
        );
        assert_eq!(
            NonlinearSolveFailure::IterationLimit.diagnostic_key(),
            "simulation_physical_solver_iteration_limit"
        );

        let tolerance = SolverTolerance::new(1.0e-9, 1.0e-6).unwrap();
        assert!(tolerance.accepts(9.0e-7, 1.0));
        assert!(!tolerance.accepts(2.0e-6, 1.0));
    }

    #[test]
    #[allow(
        clippy::float_arithmetic,
        reason = "Reference solution deliberately exercises second-order timestep refinement."
    )]
    fn harmonic_oscillator_meets_second_order_timestep_criterion() {
        fn integrate(step: f64, steps: u32) -> f64 {
            let mut position = 1.0_f64;
            let mut velocity = 0.0_f64;
            for _ in 0..steps {
                let acceleration = -position;
                position += velocity * step + 0.5 * acceleration * step * step;
                let next_acceleration = -position;
                velocity += acceleration.mul_add(0.5, 0.5 * next_acceleration) * step;
            }
            (position - 1.0_f64.cos()).abs()
        }

        let criteria = TimestepConvergenceCriteria {
            minimum_order: 1.95,
            maximum_fine_error: 2.0e-5,
        };
        let assessment = criteria
            .assess(
                integrate(0.05, 20),
                integrate(0.025, 40),
                integrate(0.0125, 80),
                2.0,
            )
            .unwrap();

        assert!(assessment.accepted, "assessment: {assessment:?}");
    }

    #[test]
    #[allow(
        clippy::float_arithmetic,
        reason = "Reference solution checks bounded physical energy error."
    )]
    fn harmonic_oscillator_conserves_energy_within_reference_tolerance() {
        let step = 0.01_f64;
        let mut position = 1.0_f64;
        let mut velocity = 0.0_f64;
        let initial_energy = 0.5_f64;
        let mut maximum_error = 0.0_f64;

        for _ in 0..10_000 {
            let acceleration = -position;
            position += velocity * step + 0.5 * acceleration * step * step;
            let next_acceleration = -position;
            velocity += acceleration.mul_add(0.5, 0.5 * next_acceleration) * step;
            let energy = 0.5 * velocity * velocity + 0.5 * position * position;
            maximum_error = maximum_error.max((energy - initial_energy).abs());
        }

        assert!(maximum_error <= 1.25e-5, "energy error: {maximum_error}");
    }
}
