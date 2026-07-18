use crate::StoreError;
use crate::definition::ItemDefinition;
use crate::key::ParameterKey;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Builder for creating a `ParameterObjectDefinition`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParameterObjectDefinitionBuilder {
    description: ShareableString,
    items: BTreeMap<ParameterKey, ItemDefinition>,
}

impl ParameterObjectDefinitionBuilder {
    /// Creates a new `ParameterObjectDefinitionBuilder` with a description.
    pub fn new<S: Into<ShareableString>>(description: S) -> Self {
        Self {
            description: description.into(),
            items: BTreeMap::new(),
        }
    }

    /// Returns a new builder inherited from an existing `ParameterObjectDefinition`.
    ///
    /// This method will overwrite existing parameter with the same keys.
    pub fn inherit(mut self, definition: ParameterObjectDefinition) -> Self {
        self.items
            .extend(definition.items.iter().map(|(k, v)| (k.clone(), v.clone())));
        self
    }

    /// Returns a new builder inherited from an existing `ParameterObjectDefinition`,
    /// checking for conflicts.
    ///
    /// # Errors
    ///
    /// Returns `StoreError::KeyConflict` if any key already exists in the builder.
    pub fn inherit_with_check(
        mut self,
        definition: ParameterObjectDefinition,
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

    /// Returns a new builder with parameter inherited from another builder.
    ///
    /// This method will overwrite existing parameter with the same keys.
    pub fn inherit_from_builder(mut self, builder: ParameterObjectDefinitionBuilder) -> Self {
        self.items.extend(builder.items);
        self
    }

    /// Returns a new builder inherited from another builder, checking for conflicts.
    ///
    /// # Errors
    ///
    /// Returns `StoreError::KeyConflict` if any parameter key already exists in the builder.
    pub fn inherit_from_builder_with_check(
        mut self,
        builder: ParameterObjectDefinitionBuilder,
    ) -> Result<Self, StoreError> {
        for key in builder.items.keys() {
            if self.items.contains_key(key) {
                return Err(StoreError::KeyConflict(key.key.to_string()));
            }
        }
        self.items.extend(builder.items);
        Ok(self)
    }

    /// Returns a new builder with the parameter inserted.
    ///
    /// This method will overwrite existing parameter with the same keys.
    pub fn with<K: Into<ParameterKey>>(mut self, key: K, parameter: ItemDefinition) -> Self {
        self.insert(key, parameter);
        self
    }

    /// Inserts a parameter into the current builder.
    ///
    /// This method will overwrite existing parameter with the same keys.
    pub fn insert<K: Into<ParameterKey>>(&mut self, key: K, parameter: ItemDefinition) {
        let key = key.into();
        self.items.insert(key, parameter);
    }

    /// Returns a new builder with the parameter removed.
    pub fn without<S: Into<ShareableString>>(mut self, key: S) -> Self {
        self.remove(key);
        self
    }

    /// Removes a parameter from the current builder.
    pub fn remove<S: Into<ShareableString>>(&mut self, key: S) {
        self.items.remove(&key.into());
    }

    /// Builds the `ParameterObjectDefinition`.
    pub fn finish(self) -> ParameterObjectDefinition {
        ParameterObjectDefinition {
            description: self.description,
            items: Arc::new(self.items),
        }
    }
}

/// Definition for an object, which is a collection of named parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterObjectDefinition {
    description: ShareableString,
    items: Arc<BTreeMap<ParameterKey, ItemDefinition>>,
}

impl ParameterObjectDefinition {
    /// Returns a new `ParameterObjectDefinitionBuilder` with the specified description.
    pub fn builder<S: Into<ShareableString>>(description: S) -> ParameterObjectDefinitionBuilder {
        ParameterObjectDefinitionBuilder::new(description)
    }

    /// Returns a new `ParameterObjectDefinitionBuilder` initialized with the parameter of this definition.
    ///
    /// The new builder will have the specified description and a copy of the current parameter.
    pub fn inherit<S: Into<ShareableString>>(
        &self,
        description: S,
    ) -> ParameterObjectDefinitionBuilder {
        ParameterObjectDefinitionBuilder {
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

    /// Returns the number of parameter in the object.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the object contains a parameter with the specified key.
    pub fn contains<S: Into<ShareableString>>(&self, key: S) -> bool {
        self.items.contains_key(&key.into())
    }

    /// Returns true if the object contains a parameter with the specified key string.
    pub fn contains_str(&self, key: &str) -> bool {
        self.items.contains_key(key)
    }

    /// Returns a reference to the parameter definition for the specified key.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ItemDefinition> {
        self.items.get(&key.into())
    }

    /// Returns a reference to the parameter definition for the specified key string.
    pub fn get_str(&self, key: &str) -> Option<&ItemDefinition> {
        self.items.get(key)
    }

    /// Returns an iterator over the keys of the parameter.
    pub fn keys(&self) -> impl Iterator<Item = &ParameterKey> {
        self.items.keys()
    }

    /// Returns an iterator over the parameter definitions.
    pub fn iter(&self) -> impl Iterator<Item = (&ParameterKey, &ItemDefinition)> {
        self.items.iter()
    }

    /// Returns a new `ParameterObjectDefinition` with strings laundered through the provided store.
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

impl PartialEq<&ParameterObjectDefinition> for ParameterObjectDefinition {
    fn eq(&self, other: &&ParameterObjectDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<ParameterObjectDefinition> for &ParameterObjectDefinition {
    fn eq(&self, other: &ParameterObjectDefinition) -> bool {
        *self == other
    }
}
