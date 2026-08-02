use crate::definition::ParameterObjectDefinition;
use crate::editable::ItemEditable;
use crate::frozen::ParameterObjectFrozen;
use crate::key::ParameterKey;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents a set of items for an object in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterObjectEditable {
    /// The definition of the object.
    definition: ParameterObjectDefinition,
    /// The items of the object.
    items: BTreeMap<ParameterKey, ItemEditable>,
}

impl ParameterObjectEditable {
    /// Creates a new `ParameterObjectEditable` from a `ParameterObjectFrozen`.
    #[must_use]
    pub fn new_from_frozen(frozen_object: &ParameterObjectFrozen) -> Self {
        Self {
            definition: frozen_object.definition().clone(),
            items: frozen_object
                .iter()
                .map(|(key, value)| (key.clone(), ItemEditable::new_from_frozen(value)))
                .collect(),
        }
    }

    /// Creates a new `ParameterObjectFrozen` from this `ParameterObjectEditable`.
    #[must_use]
    pub fn freeze(&self) -> ParameterObjectFrozen {
        ParameterObjectFrozen::new_from_editable(self)
    }

    /// Returns a reference to the parameter with the specified key if it exists.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ItemEditable> {
        self.items.get(&key.into())
    }

    /// Returns a mutable reference to the parameter with the specified key if it exists.
    pub fn get_mut<S: AsRef<str>>(&mut self, key: S) -> Option<&mut ItemEditable> {
        self.items.get_mut(key.as_ref())
    }

    /// Returns an iterator over the key-parameter pairs in the object.
    pub fn iter(&self) -> impl Iterator<Item = (&ParameterKey, &ItemEditable)> {
        self.items.iter()
    }

    /// Returns a reference to the object definition.
    #[must_use]
    pub fn definition(&self) -> &ParameterObjectDefinition {
        &self.definition
    }
}

impl PartialEq<&ParameterObjectEditable> for ParameterObjectEditable {
    fn eq(&self, other: &&ParameterObjectEditable) -> bool {
        self == *other
    }
}

impl PartialEq<ParameterObjectEditable> for &ParameterObjectEditable {
    fn eq(&self, other: &ParameterObjectEditable) -> bool {
        *self == other
    }
}

impl TreePrint for ParameterObjectEditable {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "Editable Parameter Object ({})",
            self.definition.description()
        )?;

        let child_prefix = Self::child_prefix(prefix, last);

        let mut item_iter = self.items.iter().peekable();

        while let Some((key, item)) = item_iter.next() {
            let is_last = item_iter.peek().is_none();
            item.tree_print(f, key.as_str(), &child_prefix, is_last)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for ParameterObjectEditable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_print(f, "", "", true)
    }
}
