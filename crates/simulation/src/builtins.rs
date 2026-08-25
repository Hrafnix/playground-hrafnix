use crate::component::{
    ComponentBehavior, ComponentCapabilities, ComponentCapability, ComponentDefinition,
    ComponentFactory, ComponentTypeId, ComponentUpdate, InvalidComponentTypeId,
    ParameterDefinition, PortDefinition, PortDirection, RuntimeValues, SemanticVersion,
    StepContext,
};
use crate::diagnostic::{Diagnostic, DiagnosticCategory, DiagnosticSeverity, EntityReference};
use crate::identity::ComponentId;
use crate::parameter::ParameterValueType;
use crate::registry::{ComponentRegistry, RegistryError};
use crate::signal_expression::CompiledSignalExpression;
use crate::value::RuntimeValue;
use std::sync::Arc;
use units::{UnitFamilyId, UnitId};

/// Failure while installing the standard signal component library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltinRegistrationError {
    /// A hard-coded built-in type ID violated the ID contract.
    InvalidTypeId(InvalidComponentTypeId),
    /// The registry already contains a built-in type ID.
    Registry(RegistryError),
}

impl From<InvalidComponentTypeId> for BuiltinRegistrationError {
    fn from(error: InvalidComponentTypeId) -> Self {
        Self::InvalidTypeId(error)
    }
}

impl From<RegistryError> for BuiltinRegistrationError {
    fn from(error: RegistryError) -> Self {
        Self::Registry(error)
    }
}

/// Registers the standard signal/control primitive library.
///
/// # Errors
///
/// Returns an error if any stable type ID is invalid or already installed.
pub fn register_signal_builtins(
    registry: &mut ComponentRegistry,
) -> Result<(), BuiltinRegistrationError> {
    for (definition, kind) in definitions()? {
        registry.register_with_factory(definition, Arc::new(BuiltinFactory { kind }))?;
    }
    Ok(())
}

/// Runtime algorithm selected by one built-in definition.
#[derive(Debug, Clone, Copy)]
enum BuiltinKind {
    /// Constant scalar source.
    Constant,
    /// Time-switched scalar source.
    Step,
    /// Scalar multiplication.
    Gain,
    /// Two-input scalar addition.
    Add,
    /// Forward-Euler integrator.
    Integrator,
    /// One-step discrete delay.
    Delay,
    /// Pass-through probe component.
    Probe,
    /// Time-dependent linear ramp source.
    Ramp,
    /// Two-input scalar multiplication.
    Multiply,
    /// Forward-Euler first-order transfer function.
    FirstOrderTransfer,
    /// Scalar conversion between compatible units.
    UnitConversion,
    /// Compiled scalar expression over input and simulation time.
    Expression,
    /// Two-input scalar subtraction.
    Subtract,
    /// Two-input scalar division.
    Divide,
    /// Inclusive scalar range limiter.
    Clamp,
    /// Boolean-selected scalar routing.
    Switch,
    /// Scalar greater-than comparison.
    GreaterThan,
    /// Boolean inversion.
    BooleanNot,
    /// Clamped two-point linear lookup.
    Lookup,
    /// Inclusive scalar range assertion.
    Assertion,
}

/// Factory for one built-in algorithm kind.
#[derive(Debug)]
struct BuiltinFactory {
    /// Algorithm selected by the registered definition.
    kind: BuiltinKind,
}

impl ComponentFactory for BuiltinFactory {
    fn create(
        &self,
        component_id: ComponentId,
        parameters: &RuntimeValues,
    ) -> Result<Box<dyn ComponentBehavior>, Diagnostic> {
        let expression = if matches!(self.kind, BuiltinKind::Expression) {
            let source = string_parameter(parameters, "expression", component_id)?;
            Some(CompiledSignalExpression::compile(source).map_err(|_| {
                runtime_diagnostic(
                    component_id,
                    "expression",
                    "simulation_runtime_invalid_signal_expression",
                )
            })?)
        } else {
            None
        };
        Ok(Box::new(BuiltinBehavior {
            component_id,
            kind: self.kind,
            expression,
        }))
    }
}

/// Stateless algorithm object; all mutable simulation state is runtime-owned.
#[derive(Debug)]
struct BuiltinBehavior {
    /// Instance identity used for diagnostics.
    component_id: ComponentId,
    /// Selected primitive algorithm.
    kind: BuiltinKind,
    /// Precompiled algorithm for the Expression primitive.
    expression: Option<CompiledSignalExpression>,
}

impl ComponentBehavior for BuiltinBehavior {
    fn initialize(
        &self,
        context: StepContext,
        parameters: &RuntimeValues,
    ) -> Result<ComponentUpdate, Diagnostic> {
        match self.kind {
            BuiltinKind::Constant => Ok(output(
                parameter(parameters, "value", self.component_id)?.clone(),
            )),
            BuiltinKind::Step => self.step_source(context, parameters),
            BuiltinKind::Integrator | BuiltinKind::Delay | BuiltinKind::FirstOrderTransfer => {
                let initial = parameter(parameters, "initial_value", self.component_id)?;
                Ok(ComponentUpdate {
                    outputs: values([("out", initial.clone())]),
                    next_state: values([("value", initial.clone())]),
                })
            }
            BuiltinKind::Ramp => self.ramp_source(context, parameters),
            BuiltinKind::Gain
            | BuiltinKind::Add
            | BuiltinKind::Multiply
            | BuiltinKind::Subtract
            | BuiltinKind::Divide
            | BuiltinKind::Clamp
            | BuiltinKind::Switch
            | BuiltinKind::Lookup
            | BuiltinKind::Assertion
            | BuiltinKind::UnitConversion
            | BuiltinKind::Expression
            | BuiltinKind::Probe => Ok(output(RuntimeValue::Scalar(0.0))),
            BuiltinKind::GreaterThan | BuiltinKind::BooleanNot => {
                Ok(output(RuntimeValue::Boolean(false)))
            }
        }
    }

    #[allow(
        clippy::float_arithmetic,
        reason = "Signal primitives perform validated finite scalar arithmetic."
    )]
    fn evaluate(
        &self,
        context: StepContext,
        parameters: &RuntimeValues,
        inputs: &RuntimeValues,
        state: &RuntimeValues,
    ) -> Result<ComponentUpdate, Diagnostic> {
        match self.kind {
            BuiltinKind::Constant => Ok(output(
                parameter(parameters, "value", self.component_id)?.clone(),
            )),
            BuiltinKind::Step => self.step_source(context, parameters),
            BuiltinKind::Gain => {
                let input = scalar(
                    input(inputs, "in", self.component_id)?,
                    self.component_id,
                    "in",
                )?;
                let gain = scalar(
                    parameter(parameters, "gain", self.component_id)?,
                    self.component_id,
                    "gain",
                )?;
                finite_output(input * gain, self.component_id)
            }
            BuiltinKind::Add => {
                let left = scalar(
                    input(inputs, "a", self.component_id)?,
                    self.component_id,
                    "a",
                )?;
                let right = scalar(
                    input(inputs, "b", self.component_id)?,
                    self.component_id,
                    "b",
                )?;
                finite_output(left + right, self.component_id)
            }
            BuiltinKind::Multiply => {
                let left = scalar(
                    input(inputs, "a", self.component_id)?,
                    self.component_id,
                    "a",
                )?;
                let right = scalar(
                    input(inputs, "b", self.component_id)?,
                    self.component_id,
                    "b",
                )?;
                finite_output(left * right, self.component_id)
            }
            BuiltinKind::Subtract => {
                let left = scalar(
                    input(inputs, "a", self.component_id)?,
                    self.component_id,
                    "a",
                )?;
                let right = scalar(
                    input(inputs, "b", self.component_id)?,
                    self.component_id,
                    "b",
                )?;
                finite_output(left - right, self.component_id)
            }
            BuiltinKind::Divide => {
                let numerator = scalar(
                    input(inputs, "a", self.component_id)?,
                    self.component_id,
                    "a",
                )?;
                let denominator = scalar(
                    input(inputs, "b", self.component_id)?,
                    self.component_id,
                    "b",
                )?;
                if denominator == 0.0 {
                    return Err(runtime_diagnostic(
                        self.component_id,
                        "b",
                        "simulation_runtime_division_by_zero",
                    ));
                }
                finite_output(numerator / denominator, self.component_id)
            }
            BuiltinKind::Clamp => {
                let value = scalar(
                    input(inputs, "in", self.component_id)?,
                    self.component_id,
                    "in",
                )?;
                let minimum = scalar(
                    parameter(parameters, "minimum", self.component_id)?,
                    self.component_id,
                    "minimum",
                )?;
                let maximum = scalar(
                    parameter(parameters, "maximum", self.component_id)?,
                    self.component_id,
                    "maximum",
                )?;
                if minimum > maximum {
                    return Err(runtime_diagnostic(
                        self.component_id,
                        "minimum",
                        "simulation_runtime_invalid_range",
                    ));
                }
                finite_output(value.clamp(minimum, maximum), self.component_id)
            }
            BuiltinKind::Switch => {
                let select = boolean(
                    input(inputs, "select", self.component_id)?,
                    self.component_id,
                    "select",
                )?;
                let key = if select { "true" } else { "false" };
                Ok(output(input(inputs, key, self.component_id)?.clone()))
            }
            BuiltinKind::GreaterThan => {
                let left = scalar(
                    input(inputs, "a", self.component_id)?,
                    self.component_id,
                    "a",
                )?;
                let right = scalar(
                    input(inputs, "b", self.component_id)?,
                    self.component_id,
                    "b",
                )?;
                Ok(output(RuntimeValue::Boolean(left > right)))
            }
            BuiltinKind::BooleanNot => {
                let value = boolean(
                    input(inputs, "in", self.component_id)?,
                    self.component_id,
                    "in",
                )?;
                Ok(output(RuntimeValue::Boolean(!value)))
            }
            BuiltinKind::Lookup => self.lookup(inputs, parameters),
            BuiltinKind::Assertion => {
                let value = scalar(
                    input(inputs, "in", self.component_id)?,
                    self.component_id,
                    "in",
                )?;
                let minimum = scalar(
                    parameter(parameters, "minimum", self.component_id)?,
                    self.component_id,
                    "minimum",
                )?;
                let maximum = scalar(
                    parameter(parameters, "maximum", self.component_id)?,
                    self.component_id,
                    "maximum",
                )?;
                if minimum > maximum {
                    return Err(runtime_diagnostic(
                        self.component_id,
                        "minimum",
                        "simulation_runtime_invalid_range",
                    ));
                }
                if value < minimum || value > maximum {
                    return Err(runtime_diagnostic(
                        self.component_id,
                        "in",
                        "simulation_runtime_assertion_failed",
                    ));
                }
                finite_output(value, self.component_id)
            }
            BuiltinKind::Integrator => {
                let input_value = scalar(
                    input(inputs, "in", self.component_id)?,
                    self.component_id,
                    "in",
                )?;
                let current = scalar(
                    input(state, "value", self.component_id)?,
                    self.component_id,
                    "value",
                )?;
                let next = finite(
                    current + input_value * context.timestep,
                    self.component_id,
                    "out",
                )?;
                Ok(ComponentUpdate {
                    outputs: values([("out", RuntimeValue::Scalar(next))]),
                    next_state: values([("value", RuntimeValue::Scalar(next))]),
                })
            }
            BuiltinKind::Delay => {
                let next = input(inputs, "in", self.component_id)?;
                Ok(ComponentUpdate {
                    outputs: values([("out", next.clone())]),
                    next_state: values([("value", next.clone())]),
                })
            }
            BuiltinKind::Ramp => self.ramp_source(context, parameters),
            BuiltinKind::FirstOrderTransfer => {
                let input_value = scalar(
                    input(inputs, "in", self.component_id)?,
                    self.component_id,
                    "in",
                )?;
                let current = scalar(
                    input(state, "value", self.component_id)?,
                    self.component_id,
                    "value",
                )?;
                let gain = scalar(
                    parameter(parameters, "gain", self.component_id)?,
                    self.component_id,
                    "gain",
                )?;
                let time_constant = scalar(
                    parameter(parameters, "time_constant", self.component_id)?,
                    self.component_id,
                    "time_constant",
                )?;
                if time_constant <= 0.0 {
                    return Err(runtime_diagnostic(
                        self.component_id,
                        "time_constant",
                        "simulation_runtime_nonpositive_time_constant",
                    ));
                }
                let next = finite(
                    current + context.timestep * (gain * input_value - current) / time_constant,
                    self.component_id,
                    "out",
                )?;
                Ok(ComponentUpdate {
                    outputs: values([("out", RuntimeValue::Scalar(next))]),
                    next_state: values([("value", RuntimeValue::Scalar(next))]),
                })
            }
            BuiltinKind::UnitConversion => {
                let value = scalar(
                    input(inputs, "in", self.component_id)?,
                    self.component_id,
                    "in",
                )?;
                let from_unit = unit_parameter(parameters, "from_unit", self.component_id)?;
                let to_unit = unit_parameter(parameters, "to_unit", self.component_id)?;
                let converted =
                    units::conversion::convert(value, from_unit, to_unit).map_err(|_| {
                        runtime_diagnostic(
                            self.component_id,
                            "to_unit",
                            "simulation_runtime_incompatible_units",
                        )
                    })?;
                Ok(output(RuntimeValue::ScalarWithUnit {
                    value: converted,
                    unit: to_unit,
                }))
            }
            BuiltinKind::Expression => {
                let input_value = scalar(
                    input(inputs, "in", self.component_id)?,
                    self.component_id,
                    "in",
                )?;
                let value = self
                    .expression
                    .as_ref()
                    .and_then(|expression| expression.evaluate(input_value, context.time))
                    .ok_or_else(|| {
                        runtime_diagnostic(
                            self.component_id,
                            "out",
                            "simulation_runtime_nonfinite_value",
                        )
                    })?;
                finite_output(value, self.component_id)
            }
            BuiltinKind::Probe => Ok(output(input(inputs, "in", self.component_id)?.clone())),
        }
    }
}

impl BuiltinBehavior {
    /// Interpolates one clamped two-point lookup table.
    #[allow(
        clippy::float_arithmetic,
        reason = "Linear interpolation performs validated finite scalar arithmetic."
    )]
    fn lookup(
        &self,
        inputs: &RuntimeValues,
        parameters: &RuntimeValues,
    ) -> Result<ComponentUpdate, Diagnostic> {
        let input_value = scalar(
            input(inputs, "in", self.component_id)?,
            self.component_id,
            "in",
        )?;
        let x0 = scalar(
            parameter(parameters, "x0", self.component_id)?,
            self.component_id,
            "x0",
        )?;
        let x1 = scalar(
            parameter(parameters, "x1", self.component_id)?,
            self.component_id,
            "x1",
        )?;
        let y0 = scalar(
            parameter(parameters, "y0", self.component_id)?,
            self.component_id,
            "y0",
        )?;
        let y1 = scalar(
            parameter(parameters, "y1", self.component_id)?,
            self.component_id,
            "y1",
        )?;
        if x0 >= x1 {
            return Err(runtime_diagnostic(
                self.component_id,
                "x0",
                "simulation_runtime_invalid_lookup_domain",
            ));
        }
        let fraction = ((input_value - x0) / (x1 - x0)).clamp(0.0, 1.0);
        finite_output(y0 + fraction * (y1 - y0), self.component_id)
    }

    /// Computes the Step source at the current grid time.
    fn step_source(
        &self,
        context: StepContext,
        parameters: &RuntimeValues,
    ) -> Result<ComponentUpdate, Diagnostic> {
        let step_time = scalar(
            parameter(parameters, "step_time", self.component_id)?,
            self.component_id,
            "step_time",
        )?;
        let key = if context.time >= step_time {
            "final_value"
        } else {
            "initial_value"
        };
        Ok(output(
            parameter(parameters, key, self.component_id)?.clone(),
        ))
    }

    /// Computes a ramp with a constant pre-start value.
    #[allow(
        clippy::float_arithmetic,
        reason = "The ramp primitive performs validated finite scalar arithmetic."
    )]
    fn ramp_source(
        &self,
        context: StepContext,
        parameters: &RuntimeValues,
    ) -> Result<ComponentUpdate, Diagnostic> {
        let initial = scalar(
            parameter(parameters, "initial_value", self.component_id)?,
            self.component_id,
            "initial_value",
        )?;
        let slope = scalar(
            parameter(parameters, "slope", self.component_id)?,
            self.component_id,
            "slope",
        )?;
        let start_time = scalar(
            parameter(parameters, "start_time", self.component_id)?,
            self.component_id,
            "start_time",
        )?;
        let elapsed = (context.time - start_time).max(0.0);
        finite_output(initial + slope * elapsed, self.component_id)
    }
}

/// Produces all built-in metadata paired with its runtime algorithm.
fn definitions() -> Result<Vec<(ComponentDefinition, BuiltinKind)>, InvalidComponentTypeId> {
    Ok(vec![
        (
            definition(
                "signal.constant",
                "Constant",
                vec![parameter_definition("value", "0.0")],
                vec![output_port("out")],
                [ComponentCapability::Deterministic],
            )?,
            BuiltinKind::Constant,
        ),
        (
            definition(
                "signal.step",
                "Step",
                vec![
                    parameter_definition("initial_value", "0.0"),
                    parameter_definition("final_value", "1.0"),
                    parameter_definition("step_time", "0.0"),
                ],
                vec![output_port("out")],
                [
                    ComponentCapability::TimeDependent,
                    ComponentCapability::Deterministic,
                ],
            )?,
            BuiltinKind::Step,
        ),
        (
            definition(
                "signal.gain",
                "Gain",
                vec![parameter_definition("gain", "1.0")],
                vec![input_port("in"), output_port("out")],
                [
                    ComponentCapability::DirectFeedthrough,
                    ComponentCapability::Deterministic,
                ],
            )?,
            BuiltinKind::Gain,
        ),
        (
            definition(
                "signal.add",
                "Add",
                vec![],
                vec![input_port("a"), input_port("b"), output_port("out")],
                [
                    ComponentCapability::DirectFeedthrough,
                    ComponentCapability::Deterministic,
                ],
            )?,
            BuiltinKind::Add,
        ),
        (
            definition(
                "signal.integrator",
                "Integrator",
                vec![parameter_definition("initial_value", "0.0")],
                vec![input_port("in"), output_port("out")],
                [
                    ComponentCapability::Stateful,
                    ComponentCapability::AlgebraicLoopBreak,
                    ComponentCapability::Deterministic,
                ],
            )?,
            BuiltinKind::Integrator,
        ),
        (
            definition(
                "signal.delay",
                "Delay",
                vec![parameter_definition("initial_value", "0.0")],
                vec![input_port("in"), output_port("out")],
                [
                    ComponentCapability::Stateful,
                    ComponentCapability::AlgebraicLoopBreak,
                    ComponentCapability::Deterministic,
                ],
            )?,
            BuiltinKind::Delay,
        ),
        (
            definition(
                "signal.probe",
                "Probe",
                vec![],
                vec![input_port("in"), output_port("out")],
                [
                    ComponentCapability::DirectFeedthrough,
                    ComponentCapability::Deterministic,
                ],
            )?,
            BuiltinKind::Probe,
        ),
        (
            definition(
                "signal.ramp",
                "Ramp",
                vec![
                    parameter_definition("initial_value", "0.0"),
                    parameter_definition("slope", "1.0"),
                    parameter_definition("start_time", "0.0"),
                ],
                vec![output_port("out")],
                [
                    ComponentCapability::TimeDependent,
                    ComponentCapability::Deterministic,
                ],
            )?,
            BuiltinKind::Ramp,
        ),
        (
            definition(
                "signal.multiply",
                "Multiply",
                vec![],
                vec![input_port("a"), input_port("b"), output_port("out")],
                [
                    ComponentCapability::DirectFeedthrough,
                    ComponentCapability::Deterministic,
                ],
            )?,
            BuiltinKind::Multiply,
        ),
        (
            definition(
                "signal.first_order_transfer",
                "First-Order Transfer Function",
                vec![
                    parameter_definition("gain", "1.0"),
                    parameter_definition("time_constant", "1.0"),
                    parameter_definition("initial_value", "0.0"),
                ],
                vec![input_port("in"), output_port("out")],
                [
                    ComponentCapability::Stateful,
                    ComponentCapability::AlgebraicLoopBreak,
                    ComponentCapability::Deterministic,
                ],
            )?,
            BuiltinKind::FirstOrderTransfer,
        ),
        (
            definition(
                "signal.unit_conversion",
                "Unit Conversion",
                vec![
                    unit_parameter_definition("from_unit", UnitId::Time_Second),
                    unit_parameter_definition("to_unit", UnitId::Time_Second),
                ],
                vec![
                    input_port("in"),
                    unit_output_port("out", UnitId::Time_Second),
                ],
                [
                    ComponentCapability::DirectFeedthrough,
                    ComponentCapability::Deterministic,
                ],
            )?,
            BuiltinKind::UnitConversion,
        ),
        (
            definition(
                "signal.expression",
                "Expression",
                vec![string_parameter_definition("expression", "x")],
                vec![input_port("in"), output_port("out")],
                [
                    ComponentCapability::DirectFeedthrough,
                    ComponentCapability::TimeDependent,
                    ComponentCapability::Deterministic,
                ],
            )?,
            BuiltinKind::Expression,
        ),
        (
            definition(
                "signal.subtract",
                "Subtract",
                vec![],
                vec![input_port("a"), input_port("b"), output_port("out")],
                direct_feedthrough(),
            )?,
            BuiltinKind::Subtract,
        ),
        (
            definition(
                "signal.divide",
                "Divide",
                vec![],
                vec![input_port("a"), input_port("b"), output_port("out")],
                direct_feedthrough(),
            )?,
            BuiltinKind::Divide,
        ),
        (
            definition(
                "signal.clamp",
                "Clamp",
                vec![
                    parameter_definition("minimum", "0.0"),
                    parameter_definition("maximum", "1.0"),
                ],
                vec![input_port("in"), output_port("out")],
                direct_feedthrough(),
            )?,
            BuiltinKind::Clamp,
        ),
        (
            definition(
                "signal.switch",
                "Switch",
                vec![],
                vec![
                    boolean_input_port("select"),
                    input_port("false"),
                    input_port("true"),
                    output_port("out"),
                ],
                direct_feedthrough(),
            )?,
            BuiltinKind::Switch,
        ),
        (
            definition(
                "signal.greater_than",
                "Greater Than",
                vec![],
                vec![input_port("a"), input_port("b"), boolean_output_port("out")],
                direct_feedthrough(),
            )?,
            BuiltinKind::GreaterThan,
        ),
        (
            definition(
                "signal.boolean_not",
                "Boolean Not",
                vec![],
                vec![boolean_input_port("in"), boolean_output_port("out")],
                direct_feedthrough(),
            )?,
            BuiltinKind::BooleanNot,
        ),
        (
            definition(
                "signal.lookup",
                "Lookup",
                vec![
                    parameter_definition("x0", "0.0"),
                    parameter_definition("y0", "0.0"),
                    parameter_definition("x1", "1.0"),
                    parameter_definition("y1", "1.0"),
                ],
                vec![input_port("in"), output_port("out")],
                direct_feedthrough(),
            )?,
            BuiltinKind::Lookup,
        ),
        (
            definition(
                "signal.assertion",
                "Assertion",
                vec![
                    parameter_definition("minimum", "-1.0e300"),
                    parameter_definition("maximum", "1.0e300"),
                ],
                vec![input_port("in"), output_port("out")],
                direct_feedthrough(),
            )?,
            BuiltinKind::Assertion,
        ),
    ])
}

/// Returns the standard deterministic direct-feedthrough capabilities.
const fn direct_feedthrough() -> [ComponentCapability; 2] {
    [
        ComponentCapability::DirectFeedthrough,
        ComponentCapability::Deterministic,
    ]
}

/// Creates shared metadata for one signal primitive.
fn definition<const N: usize>(
    type_id: &str,
    display_name: &str,
    parameters: Vec<ParameterDefinition>,
    ports: Vec<PortDefinition>,
    capabilities: [ComponentCapability; N],
) -> Result<ComponentDefinition, InvalidComponentTypeId> {
    Ok(ComponentDefinition {
        type_id: ComponentTypeId::new(type_id)?,
        version: SemanticVersion {
            major: 1,
            minor: 0,
            patch: 0,
        },
        display_name: display_name.into(),
        category: category(type_id).into(),
        aliases: vec![],
        tags: vec!["deterministic".into()],
        documentation: documentation(type_id).into(),
        parameters,
        ports,
        capabilities: ComponentCapabilities::new(capabilities),
        deprecation: None,
    })
}

/// Creates one scalar parameter schema.
fn parameter_definition(key: &str, default_expression: &str) -> ParameterDefinition {
    ParameterDefinition {
        key: key.into(),
        display_name: key.into(),
        description: format!("Configuration value for `{key}`.").into(),
        value_type: ParameterValueType::Scalar,
        default_expression: default_expression.into(),
    }
}

/// Creates one unit-selector parameter schema.
fn unit_parameter_definition(key: &str, default_unit: UnitId) -> ParameterDefinition {
    ParameterDefinition {
        key: key.into(),
        display_name: key.into(),
        description: format!("Unit selected by `{key}`.").into(),
        value_type: ParameterValueType::Unit(UnitFamilyId::Time),
        default_expression: default_unit.string_id().into(),
    }
}

/// Creates one literal string parameter schema.
fn string_parameter_definition(key: &str, default_value: &str) -> ParameterDefinition {
    ParameterDefinition {
        key: key.into(),
        display_name: key.into(),
        description: format!("Text configuration for `{key}`.").into(),
        value_type: ParameterValueType::String,
        default_expression: default_value.into(),
    }
}

/// Creates one required scalar input schema.
fn input_port(key: &str) -> PortDefinition {
    port(key, PortDirection::Input, true)
}

/// Creates one scalar output schema.
fn output_port(key: &str) -> PortDefinition {
    port(key, PortDirection::Output, false)
}

/// Creates one required Boolean input schema.
fn boolean_input_port(key: &str) -> PortDefinition {
    typed_port(key, PortDirection::Input, true, ParameterValueType::Boolean)
}

/// Creates one Boolean output schema.
fn boolean_output_port(key: &str) -> PortDefinition {
    typed_port(
        key,
        PortDirection::Output,
        false,
        ParameterValueType::Boolean,
    )
}

/// Creates one unit-bearing scalar output schema.
fn unit_output_port(key: &str, unit: UnitId) -> PortDefinition {
    PortDefinition {
        key: key.into(),
        display_name: key.into(),
        description: format!("Unit-bearing output `{key}`.").into(),
        direction: PortDirection::Output,
        value_type: ParameterValueType::ScalarWithUnit(unit),
        unit: Some(unit),
        required: false,
    }
}

/// Creates common scalar port metadata.
fn port(key: &str, direction: PortDirection, required: bool) -> PortDefinition {
    typed_port(key, direction, required, ParameterValueType::Scalar)
}

/// Creates common typed port metadata.
fn typed_port(
    key: &str,
    direction: PortDirection,
    required: bool,
    value_type: ParameterValueType,
) -> PortDefinition {
    PortDefinition {
        key: key.into(),
        display_name: key.into(),
        description: format!("{direction:?} signal `{key}`.").into(),
        direction,
        value_type,
        unit: None,
        required,
    }
}

/// Returns the catalog category for one stable component type.
fn category(type_id: &str) -> &'static str {
    match type_id {
        "signal.constant" | "signal.step" | "signal.ramp" => "Signal/Sources",
        "signal.integrator" | "signal.delay" | "signal.first_order_transfer" => "Signal/Control",
        "signal.switch" => "Signal/Routing",
        "signal.greater_than" | "signal.boolean_not" => "Signal/Logic",
        "signal.lookup" => "Signal/Lookup",
        "signal.assertion" | "signal.probe" => "Signal/Sinks",
        "signal.unit_conversion" => "Signal/Units",
        "signal.expression" => "Signal/Expressions",
        "signal.gain" | "signal.add" | "signal.multiply" | "signal.subtract" | "signal.divide"
        | "signal.clamp" => "Signal/Math",
        _ => "Signal",
    }
}

/// Returns user-facing behavior documentation for one stable component type.
fn documentation(type_id: &str) -> &'static str {
    match type_id {
        "signal.constant" => "Emits the configured value at every sample.",
        "signal.step" => "Switches from the initial value to the final value at the step time.",
        "signal.gain" => "Multiplies the input by a configured scalar gain.",
        "signal.add" => "Adds two scalar inputs.",
        "signal.integrator" => "Integrates its input using deterministic forward Euler updates.",
        "signal.delay" => "Delays its input by one fixed simulation step.",
        "signal.probe" => "Passes a signal through for explicit graph-level observation.",
        "signal.ramp" => "Emits a linear ramp after the configured start time.",
        "signal.multiply" => "Multiplies two scalar inputs.",
        "signal.first_order_transfer" => {
            "Applies a first-order transfer function using forward Euler updates."
        }
        "signal.unit_conversion" => "Converts a scalar between compatible declared units.",
        "signal.expression" => "Evaluates a compiled scalar expression over input and time.",
        "signal.subtract" => "Subtracts input b from input a.",
        "signal.divide" => "Divides input a by nonzero input b.",
        "signal.clamp" => "Limits a scalar to an inclusive configured range.",
        "signal.switch" => "Routes one of two scalar inputs using a Boolean selector.",
        "signal.greater_than" => "Emits true when input a is greater than input b.",
        "signal.boolean_not" => "Inverts a Boolean input.",
        "signal.lookup" => "Interpolates between two points and clamps outside the domain.",
        "signal.assertion" => "Fails the run when its input leaves an inclusive configured range.",
        _ => "Deterministic signal component.",
    }
}

/// Extracts a Boolean runtime value.
fn boolean(
    value: &RuntimeValue,
    component_id: ComponentId,
    field: &str,
) -> Result<bool, Diagnostic> {
    match value {
        RuntimeValue::Boolean(value) => Ok(*value),
        RuntimeValue::Integer(_)
        | RuntimeValue::Scalar(_)
        | RuntimeValue::ScalarWithUnit { .. }
        | RuntimeValue::String(_)
        | RuntimeValue::Identifier(_)
        | RuntimeValue::Path(_)
        | RuntimeValue::Table(_)
        | RuntimeValue::Unit(_) => Err(runtime_diagnostic(
            component_id,
            field,
            "simulation_runtime_expected_boolean",
        )),
    }
}

/// Returns a required value or a stable missing-value diagnostic.
fn input<'a>(
    values: &'a RuntimeValues,
    key: &str,
    component_id: ComponentId,
) -> Result<&'a RuntimeValue, Diagnostic> {
    values
        .get(key)
        .ok_or_else(|| runtime_diagnostic(component_id, key, "simulation_runtime_missing_value"))
}

/// Returns a required parameter.
fn parameter<'a>(
    values: &'a RuntimeValues,
    key: &str,
    component_id: ComponentId,
) -> Result<&'a RuntimeValue, Diagnostic> {
    input(values, key, component_id)
}

/// Extracts a unitless scalar runtime value.
fn scalar(value: &RuntimeValue, component_id: ComponentId, field: &str) -> Result<f64, Diagnostic> {
    match value {
        RuntimeValue::Scalar(value) => Ok(*value),
        RuntimeValue::Boolean(_)
        | RuntimeValue::Integer(_)
        | RuntimeValue::ScalarWithUnit { .. }
        | RuntimeValue::String(_)
        | RuntimeValue::Identifier(_)
        | RuntimeValue::Path(_)
        | RuntimeValue::Table(_)
        | RuntimeValue::Unit(_) => Err(runtime_diagnostic(
            component_id,
            field,
            "simulation_runtime_expected_scalar",
        )),
    }
}

/// Extracts a unit selector runtime value.
fn unit_parameter(
    parameters: &RuntimeValues,
    key: &str,
    component_id: ComponentId,
) -> Result<UnitId, Diagnostic> {
    match parameter(parameters, key, component_id)? {
        RuntimeValue::Unit(unit) => Ok(*unit),
        RuntimeValue::Boolean(_)
        | RuntimeValue::Integer(_)
        | RuntimeValue::Scalar(_)
        | RuntimeValue::ScalarWithUnit { .. }
        | RuntimeValue::String(_)
        | RuntimeValue::Identifier(_)
        | RuntimeValue::Path(_)
        | RuntimeValue::Table(_) => Err(runtime_diagnostic(
            component_id,
            key,
            "simulation_runtime_expected_unit",
        )),
    }
}

/// Extracts a string configuration value.
fn string_parameter<'a>(
    parameters: &'a RuntimeValues,
    key: &str,
    component_id: ComponentId,
) -> Result<&'a str, Diagnostic> {
    match parameter(parameters, key, component_id)? {
        RuntimeValue::String(value) => Ok(value.as_str()),
        RuntimeValue::Boolean(_)
        | RuntimeValue::Integer(_)
        | RuntimeValue::Scalar(_)
        | RuntimeValue::ScalarWithUnit { .. }
        | RuntimeValue::Identifier(_)
        | RuntimeValue::Path(_)
        | RuntimeValue::Table(_)
        | RuntimeValue::Unit(_) => Err(runtime_diagnostic(
            component_id,
            key,
            "simulation_runtime_expected_string",
        )),
    }
}

/// Validates a computed finite scalar.
fn finite(value: f64, component_id: ComponentId, field: &str) -> Result<f64, Diagnostic> {
    value.is_finite().then_some(value).ok_or_else(|| {
        runtime_diagnostic(component_id, field, "simulation_runtime_nonfinite_value")
    })
}

/// Creates one scalar output update after finite validation.
fn finite_output(value: f64, component_id: ComponentId) -> Result<ComponentUpdate, Diagnostic> {
    Ok(output(RuntimeValue::Scalar(finite(
        value,
        component_id,
        "out",
    )?)))
}

/// Creates a stateless single-output update.
fn output(value: RuntimeValue) -> ComponentUpdate {
    ComponentUpdate {
        outputs: values([("out", value)]),
        next_state: RuntimeValues::new(),
    }
}

/// Collects static string keys into runtime storage.
fn values<const N: usize>(entries: [(&str, RuntimeValue); N]) -> RuntimeValues {
    entries
        .into_iter()
        .map(|(key, value)| (key.into(), value))
        .collect()
}

/// Creates a component-scoped runtime diagnostic.
fn runtime_diagnostic(component_id: ComponentId, field: &str, key: &'static str) -> Diagnostic {
    Diagnostic::new(
        DiagnosticSeverity::Error,
        DiagnosticCategory::Runtime,
        Some(EntityReference::Component(component_id)),
        Some(field.into()),
        key,
    )
}

#[cfg(test)]
mod tests {
    use super::{BuiltinBehavior, BuiltinKind, register_signal_builtins, values};
    use crate::component::{ComponentBehavior, ComponentCapability, RuntimeValues, StepContext};
    use crate::identity::{ComponentId, RunId};
    use crate::registry::ComponentRegistry;
    use crate::value::RuntimeValue;
    use units::UnitId;

    fn context(time: f64) -> StepContext {
        StepContext {
            run_id: RunId::from_raw(1),
            time,
            timestep: 0.25,
            step_index: 0,
        }
    }

    #[test]
    fn registers_all_available_primitives_with_factories() {
        let mut registry = ComponentRegistry::new();
        register_signal_builtins(&mut registry).unwrap();

        assert_eq!(registry.iter().count(), 20);
        for definition in registry.iter() {
            assert!(registry.factory(&definition.type_id).is_some());
        }
    }

    #[test]
    fn step_switches_on_its_declared_grid_time() {
        let behavior = BuiltinBehavior {
            component_id: ComponentId::from_raw(1),
            kind: BuiltinKind::Step,
            expression: None,
        };
        let parameters = values([
            ("initial_value", RuntimeValue::Scalar(2.0)),
            ("final_value", RuntimeValue::Scalar(5.0)),
            ("step_time", RuntimeValue::Scalar(0.5)),
        ]);

        assert_eq!(
            behavior
                .initialize(context(0.25), &parameters)
                .unwrap()
                .outputs["out"],
            RuntimeValue::Scalar(2.0)
        );
        assert_eq!(
            behavior
                .initialize(context(0.5), &parameters)
                .unwrap()
                .outputs["out"],
            RuntimeValue::Scalar(5.0)
        );
    }

    #[test]
    fn integrator_returns_atomic_forward_euler_update() {
        let behavior = BuiltinBehavior {
            component_id: ComponentId::from_raw(1),
            kind: BuiltinKind::Integrator,
            expression: None,
        };
        let update = behavior
            .evaluate(
                context(0.0),
                &RuntimeValues::new(),
                &values([("in", RuntimeValue::Scalar(4.0))]),
                &values([("value", RuntimeValue::Scalar(1.0))]),
            )
            .unwrap();

        assert_eq!(update.outputs["out"], RuntimeValue::Scalar(2.0));
        assert_eq!(update.next_state["value"], RuntimeValue::Scalar(2.0));
    }

    #[test]
    fn stateful_definitions_break_direct_feedthrough() {
        let mut registry = ComponentRegistry::new();
        register_signal_builtins(&mut registry).unwrap();
        let integrator = registry
            .iter()
            .find(|definition| definition.type_id.as_str() == "signal.integrator")
            .unwrap();

        assert!(
            integrator
                .capabilities
                .contains(ComponentCapability::Stateful)
        );
        assert!(
            !integrator
                .capabilities
                .contains(ComponentCapability::DirectFeedthrough)
        );
    }

    #[test]
    fn ramp_observes_start_boundary() {
        let behavior = BuiltinBehavior {
            component_id: ComponentId::from_raw(1),
            kind: BuiltinKind::Ramp,
            expression: None,
        };
        let parameters = values([
            ("initial_value", RuntimeValue::Scalar(3.0)),
            ("slope", RuntimeValue::Scalar(2.0)),
            ("start_time", RuntimeValue::Scalar(0.5)),
        ]);

        assert_eq!(
            behavior
                .initialize(context(0.25), &parameters)
                .unwrap()
                .outputs["out"],
            RuntimeValue::Scalar(3.0)
        );
        assert_eq!(
            behavior
                .initialize(context(1.0), &parameters)
                .unwrap()
                .outputs["out"],
            RuntimeValue::Scalar(4.0)
        );
    }

    #[test]
    fn first_order_transfer_has_exact_euler_sequence() {
        let behavior = BuiltinBehavior {
            component_id: ComponentId::from_raw(1),
            kind: BuiltinKind::FirstOrderTransfer,
            expression: None,
        };
        let parameters = values([
            ("gain", RuntimeValue::Scalar(2.0)),
            ("time_constant", RuntimeValue::Scalar(0.5)),
            ("initial_value", RuntimeValue::Scalar(0.0)),
        ]);
        let initialized = behavior.initialize(context(0.0), &parameters).unwrap();
        let first = behavior
            .evaluate(
                context(0.0),
                &parameters,
                &values([("in", RuntimeValue::Scalar(1.0))]),
                &initialized.next_state,
            )
            .unwrap();
        let second = behavior
            .evaluate(
                context(0.25),
                &parameters,
                &values([("in", RuntimeValue::Scalar(1.0))]),
                &first.next_state,
            )
            .unwrap();

        assert_eq!(first.outputs["out"], RuntimeValue::Scalar(1.0));
        assert_eq!(second.outputs["out"], RuntimeValue::Scalar(1.5));
    }

    #[test]
    fn unit_conversion_uses_declared_units() {
        let behavior = BuiltinBehavior {
            component_id: ComponentId::from_raw(1),
            kind: BuiltinKind::UnitConversion,
            expression: None,
        };
        let parameters = values([
            ("from_unit", RuntimeValue::Unit(UnitId::Time_Minute)),
            ("to_unit", RuntimeValue::Unit(UnitId::Time_Second)),
        ]);
        let update = behavior
            .evaluate(
                context(0.0),
                &parameters,
                &values([("in", RuntimeValue::Scalar(2.0))]),
                &RuntimeValues::new(),
            )
            .unwrap();

        assert_eq!(
            update.outputs["out"],
            RuntimeValue::ScalarWithUnit {
                value: 120.0,
                unit: UnitId::Time_Second,
            }
        );
    }

    #[test]
    fn expression_factory_compiles_once_and_evaluates_runtime_symbols() {
        let factory = super::BuiltinFactory {
            kind: BuiltinKind::Expression,
        };
        let parameters = values([("expression", RuntimeValue::String("2 * x + time".into()))]);
        let behavior =
            super::ComponentFactory::create(&factory, ComponentId::from_raw(1), &parameters)
                .unwrap();

        let update = behavior
            .evaluate(
                context(0.5),
                &parameters,
                &values([("in", RuntimeValue::Scalar(3.0))]),
                &RuntimeValues::new(),
            )
            .unwrap();

        assert_eq!(update.outputs["out"], RuntimeValue::Scalar(6.5));
    }

    #[test]
    fn phase_seven_primitives_initialize_with_declared_output_types() {
        for kind in [
            BuiltinKind::Subtract,
            BuiltinKind::Divide,
            BuiltinKind::Clamp,
            BuiltinKind::Switch,
            BuiltinKind::Lookup,
            BuiltinKind::Assertion,
        ] {
            assert_eq!(
                behavior(kind)
                    .initialize(context(0.0), &RuntimeValues::new())
                    .unwrap()
                    .outputs["out"],
                RuntimeValue::Scalar(0.0)
            );
        }
        for kind in [BuiltinKind::GreaterThan, BuiltinKind::BooleanNot] {
            assert_eq!(
                behavior(kind)
                    .initialize(context(0.0), &RuntimeValues::new())
                    .unwrap()
                    .outputs["out"],
                RuntimeValue::Boolean(false)
            );
        }
    }

    #[test]
    fn phase_seven_math_control_and_routing_behaviors_match_references() {
        let empty = RuntimeValues::new();
        assert_output(
            BuiltinKind::Subtract,
            &empty,
            &values([
                ("a", RuntimeValue::Scalar(7.0)),
                ("b", RuntimeValue::Scalar(2.0)),
            ]),
            RuntimeValue::Scalar(5.0),
        );
        assert_output(
            BuiltinKind::Divide,
            &empty,
            &values([
                ("a", RuntimeValue::Scalar(7.0)),
                ("b", RuntimeValue::Scalar(2.0)),
            ]),
            RuntimeValue::Scalar(3.5),
        );
        assert_output(
            BuiltinKind::Clamp,
            &values([
                ("minimum", RuntimeValue::Scalar(-1.0)),
                ("maximum", RuntimeValue::Scalar(1.0)),
            ]),
            &values([("in", RuntimeValue::Scalar(4.0))]),
            RuntimeValue::Scalar(1.0),
        );
        assert_output(
            BuiltinKind::Switch,
            &empty,
            &values([
                ("select", RuntimeValue::Boolean(true)),
                ("false", RuntimeValue::Scalar(2.0)),
                ("true", RuntimeValue::Scalar(9.0)),
            ]),
            RuntimeValue::Scalar(9.0),
        );
    }

    #[test]
    fn phase_seven_logic_lookup_and_assertion_behaviors_match_references() {
        let empty = RuntimeValues::new();
        assert_output(
            BuiltinKind::GreaterThan,
            &empty,
            &values([
                ("a", RuntimeValue::Scalar(3.0)),
                ("b", RuntimeValue::Scalar(2.0)),
            ]),
            RuntimeValue::Boolean(true),
        );
        assert_output(
            BuiltinKind::BooleanNot,
            &empty,
            &values([("in", RuntimeValue::Boolean(true))]),
            RuntimeValue::Boolean(false),
        );
        let lookup_parameters = values([
            ("x0", RuntimeValue::Scalar(0.0)),
            ("y0", RuntimeValue::Scalar(10.0)),
            ("x1", RuntimeValue::Scalar(2.0)),
            ("y1", RuntimeValue::Scalar(20.0)),
        ]);
        assert_output(
            BuiltinKind::Lookup,
            &lookup_parameters,
            &values([("in", RuntimeValue::Scalar(1.0))]),
            RuntimeValue::Scalar(15.0),
        );
        assert_output(
            BuiltinKind::Assertion,
            &values([
                ("minimum", RuntimeValue::Scalar(0.0)),
                ("maximum", RuntimeValue::Scalar(2.0)),
            ]),
            &values([("in", RuntimeValue::Scalar(2.0))]),
            RuntimeValue::Scalar(2.0),
        );
    }

    #[test]
    fn phase_seven_boundaries_are_explicit_and_diagnostic() {
        let divide = behavior(BuiltinKind::Divide)
            .evaluate(
                context(0.0),
                &RuntimeValues::new(),
                &values([
                    ("a", RuntimeValue::Scalar(1.0)),
                    ("b", RuntimeValue::Scalar(0.0)),
                ]),
                &RuntimeValues::new(),
            )
            .unwrap_err();
        assert_eq!(divide.message_key(), "simulation_runtime_division_by_zero");

        let invalid_range = values([
            ("minimum", RuntimeValue::Scalar(2.0)),
            ("maximum", RuntimeValue::Scalar(1.0)),
        ]);
        let clamp = behavior(BuiltinKind::Clamp)
            .evaluate(
                context(0.0),
                &invalid_range,
                &values([("in", RuntimeValue::Scalar(1.0))]),
                &RuntimeValues::new(),
            )
            .unwrap_err();
        assert_eq!(clamp.message_key(), "simulation_runtime_invalid_range");

        let assertion = behavior(BuiltinKind::Assertion)
            .evaluate(
                context(0.0),
                &values([
                    ("minimum", RuntimeValue::Scalar(0.0)),
                    ("maximum", RuntimeValue::Scalar(1.0)),
                ]),
                &values([("in", RuntimeValue::Scalar(2.0))]),
                &RuntimeValues::new(),
            )
            .unwrap_err();
        assert_eq!(
            assertion.message_key(),
            "simulation_runtime_assertion_failed"
        );
    }

    #[test]
    fn phase_seven_lookup_clamps_and_rejects_degenerate_domains() {
        let parameters = values([
            ("x0", RuntimeValue::Scalar(0.0)),
            ("y0", RuntimeValue::Scalar(10.0)),
            ("x1", RuntimeValue::Scalar(2.0)),
            ("y1", RuntimeValue::Scalar(20.0)),
        ]);
        assert_output(
            BuiltinKind::Lookup,
            &parameters,
            &values([("in", RuntimeValue::Scalar(-4.0))]),
            RuntimeValue::Scalar(10.0),
        );
        let invalid = behavior(BuiltinKind::Lookup)
            .evaluate(
                context(0.0),
                &values([
                    ("x0", RuntimeValue::Scalar(1.0)),
                    ("y0", RuntimeValue::Scalar(0.0)),
                    ("x1", RuntimeValue::Scalar(1.0)),
                    ("y1", RuntimeValue::Scalar(2.0)),
                ]),
                &values([("in", RuntimeValue::Scalar(1.0))]),
                &RuntimeValues::new(),
            )
            .unwrap_err();
        assert_eq!(
            invalid.message_key(),
            "simulation_runtime_invalid_lookup_domain"
        );
    }

    #[test]
    fn phase_seven_definitions_declare_scalar_and_boolean_units() {
        let mut registry = ComponentRegistry::new();
        register_signal_builtins(&mut registry).unwrap();
        for definition in registry.iter().filter(|definition| {
            matches!(
                definition.type_id.as_str(),
                "signal.subtract"
                    | "signal.divide"
                    | "signal.clamp"
                    | "signal.switch"
                    | "signal.greater_than"
                    | "signal.boolean_not"
                    | "signal.lookup"
                    | "signal.assertion"
            )
        }) {
            assert!(definition.ports.iter().all(|port| port.unit.is_none()));
        }
    }

    #[test]
    fn phase_seven_fresh_instances_reset_to_identical_initial_outputs() {
        for kind in [
            BuiltinKind::Subtract,
            BuiltinKind::Divide,
            BuiltinKind::Clamp,
            BuiltinKind::Switch,
            BuiltinKind::GreaterThan,
            BuiltinKind::BooleanNot,
            BuiltinKind::Lookup,
            BuiltinKind::Assertion,
        ] {
            let first = behavior(kind)
                .initialize(context(0.0), &RuntimeValues::new())
                .unwrap();
            let second = behavior(kind)
                .initialize(context(0.0), &RuntimeValues::new())
                .unwrap();
            assert_eq!(first, second);
        }
    }

    fn behavior(kind: BuiltinKind) -> BuiltinBehavior {
        BuiltinBehavior {
            component_id: ComponentId::from_raw(1),
            kind,
            expression: None,
        }
    }

    fn assert_output(
        kind: BuiltinKind,
        parameters: &RuntimeValues,
        inputs: &RuntimeValues,
        expected: RuntimeValue,
    ) {
        let mut update = behavior(kind)
            .evaluate(context(0.0), parameters, inputs, &RuntimeValues::new())
            .unwrap();
        assert_eq!(update.outputs.remove("out"), Some(expected));
    }
}
