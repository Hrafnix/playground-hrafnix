use crate::definition::NumberDefinition;
use crate::key::StoreKey;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::sync::Arc;

/// Definition for a table, which is a collection of named columns each having a `BasicDefinition`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableDefinition {
    description: ShareableString,
    columns: Arc<Vec<(StoreKey, NumberDefinition)>>,
}

impl TableDefinition {
    /// Creates a new `TableDefinition` with a description and a list of columns.
    pub fn new<S1: Into<ShareableString>, K: Into<StoreKey>>(
        description: S1,
        columns: Vec<(K, NumberDefinition)>,
    ) -> Self {
        Self {
            description: description.into(),
            columns: Arc::new(
                columns
                    .into_iter()
                    .map(|(id, item)| (id.into(), item))
                    .collect(),
            ),
        }
    }

    /// Returns the description of the table.
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns true if the table contains a column with the specified key.
    pub fn contains_key<S: Into<ShareableString>>(&self, key: S) -> bool {
        let key = key.into();
        for (column_key, _) in self.columns.iter() {
            if column_key == &key {
                return true;
            }
        }
        false
    }

    /// Returns a reference to the column definition for the specified key.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&NumberDefinition> {
        let key = key.into();
        for (column_key, column_def) in self.columns.iter() {
            if column_key == &key {
                return Some(column_def);
            }
        }
        None
    }

    /// Returns a reference to the column definition for the specified index.
    pub fn get_by_index(&self, index: usize) -> Option<&NumberDefinition> {
        self.columns.get(index).map(|(_, v)| v)
    }

    /// Returns the index of the column with the specified key.
    pub fn get_column_index_by_name<S: Into<ShareableString>>(&self, key: S) -> Option<usize> {
        let key = key.into();
        for (index, (column_key, _)) in self.columns.iter().enumerate() {
            if column_key == &key {
                return Some(index);
            }
        }
        None
    }

    /// Returns true if the table contains a column with the specified key string.
    pub fn contains_key_str(&self, key: &str) -> bool {
        self.columns.iter().any(|(k, _)| k.as_str() == key)
    }

    /// Returns a reference to the column definition for the specified key string.
    pub fn get_str(&self, key: &str) -> Option<&NumberDefinition> {
        self.columns
            .iter()
            .find(|(k, _)| k.as_str() == key)
            .map(|(_, v)| v)
    }

    /// Returns an iterator over the keys of the columns.
    pub fn keys(&self) -> impl Iterator<Item = &StoreKey> {
        self.columns.iter().map(|(k, _)| k)
    }

    /// Returns an iterator over the column definitions.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &NumberDefinition)> {
        self.columns.iter().map(|(k, v)| (k, v))
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

impl TreePrint for TableDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Table",
            prefix,
            Self::branch_char(last),
            label,
            self.description(),
        )?;

        let child_prefix = Self::child_prefix(prefix, last);

        let column_count = self.columns.len();

        for (i, (key, column)) in self.columns.iter().enumerate() {
            let is_last = i == column_count - 1;
            column.tree_print(f, key.as_str(), &child_prefix, is_last)?;
        }

        Ok(())
    }
}
