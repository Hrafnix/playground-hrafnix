use crate::metadata::data::Metadata;

/// Represents User Model data from a Hrafnix model file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelFile {
    /// Descriptive information about the model.
    pub metadata: Metadata,
}
