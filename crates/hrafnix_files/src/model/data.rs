use crate::background_decoration::data::BackgroundDecoration;
use crate::foreground_decoration::data::ForegroundDecoration;
use crate::metadata::data::Metadata;

/// Represents User Model data from a Hrafnix model file.
///
/// Fields are private; use the read-only accessors.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelFile {
    /// Descriptive information about the model.
    metadata: Metadata,
    /// Decorations behind the model's contents, in drawing order.
    background_decorations: Vec<BackgroundDecoration>,
    /// Decorations in front of the model's contents, in drawing order.
    foreground_decorations: Vec<ForegroundDecoration>,
}

impl ModelFile {
    /// Creates model data with metadata and ordered decoration layers.
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

    /// Returns the model's descriptive information.
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
