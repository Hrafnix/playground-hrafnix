# Component Catalog

## Add

`signal.add` | Signal/Math | 1.0.0

Adds two scalar inputs.

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `a` | input | `Scalar` | Input signal `a`. |
| `b` | input | `Scalar` | Input signal `b`. |
| `out` | output | `Scalar` | Output signal `out`. |

## Assertion

`signal.assertion` | Signal/Sinks | 1.0.0

Fails the run when its input leaves an inclusive configured range.

### Parameters

| Key | Type | Default | Description |
|---|---|---|---|
| `minimum` | `Scalar` | `-1.0e300` | Configuration value for `minimum`. |
| `maximum` | `Scalar` | `1.0e300` | Configuration value for `maximum`. |

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `in` | input | `Scalar` | Input signal `in`. |
| `out` | output | `Scalar` | Output signal `out`. |

## Boolean Not

`signal.boolean_not` | Signal/Logic | 1.0.0

Inverts a Boolean input.

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `in` | input | `Boolean` | Input signal `in`. |
| `out` | output | `Boolean` | Output signal `out`. |

## Clamp

`signal.clamp` | Signal/Math | 1.0.0

Limits a scalar to an inclusive configured range.

### Parameters

| Key | Type | Default | Description |
|---|---|---|---|
| `minimum` | `Scalar` | `0.0` | Configuration value for `minimum`. |
| `maximum` | `Scalar` | `1.0` | Configuration value for `maximum`. |

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `in` | input | `Scalar` | Input signal `in`. |
| `out` | output | `Scalar` | Output signal `out`. |

## Constant

`signal.constant` | Signal/Sources | 1.0.0

Emits the configured value at every sample.

### Parameters

| Key | Type | Default | Description |
|---|---|---|---|
| `value` | `Scalar` | `0.0` | Configuration value for `value`. |

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `out` | output | `Scalar` | Output signal `out`. |

## Delay

`signal.delay` | Signal/Control | 1.0.0

Delays its input by one fixed simulation step.

### Parameters

| Key | Type | Default | Description |
|---|---|---|---|
| `initial_value` | `Scalar` | `0.0` | Configuration value for `initial_value`. |

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `in` | input | `Scalar` | Input signal `in`. |
| `out` | output | `Scalar` | Output signal `out`. |

## Divide

`signal.divide` | Signal/Math | 1.0.0

Divides input a by nonzero input b.

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `a` | input | `Scalar` | Input signal `a`. |
| `b` | input | `Scalar` | Input signal `b`. |
| `out` | output | `Scalar` | Output signal `out`. |

## Expression

`signal.expression` | Signal/Expressions | 1.0.0

Evaluates a compiled scalar expression over input and time.

### Parameters

| Key | Type | Default | Description |
|---|---|---|---|
| `expression` | `String` | `x` | Text configuration for `expression`. |

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `in` | input | `Scalar` | Input signal `in`. |
| `out` | output | `Scalar` | Output signal `out`. |

## First-Order Transfer Function

`signal.first_order_transfer` | Signal/Control | 1.0.0

Applies a first-order transfer function using forward Euler updates.

### Parameters

| Key | Type | Default | Description |
|---|---|---|---|
| `gain` | `Scalar` | `1.0` | Configuration value for `gain`. |
| `time_constant` | `Scalar` | `1.0` | Configuration value for `time_constant`. |
| `initial_value` | `Scalar` | `0.0` | Configuration value for `initial_value`. |

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `in` | input | `Scalar` | Input signal `in`. |
| `out` | output | `Scalar` | Output signal `out`. |

## Gain

`signal.gain` | Signal/Math | 1.0.0

Multiplies the input by a configured scalar gain.

### Parameters

| Key | Type | Default | Description |
|---|---|---|---|
| `gain` | `Scalar` | `1.0` | Configuration value for `gain`. |

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `in` | input | `Scalar` | Input signal `in`. |
| `out` | output | `Scalar` | Output signal `out`. |

## Greater Than

`signal.greater_than` | Signal/Logic | 1.0.0

Emits true when input a is greater than input b.

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `a` | input | `Scalar` | Input signal `a`. |
| `b` | input | `Scalar` | Input signal `b`. |
| `out` | output | `Boolean` | Output signal `out`. |

## Integrator

`signal.integrator` | Signal/Control | 1.0.0

Integrates its input using deterministic forward Euler updates.

### Parameters

| Key | Type | Default | Description |
|---|---|---|---|
| `initial_value` | `Scalar` | `0.0` | Configuration value for `initial_value`. |

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `in` | input | `Scalar` | Input signal `in`. |
| `out` | output | `Scalar` | Output signal `out`. |

## Lookup

`signal.lookup` | Signal/Lookup | 1.0.0

Interpolates between two points and clamps outside the domain.

### Parameters

| Key | Type | Default | Description |
|---|---|---|---|
| `x0` | `Scalar` | `0.0` | Configuration value for `x0`. |
| `y0` | `Scalar` | `0.0` | Configuration value for `y0`. |
| `x1` | `Scalar` | `1.0` | Configuration value for `x1`. |
| `y1` | `Scalar` | `1.0` | Configuration value for `y1`. |

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `in` | input | `Scalar` | Input signal `in`. |
| `out` | output | `Scalar` | Output signal `out`. |

## Multiply

`signal.multiply` | Signal/Math | 1.0.0

Multiplies two scalar inputs.

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `a` | input | `Scalar` | Input signal `a`. |
| `b` | input | `Scalar` | Input signal `b`. |
| `out` | output | `Scalar` | Output signal `out`. |

## Probe

`signal.probe` | Signal/Sinks | 1.0.0

Passes a signal through for explicit graph-level observation.

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `in` | input | `Scalar` | Input signal `in`. |
| `out` | output | `Scalar` | Output signal `out`. |

## Ramp

`signal.ramp` | Signal/Sources | 1.0.0

Emits a linear ramp after the configured start time.

### Parameters

| Key | Type | Default | Description |
|---|---|---|---|
| `initial_value` | `Scalar` | `0.0` | Configuration value for `initial_value`. |
| `slope` | `Scalar` | `1.0` | Configuration value for `slope`. |
| `start_time` | `Scalar` | `0.0` | Configuration value for `start_time`. |

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `out` | output | `Scalar` | Output signal `out`. |

## Step

`signal.step` | Signal/Sources | 1.0.0

Switches from the initial value to the final value at the step time.

### Parameters

| Key | Type | Default | Description |
|---|---|---|---|
| `initial_value` | `Scalar` | `0.0` | Configuration value for `initial_value`. |
| `final_value` | `Scalar` | `1.0` | Configuration value for `final_value`. |
| `step_time` | `Scalar` | `0.0` | Configuration value for `step_time`. |

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `out` | output | `Scalar` | Output signal `out`. |

## Subtract

`signal.subtract` | Signal/Math | 1.0.0

Subtracts input b from input a.

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `a` | input | `Scalar` | Input signal `a`. |
| `b` | input | `Scalar` | Input signal `b`. |
| `out` | output | `Scalar` | Output signal `out`. |

## Switch

`signal.switch` | Signal/Routing | 1.0.0

Routes one of two scalar inputs using a Boolean selector.

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `select` | input | `Boolean` | Input signal `select`. |
| `false` | input | `Scalar` | Input signal `false`. |
| `true` | input | `Scalar` | Input signal `true`. |
| `out` | output | `Scalar` | Output signal `out`. |

## Unit Conversion

`signal.unit_conversion` | Signal/Units | 1.0.0

Converts a scalar between compatible declared units.

### Parameters

| Key | Type | Default | Description |
|---|---|---|---|
| `from_unit` | `Unit(Time)` | `u_time_second` | Unit selected by `from_unit`. |
| `to_unit` | `Unit(Time)` | `u_time_second` | Unit selected by `to_unit`. |

### Ports

| Key | Direction | Type | Description |
|---|---|---|---|
| `in` | input | `Scalar` | Input signal `in`. |
| `out` | output | `ScalarWithUnit(Time_Second)` | Unit-bearing output `out`. |

