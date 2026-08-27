//! Immediate-mode application shell for the simulation document controller.

#![allow(
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    clippy::float_arithmetic,
    reason = "egui renders in f32 screen coordinates while documents and results persist f64 values"
)]

use crate::controller::{CommandOutcome, DocumentCommand, DocumentController};
use eframe::egui::{self, Color32, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};
use shareable_string::ShareableString;
use simulation::component::{
    ComponentAppearance, ComponentTypeId, NormalizedPosition, ParameterDefinition, PortDefinition,
    PortDirection, SemanticVersion,
};
use simulation::diagnostic::DiagnosticSeverity;
use simulation::document::{CanvasPosition, ComponentReference, PortEndpoint};
use simulation::identity::{ComponentId, DocumentId};
use simulation::results::{RunStatus, SignalSeries};
use simulation::value::RuntimeValue;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

/// Width reserved for the component library.
const PALETTE_WIDTH: f32 = 210.0;
/// Width reserved for inspection and diagnostics.
const INSPECTOR_WIDTH: f32 = 300.0;
/// Height reserved for results.
const RESULTS_HEIGHT: f32 = 230.0;
/// Stable visual node width.
const NODE_WIDTH: f32 = 170.0;
/// Stable visual node height.
const NODE_HEIGHT: f32 = 82.0;
/// Radius of one canvas port icon.
const PORT_RADIUS: f32 = 5.0;

/// Cached visual metadata loaded from a component definition or artifact.
#[derive(Debug, Clone)]
struct ComponentPresentation {
    /// Public ports rendered on the node.
    ports: Vec<PortDefinition>,
    /// Explicit normalized public-port locations.
    port_locations: BTreeMap<ShareableString, NormalizedPosition>,
    /// Optional embedded SVG prepared for egui's shared byte loader.
    icon: Option<EmbeddedSvg>,
}

/// Shared embedded SVG data and its content-addressed cache key.
#[derive(Debug, Clone)]
struct EmbeddedSvg {
    /// Stable egui byte-loader URI.
    uri: String,
    /// Shared SVG source bytes.
    bytes: Arc<[u8]>,
}

/// In-progress component movement before one atomic command commit.
#[derive(Debug, Clone, Copy)]
struct ComponentDrag {
    /// Component currently owning the pointer gesture.
    component_id: ComponentId,
    /// Latest preview position in canvas coordinates.
    position: CanvasPosition,
}

/// Mutable UI state wrapped around one command-driven controller.
#[derive(Debug)]
pub struct EditorApp {
    /// Open model and application services.
    controller: DocumentController,
    /// Selected root component.
    selected_component: Option<ComponentId>,
    /// Case-insensitive palette query.
    palette_filter: String,
    /// Native path field used by open and save-as.
    path_input: String,
    /// Latest concise application status.
    status: String,
    /// Parameter text drafts keyed by selected component and parameter.
    parameter_drafts: BTreeMap<(ComponentId, ShareableString), String>,
    /// Output endpoint waiting to be connected.
    pending_source: Option<PortEndpoint>,
    /// Live component position while a pointer drag is active.
    component_drag: Option<ComponentDrag>,
    /// Alternating palette placement offset.
    placement_index: u32,
    /// Native artifacts opened or saved during this session.
    recent_artifacts: Vec<PathBuf>,
    /// Built-in presentation data keyed by exact registry identity.
    builtin_presentations: BTreeMap<(ComponentTypeId, SemanticVersion), ComponentPresentation>,
    /// Verified custom presentation data keyed by root instance.
    custom_presentations: BTreeMap<ComponentId, ComponentPresentation>,
    /// Whether SVG image loaders were installed on the current context.
    image_loaders_installed: bool,
}

impl EditorApp {
    /// Creates an editor with a new empty source model.
    ///
    /// # Errors
    ///
    /// Returns an error if the standard component registry cannot be installed.
    pub fn new(document_id: DocumentId, timestamp: &str) -> Result<Self, String> {
        let controller =
            DocumentController::new(document_id, timestamp).map_err(|error| error.to_string())?;
        let builtin_presentations = controller
            .palette()
            .map(|definition| {
                (
                    (definition.type_id.clone(), definition.version),
                    component_presentation(definition.ports.clone(), definition.appearance.clone()),
                )
            })
            .collect();
        Ok(Self {
            controller,
            selected_component: None,
            palette_filter: String::new(),
            path_input: "signal-model.json".into(),
            status: "Ready".into(),
            parameter_drafts: BTreeMap::new(),
            pending_source: None,
            component_drag: None,
            placement_index: 0,
            recent_artifacts: Vec::new(),
            builtin_presentations,
            custom_presentations: BTreeMap::new(),
            image_loaders_installed: false,
        })
    }

    /// Draws one complete editor frame.
    pub fn show(&mut self, ui: &mut egui::Ui) {
        if !self.image_loaders_installed {
            egui_extras::install_image_loaders(ui.ctx());
            self.image_loaders_installed = true;
        }
        Self::apply_visual_style(ui.ctx());
        self.show_toolbar(ui);
        self.show_palette(ui);
        self.show_inspector(ui);
        self.show_results(ui);
        self.show_canvas(ui);
    }

    /// Applies a restrained workbench palette.
    fn apply_visual_style(context: &egui::Context) {
        let mut visuals = egui::Visuals::light();
        visuals.panel_fill = Color32::from_rgb(238, 239, 235);
        visuals.window_fill = Color32::from_rgb(250, 250, 247);
        visuals.selection.bg_fill = Color32::from_rgb(20, 117, 112);
        visuals.widgets.active.bg_fill = Color32::from_rgb(20, 117, 112);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(218, 226, 221);
        context.set_visuals(visuals);
    }

    /// Draws file, history, validation, and execution controls.
    fn show_toolbar(&mut self, root: &mut egui::Ui) {
        egui::Panel::top("editor_toolbar").show(root, |ui| {
            ui.set_height(42.0);
            ui.horizontal_centered(|ui| {
                ui.strong("Signal Workbench");
                ui.separator();
                if ui
                    .button("New")
                    .on_hover_text("Create a new model")
                    .clicked()
                {
                    self.new_document();
                }
                if ui
                    .button("Open")
                    .on_hover_text("Open the path shown")
                    .clicked()
                {
                    self.open_document();
                }
                if ui
                    .button("Recover")
                    .on_hover_text("Restore the autosave for the path shown")
                    .clicked()
                {
                    self.recover_document();
                }
                if ui
                    .button("Save")
                    .on_hover_text("Save the current model")
                    .clicked()
                {
                    self.save_document(false);
                }
                if ui
                    .button("Save as")
                    .on_hover_text("Save to the path shown")
                    .clicked()
                {
                    self.save_document(true);
                }
                ui.add_sized(
                    [210.0, 24.0],
                    egui::TextEdit::singleline(&mut self.path_input),
                );
                ui.separator();
                if ui.button("Undo").clicked() && self.controller.undo() {
                    self.status = "Undid command".into();
                    self.selected_component = None;
                    self.component_drag = None;
                    self.refresh_custom_presentations();
                    self.autosave();
                }
                if ui.button("Redo").clicked() && self.controller.redo() {
                    self.status = "Redid command".into();
                    self.selected_component = None;
                    self.component_drag = None;
                    self.refresh_custom_presentations();
                    self.autosave();
                }
                ui.separator();
                if ui.button("Validate").clicked() {
                    self.controller.validate();
                    self.status = format!(
                        "Validation: {} diagnostic(s)",
                        self.controller.diagnostics().len()
                    );
                }
                if ui.button("Run").clicked() {
                    self.run_model();
                }
                let dirty = if self.controller.is_dirty() {
                    "Modified"
                } else {
                    "Saved"
                };
                ui.separator();
                ui.label(format!("{dirty} | {}", self.status));
            });
        });
    }

    /// Draws the searchable registry catalog.
    fn show_palette(&mut self, root: &mut egui::Ui) {
        egui::Panel::left("component_palette")
            .exact_size(PALETTE_WIDTH)
            .resizable(false)
            .show(root, |ui| {
                ui.heading("Components");
                ui.add(
                    egui::TextEdit::singleline(&mut self.palette_filter)
                        .hint_text("Search library"),
                );
                ui.separator();
                let query = self.palette_filter.to_lowercase();
                let entries: Vec<_> = self
                    .controller
                    .palette()
                    .filter(|definition| {
                        query.is_empty()
                            || definition
                                .display_name
                                .as_str()
                                .to_lowercase()
                                .contains(&query)
                            || definition.type_id.as_str().contains(&query)
                    })
                    .map(|definition| {
                        (
                            definition.type_id.clone(),
                            definition.version,
                            definition.display_name.clone(),
                            definition.category.clone(),
                            definition.documentation.clone(),
                        )
                    })
                    .collect();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (type_id, version, name, category, documentation) in entries {
                        let response = ui
                            .add_sized(
                                [190.0, 42.0],
                                egui::Button::new(format!(
                                    "{}\n{}",
                                    name.as_str(),
                                    category.as_str()
                                )),
                            )
                            .on_hover_text(documentation.as_str());
                        if response.clicked() {
                            self.add_component(type_id, version, &name);
                        }
                    }
                });
                if !self.recent_artifacts.is_empty() {
                    ui.separator();
                    ui.strong("Recent");
                    let recent = self.recent_artifacts.clone();
                    for path in recent {
                        if ui
                            .button(
                                path.file_name()
                                    .and_then(|name| name.to_str())
                                    .unwrap_or("model"),
                            )
                            .on_hover_text(path.display().to_string())
                            .clicked()
                        {
                            self.path_input = path.display().to_string();
                            self.open_document();
                        }
                    }
                }
            });
    }

    /// Draws selected-component controls and navigable diagnostics.
    fn show_inspector(&mut self, root: &mut egui::Ui) {
        egui::Panel::right("editor_inspector")
            .exact_size(INSPECTOR_WIDTH)
            .resizable(false)
            .show(root, |ui| {
                ui.heading("Inspector");
                self.show_simulation_settings(ui);
                ui.separator();
                if let Some(component_id) = self.selected_component {
                    self.show_component_inspector(ui, component_id);
                } else {
                    ui.label("Select a component on the canvas.");
                }
                ui.add_space(12.0);
                ui.separator();
                ui.heading("Diagnostics");
                if self.controller.diagnostics().is_empty() {
                    ui.label("No diagnostics");
                } else {
                    egui::ScrollArea::vertical()
                        .max_height(220.0)
                        .show(ui, |ui| {
                            for diagnostic in self.controller.diagnostics() {
                                let color = match diagnostic.severity() {
                                    DiagnosticSeverity::Error => Color32::from_rgb(174, 52, 45),
                                    DiagnosticSeverity::Warning => Color32::from_rgb(172, 106, 20),
                                    DiagnosticSeverity::Information | DiagnosticSeverity::Debug => {
                                        Color32::from_rgb(42, 91, 112)
                                    }
                                };
                                ui.colored_label(color, diagnostic.message_key().as_str());
                            }
                        });
                }
            });
    }

    /// Draws command-driven fixed-step settings.
    fn show_simulation_settings(&mut self, ui: &mut egui::Ui) {
        let mut settings = self.controller.document().simulation;
        let mut changed = false;
        egui::CollapsingHeader::new("Simulation")
            .default_open(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Start");
                    changed |= ui
                        .add(egui::DragValue::new(&mut settings.start_time))
                        .changed();
                });
                ui.horizontal(|ui| {
                    ui.label("Stop");
                    changed |= ui
                        .add(egui::DragValue::new(&mut settings.stop_time))
                        .changed();
                });
                ui.horizontal(|ui| {
                    ui.label("Step");
                    changed |= ui
                        .add(egui::DragValue::new(&mut settings.timestep).speed(0.001))
                        .changed();
                });
                ui.horizontal(|ui| {
                    ui.label("Seed");
                    changed |= ui
                        .add(egui::DragValue::new(&mut settings.random_seed))
                        .changed();
                });
            });
        if changed {
            self.apply(DocumentCommand::SetSimulation(settings));
        }
    }

    /// Draws metadata, parameters, ports, and deletion for one selected component.
    fn show_component_inspector(&mut self, ui: &mut egui::Ui, component_id: ComponentId) {
        let Some(component) = self
            .controller
            .document()
            .root
            .components
            .iter()
            .find(|component| component.id == component_id)
            .cloned()
        else {
            self.selected_component = None;
            return;
        };
        ui.strong(component.name.as_str());
        ui.small(component.id.to_string());
        let ComponentReference::BuiltIn { type_id, version } = &component.component else {
            ui.label("Custom component");
            return;
        };
        let Some(definition) = self.controller.definition(type_id, *version) else {
            ui.colored_label(Color32::from_rgb(174, 52, 45), "Unavailable built-in");
            return;
        };
        let parameters = definition.parameters.clone();
        let ports = definition.ports.clone();
        ui.label(definition.documentation.as_str());
        ui.separator();
        ui.strong("Parameters");
        for parameter in parameters {
            self.show_parameter(ui, component_id, &component, &parameter);
        }
        ui.separator();
        ui.strong("Ports");
        for port in ports {
            self.show_port_action(ui, component_id, &port);
        }
        ui.add_space(10.0);
        if ui
            .button("Remove component")
            .on_hover_text("Also removes attached links and probes")
            .clicked()
        {
            self.apply(DocumentCommand::RemoveComponent { component_id });
            self.selected_component = None;
        }
    }

    /// Draws and commits one parameter expression.
    fn show_parameter(
        &mut self,
        ui: &mut egui::Ui,
        component_id: ComponentId,
        component: &simulation::document::ComponentInstance,
        parameter: &ParameterDefinition,
    ) {
        let draft_key = (component_id, parameter.key.clone());
        let initial = component
            .parameter_overrides
            .get(&parameter.key)
            .unwrap_or(&parameter.default_expression)
            .to_string();
        let draft = self
            .parameter_drafts
            .entry(draft_key.clone())
            .or_insert(initial);
        ui.label(parameter.display_name.as_str());
        let response = ui.add(egui::TextEdit::singleline(draft).desired_width(f32::INFINITY));
        if response.lost_focus() && response.changed() {
            let expression: ShareableString = draft.clone().into();
            self.apply(DocumentCommand::SetParameter {
                component_id,
                key: parameter.key.clone(),
                expression,
            });
            self.parameter_drafts.remove(&draft_key);
        }
    }

    /// Draws connect/probe actions for one port.
    fn show_port_action(
        &mut self,
        ui: &mut egui::Ui,
        component_id: ComponentId,
        port: &PortDefinition,
    ) {
        let endpoint = PortEndpoint {
            component_id,
            port_key: port.key.clone(),
        };
        ui.horizontal(|ui| {
            ui.label(format!(
                "{} ({:?})",
                port.display_name.as_str(),
                port.direction
            ));
            match port.direction {
                PortDirection::Output => {
                    if ui
                        .button("Link")
                        .on_hover_text("Start a connection here")
                        .clicked()
                    {
                        self.pending_source = Some(endpoint.clone());
                        self.status = format!("Linking from {}", port.key.as_str());
                    }
                    if ui
                        .button("Probe")
                        .on_hover_text("Record this output")
                        .clicked()
                    {
                        self.apply(DocumentCommand::AddProbe {
                            target: endpoint,
                            display_name: port.display_name.clone(),
                        });
                    }
                }
                PortDirection::Input => {
                    if self.pending_source.is_some()
                        && ui
                            .button("Connect")
                            .on_hover_text("Complete pending link")
                            .clicked()
                    {
                        if let Some(source) = self.pending_source.take() {
                            self.apply(DocumentCommand::Connect {
                                source,
                                target: endpoint,
                            });
                        }
                    }
                }
            }
        });
    }

    /// Draws result status, plot, and sample table.
    fn show_results(&mut self, root: &mut egui::Ui) {
        egui::Panel::bottom("simulation_results")
            .exact_size(RESULTS_HEIGHT)
            .resizable(true)
            .show(root, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Results");
                    if let Some(run) = self.controller.last_run() {
                        let color = match run.status {
                            RunStatus::Completed => Color32::from_rgb(20, 117, 112),
                            RunStatus::Failed => Color32::from_rgb(174, 52, 45),
                            RunStatus::Cancelled => Color32::from_rgb(172, 106, 20),
                        };
                        ui.colored_label(color, format!("{:?}", run.status));
                    }
                });
                let series = self
                    .controller
                    .last_run()
                    .map(|run| run.series.clone())
                    .unwrap_or_default();
                if series.is_empty() {
                    ui.label("Run a model with probes to inspect sampled values.");
                    return;
                }
                ui.columns(2, |columns| {
                    if let Some((plot_ui, remainder)) = columns.split_first_mut()
                        && let Some(table_ui) = remainder.first_mut()
                    {
                        show_plot(plot_ui, &series);
                        show_result_table(table_ui, &series);
                    }
                });
            });
    }

    /// Draws persisted nodes and connections in the central workspace.
    fn show_canvas(&mut self, root: &mut egui::Ui) {
        egui::CentralPanel::default().show(root, |ui| {
            let available = ui.available_rect_before_wrap();
            ui.painter()
                .rect_filled(available, 0.0, Color32::from_rgb(246, 246, 241));
            draw_grid(ui, available);
            let components = self.controller.document().root.components.clone();
            let connections = self.controller.document().root.connections.clone();
            let presentations: BTreeMap<_, _> = components
                .iter()
                .filter_map(|component| {
                    let presentation = match &component.component {
                        ComponentReference::BuiltIn { type_id, version } => {
                            let definition = self.controller.definition(type_id, *version)?;
                            self.builtin_presentations
                                .get(&(type_id.clone(), definition.version))
                                .cloned()
                        }
                        ComponentReference::Custom { .. } => {
                            self.custom_presentations.get(&component.id).cloned()
                        }
                    }?;
                    Some((component.id, presentation))
                })
                .collect();
            let mut node_rects = BTreeMap::new();
            let mut responses = BTreeMap::new();
            let mut completed_move = None;
            for component in &components {
                let preview_position = self
                    .component_drag
                    .filter(|drag| drag.component_id == component.id)
                    .map_or(component.position, |drag| drag.position);
                let response = ui.interact(
                    node_rect(available, preview_position),
                    egui::Id::new(component.id.as_raw()),
                    Sense::click_and_drag(),
                );
                if response.drag_started() {
                    self.component_drag = Some(ComponentDrag {
                        component_id: component.id,
                        position: component.position,
                    });
                    self.selected_component = Some(component.id);
                    self.parameter_drafts.clear();
                }
                if response.dragged()
                    && let Some(drag) = self
                        .component_drag
                        .as_mut()
                        .filter(|drag| drag.component_id == component.id)
                {
                    drag.position =
                        translated_position(drag.position, response.drag_delta(), available.size());
                    ui.ctx().request_repaint();
                }
                if response.drag_stopped()
                    && let Some(drag) = self
                        .component_drag
                        .take()
                        .filter(|drag| drag.component_id == component.id)
                {
                    completed_move = Some(drag);
                }
                let display_position = completed_move
                    .filter(|drag| drag.component_id == component.id)
                    .or_else(|| {
                        self.component_drag
                            .filter(|drag| drag.component_id == component.id)
                    })
                    .map_or(component.position, |drag| drag.position);
                node_rects.insert(component.id, node_rect(available, display_position));
                responses.insert(component.id, response);
            }
            for connection in connections {
                let source = components
                    .iter()
                    .find(|component| component.id == connection.source.component_id);
                let target = components
                    .iter()
                    .find(|component| component.id == connection.target.component_id);
                if let (Some(source), Some(target)) = (source, target) {
                    let source_rect = node_rects
                        .get(&source.id)
                        .copied()
                        .unwrap_or_else(|| node_rect(available, source.position));
                    let target_rect = node_rects
                        .get(&target.id)
                        .copied()
                        .unwrap_or_else(|| node_rect(available, target.position));
                    let start = port_position(
                        source_rect,
                        presentations
                            .get(&source.id)
                            .map(|presentation| presentation.ports.as_slice()),
                        presentations
                            .get(&source.id)
                            .map(|presentation| &presentation.port_locations),
                        connection.source.port_key.as_str(),
                    )
                    .unwrap_or_else(|| source_rect.right_center());
                    let end = port_position(
                        target_rect,
                        presentations
                            .get(&target.id)
                            .map(|presentation| presentation.ports.as_slice()),
                        presentations
                            .get(&target.id)
                            .map(|presentation| &presentation.port_locations),
                        connection.target.port_key.as_str(),
                    )
                    .unwrap_or_else(|| target_rect.left_center());
                    ui.painter().line_segment(
                        [start, end],
                        Stroke::new(2.5, Color32::from_rgb(20, 117, 112)),
                    );
                }
            }
            for component in components {
                let presentation = presentations.get(&component.id).cloned();
                let rect = node_rects
                    .get(&component.id)
                    .copied()
                    .unwrap_or_else(|| node_rect(available, component.position));
                let Some(response) = responses.get(&component.id) else {
                    continue;
                };
                self.show_node(
                    ui,
                    rect,
                    response,
                    &component,
                    presentation
                        .as_ref()
                        .map_or(&[][..], |presentation| presentation.ports.as_slice()),
                    presentation
                        .as_ref()
                        .map(|presentation| &presentation.port_locations),
                    presentation
                        .as_ref()
                        .and_then(|presentation| presentation.icon.as_ref()),
                );
            }
            if let Some(drag) = completed_move {
                self.apply(DocumentCommand::MoveComponent {
                    component_id: drag.component_id,
                    position: drag.position,
                });
            }
        });
    }

    /// Draws and interacts with one fixed-size canvas node.
    fn show_node(
        &mut self,
        ui: &mut egui::Ui,
        rect: Rect,
        response: &egui::Response,
        component: &simulation::document::ComponentInstance,
        ports: &[PortDefinition],
        port_locations: Option<&BTreeMap<ShareableString, NormalizedPosition>>,
        icon: Option<&EmbeddedSvg>,
    ) {
        let selected = self.selected_component == Some(component.id);
        let fill = if selected {
            Color32::from_rgb(222, 237, 232)
        } else {
            Color32::WHITE
        };
        ui.painter().rect(
            rect,
            5.0,
            fill,
            Stroke::new(
                if selected { 2.0 } else { 1.0 },
                if selected {
                    Color32::from_rgb(20, 117, 112)
                } else {
                    Color32::from_rgb(108, 112, 107)
                },
            ),
            StrokeKind::Outside,
        );
        if let Some(icon) = icon {
            let icon_rect =
                Rect::from_min_size(rect.min + Vec2::new(12.0, 27.0), Vec2::new(44.0, 44.0));
            egui::Image::from_bytes(icon.uri.clone(), Arc::clone(&icon.bytes))
                .fit_to_exact_size(icon_rect.size())
                .paint_at(ui, icon_rect);
        }
        let label_offset = if icon.is_some() { 68.0 } else { 12.0 };
        ui.painter().text(
            rect.min + Vec2::new(label_offset, 14.0),
            egui::Align2::LEFT_TOP,
            component.name.as_str(),
            FontId::proportional(16.0),
            Color32::from_rgb(27, 31, 29),
        );
        let source_label = match &component.component {
            ComponentReference::BuiltIn { type_id, .. } => type_id.as_str(),
            ComponentReference::Custom { .. } => "custom component",
        };
        ui.painter().text(
            rect.min + Vec2::new(label_offset, 44.0),
            egui::Align2::LEFT_TOP,
            source_label,
            FontId::monospace(11.0),
            Color32::from_rgb(91, 96, 92),
        );
        for port in ports {
            let Some(position) =
                port_position(rect, Some(ports), port_locations, port.key.as_str())
            else {
                continue;
            };
            let icon_rect = Rect::from_center_size(position, Vec2::splat(PORT_RADIUS * 3.0));
            ui.interact(
                icon_rect,
                egui::Id::new((component.id.as_raw(), port.key.as_str())),
                Sense::hover(),
            )
            .on_hover_text(format!(
                "{} ({:?})",
                port.display_name.as_str(),
                port.direction
            ));
            match port.direction {
                PortDirection::Input => {
                    ui.painter().circle(
                        position,
                        PORT_RADIUS,
                        Color32::WHITE,
                        Stroke::new(2.0, Color32::from_rgb(52, 87, 139)),
                    );
                }
                PortDirection::Output => {
                    ui.painter().circle(
                        position,
                        PORT_RADIUS,
                        Color32::from_rgb(20, 117, 112),
                        Stroke::new(1.5, Color32::WHITE),
                    );
                }
            }
        }
        if response.clicked() {
            self.selected_component = Some(component.id);
            self.parameter_drafts.clear();
        }
    }

    /// Adds one palette item through a document command.
    fn add_component(
        &mut self,
        type_id: ComponentTypeId,
        version: SemanticVersion,
        display_name: &ShareableString,
    ) {
        let column = self.placement_index % 3;
        let row = self.placement_index / 3;
        self.placement_index = self.placement_index.saturating_add(1);
        let outcome = self.controller.execute(DocumentCommand::AddBuiltIn {
            type_id,
            version: Some(version),
            name: format!("{} {}", display_name.as_str(), self.placement_index).into(),
            position: CanvasPosition {
                x: 40.0 + f64::from(column) * 220.0,
                y: 40.0 + f64::from(row) * 125.0,
            },
        });
        match outcome {
            Ok(CommandOutcome::Component(id)) => {
                self.selected_component = Some(id);
                self.status = "Component added".into();
                self.autosave();
            }
            Ok(
                CommandOutcome::Updated | CommandOutcome::Connection(_) | CommandOutcome::Probe(_),
            ) => {
                self.status = "Unexpected command result".into();
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    /// Applies a mutation and exposes failures in the toolbar.
    fn apply(&mut self, command: DocumentCommand) {
        self.status = match self.controller.execute(command) {
            Ok(_outcome) => {
                self.autosave();
                "Model changed".into()
            }
            Err(error) => error.to_string(),
        };
    }

    /// Replaces the open document with a new model.
    fn new_document(&mut self) {
        let raw_id = u128::from(self.placement_index).saturating_add(10_000);
        match DocumentController::new(DocumentId::from_raw(raw_id), "unsaved") {
            Ok(controller) => {
                self.controller = controller;
                self.selected_component = None;
                self.parameter_drafts.clear();
                self.pending_source = None;
                self.component_drag = None;
                self.custom_presentations.clear();
                self.status = "New model".into();
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    /// Opens the path field as a native model.
    fn open_document(&mut self) {
        let path = PathBuf::from(&self.path_input);
        match DocumentController::open(&path) {
            Ok(controller) => {
                self.controller = controller;
                self.remember(path);
                self.selected_component = None;
                self.parameter_drafts.clear();
                self.pending_source = None;
                self.component_drag = None;
                self.refresh_custom_presentations();
                self.status = "Model opened".into();
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    /// Restores the autosave sidecar for the path field.
    fn recover_document(&mut self) {
        let path = PathBuf::from(&self.path_input);
        match DocumentController::recover(&path) {
            Ok(controller) => {
                self.controller = controller;
                self.remember(path);
                self.selected_component = None;
                self.parameter_drafts.clear();
                self.pending_source = None;
                self.component_drag = None;
                self.refresh_custom_presentations();
                self.status = "Recovered autosave".into();
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    /// Saves either to the current location or path field.
    fn save_document(&mut self, save_as: bool) {
        let result = if save_as || self.controller.path().is_none() {
            self.controller.save_as(PathBuf::from(&self.path_input))
        } else {
            self.controller.save()
        };
        self.status = match result {
            Ok(()) => {
                self.remember(PathBuf::from(&self.path_input));
                "Model saved".into()
            }
            Err(error) => error.to_string(),
        };
    }

    /// Runs the open model and reports terminal status.
    fn run_model(&mut self) {
        self.status = match self.controller.run() {
            Ok(run) => format!("Run {:?}: {} series", run.status, run.series.len()),
            Err(error) => error.to_string(),
        };
    }

    /// Writes the current dirty model to the path field's recovery sidecar.
    fn autosave(&mut self) {
        if let Err(error) = self.controller.autosave(PathBuf::from(&self.path_input)) {
            self.status = format!("Autosave failed: {error}");
        }
    }

    /// Promotes one artifact to the front of the session recent list.
    fn remember(&mut self, path: PathBuf) {
        self.recent_artifacts.retain(|candidate| candidate != &path);
        self.recent_artifacts.insert(0, path);
        self.recent_artifacts.truncate(6);
    }

    /// Reloads verified custom appearance data for root component instances.
    fn refresh_custom_presentations(&mut self) {
        self.custom_presentations = self
            .controller
            .document()
            .root
            .components
            .iter()
            .filter_map(|component| {
                let document = self
                    .controller
                    .custom_component_document(&component.component)?;
                Some((
                    component.id,
                    component_presentation(
                        document
                            .public_ports
                            .into_iter()
                            .map(|port| port.definition)
                            .collect(),
                        document.appearance,
                    ),
                ))
            })
            .collect();
    }
}

/// Converts serializable component appearance into cached editor resources.
fn component_presentation(
    ports: Vec<PortDefinition>,
    appearance: ComponentAppearance,
) -> ComponentPresentation {
    let icon = appearance.icon_svg.as_ref().map(|svg| EmbeddedSvg {
        uri: format!(
            "bytes://component-{}.svg",
            blake3::hash(svg.as_str().as_bytes()).to_hex()
        ),
        bytes: Arc::from(svg.as_str().as_bytes()),
    });
    ComponentPresentation {
        ports,
        port_locations: appearance.port_locations,
        icon,
    }
}

/// Converts persisted canvas coordinates into panel coordinates.
fn canvas_pos(canvas: Rect, position: CanvasPosition) -> Pos2 {
    canvas.min + Vec2::new(position.x as f32, position.y as f32)
}

/// Returns the fixed canvas rectangle for one persisted node location.
fn node_rect(canvas: Rect, position: CanvasPosition) -> Rect {
    Rect::from_min_size(
        canvas_pos(canvas, position),
        Vec2::new(NODE_WIDTH, NODE_HEIGHT),
    )
}

/// Applies one frame of pointer movement while keeping the node on the canvas.
fn translated_position(position: CanvasPosition, delta: Vec2, canvas_size: Vec2) -> CanvasPosition {
    CanvasPosition {
        x: (position.x + f64::from(delta.x))
            .clamp(0.0, f64::from((canvas_size.x - NODE_WIDTH).max(0.0))),
        y: (position.y + f64::from(delta.y))
            .clamp(0.0, f64::from((canvas_size.y - NODE_HEIGHT).max(0.0))),
    }
}

/// Places one port evenly along the node edge selected by its direction.
fn port_position(
    rect: Rect,
    ports: Option<&[PortDefinition]>,
    locations: Option<&BTreeMap<ShareableString, NormalizedPosition>>,
    port_key: &str,
) -> Option<Pos2> {
    let ports = ports?;
    let port = ports.iter().find(|port| port.key.as_str() == port_key)?;
    if let Some(location) = locations.and_then(|locations| locations.get(&port.key))
        && location.x.is_finite()
        && location.y.is_finite()
    {
        return Some(Pos2::new(
            rect.left() + rect.width() * location.x.clamp(0.0, 1.0),
            rect.top() + rect.height() * location.y.clamp(0.0, 1.0),
        ));
    }
    let matching: Vec<_> = ports
        .iter()
        .filter(|candidate| candidate.direction == port.direction)
        .collect();
    let index = matching
        .iter()
        .position(|candidate| candidate.key == port.key)?;
    let y_fraction = (index + 1) as f32 / (matching.len() + 1) as f32;
    let x = match port.direction {
        PortDirection::Input => rect.left(),
        PortDirection::Output => rect.right(),
    };
    Some(Pos2::new(x, rect.top() + rect.height() * y_fraction))
}

/// Draws a subtle fixed grid for spatial orientation.
fn draw_grid(ui: &egui::Ui, rect: Rect) {
    let painter = ui.painter();
    let color = Color32::from_rgb(226, 227, 220);
    let mut x = rect.left();
    while x < rect.right() {
        painter.line_segment(
            [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
            Stroke::new(1.0, color),
        );
        x += 24.0;
    }
    let mut y = rect.top();
    while y < rect.bottom() {
        painter.line_segment(
            [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
            Stroke::new(1.0, color),
        );
        y += 24.0;
    }
}

/// Draws all scalar series in one normalized result viewport.
fn show_plot(ui: &mut egui::Ui, series: &[SignalSeries]) {
    let desired = Vec2::new(ui.available_width(), 150.0);
    let (rect, _response) = ui.allocate_exact_size(desired, Sense::hover());
    ui.painter().rect(
        rect,
        3.0,
        Color32::from_rgb(250, 250, 247),
        Stroke::new(1.0, Color32::from_rgb(183, 186, 180)),
        StrokeKind::Inside,
    );
    let colors = [
        Color32::from_rgb(20, 117, 112),
        Color32::from_rgb(199, 76, 54),
        Color32::from_rgb(52, 87, 139),
        Color32::from_rgb(155, 105, 27),
    ];
    for (series_index, signal) in series.iter().enumerate() {
        let samples: Vec<_> = signal
            .timestamps
            .iter()
            .zip(&signal.values)
            .filter_map(|(time, value)| scalar(value).map(|number| (*time, number)))
            .collect();
        let Some((minimum, maximum)) = scalar_range(&samples) else {
            continue;
        };
        let Some((start, stop)) = signal.time_range() else {
            continue;
        };
        let points: Vec<_> = samples
            .iter()
            .filter_map(|(time, value)| {
                plot_point(
                    rect.shrink(8.0),
                    *time,
                    *value,
                    start,
                    stop,
                    minimum,
                    maximum,
                )
            })
            .collect();
        let color = colors
            .iter()
            .copied()
            .cycle()
            .nth(series_index)
            .unwrap_or(Color32::BLACK);
        ui.painter()
            .add(egui::Shape::line(points, Stroke::new(2.0, color)));
    }
}

/// Draws a compact row-oriented sample table.
fn show_result_table(ui: &mut egui::Ui, series: &[SignalSeries]) {
    egui::ScrollArea::both().show(ui, |ui| {
        egui::Grid::new("result_table")
            .striped(true)
            .show(ui, |ui| {
                ui.strong("time");
                for signal in series {
                    ui.strong(signal.display_name.as_str());
                }
                ui.end_row();
                let row_count = series
                    .iter()
                    .map(|signal| signal.timestamps.len())
                    .max()
                    .unwrap_or(0);
                for row in 0..row_count {
                    ui.monospace(
                        series
                            .first()
                            .and_then(|signal| signal.timestamps.get(row))
                            .map_or_else(String::new, |time| format!("{time:.5}")),
                    );
                    for signal in series {
                        ui.monospace(
                            signal
                                .values
                                .get(row)
                                .map_or_else(String::new, format_runtime_value),
                        );
                    }
                    ui.end_row();
                }
            });
    });
}

/// Extracts one plottable scalar sample.
const fn scalar(value: &RuntimeValue) -> Option<f64> {
    match value {
        RuntimeValue::Scalar(value) | RuntimeValue::ScalarWithUnit { value, .. } => Some(*value),
        RuntimeValue::Boolean(_)
        | RuntimeValue::Integer(_)
        | RuntimeValue::String(_)
        | RuntimeValue::Identifier(_)
        | RuntimeValue::Path(_)
        | RuntimeValue::Table(_)
        | RuntimeValue::Unit(_) => None,
    }
}

/// Formats one runtime value for the result table.
fn format_runtime_value(value: &RuntimeValue) -> String {
    match value {
        RuntimeValue::Boolean(value) => value.to_string(),
        RuntimeValue::Integer(value) => value.to_string(),
        RuntimeValue::Scalar(value) => value.to_string(),
        RuntimeValue::ScalarWithUnit { value, unit } => format!("{value} {unit:?}"),
        RuntimeValue::String(value)
        | RuntimeValue::Identifier(value)
        | RuntimeValue::Path(value) => value.to_string(),
        RuntimeValue::Table(table) => format!("{} row(s)", table.rows().len()),
        RuntimeValue::Unit(unit) => format!("{unit:?}"),
    }
}

/// Returns finite scalar bounds for one series.
fn scalar_range(samples: &[(f64, f64)]) -> Option<(f64, f64)> {
    let first = samples.first()?.1;
    let mut minimum = first;
    let mut maximum = first;
    for (_time, value) in samples {
        minimum = minimum.min(*value);
        maximum = maximum.max(*value);
    }
    Some((minimum, maximum))
}

/// Maps one data sample into a plot rectangle.
fn plot_point(
    rect: Rect,
    time: f64,
    value: f64,
    start: f64,
    stop: f64,
    minimum: f64,
    maximum: f64,
) -> Option<Pos2> {
    let time_span = stop - start;
    let value_span = maximum - minimum;
    if !time_span.is_finite() || time_span <= 0.0 {
        return None;
    }
    let x_fraction = ((time - start) / time_span) as f32;
    let y_fraction = if value_span.abs() <= f64::EPSILON {
        0.5
    } else {
        ((value - minimum) / value_span) as f32
    };
    Some(Pos2::new(
        rect.left() + rect.width() * x_fraction,
        rect.bottom() - rect.height() * y_fraction,
    ))
}

#[cfg(test)]
mod tests {
    use super::{EditorApp, NODE_HEIGHT, port_position, translated_position};
    use eframe::egui::{Pos2, Rect, Vec2};
    use simulation::component::{NormalizedPosition, PortDefinition, PortDirection};
    use simulation::document::CanvasPosition;
    use simulation::identity::DocumentId;
    use simulation::parameter::ParameterValueType;
    use std::collections::BTreeMap;

    fn port(key: &str, direction: PortDirection) -> PortDefinition {
        PortDefinition {
            key: key.into(),
            display_name: key.into(),
            description: "".into(),
            direction,
            value_type: ParameterValueType::Scalar,
            unit: None,
            required: direction == PortDirection::Input,
        }
    }

    fn assert_position(actual: Option<Pos2>, expected: Pos2) {
        let actual = actual.expect("port should have a position");
        assert!((actual.x - expected.x).abs() < 0.001);
        assert!((actual.y - expected.y).abs() < 0.001);
    }

    #[test]
    fn ports_are_spaced_on_their_directional_edges() {
        let rect = Rect::from_min_max(Pos2::new(10.0, 20.0), Pos2::new(110.0, 100.0));
        let ports = [
            port("a", PortDirection::Input),
            port("b", PortDirection::Input),
            port("out", PortDirection::Output),
        ];

        assert_position(
            port_position(rect, Some(&ports), None, "a"),
            Pos2::new(10.0, 20.0 + 80.0 / 3.0),
        );
        assert_position(
            port_position(rect, Some(&ports), None, "b"),
            Pos2::new(10.0, 20.0 + 160.0 / 3.0),
        );
        assert_position(
            port_position(rect, Some(&ports), None, "out"),
            Pos2::new(110.0, 60.0),
        );
    }

    #[test]
    fn explicit_port_location_overrides_directional_edge() {
        let rect = Rect::from_min_max(Pos2::new(10.0, 20.0), Pos2::new(110.0, 100.0));
        let ports = [port("out", PortDirection::Output)];
        let locations = BTreeMap::from([("out".into(), NormalizedPosition { x: 0.25, y: 0.75 })]);

        assert_position(
            port_position(rect, Some(&ports), Some(&locations), "out"),
            Pos2::new(35.0, 80.0),
        );
    }

    #[test]
    fn editor_caches_every_builtin_icon() {
        let app = EditorApp::new(DocumentId::from_raw(1), "test").unwrap();

        assert_eq!(app.builtin_presentations.len(), 20);
        assert!(
            app.builtin_presentations
                .values()
                .all(|presentation| presentation.icon.is_some())
        );
    }

    #[test]
    fn dragged_position_moves_and_stays_inside_canvas() {
        assert_eq!(
            translated_position(
                CanvasPosition { x: 20.0, y: 30.0 },
                Vec2::new(15.0, -10.0),
                Vec2::new(500.0, 400.0),
            ),
            CanvasPosition { x: 35.0, y: 20.0 }
        );
        assert_eq!(
            translated_position(
                CanvasPosition { x: 20.0, y: 30.0 },
                Vec2::new(-100.0, 500.0),
                Vec2::new(500.0, 400.0),
            ),
            CanvasPosition {
                x: 0.0,
                y: 400.0 - f64::from(NODE_HEIGHT),
            }
        );
    }
}
