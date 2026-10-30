use crate::metadata::data::Metadata;

/// Represents User component data from a Hrafnix component file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentFile {
    /// Descriptive information about the component.
    pub metadata: Metadata,
}
