use crate::definition::ItemDefinition;
use crate::frozen::{BasicFrozen, MapFrozen, StructFrozen, TableFrozen};
use crate::store::TreePrint;
use serde::{Deserialize, Serialize};

/// Represents a parameter value in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemFrozen {
    /// A basic parameter.
    Basic(BasicFrozen),
    /// A table parameter.
    Table(TableFrozen),
    /// A struct parameter.
    Struct(StructFrozen),
    /// A map parameter.
    Map(MapFrozen),
}

impl ItemFrozen {
    /// Returns the parameter definition.
    pub fn definition(&self) -> ItemDefinition {
        match self {
            ItemFrozen::Basic(b) => {
                ItemDefinition::new(b.definition().description(), b.definition().clone())
            }
            ItemFrozen::Table(t) => {
                ItemDefinition::new(t.definition().description(), t.definition().clone())
            }
            ItemFrozen::Struct(s) => {
                ItemDefinition::new(s.definition().description(), s.definition().clone())
            }
            ItemFrozen::Map(m) => {
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
    pub fn get_basic(&self) -> Option<&BasicFrozen> {
        match self {
            Self::Basic(b) => Some(b),
            _ => None,
        }
    }

    /// Returns the table value if this parameter is a table parameter.
    pub fn get_table(&self) -> Option<&TableFrozen> {
        match self {
            Self::Table(t) => Some(t),
            _ => None,
        }
    }

    /// Returns the struct value if this parameter is a struct parameter.
    pub fn get_struct(&self) -> Option<&StructFrozen> {
        match self {
            Self::Struct(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the map value if this parameter is a map parameter.
    pub fn get_map(&self) -> Option<&MapFrozen> {
        match self {
            Self::Map(m) => Some(m),
            _ => None,
        }
    }
}

impl TreePrint for ItemFrozen {
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
