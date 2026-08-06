use crate::BasicDefinition;
use shareable_string::{ShareableString, SharedStringStore};

/// Represents basic input data in the system.
///
/// The `BasicInputData` struct is used to encapsulate
/// the definition of a basic input data item.
#[derive(Debug, Clone, PartialEq)]
pub struct BasicInputData {
    definition: BasicDefinition,
    data: ShareableString,
}

impl BasicInputData {
    pub(crate) const fn new(definition: BasicDefinition, data: ShareableString) -> Self {
        Self { definition, data }
    }

    /// Returns a reference to the definition of the basic input data.
    #[must_use]
    pub const fn definition(&self) -> &BasicDefinition {
        &self.definition
    }

    /// Returns a reference to the data of the basic input data.
    #[must_use]
    pub const fn data(&self) -> &ShareableString {
        &self.data
    }

    /// Returns a new `BasicInputData` with strings laundered through the provided store.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            definition: self.definition.launder(store),
            data: store.launder(&self.data),
        }
    }
}
