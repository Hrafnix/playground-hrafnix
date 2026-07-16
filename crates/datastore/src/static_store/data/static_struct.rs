use crate::StoreError;
use crate::definition::{StructDefinition, StructItemDefinition};
use crate::key::StoreKey;
use crate::static_store::data::{StaticBasic, StaticTable};
use crate::store::TreePrint;
use crate::store::data::{Container, ContainerDefinition, ContainerItem};
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents an item in a static struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StaticStructItem {
    /// A basic value.
    Basic(StaticBasic),
    /// A table value.
    Table(StaticTable),
}

impl StaticStructItem {
    /// Returns the basic value if this item is a basic value.
    pub fn get_basic(&self) -> Option<&StaticBasic> {
        match self {
            StaticStructItem::Basic(basic) => Some(basic),
            _ => None,
        }
    }

    /// Returns the table value if this item is a table value.
    pub fn get_table(&self) -> Option<&StaticTable> {
        match self {
            StaticStructItem::Table(table) => Some(table),
            _ => None,
        }
    }

    /// Returns the struct item definition.
    pub fn definition(&self) -> StructItemDefinition {
        match self {
            StaticStructItem::Basic(basic) => {
                StructItemDefinition::Basic(basic.definition().clone())
            }
            StaticStructItem::Table(table) => {
                StructItemDefinition::Table(table.definition().clone())
            }
        }
    }

    /// Returns the pre-calculated BLAKE3 hash of the item.
    pub fn hash(&self) -> [u8; 32] {
        match self {
            StaticStructItem::Basic(basic) => basic.hash(),
            StaticStructItem::Table(table) => table.hash(),
        }
    }
}

impl TreePrint for StaticStructItem {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        match self {
            StaticStructItem::Basic(basic) => basic.tree_print(f, label, prefix, last),
            StaticStructItem::Table(table) => table.tree_print(f, label, prefix, last),
        }
    }
}

/// Represents a structured value in the static store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticStruct {
    /// The definition of the struct.
    definition: StructDefinition,
    /// The items in the struct.
    items: BTreeMap<StoreKey, StaticStructItem>,
    /// The pre-calculated BLAKE3 hash of the struct's content.
    hash: [u8; 32],
}

impl StaticStruct {
    /// Creates a new `StaticStruct` with a description and items.
    pub fn new<S: Into<ShareableString>>(
        description: S,
        items: BTreeMap<StoreKey, StaticStructItem>,
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

    pub(crate) fn items(&self) -> &BTreeMap<StoreKey, StaticStructItem> {
        &self.items
    }

    /// Returns a reference to the item with the specified key, if it exists.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&StaticStructItem> {
        self.items.get(&key.into())
    }

    /// Returns an iterator over the key-item pairs in the struct.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &StaticStructItem)> {
        self.items.iter()
    }

    /// Returns a reference to the struct definition.
    pub fn definition(&self) -> &StructDefinition {
        &self.definition
    }
}

impl TryFrom<ContainerItem> for StaticStructItem {
    type Error = StoreError;

    fn try_from(item: ContainerItem) -> Result<Self, Self::Error> {
        match item {
            ContainerItem::Basic(basic) => Ok(StaticStructItem::Basic(StaticBasic::from(&basic))),
            ContainerItem::Table(table) => Ok(StaticStructItem::Table(StaticTable::from(&table))),
            ContainerItem::Container(_) => Err(StoreError::NestedContainerNotSupported),
        }
    }
}

impl TryFrom<ContainerItem> for StaticStruct {
    type Error = StoreError;

    fn try_from(item: ContainerItem) -> Result<Self, Self::Error> {
        match item {
            ContainerItem::Container(c) => match c.definition() {
                ContainerDefinition::Struct(_) => StaticStruct::try_from(&c),
                _ => Err(StoreError::SchemaMismatch(
                    "Expected Struct container".into(),
                )),
            },
            _ => Err(StoreError::SchemaMismatch(
                "Expected ContainerItem::Container for StaticStruct conversion".into(),
            )),
        }
    }
}

impl TryFrom<&Container> for StaticStruct {
    type Error = StoreError;

    fn try_from(container: &Container) -> Result<Self, Self::Error> {
        let mut items = BTreeMap::new();
        for key in container.keys() {
            if let Ok(item) = container.get_item(&key) {
                items.insert(key.clone(), StaticStructItem::try_from(item)?);
            }
        }
        let description = match container.definition() {
            ContainerDefinition::Struct(def) => def.description(),
            _ => {
                return Err(StoreError::SchemaMismatch(
                    "Expected StructDefinition".into(),
                ));
            }
        };
        Self::new(description, items)
    }
}

impl TreePrint for StaticStruct {
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
