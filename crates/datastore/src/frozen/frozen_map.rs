use crate::StoreError;
use crate::definition::MapDefinition;
use crate::frozen::StructFrozen;
use crate::key::StoreKey;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents a map of parameter in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapFrozen {
    /// The definition of the map.
    definition: MapDefinition,
    /// The items in the map.
    items: BTreeMap<StoreKey, StructFrozen>,
    /// The pre-calculated BLAKE3 hash of the map's content.
    hash: [u8; 32],
}

impl MapFrozen {
    /// Creates a new `MapFrozen` with a definition.
    pub fn new(definition: MapDefinition) -> Self {
        let mut s = Self {
            definition,
            items: BTreeMap::new(),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `MapFrozen` with a description and items.
    pub fn new_from_items<S: Into<ShareableString>>(
        description: S,
        items: BTreeMap<StoreKey, StructFrozen>,
    ) -> Result<Self, StoreError> {
        let item_type = if let Some(first_item) = items.values().next() {
            let first_def = first_item.definition().clone();
            for item in items.values().skip(1) {
                if first_def != *item.definition() {
                    return Err(StoreError::SchemaMismatch(format!(
                        "FrozenMap items must have the same struct definition. Expected: {:?}, Found: {:?}",
                        first_def,
                        item.definition()
                    )));
                }
            }
            first_def
        } else {
            return Err(StoreError::MissingSchema(
                "FrozenMap cannot be empty as item type cannot be inferred".into(),
            ));
        };

        let definition = MapDefinition::new(description, item_type);
        let mut s = Self {
            definition,
            items,
            hash: [0u8; 32],
        };
        s.update_hash();
        Ok(s)
    }

    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        h.update(&[0x01]);
        h.update(b"Map");

        h.update(&(self.items.len() as u64).to_le_bytes());

        for (key, item) in &self.items {
            h.update(&key.current_blake3_hash());
            h.update(&item.hash());
        }

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the pre-calculated BLAKE3 hash of the map.
    pub fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns a reference to the item with the specified key, if it exists.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&StructFrozen> {
        self.items.get(&key.into())
    }

    /// Returns an iterator over the key-item pairs in the map.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &StructFrozen)> {
        self.items.iter()
    }

    /// Returns a reference to the map definition.
    pub fn definition(&self) -> &MapDefinition {
        &self.definition
    }

    /// Returns the number of items in the map.
    pub fn count(&self) -> usize {
        self.items.len()
    }
}

impl PartialEq<&MapFrozen> for MapFrozen {
    fn eq(&self, other: &&MapFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<MapFrozen> for &MapFrozen {
    fn eq(&self, other: &MapFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for MapFrozen {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Map",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
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
