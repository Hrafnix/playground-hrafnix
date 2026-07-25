use crate::definition::VariableObjectDefinition;
use crate::editable::ItemEditable;
use crate::frozen::VariableObjectFrozen;
use crate::key::VariableKey;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents a set of items for an object in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableObjectEditable {
    /// The definition of the object.
    definition: VariableObjectDefinition,
    /// The items of the object.
    items: BTreeMap<VariableKey, ItemEditable>,
}

impl VariableObjectEditable {
    /// Creates a new `VariableObjectEditable` from a `VariableObjectFrozen`.
    pub fn new_from_frozen(frozen_object: &VariableObjectFrozen) -> Self {
        Self {
            definition: frozen_object.definition().clone(),
            items: frozen_object
                .iter()
                .map(|(key, value)| (key.clone(), ItemEditable::new_from_frozen(value)))
                .collect(),
        }
    }

    /// Creates a new `VariableObjectFrozen` from this `VariableObjectEditable`.
    pub fn freeze(&self) -> VariableObjectFrozen {
        VariableObjectFrozen::new_from_editable(self)
    }

    /// Returns a reference to the parameter with the specified key, if it exists.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ItemEditable> {
        self.items.get(&key.into())
    }

    /// Returns a mutable reference to the parameter with the specified key, if it exists.
    pub fn get_mut<S: AsRef<str>>(&mut self, key: S) -> Option<&mut ItemEditable> {
        self.items.get_mut(key.as_ref())
    }

    /// Returns an iterator over the key-parameter pairs in the object.
    pub fn iter(&self) -> impl Iterator<Item = (&VariableKey, &ItemEditable)> {
        self.items.iter()
    }

    /// Returns a reference to the object definition.
    pub fn definition(&self) -> &VariableObjectDefinition {
        &self.definition
    }
}

impl PartialEq<&VariableObjectEditable> for VariableObjectEditable {
    fn eq(&self, other: &&VariableObjectEditable) -> bool {
        self == *other
    }
}

impl PartialEq<VariableObjectEditable> for &VariableObjectEditable {
    fn eq(&self, other: &VariableObjectEditable) -> bool {
        *self == other
    }
}

impl TreePrint for VariableObjectEditable {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "Editable Variable Object ({})",
            self.definition.description()
        )?;

        let child_prefix = Self::child_prefix(prefix, last);

        let item_count = self.items.len();

        for (i, (key, item)) in self.items.iter().enumerate() {
            let is_last = i == item_count - 1;
            item.tree_print(f, key.as_str(), &child_prefix, is_last)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for VariableObjectEditable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_print(f, "", "", true)
    }
}
