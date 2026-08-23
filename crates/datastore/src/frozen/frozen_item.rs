use crate::definition::ItemDefinitionType;
use crate::frozen::{
    BooleanFrozen, ChoiceFrozen, FileFrozen, FolderFrozen, IntegerFrozen, MapFrozen, NumberFrozen,
    NumberWithUnitsFrozen, SeparatorFrozen, StringFrozen, TabFrozen, TableFrozen,
    TableWithUnitsFrozen, UnitFrozen,
};
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};

/// Represents a parameter value in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemFrozen {
    /// A boolean parameter.
    Boolean(BooleanFrozen),
    /// A choice parameter.
    Choice(ChoiceFrozen),
    /// A file parameter.
    File(FileFrozen),
    /// A folder parameter.
    Folder(FolderFrozen),
    /// An integer parameter.
    Integer(IntegerFrozen),
    /// A map parameter.
    Map(MapFrozen),
    /// A number parameter.
    Number(NumberFrozen),
    /// A number parameter with units.
    NumberWithUnits(NumberWithUnitsFrozen),
    /// A string parameter.
    String(StringFrozen),
    /// A table parameter.
    Table(TableFrozen),
    /// A table parameter with units.
    TableWithUnits(TableWithUnitsFrozen),
    /// A unit parameter.
    Unit(UnitFrozen),
    /// A tab structural element.
    Tab(TabFrozen),
    /// A separator structural element.
    Separator(SeparatorFrozen),
}

impl ItemFrozen {
    /// Returns the parameter definition.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn definition(&self) -> ItemDefinitionType {
        match self {
            ItemFrozen::Boolean(b) => ItemDefinitionType::Boolean(b.definition().clone()),
            ItemFrozen::Choice(c) => ItemDefinitionType::Choice(c.definition().clone()),
            ItemFrozen::File(f) => ItemDefinitionType::File(f.definition().clone()),
            ItemFrozen::Folder(f) => ItemDefinitionType::Folder(f.definition().clone()),
            ItemFrozen::Integer(i) => ItemDefinitionType::Integer(i.definition().clone()),
            ItemFrozen::Map(m) => ItemDefinitionType::Map(m.definition().clone()),
            ItemFrozen::Number(n) => ItemDefinitionType::Number(n.definition().clone()),
            ItemFrozen::NumberWithUnits(nwu) => {
                ItemDefinitionType::NumberWithUnits(nwu.definition().clone())
            }
            ItemFrozen::String(s) => ItemDefinitionType::String(s.definition().clone()),
            ItemFrozen::Table(t) => ItemDefinitionType::Table(t.definition().clone()),
            ItemFrozen::TableWithUnits(twu) => {
                ItemDefinitionType::TableWithUnits(twu.definition().clone())
            }
            ItemFrozen::Unit(u) => ItemDefinitionType::Unit(u.definition().clone()),
            ItemFrozen::Tab(t) => ItemDefinitionType::Tab(t.definition().clone()),
            ItemFrozen::Separator(s) => ItemDefinitionType::Separator(s.definition().clone()),
        }
    }

    /// Returns the pre-calculated BLAKE3 hash of the parameter.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        match self {
            Self::Boolean(b) => b.hash(),
            Self::Choice(c) => c.hash(),
            Self::File(f) => f.hash(),
            Self::Folder(f) => f.hash(),
            Self::Integer(i) => i.hash(),
            Self::Map(m) => m.hash(),
            Self::Number(n) => n.hash(),
            Self::NumberWithUnits(nwu) => nwu.hash(),
            Self::String(s) => s.hash(),
            Self::Table(t) => t.hash(),
            Self::TableWithUnits(twu) => twu.hash(),
            Self::Unit(u) => u.hash(),
            Self::Tab(t) => t.hash(),
            Self::Separator(s) => s.hash(),
        }
    }

    /// Returns the choice value if this parameter is a choice parameter.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn get_choice(&self) -> Option<ChoiceFrozen> {
        match self {
            Self::Choice(c) => Some(c.clone()),
            Self::Boolean(_)
            | Self::File(_)
            | Self::Folder(_)
            | Self::Integer(_)
            | Self::Map(_)
            | Self::Number(_)
            | Self::NumberWithUnits(_)
            | Self::String(_)
            | Self::Table(_)
            | Self::TableWithUnits(_)
            | Self::Tab(_)
            | Self::Separator(_)
            | Self::Unit(_) => None,
        }
    }

    /// Returns the unit value if this parameter is a unit parameter.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn get_unit(&self) -> Option<UnitFrozen> {
        match self {
            Self::Unit(unit) => Some(unit.clone()),
            Self::Boolean(_)
            | Self::Choice(_)
            | Self::File(_)
            | Self::Folder(_)
            | Self::Integer(_)
            | Self::Map(_)
            | Self::Number(_)
            | Self::NumberWithUnits(_)
            | Self::String(_)
            | Self::Table(_)
            | Self::Tab(_)
            | Self::Separator(_)
            | Self::TableWithUnits(_) => None,
        }
    }

    /// Returns the file value if this parameter is a file parameter.
    #[must_use]
    pub const fn get_file(&self) -> Option<&FileFrozen> {
        match self {
            Self::File(f) => Some(f),
            Self::Boolean(_)
            | Self::Choice(_)
            | Self::Folder(_)
            | Self::Integer(_)
            | Self::Map(_)
            | Self::Number(_)
            | Self::NumberWithUnits(_)
            | Self::String(_)
            | Self::Table(_)
            | Self::TableWithUnits(_)
            | Self::Tab(_)
            | Self::Separator(_)
            | Self::Unit(_) => None,
        }
    }

    /// Returns the map value if this parameter is a map parameter.
    #[must_use]
    pub const fn get_map(&self) -> Option<&MapFrozen> {
        match self {
            Self::Map(m) => Some(m),
            Self::Boolean(_)
            | Self::Choice(_)
            | Self::File(_)
            | Self::Folder(_)
            | Self::Integer(_)
            | Self::Number(_)
            | Self::NumberWithUnits(_)
            | Self::String(_)
            | Self::Table(_)
            | Self::TableWithUnits(_)
            | Self::Tab(_)
            | Self::Separator(_)
            | Self::Unit(_) => None,
        }
    }

    /// Returns the number value if this parameter is a number parameter.
    #[must_use]
    pub const fn get_number(&self) -> Option<&NumberFrozen> {
        match self {
            Self::Number(n) => Some(n),
            Self::Boolean(_)
            | Self::Choice(_)
            | Self::File(_)
            | Self::Folder(_)
            | Self::Integer(_)
            | Self::Map(_)
            | Self::NumberWithUnits(_)
            | Self::String(_)
            | Self::Table(_)
            | Self::TableWithUnits(_)
            | Self::Tab(_)
            | Self::Separator(_)
            | Self::Unit(_) => None,
        }
    }

    /// Returns the string value if this parameter is a string parameter.
    #[must_use]
    pub const fn get_string(&self) -> Option<&StringFrozen> {
        match self {
            Self::String(s) => Some(s),
            Self::Boolean(_)
            | Self::Choice(_)
            | Self::File(_)
            | Self::Folder(_)
            | Self::Integer(_)
            | Self::Map(_)
            | Self::Number(_)
            | Self::NumberWithUnits(_)
            | Self::Table(_)
            | Self::TableWithUnits(_)
            | Self::Tab(_)
            | Self::Separator(_)
            | Self::Unit(_) => None,
        }
    }

    /// Returns the table value if this parameter is a table parameter.
    #[must_use]
    pub const fn get_table(&self) -> Option<&TableFrozen> {
        match self {
            Self::Table(t) => Some(t),
            Self::Boolean(_)
            | Self::Choice(_)
            | Self::File(_)
            | Self::Folder(_)
            | Self::Integer(_)
            | Self::Map(_)
            | Self::Number(_)
            | Self::NumberWithUnits(_)
            | Self::String(_)
            | Self::TableWithUnits(_)
            | Self::Tab(_)
            | Self::Separator(_)
            | Self::Unit(_) => None,
        }
    }
}

impl TreePrint for ItemFrozen {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        match self {
            Self::Boolean(boolean) => boolean.tree_print(f, label, prefix, last),
            Self::Choice(choice) => choice.tree_print(f, label, prefix, last),
            Self::File(file) => file.tree_print(f, label, prefix, last),
            Self::Folder(folder) => folder.tree_print(f, label, prefix, last),
            Self::Integer(integer) => integer.tree_print(f, label, prefix, last),
            Self::Map(map) => map.tree_print(f, label, prefix, last),
            Self::Number(number) => number.tree_print(f, label, prefix, last),
            Self::NumberWithUnits(number_with_units) => {
                number_with_units.tree_print(f, label, prefix, last)
            }
            Self::String(basic) => basic.tree_print(f, label, prefix, last),
            Self::Table(table) => table.tree_print(f, label, prefix, last),
            Self::TableWithUnits(table_with_units) => {
                table_with_units.tree_print(f, label, prefix, last)
            }
            Self::Unit(unit) => unit.tree_print(f, label, prefix, last),
            Self::Tab(tab) => tab.tree_print(f, label, prefix, last),
            Self::Separator(separator) => separator.tree_print(f, label, prefix, last),
        }
    }
}
