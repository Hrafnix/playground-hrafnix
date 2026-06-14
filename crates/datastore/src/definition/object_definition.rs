use crate::StoreError;
use crate::definition::ItemDefinition;
use crate::key::{ParameterKey, VariableKey};
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Builder for creating an `ObjectDefinition`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObjectDefinitionBuilder {
    description: ShareableString,
    parameter: BTreeMap<ParameterKey, ItemDefinition>,
    variables: BTreeMap<VariableKey, ItemDefinition>,
}

impl ObjectDefinitionBuilder {
    /// Creates a new `ObjectDefinitionBuilder` with a description.
    pub fn new<S: Into<ShareableString>>(description: S) -> Self {
        Self {
            description: description.into(),
            parameter: BTreeMap::new(),
            variables: BTreeMap::new(),
        }
    }

    /// Returns a new builder with parameter inherited from an existing `ObjectDefinition`.
    ///
    /// This method will overwrite existing parameter with the same keys.
    pub fn with_inherited(mut self, definition: ObjectDefinition) -> Self {
        self.parameter.extend(
            definition
                .parameter
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self.variables.extend(
            definition
                .variables
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self
    }

    /// Returns a new builder with parameter inherited from an existing `ObjectDefinition`,
    /// checking for conflicts.
    ///
    /// # Errors
    ///
    /// Returns `StoreError::ParameterConflict` if any parameter key already exists in the builder.
    /// Returns `StoreError::VariableConflict` if any variable key already exists in the builder.
    pub fn with_inherited_checked(
        mut self,
        definition: ObjectDefinition,
    ) -> Result<Self, StoreError> {
        for (key, _) in definition.parameter.iter() {
            if self.parameter.contains_key(key) {
                return Err(StoreError::ParameterConflict(key.key.clone()));
            }
        }
        for (key, _) in definition.variables.iter() {
            if self.variables.contains_key(key) {
                return Err(StoreError::VariableConflict(key.key.clone()));
            }
        }
        self.parameter.extend(
            definition
                .parameter
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self.variables.extend(
            definition
                .variables
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        Ok(self)
    }

    /// Returns a new builder with parameter and variables inherited from another builder.
    ///
    /// This method will overwrite existing parameter and variables with the same keys.
    pub fn with_inherited_from_builder(mut self, builder: ObjectDefinitionBuilder) -> Self {
        self.parameter.extend(builder.parameter);
        self.variables.extend(builder.variables);
        self
    }

    /// Returns a new builder with parameter inherited from another builder, checking for conflicts.
    ///
    /// # Errors
    ///
    /// Returns `StoreError::ParameterConflict` if any parameter key already exists in the builder.
    /// Returns `StoreError::VariableConflict` if any variable key already exists in the builder.
    pub fn with_inherited_from_builder_checked(
        mut self,
        builder: ObjectDefinitionBuilder,
    ) -> Result<Self, StoreError> {
        for (key, _) in builder.parameter.iter() {
            if self.parameter.contains_key(key) {
                return Err(StoreError::ParameterConflict(key.key.clone()));
            }
        }
        self.parameter.extend(builder.parameter);
        for (key, _) in builder.variables.iter() {
            if self.variables.contains_key(key) {
                return Err(StoreError::VariableConflict(key.key.clone()));
            }
        }
        self.variables.extend(builder.variables);
        Ok(self)
    }

    /// Returns a new builder with the parameter inserted.
    ///
    /// This method will overwrite existing parameter with the same keys.
    pub fn with_parameter_inserted<K: Into<ParameterKey>>(
        mut self,
        key: K,
        parameter: ItemDefinition,
    ) -> Self {
        self.insert_parameter(key, parameter);
        self
    }

    /// Inserts a parameter into the current builder.
    ///
    /// This method will overwrite existing parameter with the same keys.
    pub fn insert_parameter<K: Into<ParameterKey>>(&mut self, key: K, parameter: ItemDefinition) {
        let key = key.into();
        self.parameter.insert(key, parameter);
    }

    /// Returns a new builder with the parameter removed.
    pub fn without_parameter<S: Into<ShareableString>>(mut self, key: S) -> Self {
        self.remove_parameter(key);
        self
    }

    /// Removes a parameter from the current builder.
    pub fn remove_parameter<S: Into<ShareableString>>(&mut self, key: S) {
        self.parameter.remove(&key.into());
    }

    /// Returns a new builder with the variable inserted.
    ///
    /// This method will overwrite existing variables with the same keys.
    pub fn with_variable_inserted<K: Into<VariableKey>>(
        mut self,
        key: K,
        variable: ItemDefinition,
    ) -> Self {
        self.insert_variable(key, variable);
        self
    }

    /// Inserts a variable into the current builder.
    ///
    /// This method will overwrite existing variables with the same keys.
    pub fn insert_variable<K: Into<VariableKey>>(&mut self, key: K, variable: ItemDefinition) {
        let key = key.into();
        self.variables.insert(key, variable);
    }

    /// Returns a new builder with the variable removed.
    pub fn without_variable<S: Into<ShareableString>>(mut self, key: S) -> Self {
        self.remove_variable(key);
        self
    }

    /// Removes a variable from the current builder.
    pub fn remove_variable<S: Into<ShareableString>>(&mut self, key: S) {
        self.variables.remove(&key.into());
    }

    /// Builds the `ObjectDefinition`.
    pub fn finish(self) -> ObjectDefinition {
        ObjectDefinition {
            description: self.description,
            parameter: Arc::new(self.parameter),
            variables: Arc::new(self.variables),
        }
    }
}

/// Definition for an object, which is a collection of named parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ObjectDefinition {
    description: ShareableString,
    parameter: Arc<BTreeMap<ParameterKey, ItemDefinition>>,
    variables: Arc<BTreeMap<VariableKey, ItemDefinition>>,
}

impl ObjectDefinition {
    /// Returns a new `ObjectDefinitionBuilder` with the specified description.
    pub fn builder<S: Into<ShareableString>>(description: S) -> ObjectDefinitionBuilder {
        ObjectDefinitionBuilder::new(description)
    }

    /// Returns a new `ObjectDefinitionBuilder` initialized with the parameter of this definition.
    ///
    /// The new builder will have the specified description and a copy of the current parameter.
    pub fn new_inherit<S: Into<ShareableString>>(&self, description: S) -> ObjectDefinitionBuilder {
        ObjectDefinitionBuilder {
            description: description.into(),
            parameter: BTreeMap::clone(&self.parameter),
            variables: BTreeMap::clone(&self.variables),
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
    pub fn parameter_count(&self) -> usize {
        self.parameter.len()
    }

    /// Returns true if the object contains a parameter with the specified key.
    pub fn parameter_contains_key<S: Into<ShareableString>>(&self, key: S) -> bool {
        self.parameter.contains_key(&key.into())
    }

    /// Returns true if the object contains a parameter with the specified key string.
    pub fn parameter_contains_key_str(&self, key: &str) -> bool {
        self.parameter.contains_key(key)
    }

    /// Returns a reference to the parameter definition for the specified key.
    pub fn parameter_get<S: Into<ShareableString>>(&self, key: S) -> Option<&ItemDefinition> {
        self.parameter.get(&key.into())
    }

    /// Returns a reference to the parameter definition for the specified key string.
    pub fn parameter_get_str(&self, key: &str) -> Option<&ItemDefinition> {
        self.parameter.get(key)
    }

    /// Returns an iterator over the keys of the parameter.
    pub fn parameter_keys(&self) -> impl Iterator<Item = &ParameterKey> {
        self.parameter.keys()
    }

    /// Returns an iterator over the parameter definitions.
    pub fn parameter_iter(&self) -> impl Iterator<Item = (&ParameterKey, &ItemDefinition)> {
        self.parameter.iter()
    }

    /// Returns the number of variables in the object.
    pub fn variable_count(&self) -> usize {
        self.variables.len()
    }

    /// Returns true if the object contains a variable with the specified key.
    pub fn variable_contains_key<S: Into<ShareableString>>(&self, key: S) -> bool {
        self.variables.contains_key(&key.into())
    }

    /// Returns true if the object contains a variable with the specified key string.
    pub fn variable_contains_key_str(&self, key: &str) -> bool {
        self.variables.contains_key(key)
    }

    /// Returns a reference to the variable definition for the specified key.
    pub fn variable_get<S: Into<ShareableString>>(&self, key: S) -> Option<&ItemDefinition> {
        self.variables.get(&key.into())
    }

    /// Returns a reference to the variable definition for the specified key string.
    pub fn variable_get_str(&self, key: &str) -> Option<&ItemDefinition> {
        self.variables.get(key)
    }

    /// Returns an iterator over the keys of the variables.
    pub fn variable_keys(&self) -> impl Iterator<Item = &VariableKey> {
        self.variables.keys()
    }

    /// Returns an iterator over the variable definitions.
    pub fn variable_iter(&self) -> impl Iterator<Item = (&VariableKey, &ItemDefinition)> {
        self.variables.iter()
    }

    /// Returns a new `ObjectDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            parameter: Arc::new(
                self.parameter
                    .iter()
                    .map(|(k, v)| (k.launder(store), v.launder(store)))
                    .collect(),
            ),
            variables: Arc::new(
                self.variables
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
