use crate::background_decoration::data::BackgroundDecoration;
use crate::foreground_decoration::data::ForegroundDecoration;
use crate::metadata::data::Metadata;

/// Represents User component data from a Hrafnix component file.
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentFile {
    /// Descriptive information about the component.
    pub metadata: Metadata,
    /// Decorations behind the component's contents, in drawing order.
    pub background_decorations: Vec<BackgroundDecoration>,
    /// Decorations in front of the component's contents, in drawing order.
    pub foreground_decorations: Vec<ForegroundDecoration>,
}
