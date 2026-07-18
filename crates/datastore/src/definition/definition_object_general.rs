use crate::StoreError;
use crate::definition::ItemDefinitionType;
use crate::key::StoreKey;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Builder for creating an `ObjectDefinition`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObjectDefinitionBuilder {
    description: ShareableString,
    items: BTreeMap<StoreKey, ItemDefinitionType>,
}

impl ObjectDefinitionBuilder {
    /// Creates a new `ObjectDefinitionBuilder` with a description.
    pub fn new<S: Into<ShareableString>>(description: S) -> Self {
        Self {
            description: description.into(),
            items: BTreeMap::new(),
        }
    }

    /// Returns a new builder with inherited from an existing `ObjectDefinition`.
    ///
    /// This method will overwrite existing parameter with the same keys.
    pub fn inherit(mut self, definition: ObjectDefinition) -> Self {
        self.items
            .extend(definition.items.iter().map(|(k, v)| (k.clone(), v.clone())));
        self
    }

    /// Returns a new builder with inherited from an existing `ObjectDefinition`,
    /// checking for conflicts.
    ///
    /// # Errors
    ///
    /// Returns `StoreError::KeyConflict` if any key already exists in the builder.
    pub fn inherit_with_check(mut self, definition: ObjectDefinition) -> Result<Self, StoreError> {
        for key in definition.items.keys() {
            if self.items.contains_key(key) {
                return Err(StoreError::KeyConflict(key.key.to_string()));
            }
        }
        self.items
            .extend(definition.items.iter().map(|(k, v)| (k.clone(), v.clone())));
        Ok(self)
    }

    /// Returns a new builder inherited from another builder.
    ///
    /// This method will overwrite existing keys.
    pub fn inherit_from_builder(mut self, builder: ObjectDefinitionBuilder) -> Self {
        self.items.extend(builder.items);
        self
    }

    /// Returns a new builder inherited from another builder, checking for conflicts.
    ///
    /// # Errors
    ///
    /// Returns `StoreError::KeyConflict` if any key already exists in the builder.
    pub fn inherit_from_builder_with_check(
        mut self,
        builder: ObjectDefinitionBuilder,
    ) -> Result<Self, StoreError> {
        for key in builder.items.keys() {
            if self.items.contains_key(key) {
                return Err(StoreError::KeyConflict(key.key.to_string()));
            }
        }
        self.items.extend(builder.items);
        Ok(self)
    }

    /// Returns a new builder with the item inserted.
    ///
    /// This method will overwrite existing item with the same keys.
    pub fn with<K: Into<StoreKey>, T: Into<ItemDefinitionType>>(
        mut self,
        key: K,
        parameter: T,
    ) -> Self {
        self.insert(key, parameter.into());
        self
    }

    /// Inserts an item into the current builder.
    ///
    /// This method will overwrite existing item with the same keys.
    pub fn insert<K: Into<StoreKey>, T: Into<ItemDefinitionType>>(&mut self, key: K, parameter: T) {
        let key = key.into();
        self.items.insert(key, parameter.into());
    }

    /// Returns a new builder with the item removed.
    pub fn without<S: Into<ShareableString>>(mut self, key: S) -> Self {
        self.remove(key);
        self
    }

    /// Removes an item from the current builder.
    pub fn remove<S: Into<ShareableString>>(&mut self, key: S) {
        self.items.remove(&key.into());
    }

    /// Builds the `ObjectDefinition`.
    pub fn finish(self) -> ObjectDefinition {
        ObjectDefinition {
            description: self.description,
            items: Arc::new(self.items),
        }
    }
}

/// Definition for an object, which is a collection of named items.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ObjectDefinition {
    description: ShareableString,
    items: Arc<BTreeMap<StoreKey, ItemDefinitionType>>,
}

impl ObjectDefinition {
    /// Returns a new `ObjectDefinitionBuilder` with the specified description.
    pub fn builder<S: Into<ShareableString>>(description: S) -> ObjectDefinitionBuilder {
        ObjectDefinitionBuilder::new(description)
    }

    /// Returns a new `ObjectDefinitionBuilder` initialized with the items of this definition.
    ///
    /// The new builder will have the specified description and a copy of the current items.
    pub fn inherit<S: Into<ShareableString>>(&self, description: S) -> ObjectDefinitionBuilder {
        ObjectDefinitionBuilder {
            description: description.into(),
            items: BTreeMap::clone(&self.items),
        }
    }

    /// Returns the description of the object.
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the description.
    pub fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns the number of items in the object.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the object contains an item with the specified key.
    pub fn contains<S: Into<ShareableString>>(&self, key: S) -> bool {
        self.items.contains_key(&key.into())
    }

    /// Returns true if the object contains an item with the specified key string.
    pub fn contains_str(&self, key: &str) -> bool {
        self.items.contains_key(key)
    }

    /// Returns a reference to the item definition for the specified key.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ItemDefinitionType> {
        self.items.get(&key.into())
    }

    /// Returns a reference to the item definition for the specified key string.
    pub fn get_str(&self, key: &str) -> Option<&ItemDefinitionType> {
        self.items.get(key)
    }

    /// Returns an iterator over the keys of the items.
    pub fn keys(&self) -> impl Iterator<Item = &StoreKey> {
        self.items.keys()
    }

    /// Returns an iterator over the item definitions.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &ItemDefinitionType)> {
        self.items.iter()
    }

    /// Returns a new `ObjectDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            items: Arc::new(
                self.items
                    .iter()
                    .map(|(k, v)| (k.launder(store), v.launder(store)))
                    .collect(),
            ),
        }
    }
}

impl PartialEq<&ObjectDefinition> for ObjectDefinition {
    fn eq(&self, other: &&ObjectDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<ObjectDefinition> for &ObjectDefinition {
    fn eq(&self, other: &ObjectDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for ObjectDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(f, "Object Definition ({})", self.description())?;

        let child_prefix = Self::child_prefix(prefix, last);

        let item_count = self.items.len();

        for (i, (key, item)) in self.items.iter().enumerate() {
            let is_last = i == item_count - 1;
            item.tree_print(f, key.as_str(), &child_prefix, is_last)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for ObjectDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_print(f, "", "", true)
    }
}
