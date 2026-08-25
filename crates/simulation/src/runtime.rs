use crate::component::{
    ComponentBehavior, ComponentCapability, ComponentUpdate, RuntimeValues, StepContext,
};
use crate::diagnostic::{Diagnostic, DiagnosticCategory, DiagnosticSeverity, EntityReference};
use crate::identity::{ComponentId, RunId};
use crate::parameter::ParameterValueType;
use crate::registry::ComponentRegistry;
use crate::resolve::{ResolvedComponentSource, ResolvedModel};
use crate::results::{RunStatus, SignalSeries, SimulationRun};
use crate::schedule::{SystemSchedule, build_schedule};
use crate::timing::FixedStepPlan;
use crate::validation::{ValidationLimits, validate_model};
use crate::value::RuntimeValue;
use datastore::prelude::{
    NumberDefinition, NumberWithUnitsDefinition, ParameterObjectDefinition, ParameterObjectFrozen,
    parameter_key,
};
use expression_engine::prelude::{ExpressionEngine, ParameterObjectInputData};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Diagnostics that prevent runtime construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeBuildFailure {
    /// Run-blocking diagnostics in deterministic order.
    pub diagnostics: Vec<Diagnostic>,
}

/// One configured executable component instance.
#[derive(Debug)]
struct RuntimeComponent {
    /// Immutable algorithm instance.
    behavior: Box<dyn ComponentBehavior>,
    /// Evaluated configuration values.
    parameters: RuntimeValues,
    /// Atomically committed owned state.
    state: RuntimeValues,
    /// Atomically committed outputs.
    outputs: RuntimeValues,
    /// Whether this component owns transition state.
    stateful: bool,
    /// Whether outputs must be recomputed on each sample grid point.
    sample_evaluated: bool,
}

/// Prepared, reusable synchronous fixed-step runtime for one resolved model.
#[derive(Debug)]
pub struct SimulationRuntime {
    /// Immutable resolved source snapshot.
    model: ResolvedModel,
    /// Deterministic root-system execution plan.
    schedule: SystemSchedule,
    /// Configured root-system component instances.
    components: BTreeMap<ComponentId, RuntimeComponent>,
}

impl SimulationRuntime {
    /// Validates and configures a runtime from a resolved model and executable registry.
    ///
    /// # Errors
    ///
    /// Returns all deterministic configuration diagnostics discovered before execution.
    pub fn new(
        model: &ResolvedModel,
        registry: &ComponentRegistry,
    ) -> Result<Self, RuntimeBuildFailure> {
        let diagnostics = validate_model(model, ValidationLimits::default());
        if !diagnostics.is_empty() {
            return Err(RuntimeBuildFailure { diagnostics });
        }
        let schedule = build_schedule(model).map_err(|failure| RuntimeBuildFailure {
            diagnostics: vec![failure.diagnostic],
        })?;

        let engine = ExpressionEngine::new();
        let mut components = BTreeMap::new();
        let mut diagnostics = Vec::new();
        for component in &model.root.components {
            let ResolvedComponentSource::BuiltIn { type_id, .. } = &component.source else {
                diagnostics.push(build_diagnostic(
                    component.id,
                    "component",
                    "simulation_runtime_custom_component_not_supported",
                ));
                continue;
            };
            let Some(factory) = registry.factory(type_id) else {
                diagnostics.push(build_diagnostic(
                    component.id,
                    "component",
                    "simulation_runtime_missing_factory",
                ));
                continue;
            };
            let parameters = match evaluate_parameters(component, &engine) {
                Ok(parameters) => parameters,
                Err(mut errors) => {
                    diagnostics.append(&mut errors);
                    continue;
                }
            };
            components.insert(
                component.id,
                RuntimeComponent {
                    behavior: factory.create(component.id),
                    parameters,
                    state: RuntimeValues::new(),
                    outputs: RuntimeValues::new(),
                    stateful: component
                        .capabilities
                        .contains(ComponentCapability::Stateful),
                    sample_evaluated: component
                        .capabilities
                        .contains(ComponentCapability::DirectFeedthrough)
                        || component
                            .capabilities
                            .contains(ComponentCapability::TimeDependent),
                },
            );
        }
        if !diagnostics.is_empty() {
            return Err(RuntimeBuildFailure { diagnostics });
        }

        Ok(Self {
            model: model.clone(),
            schedule,
            components,
        })
    }

    /// Clears committed state and outputs so the next run starts from initialization.
    pub fn reset(&mut self) {
        for component in self.components.values_mut() {
            component.state.clear();
            component.outputs.clear();
        }
    }

    /// Executes the complete fixed-step grid synchronously.
    #[must_use]
    pub fn run(&mut self, run_id: RunId) -> SimulationRun {
        self.reset();
        let mut series = self.empty_series();
        let Ok(plan) = FixedStepPlan::new(
            self.model.simulation.start_time,
            self.model.simulation.stop_time,
            self.model.simulation.timestep,
        ) else {
            return self.failed_run(
                run_id,
                series,
                runtime_diagnostic(None, "simulation", "simulation_runtime_invalid_timing"),
            );
        };
        let initial_context = self.context(run_id, self.model.simulation.start_time, 0);
        if let Err(diagnostic) = self.initialize(initial_context) {
            return self.failed_run(run_id, series, diagnostic);
        }
        if let Err(diagnostic) = self.evaluate_sample(initial_context) {
            return self.failed_run(run_id, series, diagnostic);
        }
        if let Err(diagnostic) = self.capture(initial_context.time, &mut series) {
            return self.failed_run(run_id, series, diagnostic);
        }

        for step_index in 0..plan.step_count() {
            let Some(time) = plan.sample_time(step_index) else {
                return self.failed_run(
                    run_id,
                    series,
                    runtime_diagnostic(None, "time", "simulation_runtime_invalid_grid_index"),
                );
            };
            let context = self.context(run_id, time, step_index);
            if let Err(diagnostic) = self.transition_state(context) {
                return self.failed_run(run_id, series, diagnostic);
            }
            let Some(next_index) = step_index.checked_add(1) else {
                return self.failed_run(
                    run_id,
                    series,
                    runtime_diagnostic(None, "step_index", "simulation_runtime_step_overflow"),
                );
            };
            let Some(next_time) = plan.sample_time(next_index) else {
                return self.failed_run(
                    run_id,
                    series,
                    runtime_diagnostic(None, "time", "simulation_runtime_invalid_grid_index"),
                );
            };
            let next_context = self.context(run_id, next_time, next_index);
            if let Err(diagnostic) = self.evaluate_sample(next_context) {
                return self.failed_run(run_id, series, diagnostic);
            }
            if let Err(diagnostic) = self.capture(next_time, &mut series) {
                return self.failed_run(run_id, series, diagnostic);
            }
        }

        let final_context = self.context(
            run_id,
            plan.sample_time(plan.step_count())
                .unwrap_or(self.model.simulation.stop_time),
            plan.step_count(),
        );
        if let Err(diagnostic) = self.finalize(final_context) {
            return self.failed_run(run_id, series, diagnostic);
        }

        SimulationRun {
            run_id,
            source_document_id: self.model.document_id,
            settings: self.model.simulation,
            status: RunStatus::Completed,
            diagnostics: vec![],
            series,
        }
    }

    /// Initializes every component before current-time propagation.
    fn initialize(&mut self, context: StepContext) -> Result<(), Diagnostic> {
        for component in self.components.values_mut() {
            let update = component
                .behavior
                .initialize(context, &component.parameters)?;
            component.outputs = update.outputs;
            component.state = update.next_state;
        }
        Ok(())
    }

    /// Recomputes time-dependent and direct-feedthrough outputs in schedule order.
    fn evaluate_sample(&mut self, context: StepContext) -> Result<(), Diagnostic> {
        for component_id in &self.schedule.component_order {
            let should_evaluate = self
                .components
                .get(component_id)
                .is_some_and(|component| !component.stateful && component.sample_evaluated);
            if !should_evaluate {
                continue;
            }
            let inputs = self.inputs_for(*component_id)?;
            let Some(component) = self.components.get_mut(component_id) else {
                continue;
            };
            let update = component.behavior.evaluate(
                context,
                &component.parameters,
                &inputs,
                &component.state,
            )?;
            component.outputs = update.outputs;
        }
        Ok(())
    }

    /// Computes every stateful update before committing any of them.
    fn transition_state(&mut self, context: StepContext) -> Result<(), Diagnostic> {
        let mut pending = Vec::<(ComponentId, ComponentUpdate)>::new();
        for component_id in &self.schedule.component_order {
            if !self
                .components
                .get(component_id)
                .is_some_and(|component| component.stateful)
            {
                continue;
            }
            let inputs = self.inputs_for(*component_id)?;
            let Some(component) = self.components.get(component_id) else {
                continue;
            };
            let update = component.behavior.evaluate(
                context,
                &component.parameters,
                &inputs,
                &component.state,
            )?;
            pending.push((*component_id, update));
        }
        for (component_id, update) in pending {
            if let Some(component) = self.components.get_mut(&component_id) {
                component.outputs = update.outputs;
                component.state = update.next_state;
            }
        }
        Ok(())
    }

    /// Finalizes every configured component after the last sample.
    fn finalize(&self, context: StepContext) -> Result<(), Diagnostic> {
        for component in self.components.values() {
            component
                .behavior
                .finalize(context, &component.parameters, &component.state)?;
        }
        Ok(())
    }

    /// Collects current source outputs for one target component.
    fn inputs_for(&self, target_id: ComponentId) -> Result<RuntimeValues, Diagnostic> {
        let mut inputs = RuntimeValues::new();
        for connection in self
            .model
            .root
            .connections
            .iter()
            .filter(|connection| connection.target.component_id == target_id)
        {
            let value = self
                .components
                .get(&connection.source.component_id)
                .and_then(|source| source.outputs.get(&connection.source.port_key))
                .cloned()
                .ok_or_else(|| {
                    runtime_diagnostic(
                        Some(target_id),
                        connection.target.port_key.as_str(),
                        "simulation_runtime_missing_connected_output",
                    )
                })?;
            inputs.insert(connection.target.port_key.clone(), value);
        }
        Ok(inputs)
    }

    /// Captures every model probe at one grid time.
    fn capture(&self, time: f64, series: &mut [SignalSeries]) -> Result<(), Diagnostic> {
        for (probe, signal) in self.model.probes.iter().zip(series.iter_mut()) {
            let value = self
                .components
                .get(&probe.target.component_id)
                .and_then(|component| component.outputs.get(&probe.target.port_key))
                .cloned()
                .ok_or_else(|| {
                    runtime_diagnostic(
                        Some(probe.target.component_id),
                        probe.target.port_key.as_str(),
                        "simulation_runtime_missing_probe_value",
                    )
                })?;
            signal.timestamps.push(time);
            signal.values.push(value);
        }
        Ok(())
    }

    /// Creates empty result series in persisted probe order.
    fn empty_series(&self) -> Vec<SignalSeries> {
        self.model
            .probes
            .iter()
            .map(|probe| SignalSeries {
                probe_id: probe.id,
                source: probe.target.clone(),
                display_name: probe.display_name.clone(),
                timestamps: Vec::new(),
                values: Vec::new(),
            })
            .collect()
    }

    /// Creates a step context from immutable run settings.
    const fn context(&self, run_id: RunId, time: f64, step_index: u64) -> StepContext {
        StepContext {
            run_id,
            time,
            timestep: self.model.simulation.timestep,
            step_index,
        }
    }

    /// Builds a failed run retaining any samples captured before the error.
    fn failed_run(
        &self,
        run_id: RunId,
        series: Vec<SignalSeries>,
        diagnostic: Diagnostic,
    ) -> SimulationRun {
        SimulationRun {
            run_id,
            source_document_id: self.model.document_id,
            settings: self.model.simulation,
            status: RunStatus::Failed,
            diagnostics: vec![diagnostic],
            series,
        }
    }
}

/// Evaluates every component parameter once through the expression engine.
fn evaluate_parameters(
    component: &crate::resolve::ResolvedComponent,
    engine: &ExpressionEngine,
) -> Result<RuntimeValues, Vec<Diagnostic>> {
    let mut values = RuntimeValues::new();
    let mut diagnostics = Vec::new();
    for parameter in &component.parameters {
        let expression = component
            .parameter_overrides
            .get(&parameter.key)
            .unwrap_or(&parameter.default_expression);
        let definition = match parameter.value_type {
            ParameterValueType::Scalar => ParameterObjectDefinition::builder("Runtime parameter")
                .with(
                    parameter_key!("p_value"),
                    NumberDefinition::new_with_default(&parameter.description, expression),
                )
                .finish(),
            ParameterValueType::ScalarWithUnit(unit) => {
                ParameterObjectDefinition::builder("Runtime parameter")
                    .with(
                        parameter_key!("p_value"),
                        NumberWithUnitsDefinition::new_with_default(
                            &parameter.description,
                            expression,
                            unit,
                        ),
                    )
                    .finish()
            }
            ParameterValueType::Boolean
            | ParameterValueType::Choice
            | ParameterValueType::File
            | ParameterValueType::Folder
            | ParameterValueType::Integer
            | ParameterValueType::String
            | ParameterValueType::Table
            | ParameterValueType::TableWithUnits(_)
            | ParameterValueType::Unit(_) => {
                diagnostics.push(build_diagnostic(
                    component.id,
                    parameter.key.as_str(),
                    "simulation_runtime_unsupported_parameter_type",
                ));
                continue;
            }
        };
        let input = ParameterObjectInputData::new(&ParameterObjectFrozen::new(definition));
        match engine.evaluate_parameters(&input) {
            Ok(computed) => {
                let Some(item) = computed.get("p_value") else {
                    diagnostics.push(build_diagnostic(
                        component.id,
                        parameter.key.as_str(),
                        "simulation_runtime_missing_parameter_value",
                    ));
                    continue;
                };
                match RuntimeValue::try_from(item) {
                    Ok(value) => {
                        values.insert(parameter.key.clone(), value);
                    }
                    Err(_) => diagnostics.push(build_diagnostic(
                        component.id,
                        parameter.key.as_str(),
                        "simulation_runtime_invalid_parameter_value",
                    )),
                }
            }
            Err(messages) => diagnostics.extend(messages.into_iter().map(|message| {
                Diagnostic::from_message(
                    message,
                    Some(EntityReference::Component(component.id)),
                    Some(parameter.key.clone()),
                )
            })),
        }
    }
    if diagnostics.is_empty() {
        Ok(values)
    } else {
        Err(diagnostics)
    }
}

/// Creates a component-scoped runtime-construction diagnostic.
fn build_diagnostic(component_id: ComponentId, field: &str, key: &'static str) -> Diagnostic {
    runtime_diagnostic(Some(component_id), field, key)
}

/// Creates a stable runtime diagnostic with optional component context.
fn runtime_diagnostic(
    component_id: Option<ComponentId>,
    field: &str,
    key: &'static str,
) -> Diagnostic {
    Diagnostic::new(
        DiagnosticSeverity::Error,
        DiagnosticCategory::Runtime,
        component_id.map(EntityReference::Component),
        Some(ShareableString::from(field)),
        key,
    )
}

#[cfg(test)]
mod tests {
    use super::SimulationRuntime;
    use crate::builtins::register_signal_builtins;
    use crate::component::ComponentTypeId;
    use crate::diagnostic::{Diagnostic, DiagnosticCategory, DiagnosticSeverity};
    use crate::document::{
        CanvasPosition, ComponentInstance, ComponentReference, Composition, Connection,
        DocumentHeader, LoggingPolicy, MODEL_SCHEMA_VERSION, ModelDocument, PortEndpoint,
        ProbeDefinition, SimulationSettings,
    };
    use crate::identity::{ComponentId, ConnectionId, DocumentId, ProbeId, RunId, SystemId};
    use crate::registry::ComponentRegistry;
    use crate::resolve::{CustomComponentLoader, LoadedCustomComponent, resolve_model};
    use crate::results::RunStatus;
    use crate::timing::FixedStepSemantics;
    use crate::value::RuntimeValue;
    use std::collections::BTreeMap;

    struct NoCustomComponents;

    impl CustomComponentLoader for NoCustomComponents {
        fn load(&self, _source: &str) -> Result<LoadedCustomComponent, Diagnostic> {
            Err(Diagnostic::new(
                DiagnosticSeverity::Error,
                DiagnosticCategory::Resolution,
                None,
                None,
                "unexpected_custom_component",
            ))
        }
    }

    fn instance(id: u128, type_id: &str, overrides: &[(&str, &str)]) -> ComponentInstance {
        ComponentInstance {
            id: ComponentId::from_raw(id),
            name: format!("component-{id}").into(),
            component: ComponentReference::BuiltIn {
                type_id: ComponentTypeId::new(type_id).unwrap(),
            },
            parameter_overrides: overrides
                .iter()
                .map(|(key, value)| ((*key).into(), (*value).into()))
                .collect(),
            enabled: true,
            position: CanvasPosition { x: 0.0, y: 0.0 },
        }
    }

    fn connection(id: u128, source: u128, target: u128) -> Connection {
        Connection {
            id: ConnectionId::from_raw(id),
            source: PortEndpoint {
                component_id: ComponentId::from_raw(source),
                port_key: "out".into(),
            },
            target: PortEndpoint {
                component_id: ComponentId::from_raw(target),
                port_key: "in".into(),
            },
            label: None,
            route: vec![],
        }
    }

    fn model() -> ModelDocument {
        ModelDocument {
            header: DocumentHeader {
                schema_version: MODEL_SCHEMA_VERSION,
                document_id: DocumentId::from_raw(1),
                title: "Vertical slice".into(),
                description: "".into(),
                author: "tests".into(),
                created_at: "2026-08-24T00:00:00Z".into(),
                updated_at: "2026-08-24T00:00:00Z".into(),
                migrations: vec![],
            },
            root: Composition {
                system_id: SystemId::from_raw(10),
                components: vec![
                    instance(
                        4,
                        "signal.step",
                        &[("initial_value", "1.0"), ("final_value", "1.0")],
                    ),
                    instance(3, "signal.gain", &[("gain", "2.0")]),
                    instance(1, "signal.integrator", &[("initial_value", "0.0")]),
                    instance(2, "signal.probe", &[]),
                ],
                connections: vec![
                    connection(11, 4, 3),
                    connection(12, 3, 1),
                    connection(13, 1, 2),
                ],
                annotations: BTreeMap::new(),
            },
            simulation: SimulationSettings {
                start_time: 0.0,
                stop_time: 1.0,
                timestep: 0.25,
                maximum_steps: 4,
                random_seed: 7,
                logging: LoggingPolicy::EveryStep,
                semantics: FixedStepSemantics::default(),
            },
            probes: vec![ProbeDefinition {
                id: ProbeId::from_raw(20),
                target: PortEndpoint {
                    component_id: ComponentId::from_raw(2),
                    port_key: "out".into(),
                },
                display_name: "integral".into(),
                plot_group: None,
            }],
            dependencies: vec![],
        }
    }

    #[test]
    fn vertical_slice_has_exact_samples_and_deterministic_reset_rerun() {
        let mut registry = ComponentRegistry::new();
        register_signal_builtins(&mut registry).unwrap();
        let resolved = resolve_model(&model(), &registry, &NoCustomComponents).unwrap();
        let mut runtime = SimulationRuntime::new(&resolved, &registry).unwrap();

        let first = runtime.run(RunId::from_raw(100));
        runtime.reset();
        let second = runtime.run(RunId::from_raw(101));

        assert_eq!(first.status, RunStatus::Completed);
        assert_eq!(first.series[0].timestamps, vec![0.0, 0.25, 0.5, 0.75, 1.0]);
        assert_eq!(
            first.series[0].values,
            vec![0.0, 0.5, 1.0, 1.5, 2.0]
                .into_iter()
                .map(RuntimeValue::Scalar)
                .collect::<Vec<_>>()
        );
        assert_eq!(first.series, second.series);
        assert!(first.diagnostics.is_empty());
    }
}
