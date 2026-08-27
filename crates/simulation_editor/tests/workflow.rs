//! Phase 6 author, validate, run, save, reopen, and inspect acceptance workflow.

use simulation::component::ComponentTypeId;
use simulation::document::{CanvasPosition, PortEndpoint};
use simulation::identity::{ComponentId, DocumentId};
use simulation::results::RunStatus;
use simulation::value::RuntimeValue;
use simulation_editor::controller::{
    CommandOutcome, DocumentCommand, DocumentController, recovery_path,
};
use std::fs;
use std::path::PathBuf;

/// Runs the complete command-driven native document workflow.
fn verify_workflow() -> Result<(), String> {
    let mut controller = DocumentController::new(DocumentId::from_raw(900), "2026-08-24T00:00:00Z")
        .map_err(|error| error.to_string())?;
    let constant = add(&mut controller, "signal.constant", "Input", 80.0)?;
    let gain = add(&mut controller, "signal.gain", "Gain", 360.0)?;
    controller
        .execute(DocumentCommand::SetParameter {
            component_id: constant,
            key: "value".into(),
            expression: "2.0".into(),
        })
        .map_err(|error| error.to_string())?;
    controller
        .execute(DocumentCommand::SetParameter {
            component_id: gain,
            key: "gain".into(),
            expression: "3.0".into(),
        })
        .map_err(|error| error.to_string())?;
    controller
        .execute(DocumentCommand::Connect {
            source: endpoint(constant, "out"),
            target: endpoint(gain, "in"),
        })
        .map_err(|error| error.to_string())?;
    controller
        .execute(DocumentCommand::AddProbe {
            target: endpoint(gain, "out"),
            display_name: "Amplified signal".into(),
        })
        .map_err(|error| error.to_string())?;

    controller.validate();
    if !controller.diagnostics().is_empty() {
        return Err(format!(
            "unexpected diagnostics: {:?}",
            controller.diagnostics()
        ));
    }
    let first_run = controller.run().map_err(|error| error.to_string())?.clone();
    if first_run.status != RunStatus::Completed
        || first_run
            .series
            .first()
            .and_then(|series| series.values.first())
            != Some(&RuntimeValue::Scalar(6.0))
    {
        return Err(format!("unexpected first run: {first_run:?}"));
    }

    let path = temporary_model_path();
    controller
        .save_as(&path)
        .map_err(|error| error.to_string())?;
    if controller.is_dirty() {
        return Err("saved controller remained dirty".into());
    }
    controller
        .execute(DocumentCommand::MoveComponent {
            component_id: gain,
            position: CanvasPosition { x: 420.0, y: 140.0 },
        })
        .map_err(|error| error.to_string())?;
    if component_position(&controller, gain) != Some(CanvasPosition { x: 420.0, y: 140.0 }) {
        return Err("move command did not update the component position".into());
    }
    if !controller.undo()
        || component_position(&controller, gain) != Some(CanvasPosition { x: 360.0, y: 120.0 })
    {
        return Err("undo did not restore the component position".into());
    }
    if !controller.redo()
        || component_position(&controller, gain) != Some(CanvasPosition { x: 420.0, y: 140.0 })
    {
        return Err("redo did not restore the moved component position".into());
    }
    controller
        .autosave(&path)
        .map_err(|error| error.to_string())?;
    let recovered = DocumentController::recover(&path).map_err(|error| error.to_string())?;
    if !recovered.is_dirty()
        || recovered.document().root.components != controller.document().root.components
    {
        return Err("recovery did not restore the dirty command snapshot".into());
    }
    fs::remove_file(recovery_path(&path)).map_err(|error| error.to_string())?;
    let mut reopened = DocumentController::open(&path).map_err(|error| error.to_string())?;
    let second_run = reopened.run().map_err(|error| error.to_string())?;
    let remove_result = fs::remove_file(&path).map_err(|error| error.to_string());
    remove_result?;
    if first_run.series != second_run.series {
        return Err("reopened model produced different results".into());
    }
    Ok(())
}

/// Adds a built-in through the public command surface.
fn add(
    controller: &mut DocumentController,
    type_id: &str,
    name: &str,
    x: f64,
) -> Result<ComponentId, String> {
    let outcome = controller
        .execute(DocumentCommand::AddBuiltIn {
            type_id: ComponentTypeId::new(type_id).map_err(|error| format!("{error:?}"))?,
            version: None,
            name: name.into(),
            position: CanvasPosition { x, y: 120.0 },
        })
        .map_err(|error| error.to_string())?;
    match outcome {
        CommandOutcome::Component(id) => Ok(id),
        CommandOutcome::Updated | CommandOutcome::Connection(_) | CommandOutcome::Probe(_) => {
            Err("add command did not return a component identity".into())
        }
    }
}

/// Creates one root-system endpoint.
fn endpoint(component_id: ComponentId, port_key: &str) -> PortEndpoint {
    PortEndpoint {
        component_id,
        port_key: port_key.into(),
    }
}

/// Returns one root component's persisted canvas position.
fn component_position(
    controller: &DocumentController,
    component_id: ComponentId,
) -> Option<CanvasPosition> {
    controller
        .document()
        .root
        .components
        .iter()
        .find(|component| component.id == component_id)
        .map(|component| component.position)
}

/// Returns a process-specific temporary model path.
fn temporary_model_path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "simulation-editor-workflow-{}.json",
        std::process::id()
    ))
}

#[test]
fn authors_runs_saves_and_reopens_without_bypassing_commands() {
    assert_eq!(verify_workflow(), Ok(()));
}
