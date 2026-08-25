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
use crate::value::RuntimeValue;
use std::sync::Arc;

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

/// Registers the Phase 3 signal/control primitive library.
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
}

/// Factory for one built-in algorithm kind.
#[derive(Debug)]
struct BuiltinFactory {
    /// Algorithm selected by the registered definition.
    kind: BuiltinKind,
}

impl ComponentFactory for BuiltinFactory {
    fn create(&self, component_id: ComponentId) -> Box<dyn ComponentBehavior> {
        Box::new(BuiltinBehavior {
            component_id,
            kind: self.kind,
        })
    }
}

/// Stateless algorithm object; all mutable simulation state is runtime-owned.
#[derive(Debug)]
struct BuiltinBehavior {
    /// Instance identity used for diagnostics.
    component_id: ComponentId,
    /// Selected primitive algorithm.
    kind: BuiltinKind,
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
            BuiltinKind::Integrator | BuiltinKind::Delay => {
                let initial = parameter(parameters, "initial_value", self.component_id)?;
                Ok(ComponentUpdate {
                    outputs: values([("out", initial.clone())]),
                    next_state: values([("value", initial.clone())]),
                })
            }
            BuiltinKind::Gain | BuiltinKind::Add | BuiltinKind::Probe => {
                Ok(output(RuntimeValue::Scalar(0.0)))
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
            BuiltinKind::Probe => Ok(output(input(inputs, "in", self.component_id)?.clone())),
        }
    }
}

impl BuiltinBehavior {
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
    ])
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
        category: "Signal".into(),
        aliases: vec![],
        tags: vec!["deterministic".into()],
        documentation: "".into(),
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
        description: "".into(),
        value_type: ParameterValueType::Scalar,
        default_expression: default_expression.into(),
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

/// Creates common scalar port metadata.
fn port(key: &str, direction: PortDirection, required: bool) -> PortDefinition {
    PortDefinition {
        key: key.into(),
        display_name: key.into(),
        description: "".into(),
        direction,
        value_type: ParameterValueType::Scalar,
        unit: None,
        required,
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

    fn context(time: f64) -> StepContext {
        StepContext {
            run_id: RunId::from_raw(1),
            time,
            timestep: 0.25,
            step_index: 0,
        }
    }

    #[test]
    fn registers_all_phase_three_primitives_with_factories() {
        let mut registry = ComponentRegistry::new();
        register_signal_builtins(&mut registry).unwrap();

        assert_eq!(registry.iter().count(), 7);
        for definition in registry.iter() {
            assert!(registry.factory(&definition.type_id).is_some());
        }
    }

    #[test]
    fn step_switches_on_its_declared_grid_time() {
        let behavior = BuiltinBehavior {
            component_id: ComponentId::from_raw(1),
            kind: BuiltinKind::Step,
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
}
