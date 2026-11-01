use crate::background_decoration::data::BackgroundDecoration;
use crate::foreground_decoration::data::ForegroundDecoration;
use crate::metadata::data::Metadata;

/// Represents User component data from a Hrafnix component file.
///
/// Fields are private; use the read-only accessors.
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentFile {
    /// Descriptive information about the component.
    metadata: Metadata,
    /// Decorations behind the component's contents, in drawing order.
    background_decorations: Vec<BackgroundDecoration>,
    /// Decorations in front of the component's contents, in drawing order.
    foreground_decorations: Vec<ForegroundDecoration>,
}

impl ComponentFile {
    /// Creates component data with metadata and ordered decoration layers.
    #[must_use]
    pub const fn new(
        metadata: Metadata,
        background_decorations: Vec<BackgroundDecoration>,
        foreground_decorations: Vec<ForegroundDecoration>,
    ) -> Self {
        Self {
            metadata,
            background_decorations,
            foreground_decorations,
        }
    }

    /// Returns the component's descriptive information.
    #[must_use]
    pub const fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Returns a read-only slice of background decorations in drawing order.
    #[must_use]
    pub const fn background_decorations(&self) -> &[BackgroundDecoration] {
        self.background_decorations.as_slice()
    }

    /// Returns a read-only slice of foreground decorations in drawing order.
    #[must_use]
    pub const fn foreground_decorations(&self) -> &[ForegroundDecoration] {
        self.foreground_decorations.as_slice()
    }
}
