use crate::background_decoration::data::BackgroundDecoration;
use crate::foreground_decoration::data::ForegroundDecoration;
use crate::metadata::data::Metadata;

/// Represents User Model data from a Hrafnix model file.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelFile {
    /// Descriptive information about the model.
    pub metadata: Metadata,
    /// Decorations behind the model's contents, in drawing order.
    pub background_decorations: Vec<BackgroundDecoration>,
    /// Decorations in front of the model's contents, in drawing order.
    pub foreground_decorations: Vec<ForegroundDecoration>,
}
