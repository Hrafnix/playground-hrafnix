use crate::definition::ItemDefinitionType;
use crate::traits::TreePrint;
use keys::variable_key::VariableKey;
use message::message::{Message, MessageCategory};
use shareable_string::{ShareableString, SharedStringStore};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Builder for creating a `VariableObjectDefinition`.
#[derive(Debug, Clone, Default)]
pub struct VariableObjectDefinitionBuilder {
    /// Human-readable description for the object being built.
    description: ShareableString,
    /// Keys in insertion order, used to preserve deterministic iteration.
    ordered_keys: Vec<VariableKey>,
    /// Map of item definitions keyed by their variable key.
    items: BTreeMap<VariableKey, ItemDefinitionType>,
}

impl VariableObjectDefinitionBuilder {
    /// Creates a new `VariableObjectDefinitionBuilder` with a description.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new<S: Into<ShareableString>>(description: S) -> Self {
        Self {
            description: description.into(),
            ordered_keys: Vec::new(),
            items: BTreeMap::new(),
        }
    }

    /// Returns a new builder inherited from an existing `VariableObjectDefinition`.
    ///
    /// This method will overwrite existing items with the same keys.
    /// Will keep the order of the existing keys and append new keys at the end.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn inherit(mut self, definition: &VariableObjectDefinition) -> Self {
        for key in definition.items.keys() {
            if !self.items.contains_key(key) {
                self.ordered_keys.push(key.clone());
            }
        }

        self.items
            .extend(definition.items.iter().map(|(k, v)| (k.clone(), v.clone())));

        self
    }

    /// Returns a new builder inherited from an existing `VariableObjectDefinition`,
    /// checking for conflicts.
    ///
    /// This method will overwrite existing items with the same keys.
    /// Will keep the order of the existing keys and append new keys at the end.
    ///
    /// # Errors
    ///
    /// Returns an error message if any key already exists in the builder.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn inherit_with_check(
        mut self,
        definition: &VariableObjectDefinition,
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
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn inherit_from_builder(mut self, builder: VariableObjectDefinitionBuilder) -> Self {
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
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn inherit_from_builder_with_check(
        mut self,
        builder: VariableObjectDefinitionBuilder,
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
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn with<K: Into<VariableKey>, T: Into<ItemDefinitionType>>(
        mut self,
        key: K,
        variable: T,
    ) -> Self {
        self.insert(key, variable.into());
        self
    }

    /// Inserts an item into the current builder.
    ///
    /// This method will overwrite existing items with the same keys.
    /// If the key does not exist, it will be appended to the end of the ordered keys.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn insert<K: Into<VariableKey>, T: Into<ItemDefinitionType>>(
        &mut self,
        key: K,
        variable: T,
    ) {
        let key = key.into();

        if !self.items.contains_key(&key) {
            self.ordered_keys.push(key.clone());
        }

        self.items.insert(key, variable.into());
    }

    /// Returns a new builder with the item inserted at the specified index.
    ///
    /// This method will overwrite existing items with the same keys.
    /// If the key already exists, its previous position is removed before
    /// inserting it at the requested index. The index is clamped to the
    /// current number of keys.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn with_at<K: Into<VariableKey>, T: Into<ItemDefinitionType>>(
        mut self,
        index: usize,
        key: K,
        variable: T,
    ) -> Self {
        self.insert_at(index, key, variable);
        self
    }

    /// Inserts an item into the current builder at the specified index.
    ///
    /// This method will overwrite existing items with the same keys.
    /// If the key already exists, its previous position is removed before
    /// inserting it at the requested index. The index is clamped to the
    /// current number of keys.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn insert_at<K: Into<VariableKey>, T: Into<ItemDefinitionType>>(
        &mut self,
        index: usize,
        key: K,
        variable: T,
    ) {
        let key = key.into();

        if let Some(pos) = self.ordered_keys.iter().position(|k| k == &key) {
            self.ordered_keys.remove(pos);
        }

        let index = index.min(self.ordered_keys.len());
        self.ordered_keys.insert(index, key.clone());

        self.items.insert(key, variable.into());
    }

    /// Returns a new builder with the item removed.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn without<S: Into<ShareableString>>(mut self, key: S) -> Self {
        self.remove(key);
        self
    }

    /// Removes an item from the current builder.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn remove<S: Into<ShareableString>>(&mut self, key: S) {
        let key = key.into();
        self.ordered_keys.retain(|k| k != &key);
        self.items.remove(&key);
    }

    /// Builds the `VariableObjectDefinition`.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn finish(self) -> VariableObjectDefinition {
        VariableObjectDefinition {
            description: self.description,
            ordered_keys: self.ordered_keys,
            items: Arc::new(self.items),
        }
    }
}

/// Definition for an object, which is a collection of named variables.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VariableObjectDefinition {
    /// Human-readable description of this variable object.
    description: ShareableString,
    /// Keys in insertion order, used to preserve deterministic iteration.
    ordered_keys: Vec<VariableKey>,
    /// Map of item definitions keyed by their variable key.
    items: Arc<BTreeMap<VariableKey, ItemDefinitionType>>,
}

impl VariableObjectDefinition {
    /// Returns a new `VariableObjectDefinitionBuilder` with the specified description.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn builder<S: Into<ShareableString>>(description: S) -> VariableObjectDefinitionBuilder {
        VariableObjectDefinitionBuilder::new(description)
    }

    /// Returns a new `VariableObjectDefinitionBuilder` initialized with the items of this definition.
    ///
    /// The new builder will have the specified description and a copy of the current items.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn inherit<S: Into<ShareableString>>(
        &self,
        description: S,
    ) -> VariableObjectDefinitionBuilder {
        VariableObjectDefinitionBuilder {
            description: description.into(),
            ordered_keys: self.ordered_keys.clone(),
            items: BTreeMap::clone(&self.items),
        }
    }

    /// Returns the description of the object.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
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
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the object contains an item with the specified key.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn contains<S: Into<ShareableString>>(&self, key: S) -> bool {
        self.items.contains_key(&key.into())
    }

    /// Returns true if the object contains an item with the specified key string.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn contains_str(&self, key: &str) -> bool {
        self.items.contains_key(key)
    }

    /// Returns a reference to the item definition for the specified key.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ItemDefinitionType> {
        self.items.get(&key.into())
    }

    /// Returns a reference to the item definition for the specified key string.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn get_str(&self, key: &str) -> Option<&ItemDefinitionType> {
        self.items.get(key)
    }

    /// Returns an iterator over the keys of the items.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn keys(&self) -> impl Iterator<Item = &VariableKey> {
        self.ordered_keys.iter()
    }

    /// Returns an iterator over the item definitions.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn iter(&self) -> impl Iterator<Item = (&VariableKey, &ItemDefinitionType)> {
        self.ordered_keys
            .iter()
            .filter_map(move |k| self.items.get(k).map(|v| (k, v)))
    }

    /// Returns a new `VariableObjectDefinition` with strings laundered through the provided store.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
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

impl PartialEq<&VariableObjectDefinition> for VariableObjectDefinition {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&VariableObjectDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<VariableObjectDefinition> for &VariableObjectDefinition {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &VariableObjectDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for VariableObjectDefinition {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(f, "Variable Object Definition ({})", self.description())?;

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

impl std::fmt::Display for VariableObjectDefinition {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_print(f, "", "", true)
    }
}
