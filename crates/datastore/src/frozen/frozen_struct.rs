use crate::StoreError;
use crate::definition::{StructDefinition, StructItemDefinition};
use crate::frozen::{ChoiceFrozen, FileFrozen, NumberFrozen, StringFrozen, TableFrozen};
use crate::key::StoreKey;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents an item in a frozen struct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StructItemFrozen {
    /// A choice value.
    Choice(ChoiceFrozen),
    /// A file value.
    File(FileFrozen),
    /// A number value.
    Number(NumberFrozen),
    /// A string value.
    String(StringFrozen),
    /// A table value.
    Table(TableFrozen),
}

impl StructItemFrozen {
    /// Returns the string value if this item is a string value.
    pub fn get_string(&self) -> Option<&StringFrozen> {
        match self {
            StructItemFrozen::String(string) => Some(string),
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
            StructItemFrozen::Choice(choice) => {
                StructItemDefinition::Choice(choice.definition().clone())
            }
            StructItemFrozen::File(file) => StructItemDefinition::File(file.definition().clone()),
            StructItemFrozen::Number(number) => {
                StructItemDefinition::Number(number.definition().clone())
            }
            StructItemFrozen::String(basic) => {
                StructItemDefinition::String(basic.definition().clone())
            }
            StructItemFrozen::Table(table) => {
                StructItemDefinition::Table(table.definition().clone())
            }
        }
    }

    /// Returns the pre-calculated BLAKE3 hash of the item.
    pub fn hash(&self) -> [u8; 32] {
        match self {
            StructItemFrozen::Choice(choice) => choice.hash(),
            StructItemFrozen::File(file) => file.hash(),
            StructItemFrozen::Number(number) => number.hash(),
            StructItemFrozen::String(basic) => basic.hash(),
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
            StructItemFrozen::Choice(choice) => choice.tree_print(f, label, prefix, last),
            StructItemFrozen::File(file) => file.tree_print(f, label, prefix, last),
            StructItemFrozen::Number(number) => number.tree_print(f, label, prefix, last),
            StructItemFrozen::String(basic) => basic.tree_print(f, label, prefix, last),
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
                StructItemDefinition::Choice(choice_definition) => {
                    items.insert(
                        key.clone(),
                        StructItemFrozen::Choice(ChoiceFrozen::new(choice_definition.clone())),
                    );
                }
                StructItemDefinition::File(file_definition) => {
                    items.insert(
                        key.clone(),
                        StructItemFrozen::File(FileFrozen::new(file_definition.clone())),
                    );
                }
                StructItemDefinition::Number(number_definition) => {
                    items.insert(
                        key.clone(),
                        StructItemFrozen::Number(NumberFrozen::new(number_definition.clone())),
                    );
                }
                StructItemDefinition::String(basic_definition) => {
                    items.insert(
                        key.clone(),
                        StructItemFrozen::String(StringFrozen::new(basic_definition.clone())),
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
    pub fn get_string<S: Into<ShareableString>>(&self, key: S) -> Option<&StringFrozen> {
        if let Some(item) = self.get(key) {
            item.get_string()
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
        writeln!(
            f,
            "{}{}{} ({}) Struct",
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
