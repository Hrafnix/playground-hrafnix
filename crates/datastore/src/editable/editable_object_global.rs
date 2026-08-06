use crate::definition::GlobalObjectDefinition;
use crate::editable::ItemEditable;
use crate::frozen::GlobalObjectFrozen;
use crate::key::GlobalKey;
use crate::traits::{ObjectEditable, TreePrint};
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents a set of items for an object in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalObjectEditable {
    /// The definition of the object.
    definition: GlobalObjectDefinition,
    /// The items of the object.
    items: BTreeMap<GlobalKey, ItemEditable>,
}

impl GlobalObjectEditable {
    /// Creates a new `GlobalObjectEditable` from an `GlobalObjectFrozen`.
    #[must_use]
    pub fn new_from_frozen(frozen_object: &GlobalObjectFrozen) -> Self {
        Self {
            definition: frozen_object.definition().clone(),
            items: frozen_object
                .iter()
                .map(|(key, value)| (key.clone(), ItemEditable::new_from_frozen(value)))
                .collect(),
        }
    }

    /// Creates a new `GlobalObjectFrozen` from this `GlobalObjectEditable`.
    #[must_use]
    pub fn freeze(&self) -> GlobalObjectFrozen {
        GlobalObjectFrozen::new_from_editable(self)
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
    pub fn iter(&self) -> impl Iterator<Item = (&GlobalKey, &ItemEditable)> {
        self.items.iter()
    }

    /// Returns a reference to the object definition.
    #[must_use]
    pub const fn definition(&self) -> &GlobalObjectDefinition {
        &self.definition
    }
}

impl ObjectEditable for GlobalObjectEditable {
    /// Returns a reference to the parameter with the specified key if it exists.
    fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ItemEditable> {
        self.get(key)
    }

    /// Returns a mutable reference to the parameter with the specified key if it exists.
    fn get_mut<S: AsRef<str>>(&mut self, key: S) -> Option<&mut ItemEditable> {
        self.get_mut(key)
    }
}

impl PartialEq<&GlobalObjectEditable> for GlobalObjectEditable {
    fn eq(&self, other: &&GlobalObjectEditable) -> bool {
        self == *other
    }
}

impl PartialEq<GlobalObjectEditable> for &GlobalObjectEditable {
    fn eq(&self, other: &GlobalObjectEditable) -> bool {
        *self == other
    }
}

impl TreePrint for GlobalObjectEditable {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "Global Object Editable ({})",
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

impl std::fmt::Display for GlobalObjectEditable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_print(f, "", "", true)
    }
}
