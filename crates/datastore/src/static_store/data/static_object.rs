use crate::StoreError;
use crate::definition::ObjectDefinition;
use crate::key::{ParameterKey, VariableKey};
use crate::static_store::data::ItemParameter;
use crate::store::TreePrint;
use crate::store::data::Object;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents a set of parameters for an object in the static store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticObject {
    /// The definition of the object.
    definition: ObjectDefinition,
    /// The parameter (items) of the object.
    parameter: BTreeMap<ParameterKey, ItemParameter>,
    /// The variable (items) of the object.
    variables: BTreeMap<VariableKey, ItemParameter>,
    /// The pre-calculated BLAKE3 hash of the object's content.
    hash: [u8; 32],
}

impl StaticObject {
    /// Creates a new `StaticObject` with a description and separate parameters and variables maps.
    pub fn new<S: Into<ShareableString>>(
        description: S,
        parameter: BTreeMap<ParameterKey, ItemParameter>,
        variables: BTreeMap<VariableKey, ItemParameter>,
    ) -> Self {
        let mut builder = ObjectDefinition::builder(description);
        for (k, v) in &parameter {
            builder.insert_parameter(k.clone(), v.definition());
        }
        for (k, v) in &variables {
            builder.insert_variable(k.clone(), v.definition());
        }
        let definition = builder.finish();
        let mut s = Self {
            definition,
            parameter,
            variables,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        h.update(&[0x01]);
        h.update(b"Object");

        h.update(&(self.parameter.len() as u64).to_le_bytes());

        for (key, item) in &self.parameter {
            h.update(&key.current_blake3_hash());
            h.update(&item.hash());
        }

        h.update(&(self.variables.len() as u64).to_le_bytes());

        for (key, item) in &self.variables {
            h.update(&key.current_blake3_hash());
            h.update(&item.hash());
        }

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the pre-calculated BLAKE3 hash of the object.
    pub fn hash(&self) -> [u8; 32] {
        self.hash
    }

    pub(crate) fn parameters(&self) -> &BTreeMap<ParameterKey, ItemParameter> {
        &self.parameter
    }

    /// Returns a reference to the parameter with the specified key, if it exists.
    pub fn get_parameter<S: Into<ShareableString>>(&self, key: S) -> Option<&ItemParameter> {
        self.parameter.get(&key.into())
    }

    /// Returns an iterator over the key-parameter pairs in the object.
    pub fn parameter_iter(&self) -> impl Iterator<Item = (&ParameterKey, &ItemParameter)> {
        self.parameter.iter()
    }

    pub(crate) fn variables(&self) -> &BTreeMap<VariableKey, ItemParameter> {
        &self.variables
    }

    /// Returns a reference to the variable with the specified key, if it exists.
    pub fn get_variable<S: Into<ShareableString>>(&self, key: S) -> Option<&ItemParameter> {
        self.variables.get(&key.into())
    }

    /// Returns an iterator over the key-variable pairs in the object.
    pub fn variables_iter(&self) -> impl Iterator<Item = (&VariableKey, &ItemParameter)> {
        self.variables.iter()
    }

    /// Returns a reference to the object definition.
    pub fn definition(&self) -> &ObjectDefinition {
        &self.definition
    }
}

impl TryFrom<&Object> for StaticObject {
    type Error = StoreError;

    fn try_from(object: &Object) -> Result<Self, Self::Error> {
        let mut parameter = BTreeMap::new();
        for (key, _) in object.definition().parameter_iter() {
            if let Ok(item) = object.get_item(key.as_str()) {
                parameter.insert(key.clone(), ItemParameter::try_from(item)?);
            }
        }
        let mut variables = BTreeMap::new();
        for (key, _) in object.definition().variable_iter() {
            if let Ok(item) = object.get_item(key.as_str()) {
                variables.insert(key.clone(), ItemParameter::try_from(item)?);
            }
        }
        let description = object.definition().description();
        Ok(Self::new(description, parameter, variables))
    }
}

impl TreePrint for StaticObject {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        let type_str = "Object";
        writeln!(
            f,
            "{}{}{}: {} - {}",
            prefix,
            Self::branch_char(prefix, last),
            label,
            type_str,
            self.definition.description()
        )?;
        let next_prefix = Self::next_prefix(prefix, last);
        let entries: Vec<_> = self.parameter.iter().collect();
        for (i, (key, item)) in entries.iter().enumerate() {
            let is_last = self.variables.is_empty() && i == entries.len() - 1;
            item.tree_print(f, key.as_str(), &next_prefix, is_last)?;
        }
        let entries: Vec<_> = self.variables.iter().collect();
        for (i, (key, item)) in entries.iter().enumerate() {
            let item_last = i == entries.len() - 1;
            item.tree_print(f, key.as_str(), &next_prefix, item_last)?;
        }
        Ok(())
    }
}
