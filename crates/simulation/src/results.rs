use crate::diagnostic::Diagnostic;
use crate::document::{PortEndpoint, SimulationSettings};
use crate::identity::{DocumentId, ProbeId, RunId};
use crate::value::RuntimeValue;
use shareable_string::ShareableString;

/// Terminal status of a synchronous simulation run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunStatus {
    /// Every requested timestep and sample completed.
    Completed,
    /// Initialization or stepping failed.
    Failed,
}

/// Values sampled from one persisted model probe.
#[derive(Debug, Clone, PartialEq)]
pub struct SignalSeries {
    /// Stable persisted probe identity.
    pub probe_id: ProbeId,
    /// Resolved component port sampled by the probe.
    pub source: PortEndpoint,
    /// User-facing series label.
    pub display_name: ShareableString,
    /// Fixed-grid sample times.
    pub timestamps: Vec<f64>,
    /// Runtime values corresponding one-to-one with timestamps.
    pub values: Vec<RuntimeValue>,
}

/// Metadata, diagnostics, and samples produced by one run.
#[derive(Debug, Clone, PartialEq)]
pub struct SimulationRun {
    /// Stable identity supplied by the application.
    pub run_id: RunId,
    /// Source model identity.
    pub source_document_id: DocumentId,
    /// Fixed-step settings used by the run.
    pub settings: SimulationSettings,
    /// Terminal run status.
    pub status: RunStatus,
    /// Runtime diagnostics in occurrence order.
    pub diagnostics: Vec<Diagnostic>,
    /// Probe series in persisted model order.
    pub series: Vec<SignalSeries>,
}
