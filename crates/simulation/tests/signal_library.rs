//! End-to-end regression fixtures for reusable signal-library benchmark models.

use simulation::benchmark_models::{control_benchmark, mixed_signal_benchmark};
use simulation::builtins::register_signal_builtins;
use simulation::diagnostic::{Diagnostic, DiagnosticCategory, DiagnosticSeverity};
use simulation::document::ModelDocument;
use simulation::identity::RunId;
use simulation::registry::ComponentRegistry;
use simulation::resolve::{CustomComponentLoader, LoadedCustomComponent, resolve_model};
use simulation::results::{RunStatus, SimulationRun};
use simulation::runtime::SimulationRuntime;
use simulation::value::RuntimeValue;

/// Loader that rejects custom artifacts because benchmark models are self-contained.
struct NoCustomComponents;

impl CustomComponentLoader for NoCustomComponents {
    fn load(&self, _source: &str) -> Result<LoadedCustomComponent, Diagnostic> {
        Err(Diagnostic::new(
            DiagnosticSeverity::Error,
            DiagnosticCategory::Resolution,
            None,
            Some("source".into()),
            "simulation_benchmark_unexpected_custom_component",
        ))
    }
}

/// Executes one benchmark through the normal resolver and runtime.
fn execute(model: &ModelDocument, run_id: u128) -> Result<SimulationRun, String> {
    let mut registry = ComponentRegistry::new();
    register_signal_builtins(&mut registry).map_err(|error| format!("{error:?}"))?;
    let resolved = resolve_model(model, &registry, &NoCustomComponents)
        .map_err(|error| format!("{error:?}"))?;
    let mut runtime =
        SimulationRuntime::new(&resolved, &registry).map_err(|error| format!("{error:?}"))?;
    Ok(runtime.run(RunId::from_raw(run_id)))
}

/// Verifies exact mixed and stateful benchmark references.
fn verify_benchmarks() -> Result<(), String> {
    let mixed = execute(
        &mixed_signal_benchmark().map_err(|error| format!("{error:?}"))?,
        1,
    )?;
    if mixed.status != RunStatus::Completed
        || mixed
            .series
            .first()
            .and_then(|series| series.values.first())
            != Some(&RuntimeValue::Scalar(30.0))
        || !mixed.series.first().is_some_and(|series| {
            series
                .values
                .iter()
                .all(|value| value == &RuntimeValue::Scalar(30.0))
        })
    {
        return Err(format!("unexpected mixed benchmark: {mixed:?}"));
    }

    let first = execute(
        &control_benchmark().map_err(|error| format!("{error:?}"))?,
        2,
    )?;
    let second = execute(
        &control_benchmark().map_err(|error| format!("{error:?}"))?,
        3,
    )?;
    if first.status != RunStatus::Completed || first.series != second.series {
        return Err("control benchmark did not reset deterministically".into());
    }
    let expected = [
        0.0,
        0.0,
        0.040_000_000_000_000_01,
        0.112_000_000_000_000_02,
        0.2096,
        0.32768,
        0.462_144,
        0.609_715_2,
        0.767_772_16,
        0.934_217_728,
        1.107_374_182_4,
    ];
    let actual = first
        .series
        .first()
        .map(|series| series.values.as_slice())
        .ok_or_else(|| "control benchmark produced no series".to_string())?;
    let expected_values: Vec<_> = expected.into_iter().map(RuntimeValue::Scalar).collect();
    if actual != expected_values {
        return Err(format!("unexpected control sequence: {actual:?}"));
    }
    Ok(())
}

#[test]
fn benchmark_models_are_exact_and_reset_deterministically() {
    assert_eq!(verify_benchmarks(), Ok(()));
}
