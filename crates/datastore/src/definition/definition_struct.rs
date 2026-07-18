use crate::definition::{
    ChoiceDefinition, FileDefinition, NumberDefinition, StringDefinition, TableDefinition,
};
use crate::key::StoreKey;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::collections::BTreeMap;
use std::sync::Arc;

/// The definition of an item within a struct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StructItemDefinition {
    /// A choice parameter.
    Choice(ChoiceDefinition),
    /// A file parameter.
    File(FileDefinition),
    /// A number parameter.
    Number(NumberDefinition),
    /// A string parameter.
    String(StringDefinition),
    /// A table parameter.
    Table(TableDefinition),
}

impl From<StringDefinition> for StructItemDefinition {
    fn from(definition: StringDefinition) -> Self {
        Self::String(definition)
    }
}

impl From<ChoiceDefinition> for StructItemDefinition {
    fn from(definition: ChoiceDefinition) -> Self {
        Self::Choice(definition)
    }
}

impl From<FileDefinition> for StructItemDefinition {
    fn from(definition: FileDefinition) -> Self {
        Self::File(definition)
    }
}

impl From<NumberDefinition> for StructItemDefinition {
    fn from(definition: NumberDefinition) -> Self {
        Self::Number(definition)
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
            Self::Choice(def) => Self::Choice(def.launder(store)),
            Self::File(def) => Self::File(def.launder(store)),
            Self::Number(def) => Self::Number(def.launder(store)),
            Self::String(def) => Self::String(def.launder(store)),
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

impl TreePrint for StructItemDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        match self {
            StructItemDefinition::Choice(choice) => choice.tree_print(f, label, prefix, last),
            StructItemDefinition::File(file) => file.tree_print(f, label, prefix, last),
            StructItemDefinition::Number(number) => number.tree_print(f, label, prefix, last),
            StructItemDefinition::String(string) => string.tree_print(f, label, prefix, last),
            StructItemDefinition::Table(table) => table.tree_print(f, label, prefix, last),
        }
    }
}

/// Definition for a structured parameter, which is a collection of named `StructItemDefinition`s.
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

impl TreePrint for StructDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Struct",
            prefix,
            Self::branch_char(last),
            label,
            self.description(),
        )?;

        let child_prefix = Self::child_prefix(prefix, last);

        let item_count = self.item_type.len();

        for (i, (key, item)) in self.item_type.iter().enumerate() {
            let is_last = i == item_count - 1;
            item.tree_print(f, key.as_str(), &child_prefix, is_last)?;
        }

        Ok(())
    }
}
