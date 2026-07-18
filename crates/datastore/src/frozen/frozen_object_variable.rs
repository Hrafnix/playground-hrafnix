use crate::definition::{ItemDefinitionType, VariableObjectDefinition};
use crate::frozen::{FileFrozen, ItemFrozen, MapFrozen, StringFrozen, TableFrozen};
use crate::key::VariableKey;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents a set of items for an object in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableObjectFrozen {
    /// The definition of the object.
    definition: VariableObjectDefinition,
    /// The items of the object.
    items: BTreeMap<VariableKey, ItemFrozen>,
    /// The pre-calculated BLAKE3 hash of the object's content.
    hash: [u8; 32],
}

impl VariableObjectFrozen {
    /// Creates a new `VariableObjectFrozen` with a definition.
    pub fn new(definition: VariableObjectDefinition) -> Self {
        let mut items = BTreeMap::new();
        for (item_key, item_definition_type) in definition.iter() {
            let key = item_key.clone();
            match item_definition_type {
                ItemDefinitionType::Choice(choice_def) => {
                    items.insert(
                        key,
                        ItemFrozen::Choice(crate::frozen::ChoiceFrozen::new(choice_def.clone())),
                    );
                }
                ItemDefinitionType::File(file_def) => {
                    items.insert(key, ItemFrozen::File(FileFrozen::new(file_def.clone())));
                }
                ItemDefinitionType::Map(map_def) => {
                    items.insert(key, ItemFrozen::Map(MapFrozen::new(map_def.clone())));
                }
                ItemDefinitionType::Number(number_def) => {
                    items.insert(
                        key,
                        ItemFrozen::Number(crate::frozen::NumberFrozen::new(number_def.clone())),
                    );
                }
                ItemDefinitionType::String(basic_def) => {
                    items.insert(
                        key,
                        ItemFrozen::String(StringFrozen::new(basic_def.clone())),
                    );
                }
                ItemDefinitionType::Table(table_def) => {
                    items.insert(key, ItemFrozen::Table(TableFrozen::new(table_def.clone())));
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

    /// Creates a new `VariableObjectFrozen` with a description and items.
    pub fn new_from_items<S: Into<ShareableString>>(
        description: S,
        items: BTreeMap<VariableKey, ItemFrozen>,
    ) -> Self {
        let mut builder = VariableObjectDefinition::builder(description);
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
        h.update(b"VariableObject");

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
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ItemFrozen> {
        self.items.get(&key.into())
    }

    /// Returns an iterator over the key-parameter pairs in the object.
    pub fn iter(&self) -> impl Iterator<Item = (&VariableKey, &ItemFrozen)> {
        self.items.iter()
    }

    /// Returns a reference to the object definition.
    pub fn definition(&self) -> &VariableObjectDefinition {
        &self.definition
    }
}

impl PartialEq<&VariableObjectFrozen> for VariableObjectFrozen {
    fn eq(&self, other: &&VariableObjectFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<VariableObjectFrozen> for &VariableObjectFrozen {
    fn eq(&self, other: &VariableObjectFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for VariableObjectFrozen {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "Frozen Variable Object ({})",
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

impl std::fmt::Display for VariableObjectFrozen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_print(f, "", "", true)
    }
}
