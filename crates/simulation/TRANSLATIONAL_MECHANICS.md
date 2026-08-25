# Translational Mechanics Slice

Status: Phase 9 implementation.

## Execution Boundary

Translational mechanics uses `TranslationalModel` and `TranslationalRuntime`.
It is intentionally separate from `SimulationRuntime`: signal connections are
directed and topologically scheduled, while physical nodes are bidirectional
and enforce conservative force interactions. A future mixed-domain document
schema may adapt both execution paths without changing either kernel's
causality.

A model is cloned into an immutable runtime snapshot. Every run starts from
component initial conditions, advances single-threaded with a fixed timestep,
and commits complete node samples. Reusing a runtime therefore resets exactly.

## Components

| Component | State or equation | Validation |
| --- | --- | --- |
| Mass | `F = m a`, `dx/dt = v` | `m > 0`; finite initial position and velocity |
| Spring | `F_a = -k (x_a - x_b - x_free)`, `F_b = -F_a` | distinct nodes; `k > 0`; finite free length |
| Damper | `F_a = -c (v_a - v_b)`, `F_b = -F_a` | distinct nodes; `c > 0` |
| Force source | constant signed force on one node | finite force |
| Fixed boundary | prescribed constant position and zero velocity | finite position |
| Motion source | constant-acceleration prescribed trajectory | finite trajectory coefficients |
| Position sensor | nonintrusive sampled node position | bound node |

Each node has exactly one state owner: a mass, fixed boundary, or motion source.
Springs, dampers, sources, and sensors reference those nodes. Missing owners and
multiple owners are run-construction errors. Component IDs are unique across the
model.

Positive coordinate force is reported in `NodeState::force`. A boundary's
`reaction_force` is the constraint force applied to its node, so constitutive
force plus reaction is zero. Two-port components report force at their first
port in `TranslationalSample::component_forces`; the second-port force is equal
and opposite.

## Integration

Free masses use fixed-step velocity Verlet. Prescribed motion is evaluated
analytically at every sample. Damping uses a deterministic velocity predictor
followed by force reassembly and velocity correction. Initial state is sampled
at `t = 0`, followed by one sample for every configured transition.

Nonfinite values fail the run before a sample is committed. The current slice
uses explicit linear constitutive equations and does not invoke the Phase 8
nonlinear solver service. Nonlinear components and mixed implicit networks must
use that service in a later slice.

## Energy Diagnostics

Every sample records:

- kinetic energy of masses;
- elastic potential energy of springs;
- integrated viscous dissipation;
- integrated work from force and motion sources; and
- the balance residual `stored + dissipated - external_work - initial_stored`.

Source work and damping loss use trapezoidal power integration. The run reports
the largest absolute balance residual. These diagnostics expose numerical drift
without silently modifying state.

## Reference Models

`translational::reference_models` provides:

- a unit undamped oscillator checked against `x(t) = cos(t)` and the Phase 8
  conservation threshold;
- a damped oscillator checked against its analytical displacement and positive
  dissipation; and
- a driven system containing mass, spring, damper, force source, motion source,
  and position sensors.

Tests also cover exact constant-force motion, fixed-boundary reaction, invalid
parameters and topology, sensor sample counts, source work, and deterministic
reset.

## Deferred Integration

- Persisted physical models and editor authoring.
- Mixed signal/physical scheduling and signal-to-physical adapters.
- Custom-component physical interfaces.
- Nonlinear constitutive laws, sparse assembly, and adaptive timesteps.
- Rotational mechanics and hydraulic domains.
