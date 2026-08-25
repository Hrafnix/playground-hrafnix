use crate::identity::{ComponentId, ConnectionId, DocumentId, PortId, ProbeId, SystemId};
use message::message::{Message, MessageCategory, MessageLevel};
use shareable_string::ShareableString;

/// Severity of a simulation diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    /// Debug-only information.
    Debug,
    /// Informational model guidance.
    Information,
    /// Non-blocking issue retained in run metadata.
    Warning,
    /// Run-blocking issue.
    Error,
}

/// Stable category suitable for filtering simulation and lower-layer diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCategory {
    /// Generic datastore failure.
    Datastore,
    /// Expression parsing failure.
    ExpressionParsing,
    /// Expression evaluation failure.
    ExpressionEvaluation,
    /// Document or dependency resolution failure.
    Resolution,
    /// Graph or schema validation failure.
    Validation,
    /// Runtime failure.
    Runtime,
}

/// Stable simulation entity associated with a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityReference {
    /// Document entity.
    Document(DocumentId),
    /// System entity.
    System(SystemId),
    /// Component entity.
    Component(ComponentId),
    /// Port entity.
    Port(PortId),
    /// Connection entity.
    Connection(ConnectionId),
    /// Probe entity.
    Probe(ProbeId),
}

/// Simulation-owned diagnostic that can retain an original lower-layer message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// Severity used for execution gating.
    severity: DiagnosticSeverity,
    /// Filterable source category.
    category: DiagnosticCategory,
    /// Stable affected entity when known.
    entity: Option<EntityReference>,
    /// Parameter, port, or field key when known.
    field: Option<Box<ShareableString>>,
    /// Stable untranslated message key.
    message_key: Box<ShareableString>,
    /// Original lower-layer message, preserving paths, parameters, and detail.
    source: Option<Box<Message>>,
}

impl Diagnostic {
    /// Creates a simulation-owned diagnostic without a lower-layer source message.
    #[must_use]
    pub fn new(
        severity: DiagnosticSeverity,
        category: DiagnosticCategory,
        entity: Option<EntityReference>,
        field: Option<ShareableString>,
        message_key: impl Into<ShareableString>,
    ) -> Self {
        Self {
            severity,
            category,
            entity,
            field: field.map(Box::new),
            message_key: Box::new(message_key.into()),
            source: None,
        }
    }

    /// Adapts a lower-layer message and associates it with simulation context.
    #[must_use]
    pub fn from_message(
        message: Message,
        entity: Option<EntityReference>,
        field: Option<ShareableString>,
    ) -> Self {
        Self {
            severity: severity_from_message(message.level()),
            category: category_from_message(message.category()),
            entity,
            field: field.map(Box::new),
            message_key: Box::new(message.translate_data().message_key().clone()),
            source: Some(Box::new(message)),
        }
    }

    /// Returns the diagnostic severity.
    #[must_use]
    pub const fn severity(&self) -> DiagnosticSeverity {
        self.severity
    }

    /// Returns the diagnostic category.
    #[must_use]
    pub const fn category(&self) -> DiagnosticCategory {
        self.category
    }

    /// Returns the affected simulation entity.
    #[must_use]
    pub const fn entity(&self) -> Option<EntityReference> {
        self.entity
    }

    /// Returns the affected field key.
    #[must_use]
    pub fn field(&self) -> Option<&ShareableString> {
        self.field.as_deref()
    }

    /// Returns the stable message key.
    #[must_use]
    pub const fn message_key(&self) -> &ShareableString {
        &self.message_key
    }

    /// Returns the preserved lower-layer message.
    #[must_use]
    pub fn source(&self) -> Option<&Message> {
        self.source.as_deref()
    }
}

/// Maps generic message severity into the simulation diagnostic contract.
const fn severity_from_message(level: MessageLevel) -> DiagnosticSeverity {
    match level {
        MessageLevel::Debug => DiagnosticSeverity::Debug,
        MessageLevel::Info => DiagnosticSeverity::Information,
        MessageLevel::Warning => DiagnosticSeverity::Warning,
        MessageLevel::Error => DiagnosticSeverity::Error,
    }
}

/// Maps currently closed lower-layer categories without extending the message crate.
const fn category_from_message(category: MessageCategory) -> DiagnosticCategory {
    match category {
        MessageCategory::Datastore => DiagnosticCategory::Datastore,
        MessageCategory::ExpressionParsing => DiagnosticCategory::ExpressionParsing,
        MessageCategory::ExpressionEvaluation => DiagnosticCategory::ExpressionEvaluation,
    }
}

#[cfg(test)]
mod tests {
    use super::{Diagnostic, DiagnosticCategory, DiagnosticSeverity, EntityReference};
    use crate::identity::ComponentId;
    use message::message::{Message, MessageCategory};
    use message::path::Path;

    #[test]
    fn preserves_lower_layer_message_and_adds_entity_context() {
        let mut source = Message::error_with_param(
            MessageCategory::ExpressionEvaluation,
            "bad_parameter",
            "parameter",
            "gain",
        );
        source.override_item_path(Some(Path::from(("component", "gain"))));
        let diagnostic = Diagnostic::from_message(
            source,
            Some(EntityReference::Component(ComponentId::from_raw(7))),
            Some("gain".into()),
        );

        assert_eq!(diagnostic.severity(), DiagnosticSeverity::Error);
        assert_eq!(
            diagnostic.category(),
            DiagnosticCategory::ExpressionEvaluation
        );
        assert_eq!(diagnostic.message_key().as_str(), "bad_parameter");
        assert_eq!(diagnostic.field().unwrap().as_str(), "gain");
        assert_eq!(
            diagnostic
                .source()
                .unwrap()
                .item_path()
                .unwrap()
                .to_string(),
            "component/gain"
        );
    }

    #[test]
    fn creates_native_simulation_diagnostic_without_source() {
        let diagnostic = Diagnostic::new(
            DiagnosticSeverity::Error,
            DiagnosticCategory::Validation,
            None,
            Some("schema_version".into()),
            "simulation_unsupported_schema",
        );

        assert_eq!(
            diagnostic.message_key().as_str(),
            "simulation_unsupported_schema"
        );
        assert!(diagnostic.source().is_none());
    }
}
