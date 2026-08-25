# Physical-Domain Contract

Status: approved Phase 8 design contract. Physical components are deferred to Phase 9.

## Scope

The first conservative domain is one-dimensional translational mechanics. The
generic `units` crate owns SI base-dimension exponent algebra. The `simulation`
crate owns domain variables, equation assembly, initialization, solver services,
failure policy, and numerical acceptance criteria.

Signal ports remain directed values evaluated by the existing scheduler.
Physical ports are bidirectional equation contributors and must not be adapted
into signal ports or direct-feedthrough dependencies.

## Node Variables and Signs

The translational node carries:

| Role | Variable | Dimension |
| --- | --- | --- |
| effort | force, `F` | `M L T^-2` |
| flow | velocity, `v` | `L T^-1` |
| potential | position, `x` | `L` |

Force and velocity are power conjugates because `F v` has power dimension.
Positive flow points into the component that owns a port. A component declares
effort-out, flow-out, or implicit causality for equation assembly; causality does
not change the sign convention.

## Junction Equations

For a junction with ports `i = 0..n-1`:

```text
F_i - F_0 = 0, i = 1..n-1
sum(v_i) = 0
```

The first equations enforce one shared effort. The final equation conserves the
through variable. Their instantaneous power sum is also zero:

```text
sum(F_i v_i) = F_0 sum(v_i) = 0
```

Every physical connection becomes one junction, including a two-port
connection. Fewer than two ports, mismatched variable arrays, and nonfinite
values are assembly errors.

## Component Equations

Phase 9 components must contribute residual equations instead of mutating node
values. The approved translational reference equations are:

```text
mass:    m dv/dt - sum(F_i) = 0, dx/dt - v = 0
spring:  F - k (x_a - x_b - x_free) = 0
damper:  F - c (v_a - v_b) = 0
```

Port forces use the positive-into-component convention. Sources and boundaries
impose either effort or flow and receive the conjugate variable from the solve.
Zero or negative mass, stiffness, or damping is a component validation concern,
not a solver repair.

## Initialization

Initialization preserves explicitly fixed state values and solves all algebraic
component and junction equations before the initial sample. Conflicting fixed
values are overconstraints; missing independent constraints are
underconstraints. Both are deterministic run-blocking errors. The runtime must
not silently relax fixed values or select a reference boundary.

The initial guess is assembled in stable component, port, and variable order.
An accepted solve commits all node variables and component state atomically. A
failed solve commits none of them.

## Solver Service

The simulation host injects `NonlinearSolver`; components never choose or own a
solver. A square `NonlinearProblem` supplies residuals and a row-major Jacobian
for the host-owned unknown vector. All ordering is stable and all evaluations
must be finite.

Convergence uses the infinity norm equation by equation:

```text
abs(residual) <= absolute_tolerance + relative_tolerance * equation_scale
```

Tolerances are finite and nonnegative, cannot both be zero, and iteration limits
are positive. Invalid dimensions, nonfinite values, singular Jacobians, and
iteration exhaustion have stable failure keys. There is no partially converged
success and no partial state commit. Adaptive timesteps, event localization,
and solver fallback are deferred.

## Reference Acceptance

The executable contract uses the unit oscillator `x'' + x = 0`, `x(0) = 1`,
`v(0) = 0`, whose reference solution is `x(t) = cos(t)` and total energy is
`(v^2 + x^2) / 2`.

The three-grid timestep study uses a constant refinement ratio. Both successive
observed orders must be at least 1.95 and the finest-grid position error at
`t = 1` must not exceed `2e-5`. The velocity-Verlet reference must also keep
maximum energy error at or below `1.25e-5` over 100 seconds with `dt = 0.01`.

These tests approve the equations and numerical acceptance contract. They do
not prescribe the Phase 9 integrator implementation, but any replacement must
meet or improve the same conservation and refinement criteria.

## Deferred Decisions

- Physical document schema, graphical port representation, and mixed
  signal/physical scheduling are Phase 9 integration work.
- Rotational mechanics follows only after the translational slice passes.
- Hydraulic pressure/flow conventions, compliance state, cavitation policy,
  and nonlinear reference cases require a separate design review.
- Events, contact, friction discontinuities, sparse solvers, and adaptive time
  integration remain out of scope.