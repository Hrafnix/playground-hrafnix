use crate::StoreError;
use crate::definition::PropertyDefinition;
use crate::key::StoreKey;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Builder for creating an `ObjectDefinition`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObjectDefinitionBuilder {
    description: ShareableString,
    properties: BTreeMap<StoreKey, PropertyDefinition>,
}

impl ObjectDefinitionBuilder {
    /// Creates a new `ObjectDefinitionBuilder` with a description.
    pub fn new<S: Into<ShareableString>>(description: S) -> Self {
        Self {
            description: description.into(),
            properties: BTreeMap::new(),
        }
    }

    /// Returns a new builder with properties inherited from an existing `ObjectDefinition`.
    ///
    /// This method will overwrite existing properties with the same keys.
    pub fn with_inherited(mut self, definition: ObjectDefinition) -> Self {
        self.properties.extend(
            definition
                .properties
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self
    }

    /// Returns a new builder with properties inherited from an existing `ObjectDefinition`,
    /// checking for conflicts.
    ///
    /// # Errors
    ///
    /// Returns `StoreError::PropertyConflict` if any property key already exists in the builder.
    pub fn with_inherited_checked(
        mut self,
        definition: ObjectDefinition,
    ) -> Result<Self, StoreError> {
        for (key, _) in definition.properties.iter() {
            if self.properties.contains_key(key) {
                return Err(StoreError::PropertyConflict(key.key.clone()));
            }
        }
        self.properties.extend(
            definition
                .properties
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        Ok(self)
    }

    /// Returns a new builder with properties inherited from another builder.
    ///
    /// This method will overwrite existing properties with the same keys.
    pub fn with_inherited_from_builder(mut self, builder: ObjectDefinitionBuilder) -> Self {
        self.properties.extend(builder.properties);
        self
    }

    /// Returns a new builder with properties inherited from another builder, checking for conflicts.
    ///
    /// # Errors
    ///
    /// Returns `StoreError::PropertyConflict` if any property key already exists in the builder.
    pub fn with_inherited_from_builder_checked(
        mut self,
        builder: ObjectDefinitionBuilder,
    ) -> Result<Self, StoreError> {
        for (key, _) in builder.properties.iter() {
            if self.properties.contains_key(key) {
                return Err(StoreError::PropertyConflict(key.key.clone()));
            }
        }
        self.properties.extend(builder.properties);
        Ok(self)
    }

    /// Returns a new builder with the property inserted.
    ///
    /// This method will overwrite existing properties with the same keys.
    pub fn with_inserted<K: Into<StoreKey>>(
        mut self,
        key: K,
        property: PropertyDefinition,
    ) -> Self {
        self.insert(key, property);
        self
    }

    /// Inserts a property into the current builder.
    ///
    /// This method will overwrite existing properties with the same keys.
    pub fn insert<K: Into<StoreKey>>(&mut self, key: K, property: PropertyDefinition) {
        self.properties.insert(key.into(), property);
    }

    /// Returns a new builder with the property removed.
    pub fn without<S: Into<ShareableString>>(mut self, key: S) -> Self {
        self.properties.remove(&key.into());
        self
    }

    /// Removes a property from the current builder.
    pub fn remove<S: Into<ShareableString>>(&mut self, key: S) {
        self.properties.remove(&key.into());
    }

    /// Builds the `ObjectDefinition`.
    pub fn finish(self) -> ObjectDefinition {
        ObjectDefinition {
            description: self.description,
            properties: Arc::new(self.properties),
        }
    }
}

/// Definition for an object, which is a collection of named properties.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ObjectDefinition {
    description: ShareableString,
    properties: Arc<BTreeMap<StoreKey, PropertyDefinition>>,
}

impl ObjectDefinition {
    /// Returns a new `ObjectDefinitionBuilder` with the specified description.
    pub fn builder<S: Into<ShareableString>>(description: S) -> ObjectDefinitionBuilder {
        ObjectDefinitionBuilder::new(description)
    }

    /// Returns a new `ObjectDefinitionBuilder` initialized with the properties of this definition.
    ///
    /// The new builder will have the specified description and a copy of the current properties.
    pub fn new_inherit<S: Into<ShareableString>>(&self, description: S) -> ObjectDefinitionBuilder {
        ObjectDefinitionBuilder {
            description: description.into(),
            properties: BTreeMap::clone(&self.properties),
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

    /// Returns the number of properties in the object.
    pub fn count(&self) -> usize {
        self.properties.len()
    }

    /// Returns true if the object contains a property with the specified key.
    pub fn contains_key<S: Into<ShareableString>>(&self, key: S) -> bool {
        self.properties.contains_key(&key.into())
    }

    /// Returns a reference to the property definition for the specified key.
    pub fn get<S: Into<StoreKey>>(&self, key: S) -> Option<&PropertyDefinition> {
        self.properties.get(&key.into())
    }

    /// Returns a reference to the property definition for the specified key string.
    pub fn get_str(&self, key: &str) -> Option<&PropertyDefinition> {
        self.properties.get(key)
    }

    /// Returns an iterator over the keys of the properties.
    pub fn keys(&self) -> impl Iterator<Item = &StoreKey> {
        self.properties.keys()
    }

    /// Returns true if the object contains a property with the specified key string.
    pub fn contains_key_str(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }

    /// Returns an iterator over the property definitions.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &PropertyDefinition)> {
        self.properties.iter()
    }

    /// Returns a new `ObjectDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            properties: Arc::new(
                self.properties
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
