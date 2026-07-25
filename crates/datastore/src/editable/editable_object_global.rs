use crate::definition::ObjectDefinition;
use crate::editable::ItemEditable;
use crate::frozen::ObjectFrozen;
use crate::key::GlobalKey;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents a set of items for an object in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectEditable {
    /// The definition of the object.
    definition: ObjectDefinition,
    /// The items of the object.
    items: BTreeMap<GlobalKey, ItemEditable>,
}

impl ObjectEditable {
    /// Creates a new `ObjectEditable` from an `ObjectFrozen`.
    pub fn new_from_frozen(frozen_object: &ObjectFrozen) -> Self {
        Self {
            definition: frozen_object.definition().clone(),
            items: frozen_object
                .iter()
                .map(|(key, value)| (key.clone(), ItemEditable::new_from_frozen(value)))
                .collect(),
        }
    }

    /// Creates a new `ObjectFrozen` from this `ObjectEditable`.
    pub fn freeze(&self) -> ObjectFrozen {
        ObjectFrozen::new_from_editable(self)
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
    pub fn iter(&self) -> impl Iterator<Item = (&GlobalKey, &ItemEditable)> {
        self.items.iter()
    }

    /// Returns a reference to the object definition.
    pub fn definition(&self) -> &ObjectDefinition {
        &self.definition
    }
}

impl PartialEq<&ObjectEditable> for ObjectEditable {
    fn eq(&self, other: &&ObjectEditable) -> bool {
        self == *other
    }
}

impl PartialEq<ObjectEditable> for &ObjectEditable {
    fn eq(&self, other: &ObjectEditable) -> bool {
        *self == other
    }
}

impl TreePrint for ObjectEditable {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(f, "Editable Object ({})", self.definition.description())?;

        let child_prefix = Self::child_prefix(prefix, last);

        let item_count = self.items.len();

        for (i, (key, item)) in self.items.iter().enumerate() {
            let is_last = i == item_count - 1;
            item.tree_print(f, key.as_str(), &child_prefix, is_last)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for ObjectEditable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_print(f, "", "", true)
    }
}
