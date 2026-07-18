use crate::definition::BasicDefinition;
use crate::key::StoreKey;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Definition for a table, which is a collection of named columns each having a `BasicDefinition`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableDefinition {
    description: ShareableString,
    columns: Arc<BTreeMap<StoreKey, BasicDefinition>>,
}

impl TableDefinition {
    /// Creates a new `TableDefinition` with a description and a list of columns.
    pub fn new<S1: Into<ShareableString>, K: Into<StoreKey>>(
        description: S1,
        columns: Vec<(K, BasicDefinition)>,
    ) -> Self {
        let mut cols = BTreeMap::new();
        for (id, item) in columns {
            let key = id.into();
            cols.insert(key, item);
        }
        Self {
            description: description.into(),
            columns: Arc::new(cols),
        }
    }

    /// Returns the description of the table.
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns true if the table contains a column with the specified key.
    pub fn contains_key<S: Into<ShareableString>>(&self, key: S) -> bool {
        self.columns.contains_key(&key.into())
    }

    /// Returns a reference to the column definition for the specified key.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&BasicDefinition> {
        self.columns.get(&key.into())
    }

    /// Returns true if the table contains a column with the specified key string.
    pub fn contains_key_str(&self, key: &str) -> bool {
        self.columns.iter().any(|(k, _)| k.as_str() == key)
    }

    /// Returns a reference to the column definition for the specified key string.
    pub fn get_str(&self, key: &str) -> Option<&BasicDefinition> {
        self.columns
            .iter()
            .find(|(k, _)| k.as_str() == key)
            .map(|(_, v)| v)
    }

    /// Returns an iterator over the keys of the columns.
    pub fn keys(&self) -> impl Iterator<Item = &StoreKey> {
        self.columns.keys()
    }

    /// Returns an iterator over the column definitions.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &BasicDefinition)> {
        self.columns.iter()
    }

    /// Returns the number of columns in the table.
    pub fn count(&self) -> usize {
        self.columns.len()
    }

    /// Returns a reference to the description.
    pub fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns a new `TableDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            columns: Arc::new(
                self.columns
                    .iter()
                    .map(|(id, item)| (id.launder(store), item.launder(store)))
                    .collect(),
            ),
        }
    }
}

impl PartialEq<&TableDefinition> for TableDefinition {
    fn eq(&self, other: &&TableDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<TableDefinition> for &TableDefinition {
    fn eq(&self, other: &TableDefinition) -> bool {
        *self == other
    }
}
