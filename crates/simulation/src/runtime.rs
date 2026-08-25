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
    UnitDefinition, parameter_key,
};
use expression_engine::prelude::{ExpressionEngine, ParameterObjectInputData};
use shareable_string::ShareableString;
use std::collections::BTreeMap;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

/// Shareable cooperative cancellation state for a synchronous run.
#[derive(Debug, Clone, Default)]
pub struct CancellationToken {
    /// Atomic flag shared by the application and synchronous runtime.
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Requests cancellation. The runtime observes this between atomic component updates.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

/// Per-run resource limits applied before result allocation and execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeLimits {
    /// Maximum retained samples per probe.
    pub maximum_samples: usize,
    /// Maximum retained values across all probes.
    pub maximum_result_values: usize,
}

impl Default for RuntimeLimits {
    fn default() -> Self {
        Self {
            maximum_samples: 10_000_000,
            maximum_result_values: 100_000_000,
        }
    }
}

/// Runtime-only controls that are not persisted in the source model.
#[derive(Debug, Clone, Default)]
pub struct RunOptions {
    /// Cooperative cancellation state shared with the caller.
    pub cancellation: CancellationToken,
    /// Result memory limits.
    pub limits: RuntimeLimits,
}

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
    /// Reused connected-input storage populated before each evaluation.
    inputs: RuntimeValues,
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
    /// Reused atomic state-update staging storage.
    pending_updates: Vec<(ComponentId, ComponentUpdate)>,
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
            let behavior = match factory.create(component.id, &parameters) {
                Ok(behavior) => behavior,
                Err(diagnostic) => {
                    diagnostics.push(diagnostic);
                    continue;
                }
            };
            components.insert(
                component.id,
                RuntimeComponent {
                    behavior,
                    parameters,
                    inputs: model
                        .root
                        .connections
                        .iter()
                        .filter(|connection| connection.target.component_id == component.id)
                        .map(|connection| {
                            (
                                connection.target.port_key.clone(),
                                RuntimeValue::Scalar(0.0),
                            )
                        })
                        .collect(),
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

        let stateful_count = components
            .values()
            .filter(|component| component.stateful)
            .count();
        Ok(Self {
            model: model.clone(),
            schedule,
            components,
            pending_updates: Vec::with_capacity(stateful_count),
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
        self.run_with_options(run_id, &RunOptions::default())
    }

    /// Executes with application-owned cancellation and resource limits.
    #[must_use]
    pub fn run_with_options(&mut self, run_id: RunId, options: &RunOptions) -> SimulationRun {
        self.reset();
        let Ok(plan) = FixedStepPlan::new(
            self.model.simulation.start_time,
            self.model.simulation.stop_time,
            self.model.simulation.timestep,
        ) else {
            return self.failed_run(
                run_id,
                self.empty_series(0),
                runtime_diagnostic(None, "simulation", "simulation_runtime_invalid_timing"),
            );
        };
        let Some(sample_capacity) = logged_sample_count(plan, self.model.simulation.logging) else {
            return self.failed_run(
                run_id,
                self.empty_series(0),
                runtime_diagnostic(
                    None,
                    "logging",
                    "simulation_runtime_invalid_logging_interval",
                ),
            );
        };
        let Some(result_values) = sample_capacity.checked_mul(self.model.probes.len()) else {
            return self.failed_run(
                run_id,
                self.empty_series(0),
                runtime_diagnostic(None, "results", "simulation_runtime_result_limit"),
            );
        };
        if sample_capacity > options.limits.maximum_samples
            || result_values > options.limits.maximum_result_values
        {
            return self.failed_run(
                run_id,
                self.empty_series(0),
                runtime_diagnostic(None, "results", "simulation_runtime_result_limit"),
            );
        }
        let mut series = self.empty_series(sample_capacity);
        if options.cancellation.is_cancelled() {
            return self.cancelled_run(run_id, series, None);
        }
        let initial_context = self.context(run_id, self.model.simulation.start_time, 0);
        if let Err(diagnostic) = self.initialize(initial_context) {
            return self.failed_run(run_id, series, diagnostic);
        }
        if let Err(diagnostic) = self.evaluate_sample(initial_context) {
            return self.failed_run(run_id, series, diagnostic);
        }
        if self.model.simulation.logging.captures(0, plan.step_count()) {
            if let Err(diagnostic) = self.capture(initial_context.time, &mut series) {
                return self.failed_run(run_id, series, diagnostic);
            }
        }

        for step_index in 0..plan.step_count() {
            if options.cancellation.is_cancelled() {
                let context = self.context(
                    run_id,
                    plan.sample_time(step_index)
                        .unwrap_or(self.model.simulation.stop_time),
                    step_index,
                );
                return self.cancelled_run(run_id, series, Some(context));
            }
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
            if options.cancellation.is_cancelled() {
                return self.cancelled_run(run_id, series, Some(next_context));
            }
            if self
                .model
                .simulation
                .logging
                .captures(next_index, plan.step_count())
            {
                if let Err(diagnostic) = self.capture(next_time, &mut series) {
                    return self.failed_run(run_id, series, diagnostic);
                }
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
        for order_index in 0..self.schedule.component_order.len() {
            let Some(component_id) = self.schedule.component_order.get(order_index).copied() else {
                continue;
            };
            let should_evaluate = self
                .components
                .get(&component_id)
                .is_some_and(|component| !component.stateful && component.sample_evaluated);
            if !should_evaluate {
                continue;
            }
            self.refresh_inputs(component_id)?;
            let Some(component) = self.components.get_mut(&component_id) else {
                continue;
            };
            let update = component.behavior.evaluate(
                context,
                &component.parameters,
                &component.inputs,
                &component.state,
            )?;
            component.outputs = update.outputs;
        }
        Ok(())
    }

    /// Computes every stateful update before committing any of them.
    fn transition_state(&mut self, context: StepContext) -> Result<(), Diagnostic> {
        self.pending_updates.clear();
        for order_index in 0..self.schedule.component_order.len() {
            let Some(component_id) = self.schedule.component_order.get(order_index).copied() else {
                continue;
            };
            if !self
                .components
                .get(&component_id)
                .is_some_and(|component| component.stateful)
            {
                continue;
            }
            self.refresh_inputs(component_id)?;
            let Some(component) = self.components.get(&component_id) else {
                continue;
            };
            let update = component.behavior.evaluate(
                context,
                &component.parameters,
                &component.inputs,
                &component.state,
            )?;
            self.pending_updates.push((component_id, update));
        }
        while let Some((component_id, update)) = self.pending_updates.pop() {
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
    fn refresh_inputs(&mut self, target_id: ComponentId) -> Result<(), Diagnostic> {
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
            let target = self.components.get_mut(&target_id).ok_or_else(|| {
                runtime_diagnostic(
                    Some(target_id),
                    "component",
                    "simulation_runtime_missing_component",
                )
            })?;
            let slot = target
                .inputs
                .get_mut(&connection.target.port_key)
                .ok_or_else(|| {
                    runtime_diagnostic(
                        Some(target_id),
                        connection.target.port_key.as_str(),
                        "simulation_runtime_missing_input_buffer",
                    )
                })?;
            *slot = value;
        }
        Ok(())
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
    fn empty_series(&self, capacity: usize) -> Vec<SignalSeries> {
        self.model
            .probes
            .iter()
            .map(|probe| SignalSeries {
                probe_id: probe.id,
                source: probe.target.clone(),
                display_name: probe.display_name.clone(),
                timestamps: Vec::with_capacity(capacity),
                values: Vec::with_capacity(capacity),
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

    /// Builds a cancelled run retaining samples committed before the request.
    fn cancelled_run(
        &self,
        run_id: RunId,
        series: Vec<SignalSeries>,
        final_context: Option<StepContext>,
    ) -> SimulationRun {
        if let Some(context) = final_context {
            if let Err(diagnostic) = self.finalize(context) {
                return self.failed_run(run_id, series, diagnostic);
            }
        }
        SimulationRun {
            run_id,
            source_document_id: self.model.document_id,
            settings: self.model.simulation,
            status: RunStatus::Cancelled,
            diagnostics: vec![],
            series,
        }
    }
}

/// Computes exact retained capacity for a validated logging policy.
fn logged_sample_count(
    plan: FixedStepPlan,
    logging: crate::document::LoggingPolicy,
) -> Option<usize> {
    let count = match logging {
        crate::document::LoggingPolicy::EveryStep => plan.sample_count(),
        crate::document::LoggingPolicy::EveryNthStep { interval } => {
            let quotient = plan.step_count().checked_div(interval)?;
            let includes_final = plan.step_count().checked_rem(interval) == Some(0);
            u128::from(quotient)
                .checked_add(1)?
                .checked_add(u128::from(!includes_final))?
        }
    };
    usize::try_from(count).ok()
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
        if matches!(parameter.value_type, ParameterValueType::String) {
            values.insert(
                parameter.key.clone(),
                RuntimeValue::String(expression.clone()),
            );
            continue;
        }
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
            ParameterValueType::Unit(family) => {
                ParameterObjectDefinition::builder("Runtime parameter")
                    .with(
                        parameter_key!("p_value"),
                        UnitDefinition::new_with_default(
                            &parameter.description,
                            family,
                            expression,
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
            | ParameterValueType::TableWithUnits(_) => {
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
    use super::{CancellationToken, RunOptions, RuntimeLimits, SimulationRuntime};
    use crate::builtins::register_signal_builtins;
    use crate::component::{
        ComponentBehavior, ComponentCapabilities, ComponentCapability, ComponentDefinition,
        ComponentFactory, ComponentTypeId, ComponentUpdate, RuntimeValues, SemanticVersion,
        StepContext,
    };
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
    use std::sync::Arc;
    use units::UnitId;

    #[derive(Debug)]
    struct CancellingFactory {
        cancellation: CancellationToken,
    }

    impl ComponentFactory for CancellingFactory {
        fn create(
            &self,
            component_id: ComponentId,
            _parameters: &RuntimeValues,
        ) -> Result<Box<dyn ComponentBehavior>, Diagnostic> {
            Ok(Box::new(CancellingSource {
                component_id,
                cancellation: self.cancellation.clone(),
            }))
        }
    }

    #[derive(Debug)]
    struct CancellingSource {
        component_id: ComponentId,
        cancellation: CancellationToken,
    }

    impl ComponentBehavior for CancellingSource {
        fn initialize(
            &self,
            _context: StepContext,
            _parameters: &RuntimeValues,
        ) -> Result<ComponentUpdate, Diagnostic> {
            Ok(source_update())
        }

        fn evaluate(
            &self,
            context: StepContext,
            _parameters: &RuntimeValues,
            _inputs: &RuntimeValues,
            _state: &RuntimeValues,
        ) -> Result<ComponentUpdate, Diagnostic> {
            if context.step_index == 2 {
                self.cancellation.cancel();
            }
            let _ = self.component_id;
            Ok(source_update())
        }
    }

    fn source_update() -> ComponentUpdate {
        ComponentUpdate {
            outputs: [("out".into(), RuntimeValue::Scalar(1.0))]
                .into_iter()
                .collect(),
            next_state: RuntimeValues::new(),
        }
    }

    fn cancelling_definition() -> ComponentDefinition {
        ComponentDefinition {
            type_id: ComponentTypeId::new("test.cancelling_source").unwrap(),
            version: SemanticVersion {
                major: 1,
                minor: 0,
                patch: 0,
            },
            display_name: "Cancelling source".into(),
            category: "Test".into(),
            aliases: vec![],
            tags: vec![],
            documentation: "".into(),
            parameters: vec![],
            ports: vec![crate::component::PortDefinition {
                key: "out".into(),
                display_name: "out".into(),
                description: "".into(),
                direction: crate::component::PortDirection::Output,
                value_type: crate::parameter::ParameterValueType::Scalar,
                unit: None,
                required: false,
            }],
            capabilities: ComponentCapabilities::new([
                ComponentCapability::TimeDependent,
                ComponentCapability::Deterministic,
            ]),
            deprecation: None,
        }
    }

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

    #[test]
    fn decimated_logging_includes_final_sample() {
        let mut registry = ComponentRegistry::new();
        register_signal_builtins(&mut registry).unwrap();
        let mut model = model();
        model.simulation.logging = LoggingPolicy::EveryNthStep { interval: 3 };
        let resolved = resolve_model(&model, &registry, &NoCustomComponents).unwrap();
        let mut runtime = SimulationRuntime::new(&resolved, &registry).unwrap();

        let run = runtime.run(RunId::from_raw(102));

        assert_eq!(run.status, RunStatus::Completed);
        assert_eq!(run.series[0].timestamps, vec![0.0, 0.75, 1.0]);
    }

    #[test]
    fn result_limit_fails_before_allocating_samples() {
        let mut registry = ComponentRegistry::new();
        register_signal_builtins(&mut registry).unwrap();
        let resolved = resolve_model(&model(), &registry, &NoCustomComponents).unwrap();
        let mut runtime = SimulationRuntime::new(&resolved, &registry).unwrap();
        let options = RunOptions {
            cancellation: CancellationToken::default(),
            limits: RuntimeLimits {
                maximum_samples: 4,
                maximum_result_values: 4,
            },
        };

        let run = runtime.run_with_options(RunId::from_raw(103), &options);

        assert_eq!(run.status, RunStatus::Failed);
        assert!(run.series[0].timestamps.is_empty());
        assert_eq!(
            run.diagnostics[0].message_key(),
            "simulation_runtime_result_limit"
        );
    }

    #[test]
    fn cancellation_retains_only_fully_committed_partial_samples() {
        let cancellation = CancellationToken::default();
        let mut registry = ComponentRegistry::new();
        register_signal_builtins(&mut registry).unwrap();
        registry
            .register_with_factory(
                cancelling_definition(),
                Arc::new(CancellingFactory {
                    cancellation: cancellation.clone(),
                }),
            )
            .unwrap();
        let mut model = model();
        model.root.components[0].component = ComponentReference::BuiltIn {
            type_id: ComponentTypeId::new("test.cancelling_source").unwrap(),
        };
        model.root.components[0].parameter_overrides.clear();
        let resolved = resolve_model(&model, &registry, &NoCustomComponents).unwrap();
        let mut runtime = SimulationRuntime::new(&resolved, &registry).unwrap();
        let options = RunOptions {
            cancellation,
            limits: RuntimeLimits::default(),
        };

        let run = runtime.run_with_options(RunId::from_raw(104), &options);

        assert_eq!(run.status, RunStatus::Cancelled);
        assert_eq!(run.series[0].timestamps, vec![0.0, 0.25]);
        assert_eq!(
            run.series[0].values,
            vec![RuntimeValue::Scalar(0.0), RuntimeValue::Scalar(0.5)]
        );
    }

    #[test]
    fn expression_model_has_exact_time_dependent_sequence() {
        let mut registry = ComponentRegistry::new();
        register_signal_builtins(&mut registry).unwrap();
        let mut model = model();
        model.root.components = vec![
            instance(4, "signal.constant", &[("value", "3.0")]),
            instance(3, "signal.expression", &[("expression", "2 * x + time")]),
        ];
        model.root.connections = vec![connection(11, 4, 3)];
        model.probes[0].target.component_id = ComponentId::from_raw(3);
        model.simulation.stop_time = 0.5;
        model.simulation.maximum_steps = 2;
        let resolved = resolve_model(&model, &registry, &NoCustomComponents).unwrap();
        let mut runtime = SimulationRuntime::new(&resolved, &registry).unwrap();

        let run = runtime.run(RunId::from_raw(105));

        assert_eq!(run.status, RunStatus::Completed);
        assert_eq!(run.series[0].timestamps, vec![0.0, 0.25, 0.5]);
        assert_eq!(
            run.series[0].values,
            vec![6.0, 6.25, 6.5]
                .into_iter()
                .map(RuntimeValue::Scalar)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn unit_conversion_model_adapts_selector_parameters() {
        let mut registry = ComponentRegistry::new();
        register_signal_builtins(&mut registry).unwrap();
        let mut conversion = instance(3, "signal.unit_conversion", &[]);
        conversion.parameter_overrides.insert(
            "from_unit".into(),
            UnitId::Time_Minute.string_id().as_str().into(),
        );
        conversion.parameter_overrides.insert(
            "to_unit".into(),
            UnitId::Time_Second.string_id().as_str().into(),
        );
        let mut model = model();
        model.root.components = vec![
            instance(4, "signal.constant", &[("value", "2.0")]),
            conversion,
        ];
        model.root.connections = vec![connection(11, 4, 3)];
        model.probes[0].target.component_id = ComponentId::from_raw(3);
        model.simulation.stop_time = 0.0;
        model.simulation.maximum_steps = 0;
        let resolved = resolve_model(&model, &registry, &NoCustomComponents).unwrap();
        let mut runtime = SimulationRuntime::new(&resolved, &registry).unwrap();

        let run = runtime.run(RunId::from_raw(106));

        assert_eq!(run.status, RunStatus::Completed);
        assert_eq!(
            run.series[0].values,
            vec![RuntimeValue::ScalarWithUnit {
                value: 120.0,
                unit: UnitId::Time_Second,
            }]
        );
    }

    #[test]
    fn ten_thousand_step_baseline_reuses_runtime_buffers() {
        let mut registry = ComponentRegistry::new();
        register_signal_builtins(&mut registry).unwrap();
        let mut model = model();
        model.simulation.stop_time = 2_500.0;
        model.simulation.maximum_steps = 10_000;
        model.simulation.logging = LoggingPolicy::EveryNthStep { interval: 10_000 };
        let resolved = resolve_model(&model, &registry, &NoCustomComponents).unwrap();
        let mut runtime = SimulationRuntime::new(&resolved, &registry).unwrap();
        let staging_capacity = runtime.pending_updates.capacity();
        let input_slot_counts = runtime
            .components
            .values()
            .map(|component| component.inputs.len())
            .collect::<Vec<_>>();

        let first = runtime.run(RunId::from_raw(107));
        let second = runtime.run(RunId::from_raw(108));

        assert_eq!(first.status, RunStatus::Completed);
        assert_eq!(first.series[0].timestamps, vec![0.0, 2_500.0]);
        assert_eq!(first.series, second.series);
        assert_eq!(runtime.pending_updates.capacity(), staging_capacity);
        assert_eq!(
            runtime
                .components
                .values()
                .map(|component| component.inputs.len())
                .collect::<Vec<_>>(),
            input_slot_counts
        );
    }
}
