//! Native entry point for the signal simulation workbench.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use simulation::identity::DocumentId;
use simulation_editor::app::EditorApp;
use std::time::{SystemTime, UNIX_EPOCH};

/// Launches the native command-driven editor.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let timestamp = format!("unix:{now}");
    let document_id = DocumentId::from_raw(u128::from(now));
    let mut app = EditorApp::new(document_id, &timestamp).map_err(std::io::Error::other)?;
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1440.0, 900.0]),
        ..Default::default()
    };
    eframe::run_ui_native("Signal Workbench", options, move |ui, _frame| {
        app.show(ui);
    })?;
    Ok(())
}
