use crate::definition::StructDefinition;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::sync::Arc;

/// Definition for a map parameter where keys are strings and values follow a `StructDefinition`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapDefinition {
    description: ShareableString,
    item_type: Arc<StructDefinition>,
}

impl MapDefinition {
    /// Creates a new `MapDefinition` with a description and item type.
    pub fn new<S: Into<ShareableString>>(description: S, item_type: StructDefinition) -> Self {
        Self {
            description: description.into(),
            item_type: Arc::new(item_type),
        }
    }

    /// Returns the description of the map.
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the item type definition.
    pub fn item_type(&self) -> &StructDefinition {
        self.item_type.as_ref()
    }

    /// Returns a reference to the description.
    pub fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns a new `MapDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            item_type: Arc::new(self.item_type.launder(store)),
        }
    }
}

impl PartialEq<&MapDefinition> for MapDefinition {
    fn eq(&self, other: &&MapDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<MapDefinition> for &MapDefinition {
    fn eq(&self, other: &MapDefinition) -> bool {
        *self == other
    }
}
