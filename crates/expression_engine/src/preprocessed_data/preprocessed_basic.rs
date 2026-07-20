use crate::BasicDefinition;
use shareable_string::{ShareableString, SharedStringStore};

/// Represents basic preprocessed data in the system.
///
/// The `BasicPreprocessedData` struct is used to encapsulate
/// the definition of a basic preprocessed data item.
#[derive(Debug, Clone, PartialEq)]
pub struct BasicPreprocessedData {
    definition: BasicDefinition,
    data: ShareableString,
}

impl BasicPreprocessedData {
    pub(crate) fn new(definition: BasicDefinition, data: ShareableString) -> Self {
        Self { definition, data }
    }

    /// Returns a reference to the definition of the basic preprocessed data.
    pub fn definition(&self) -> &BasicDefinition {
        &self.definition
    }

    /// Returns a reference to the data of the basic preprocessed data.
    pub fn data(&self) -> &ShareableString {
        &self.data
    }

    /// Returns a new `BasicPreprocessedData` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            definition: self.definition.launder(store),
            data: store.launder(&self.data),
        }
    }
}
