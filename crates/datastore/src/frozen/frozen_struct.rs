use crate::StoreError;
use crate::definition::{StructDefinition, StructItemDefinition};
use crate::frozen::{BasicFrozen, TableFrozen};
use crate::key::StoreKey;
use crate::store::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents an item in a frozen struct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StructItemFrozen {
    /// A basic value.
    Basic(BasicFrozen),
    /// A table value.
    Table(TableFrozen),
}

impl StructItemFrozen {
    /// Returns the basic value if this item is a basic value.
    pub fn get_basic(&self) -> Option<&BasicFrozen> {
        match self {
            StructItemFrozen::Basic(basic) => Some(basic),
            _ => None,
        }
    }

    /// Returns the table value if this item is a table value.
    pub fn get_table(&self) -> Option<&TableFrozen> {
        match self {
            StructItemFrozen::Table(table) => Some(table),
            _ => None,
        }
    }

    /// Returns the struct item definition.
    pub fn definition(&self) -> StructItemDefinition {
        match self {
            StructItemFrozen::Basic(basic) => {
                StructItemDefinition::Basic(basic.definition().clone())
            }
            StructItemFrozen::Table(table) => {
                StructItemDefinition::Table(table.definition().clone())
            }
        }
    }

    /// Returns the pre-calculated BLAKE3 hash of the item.
    pub fn hash(&self) -> [u8; 32] {
        match self {
            StructItemFrozen::Basic(basic) => basic.hash(),
            StructItemFrozen::Table(table) => table.hash(),
        }
    }
}

impl PartialEq<&StructItemFrozen> for StructItemFrozen {
    fn eq(&self, other: &&StructItemFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<StructItemFrozen> for &StructItemFrozen {
    fn eq(&self, other: &StructItemFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for StructItemFrozen {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        match self {
            StructItemFrozen::Basic(basic) => basic.tree_print(f, label, prefix, last),
            StructItemFrozen::Table(table) => table.tree_print(f, label, prefix, last),
        }
    }
}

/// Represents a structured value in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructFrozen {
    /// The definition of the struct.
    definition: StructDefinition,
    /// The items in the struct.
    items: BTreeMap<StoreKey, StructItemFrozen>,
    /// The pre-calculated BLAKE3 hash of the struct's content.
    hash: [u8; 32],
}

impl StructFrozen {
    /// Creates a new `StructFrozen` with a definition.
    pub fn new(definition: StructDefinition) -> Self {
        let mut items = BTreeMap::new();
        for (key, item_definition) in definition.iter() {
            match item_definition {
                StructItemDefinition::Basic(basic_definition) => {
                    items.insert(
                        key.clone(),
                        StructItemFrozen::Basic(BasicFrozen::new(basic_definition.clone())),
                    );
                }
                StructItemDefinition::Table(table_definition) => {
                    items.insert(
                        key.clone(),
                        StructItemFrozen::Table(TableFrozen::new(table_definition.clone())),
                    );
                }
            }
        }

        let mut s = Self {
            definition,
            items,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `StructFrozen` with a description and items.
    pub fn new_from_items<S: Into<ShareableString>>(
        description: S,
        items: BTreeMap<StoreKey, StructItemFrozen>,
    ) -> Result<Self, StoreError> {
        let items_vec: Vec<(StoreKey, StructItemDefinition)> = items
            .iter()
            .map(|(k, v)| (k.clone(), v.definition()))
            .collect();
        let definition = StructDefinition::new(description, items_vec);
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
        h.update(b"Struct");

        h.update(&(self.items.len() as u64).to_le_bytes());

        for (key, item) in &self.items {
            h.update(&key.current_blake3_hash());
            h.update(&item.hash());
        }

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the pre-calculated BLAKE3 hash of the struct.
    pub fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns a reference to the item with the specified key, if it exists.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&StructItemFrozen> {
        self.items.get(&key.into())
    }

    /// Return the basic value if this item is a basic value.
    pub fn get_basic<S: Into<ShareableString>>(&self, key: S) -> Option<&BasicFrozen> {
        if let Some(item) = self.get(key) {
            item.get_basic()
        } else {
            None
        }
    }

    /// Return the table value if this item is a table value.
    pub fn get_table<S: Into<ShareableString>>(&self, key: S) -> Option<&TableFrozen> {
        if let Some(item) = self.get(key) {
            item.get_table()
        } else {
            None
        }
    }

    /// Returns an iterator over the key-item pairs in the struct.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &StructItemFrozen)> {
        self.items.iter()
    }

    /// Returns a reference to the struct definition.
    pub fn definition(&self) -> &StructDefinition {
        &self.definition
    }
}

impl PartialEq<&StructFrozen> for StructFrozen {
    fn eq(&self, other: &&StructFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<StructFrozen> for &StructFrozen {
    fn eq(&self, other: &StructFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for StructFrozen {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        let type_str = "Struct";
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
