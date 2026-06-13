use crate::definition::{BasicDefinition, TableDefinition};
use crate::key::StoreKey;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::collections::BTreeMap;
use std::sync::Arc;

/// The definition of an item within a struct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StructItemDefinition {
    /// A basic property.
    Basic(BasicDefinition),
    /// A table property.
    Table(TableDefinition),
}

impl From<BasicDefinition> for StructItemDefinition {
    fn from(definition: BasicDefinition) -> Self {
        Self::Basic(definition)
    }
}

impl From<TableDefinition> for StructItemDefinition {
    fn from(definition: TableDefinition) -> Self {
        Self::Table(definition)
    }
}

impl StructItemDefinition {
    /// Returns a new `StructItemDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        match self {
            Self::Basic(def) => Self::Basic(def.launder(store)),
            Self::Table(def) => Self::Table(def.launder(store)),
        }
    }
}

impl PartialEq<&StructItemDefinition> for StructItemDefinition {
    fn eq(&self, other: &&StructItemDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<StructItemDefinition> for &StructItemDefinition {
    fn eq(&self, other: &StructItemDefinition) -> bool {
        *self == other
    }
}

/// Definition for a structured property, which is a collection of named `StructItemDefinition`s.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructDefinition {
    description: ShareableString,
    item_type: Arc<BTreeMap<StoreKey, StructItemDefinition>>,
}

impl StructDefinition {
    /// Creates a new `StructDefinition` with a description and a list of items.
    pub fn new<S1: Into<ShareableString>, K: Into<StoreKey>, I: Into<StructItemDefinition>>(
        description: S1,
        item_type: Vec<(K, I)>,
    ) -> Self {
        let mut items = BTreeMap::new();
        for (k, v) in item_type {
            let key = k.into();
            items.insert(key, v.into());
        }
        Self {
            description: description.into(),
            item_type: Arc::new(items),
        }
    }

    /// Returns the description of the struct.
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the struct item definition for the specified key.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&StructItemDefinition> {
        self.item_type.get(&key.into())
    }

    /// Returns a reference to the struct item definition for the specified key string.
    pub fn get_str(&self, key: &str) -> Option<&StructItemDefinition> {
        self.item_type
            .iter()
            .find(|(k, _)| k.as_str() == key)
            .map(|(_, v)| v)
    }

    /// Returns true if the struct contains an item with the specified key.
    pub fn contains_key<S: Into<ShareableString>>(&self, key: S) -> bool {
        self.item_type.contains_key(&key.into())
    }

    /// Returns an iterator over the keys of the struct items.
    pub fn keys(&self) -> impl Iterator<Item = &StoreKey> {
        self.item_type.keys()
    }

    /// Returns true if the struct contains an item with the specified key string.
    pub fn contains_key_str(&self, key: &str) -> bool {
        self.item_type.iter().any(|(k, _)| k.as_str() == key)
    }

    /// Returns an iterator over the struct item definitions.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &StructItemDefinition)> {
        self.item_type.iter()
    }

    /// Returns the number of items in the struct.
    pub fn count(&self) -> usize {
        self.item_type.len()
    }

    /// Returns a reference to the description.
    pub fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns a new `StructDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            item_type: Arc::new(
                self.item_type
                    .iter()
                    .map(|(k, v)| (k.launder(store), v.launder(store)))
                    .collect(),
            ),
        }
    }
}

impl PartialEq<&StructDefinition> for StructDefinition {
    fn eq(&self, other: &&StructDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<StructDefinition> for &StructDefinition {
    fn eq(&self, other: &StructDefinition) -> bool {
        *self == other
    }
}
