use crate::definition::ItemDefinitionType;
use crate::editable::{
    BooleanEditable, ChoiceEditable, FileEditable, MapEditable, NumberEditable, StringEditable,
    TableEditable,
};
use crate::frozen::ItemFrozen;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};

/// Represents a parameter value in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemEditable {
    /// A boolean parameter.
    Boolean(BooleanEditable),
    /// A choice parameter.
    Choice(ChoiceEditable),
    /// A file parameter.
    File(FileEditable),
    /// A map parameter.
    Map(MapEditable),
    /// A number parameter.
    Number(NumberEditable),
    /// A string parameter.
    String(StringEditable),
    /// A table parameter.
    Table(TableEditable),
}

impl ItemEditable {
    /// Creates a new `ItemEditable` instance from a given `ItemFrozen` value.
    pub fn new_from_frozen(static_item: &ItemFrozen) -> Self {
        match static_item {
            ItemFrozen::Boolean(boolean) => ItemEditable::Boolean(BooleanEditable::new(boolean)),
            ItemFrozen::Choice(choice) => ItemEditable::Choice(ChoiceEditable::new(choice)),
            ItemFrozen::File(file) => ItemEditable::File(FileEditable::new(file)),
            ItemFrozen::Map(map) => ItemEditable::Map(MapEditable::new(map)),
            ItemFrozen::Number(number) => ItemEditable::Number(NumberEditable::new(number)),
            ItemFrozen::String(string) => ItemEditable::String(StringEditable::new(string)),
            ItemFrozen::Table(table) => ItemEditable::Table(TableEditable::new(table)),
        }
    }

    /// Converts the current `ItemEditable` instance into an `ItemFrozen` instance.
    pub fn freeze(&self) -> ItemFrozen {
        match self {
            ItemEditable::Boolean(boolean) => ItemFrozen::Boolean(boolean.freeze()),
            ItemEditable::Choice(choice) => ItemFrozen::Choice(choice.freeze()),
            ItemEditable::File(file) => ItemFrozen::File(file.freeze()),
            ItemEditable::Map(map) => ItemFrozen::Map(map.freeze()),
            ItemEditable::Number(number) => ItemFrozen::Number(number.freeze()),
            ItemEditable::String(string) => ItemFrozen::String(string.freeze()),
            ItemEditable::Table(table) => ItemFrozen::Table(table.freeze()),
        }
    }

    /// Returns the parameter definition.
    pub fn definition(&self) -> ItemDefinitionType {
        match self {
            ItemEditable::Boolean(b) => ItemDefinitionType::Boolean(b.definition().clone()),
            ItemEditable::Choice(c) => ItemDefinitionType::Choice(c.definition().clone()),
            ItemEditable::File(f) => ItemDefinitionType::File(f.definition().clone()),
            ItemEditable::Map(m) => ItemDefinitionType::Map(m.definition().clone()),
            ItemEditable::Number(n) => ItemDefinitionType::Number(n.definition().clone()),
            ItemEditable::String(b) => ItemDefinitionType::String(b.definition().clone()),
            ItemEditable::Table(t) => ItemDefinitionType::Table(t.definition().clone()),
        }
    }

    /// Returns the choice value if this parameter is a choice parameter.
    pub fn get_choice(&self) -> Option<ChoiceEditable> {
        match self {
            Self::Choice(c) => Some(c.clone()),
            _ => None,
        }
    }

    /// Returns the file value if this parameter is a file parameter.
    pub fn get_file(&self) -> Option<&FileEditable> {
        match self {
            Self::File(f) => Some(f),
            _ => None,
        }
    }

    /// Returns the map value if this parameter is a map parameter.
    pub fn get_map(&self) -> Option<&MapEditable> {
        match self {
            Self::Map(m) => Some(m),
            _ => None,
        }
    }

    /// Returns the number value if this parameter is a number parameter.
    pub fn get_number(&self) -> Option<&NumberEditable> {
        match self {
            Self::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Returns the string value if this parameter is a string parameter.
    pub fn get_string(&self) -> Option<&StringEditable> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the table value if this parameter is a table parameter.
    pub fn get_table(&self) -> Option<&TableEditable> {
        match self {
            Self::Table(t) => Some(t),
            _ => None,
        }
    }
}

impl TreePrint for ItemEditable {
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
            Self::Map(map) => map.tree_print(f, label, prefix, last),
            Self::Number(number) => number.tree_print(f, label, prefix, last),
            Self::String(basic) => basic.tree_print(f, label, prefix, last),
            Self::Table(table) => table.tree_print(f, label, prefix, last),
        }
    }
}
