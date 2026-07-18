use crate::definition::ObjectDefinition;
use crate::key::StoreKey;
use crate::static_store::data::ItemParameter;
use crate::store::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents a set of items for an object in the static store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticObject {
    /// The definition of the object.
    definition: ObjectDefinition,
    /// The items of the object.
    items: BTreeMap<StoreKey, ItemParameter>,
    /// The pre-calculated BLAKE3 hash of the object's content.
    hash: [u8; 32],
}

impl StaticObject {
    /// Creates a new `StaticObject` with a description and separate parameters and variables maps.
    pub fn new<S: Into<ShareableString>>(
        description: S,
        items: BTreeMap<StoreKey, ItemParameter>,
    ) -> Self {
        let mut builder = ObjectDefinition::builder(description);
        for (k, v) in &items {
            builder.insert(k.clone(), v.definition());
        }
        let definition = builder.finish();
        let mut s = Self {
            definition,
            items,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        h.update(&[0x01]);
        h.update(b"Object");

        h.update(&(self.items.len() as u64).to_le_bytes());

        for (key, item) in &self.items {
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

    /// Returns a reference to the parameter with the specified key, if it exists.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ItemParameter> {
        self.items.get(&key.into())
    }

    /// Returns an iterator over the key-parameter pairs in the object.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &ItemParameter)> {
        self.items.iter()
    }

    /// Returns a reference to the object definition.
    pub fn definition(&self) -> &ObjectDefinition {
        &self.definition
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
        let entries: Vec<_> = self.items.iter().collect();
        for (i, (key, item)) in entries.iter().enumerate() {
            let is_last = i == entries.len() - 1;
            item.tree_print(f, key.as_str(), &next_prefix, is_last)?;
        }
        Ok(())
    }
}
