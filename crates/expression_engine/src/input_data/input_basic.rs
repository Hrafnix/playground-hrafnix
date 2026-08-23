use crate::BasicDefinition;
use shareable_string::{ShareableString, SharedStringStore};

/// Represents basic input data in the system.
///
/// The `BasicInputData` struct is used to encapsulate
/// the definition of a basic input data item.
#[derive(Debug, Clone, PartialEq)]
pub struct BasicInputData {
    /// The definition describing valid values for this input item.
    definition: BasicDefinition,
    /// The raw string value provided by the user.
    data: ShareableString,
}

impl BasicInputData {
    /// Creates a new `BasicInputData` with the given `definition` and raw `data`.
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
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            definition: self.definition.launder(store),
            data: store.launder(&self.data),
        }
    }
}
