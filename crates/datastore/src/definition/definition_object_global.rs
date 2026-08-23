use crate::definition::ItemDefinitionType;
use crate::traits::TreePrint;
use keys::global_key::GlobalKey;
use message::message::{Message, MessageCategory};
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Builder for creating a `GlobalObjectDefinition`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalObjectDefinitionBuilder {
    /// Human-readable description for the object being built.
    description: ShareableString,
    /// Keys in insertion order, used to preserve deterministic iteration.
    ordered_keys: Vec<GlobalKey>,
    /// Map of item definitions keyed by their global key.
    items: BTreeMap<GlobalKey, ItemDefinitionType>,
}

impl GlobalObjectDefinitionBuilder {
    /// Creates a new `GlobalObjectDefinitionBuilder` with a description.
    #[hotpath::measure]
    pub fn new<S: Into<ShareableString>>(description: S) -> Self {
        Self {
            description: description.into(),
            ordered_keys: Vec::new(),
            items: BTreeMap::new(),
        }
    }

    /// Returns a new builder inherited from an existing `GlobalObjectDefinition`.
    ///
    /// This method will overwrite existing items with the same keys.
    /// Will keep the order of the existing keys and append new keys at the end.
    #[must_use]
    #[hotpath::measure]
    pub fn inherit(mut self, definition: &GlobalObjectDefinition) -> Self {
        for key in definition.items.keys() {
            if !self.items.contains_key(key) {
                self.ordered_keys.push(key.clone());
            }
        }

        self.items
            .extend(definition.items.iter().map(|(k, v)| (k.clone(), v.clone())));

        self
    }

    /// Returns a new builder inherited from an existing `GlobalObjectDefinition`,
    /// checking for conflicts.
    ///
    /// This method will overwrite existing items with the same keys.
    /// Will keep the order of the existing keys and append new keys at the end.
    ///
    /// # Errors
    ///
    /// Returns an error message if any key already exists in the builder.
    #[hotpath::measure]
    pub fn inherit_with_check(
        mut self,
        definition: &GlobalObjectDefinition,
    ) -> Result<Self, Message> {
        for key in definition.items.keys() {
            if self.items.contains_key(key) {
                return Err(Message::error_with_param(
                    MessageCategory::Datastore,
                    "datastore_key_conflict",
                    "key",
                    key.key.to_string(),
                ));
            }
        }

        for key in definition.items.keys() {
            if !self.items.contains_key(key) {
                self.ordered_keys.push(key.clone());
            }
        }

        self.items
            .extend(definition.items.iter().map(|(k, v)| (k.clone(), v.clone())));

        Ok(self)
    }

    /// Returns a new builder inherited from another builder.
    ///
    /// This method will overwrite existing items with the same keys.
    /// Will keep the order of the existing keys and append new keys at the end.
    #[must_use]
    #[hotpath::measure]
    pub fn inherit_from_builder(mut self, builder: GlobalObjectDefinitionBuilder) -> Self {
        for key in builder.items.keys() {
            if !self.items.contains_key(key) {
                self.ordered_keys.push(key.clone());
            }
        }

        self.items.extend(builder.items);

        self
    }

    /// Returns a new builder inherited from another builder, checking for conflicts.
    ///
    /// This method will overwrite existing items with the same keys.
    /// Will keep the order of the existing keys and append new keys at the end.
    ///
    /// # Errors
    ///
    /// Returns an error message if any key already exists in the builder.
    #[hotpath::measure]
    pub fn inherit_from_builder_with_check(
        mut self,
        builder: GlobalObjectDefinitionBuilder,
    ) -> Result<Self, Message> {
        for key in builder.items.keys() {
            if self.items.contains_key(key) {
                return Err(Message::error_with_param(
                    MessageCategory::Datastore,
                    "datastore_key_conflict",
                    "key",
                    key.key.to_string(),
                ));
            }
        }

        for key in builder.items.keys() {
            if !self.items.contains_key(key) {
                self.ordered_keys.push(key.clone());
            }
        }

        self.items.extend(builder.items);

        Ok(self)
    }

    /// Returns a new builder with the item inserted.
    ///
    /// This method will overwrite existing items with the same keys.
    /// If the key does not exist, it will be appended to the end of the ordered keys.
    #[must_use]
    #[hotpath::measure]
    pub fn with<K: Into<GlobalKey>, T: Into<ItemDefinitionType>>(
        mut self,
        key: K,
        parameter: T,
    ) -> Self {
        self.insert(key, parameter.into());
        self
    }

    /// Inserts an item into the current builder.
    ///
    /// This method will overwrite existing items with the same keys.
    /// If the key does not exist, it will be appended to the end of the ordered keys.
    #[hotpath::measure]
    pub fn insert<K: Into<GlobalKey>, T: Into<ItemDefinitionType>>(
        &mut self,
        key: K,
        parameter: T,
    ) {
        let key = key.into();

        if !self.items.contains_key(&key) {
            self.ordered_keys.push(key.clone());
        }

        self.items.insert(key, parameter.into());
    }

    /// Returns a new builder with the item removed.
    #[must_use]
    #[hotpath::measure]
    pub fn without<S: Into<ShareableString>>(mut self, key: S) -> Self {
        self.remove(key);
        self
    }

    /// Removes an item from the current builder.
    #[hotpath::measure]
    pub fn remove<S: Into<ShareableString>>(&mut self, key: S) {
        let key = key.into();
        self.ordered_keys.retain(|k| k != &key);
        self.items.remove(&key);
    }

    /// Builds the `GlobalObjectDefinition`.
    #[must_use]
    #[hotpath::measure]
    pub fn finish(self) -> GlobalObjectDefinition {
        GlobalObjectDefinition {
            description: self.description,
            ordered_keys: self.ordered_keys,
            items: Arc::new(self.items),
        }
    }
}

/// Definition for a global object, which is a collection of named items.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalObjectDefinition {
    /// Human-readable description of this global object.
    description: ShareableString,
    /// Keys in insertion order, used to preserve deterministic iteration.
    ordered_keys: Vec<GlobalKey>,
    /// Map of item definitions keyed by their global key.
    items: Arc<BTreeMap<GlobalKey, ItemDefinitionType>>,
}

impl GlobalObjectDefinition {
    /// Returns a new `GlobalObjectDefinitionBuilder` with the specified description.
    #[hotpath::measure]
    pub fn builder<S: Into<ShareableString>>(description: S) -> GlobalObjectDefinitionBuilder {
        GlobalObjectDefinitionBuilder::new(description)
    }

    /// Returns a new `GlobalObjectDefinitionBuilder` initialized with the items of this definition.
    ///
    /// The new builder will have the specified description and a copy of the current items.
    #[hotpath::measure]
    pub fn inherit<S: Into<ShareableString>>(
        &self,
        description: S,
    ) -> GlobalObjectDefinitionBuilder {
        GlobalObjectDefinitionBuilder {
            description: description.into(),
            ordered_keys: self.ordered_keys.clone(),
            items: BTreeMap::clone(&self.items),
        }
    }

    /// Returns the description of the object.
    #[must_use]
    #[hotpath::measure]
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the description.
    #[must_use]
    pub const fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns the number of items in the object.
    #[must_use]
    #[hotpath::measure]
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the global object contains an item with the specified key.
    #[hotpath::measure]
    pub fn contains<S: Into<ShareableString>>(&self, key: S) -> bool {
        self.items.contains_key(&key.into())
    }

    /// Returns true if the global object contains an item with the specified key string.
    #[must_use]
    #[hotpath::measure]
    pub fn contains_str(&self, key: &str) -> bool {
        self.items.contains_key(key)
    }

    /// Returns a reference to the item definition for the specified key.
    #[hotpath::measure]
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ItemDefinitionType> {
        self.items.get(&key.into())
    }

    /// Returns a reference to the item definition for the specified key string.
    #[must_use]
    #[hotpath::measure]
    pub fn get_str(&self, key: &str) -> Option<&ItemDefinitionType> {
        self.items.get(key)
    }

    /// Returns an iterator over the keys of the items.
    #[hotpath::measure]
    pub fn keys(&self) -> impl Iterator<Item = &GlobalKey> {
        self.ordered_keys.iter()
    }

    /// Returns an iterator over the item definitions.
    #[hotpath::measure]
    pub fn iter(&self) -> impl Iterator<Item = (&GlobalKey, &ItemDefinitionType)> {
        self.ordered_keys
            .iter()
            .filter_map(move |k| self.items.get(k).map(|v| (k, v)))
    }

    /// Returns a new `GlobalObjectDefinition` with strings laundered through the provided store.
    #[must_use]
    #[hotpath::measure]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            ordered_keys: self.ordered_keys.iter().map(|k| k.launder(store)).collect(),
            items: Arc::new(
                self.items
                    .iter()
                    .map(|(k, v)| (k.launder(store), v.launder(store)))
                    .collect(),
            ),
        }
    }
}

impl PartialEq<&GlobalObjectDefinition> for GlobalObjectDefinition {
    #[hotpath::measure]
    fn eq(&self, other: &&GlobalObjectDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<GlobalObjectDefinition> for &GlobalObjectDefinition {
    #[hotpath::measure]
    fn eq(&self, other: &GlobalObjectDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for GlobalObjectDefinition {
    #[hotpath::measure]
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(f, "Global Object Definition ({})", self.description())?;

        let child_prefix = Self::child_prefix(prefix, last);

        let mut item_iter = self.ordered_keys.iter().peekable();

        while let Some(key) = item_iter.next() {
            let is_last = item_iter.peek().is_none();
            if let Some(item) = self.items.get(key) {
                item.tree_print(f, key.as_str(), &child_prefix, is_last)?;
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for GlobalObjectDefinition {
    #[hotpath::measure]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_print(f, "", "", true)
    }
}
