use crate::StoreError;
use crate::definition::ItemDefinitionType;
use crate::key::VariableKey;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Builder for creating a `VariableObjectDefinition`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VariableObjectDefinitionBuilder {
    description: ShareableString,
    items: BTreeMap<VariableKey, ItemDefinitionType>,
}

impl VariableObjectDefinitionBuilder {
    /// Creates a new `VariableObjectDefinitionBuilder` with a description.
    pub fn new<S: Into<ShareableString>>(description: S) -> Self {
        Self {
            description: description.into(),
            items: BTreeMap::new(),
        }
    }

    /// Returns a new builder inherited from an existing `VariableObjectDefinition`.
    ///
    /// This method will overwrite existing variables with the same keys.
    pub fn inherit(mut self, definition: VariableObjectDefinition) -> Self {
        self.items
            .extend(definition.items.iter().map(|(k, v)| (k.clone(), v.clone())));
        self
    }

    /// Returns a new builder inherited from an existing `VariableObjectDefinition`,
    /// checking for conflicts.
    ///
    /// # Errors
    ///
    /// Returns `StoreError::KeyConflict` if any key already exists in the builder.
    pub fn inherit_with_check(
        mut self,
        definition: VariableObjectDefinition,
    ) -> Result<Self, StoreError> {
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
    /// This method will overwrite existing variables with the same keys.
    pub fn inherit_from_builder(mut self, builder: VariableObjectDefinitionBuilder) -> Self {
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
        builder: VariableObjectDefinitionBuilder,
    ) -> Result<Self, StoreError> {
        for key in builder.items.keys() {
            if self.items.contains_key(key) {
                return Err(StoreError::KeyConflict(key.key.to_string()));
            }
        }
        self.items.extend(builder.items);
        Ok(self)
    }

    /// Returns a new builder with the variable inserted.
    ///
    /// This method will overwrite existing variables with the same keys.
    pub fn with<K: Into<VariableKey>, T: Into<ItemDefinitionType>>(
        mut self,
        key: K,
        variable: T,
    ) -> Self {
        self.insert(key, variable.into());
        self
    }

    /// Inserts a variable into the current builder.
    ///
    /// This method will overwrite existing variables with the same keys.
    pub fn insert<K: Into<VariableKey>, T: Into<ItemDefinitionType>>(
        &mut self,
        key: K,
        variable: T,
    ) {
        let key = key.into();
        self.items.insert(key, variable.into());
    }

    /// Returns a new builder with the variable removed.
    pub fn without<S: Into<ShareableString>>(mut self, key: S) -> Self {
        self.remove(key);
        self
    }

    /// Removes a variable from the current builder.
    pub fn remove<S: Into<ShareableString>>(&mut self, key: S) {
        self.items.remove(&key.into());
    }

    /// Builds the `VariableObjectDefinition`.
    pub fn finish(self) -> VariableObjectDefinition {
        VariableObjectDefinition {
            description: self.description,
            items: Arc::new(self.items),
        }
    }
}

/// Definition for an object, which is a collection of named variables.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableObjectDefinition {
    description: ShareableString,
    items: Arc<BTreeMap<VariableKey, ItemDefinitionType>>,
}

impl VariableObjectDefinition {
    /// Returns a new `VariableObjectDefinitionBuilder` with the specified description.
    pub fn builder<S: Into<ShareableString>>(description: S) -> VariableObjectDefinitionBuilder {
        VariableObjectDefinitionBuilder::new(description)
    }

    /// Returns a new `VariableObjectDefinitionBuilder` initialized with the variables of this definition.
    ///
    /// The new builder will have the specified description and a copy of the current variables.
    pub fn inherit<S: Into<ShareableString>>(
        &self,
        description: S,
    ) -> VariableObjectDefinitionBuilder {
        VariableObjectDefinitionBuilder {
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

    /// Returns the number of variables in the object.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the object contains a variable with the specified key.
    pub fn contains<S: Into<ShareableString>>(&self, key: S) -> bool {
        self.items.contains_key(&key.into())
    }

    /// Returns true if the object contains a variable with the specified key string.
    pub fn contains_str(&self, key: &str) -> bool {
        self.items.contains_key(key)
    }

    /// Returns a reference to the variable definition for the specified key.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ItemDefinitionType> {
        self.items.get(&key.into())
    }

    /// Returns a reference to the variable definition for the specified key string.
    pub fn get_str(&self, key: &str) -> Option<&ItemDefinitionType> {
        self.items.get(key)
    }

    /// Returns an iterator over the keys of the variables.
    pub fn keys(&self) -> impl Iterator<Item = &VariableKey> {
        self.items.keys()
    }

    /// Returns an iterator over the variable definitions.
    pub fn iter(&self) -> impl Iterator<Item = (&VariableKey, &ItemDefinitionType)> {
        self.items.iter()
    }

    /// Returns a new `VariableObjectDefinition` with strings laundered through the provided store.
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

impl PartialEq<&VariableObjectDefinition> for VariableObjectDefinition {
    fn eq(&self, other: &&VariableObjectDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<VariableObjectDefinition> for &VariableObjectDefinition {
    fn eq(&self, other: &VariableObjectDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for VariableObjectDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(f, "Variable Object Definition ({})", self.description())?;

        let child_prefix = Self::child_prefix(prefix, last);

        let item_count = self.items.len();

        for (i, (key, item)) in self.items.iter().enumerate() {
            let is_last = i == item_count - 1;
            item.tree_print(f, key.as_str(), &child_prefix, is_last)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for VariableObjectDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_print(f, "", "", true)
    }
}
