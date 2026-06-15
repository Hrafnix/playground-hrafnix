use crate::StoreError;
use crate::definition::ItemDefinition;
use crate::static_store::data::{StaticBasic, StaticMap, StaticStruct, StaticTable};
use crate::store::TreePrint;
use crate::store::data::{ContainerDefinition, ContainerItem};
use serde::{Deserialize, Serialize};

/// Represents a parameter value in the static store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemParameter {
    /// A basic parameter.
    Basic(StaticBasic),
    /// A table parameter.
    Table(StaticTable),
    /// A struct parameter.
    Struct(StaticStruct),
    /// A map parameter.
    Map(StaticMap),
}

impl TryFrom<ContainerItem> for ItemParameter {
    type Error = StoreError;

    fn try_from(item: ContainerItem) -> Result<Self, Self::Error> {
        match item {
            ContainerItem::Basic(b) => Ok(Self::Basic(StaticBasic::from(&b))),
            ContainerItem::Table(t) => Ok(Self::Table(StaticTable::from(&t))),
            ContainerItem::Container(c) => match c.definition() {
                ContainerDefinition::Struct(_) => Ok(Self::Struct(StaticStruct::try_from(&c)?)),
                ContainerDefinition::Map(_) => Ok(Self::Map(StaticMap::try_from(&c)?)),
            },
        }
    }
}

impl ItemParameter {
    /// Returns the parameter definition.
    pub fn definition(&self) -> ItemDefinition {
        match self {
            ItemParameter::Basic(b) => {
                ItemDefinition::new(b.definition().description(), b.definition().clone())
            }
            ItemParameter::Table(t) => {
                ItemDefinition::new(t.definition().description(), t.definition().clone())
            }
            ItemParameter::Struct(s) => {
                ItemDefinition::new(s.definition().description(), s.definition().clone())
            }
            ItemParameter::Map(m) => {
                ItemDefinition::new(m.definition().description(), m.definition().clone())
            }
        }
    }

    /// Returns the pre-calculated BLAKE3 hash of the parameter.
    pub fn hash(&self) -> [u8; 32] {
        match self {
            Self::Basic(b) => b.hash(),
            Self::Table(t) => t.hash(),
            Self::Struct(s) => s.hash(),
            Self::Map(m) => m.hash(),
        }
    }

    /// Returns the basic value if this parameter is a basic parameter.
    pub fn get_basic(&self) -> Option<&StaticBasic> {
        match self {
            Self::Basic(b) => Some(b),
            _ => None,
        }
    }

    /// Returns the table value if this parameter is a table parameter.
    pub fn get_table(&self) -> Option<&StaticTable> {
        match self {
            Self::Table(t) => Some(t),
            _ => None,
        }
    }

    /// Returns the struct value if this parameter is a struct parameter.
    pub fn get_struct(&self) -> Option<&StaticStruct> {
        match self {
            Self::Struct(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the map value if this parameter is a map parameter.
    pub fn get_map(&self) -> Option<&StaticMap> {
        match self {
            Self::Map(m) => Some(m),
            _ => None,
        }
    }
}

impl TreePrint for ItemParameter {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        match self {
            Self::Basic(b) => b.tree_print(f, label, prefix, last),
            Self::Table(t) => t.tree_print(f, label, prefix, last),
            Self::Struct(s) => s.tree_print(f, label, prefix, last),
            Self::Map(m) => m.tree_print(f, label, prefix, last),
        }
    }
}
