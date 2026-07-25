use crate::definition::{
    BooleanDefinition, ChoiceDefinition, MapDefinition, NumberDefinition, StringDefinition,
    TableDefinition,
};
use crate::prelude::FileDefinition;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::SharedStringStore;

/// The type of item definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemDefinitionType {
    /// A boolean item.
    Boolean(BooleanDefinition),
    /// A choice item.
    Choice(ChoiceDefinition),
    /// A file item.
    File(FileDefinition),
    /// A map item.
    Map(MapDefinition),
    /// A number item.
    Number(NumberDefinition),
    /// A string item.
    String(StringDefinition),
    /// A table item.
    Table(TableDefinition),
}

impl From<StringDefinition> for ItemDefinitionType {
    fn from(definition: StringDefinition) -> Self {
        ItemDefinitionType::String(definition)
    }
}

impl From<BooleanDefinition> for ItemDefinitionType {
    fn from(definition: BooleanDefinition) -> Self {
        ItemDefinitionType::Boolean(definition)
    }
}

impl From<ChoiceDefinition> for ItemDefinitionType {
    fn from(definition: ChoiceDefinition) -> Self {
        ItemDefinitionType::Choice(definition)
    }
}

impl From<FileDefinition> for ItemDefinitionType {
    fn from(definition: FileDefinition) -> Self {
        ItemDefinitionType::File(definition)
    }
}

impl From<MapDefinition> for ItemDefinitionType {
    fn from(definition: MapDefinition) -> Self {
        ItemDefinitionType::Map(definition)
    }
}

impl From<NumberDefinition> for ItemDefinitionType {
    fn from(definition: NumberDefinition) -> Self {
        ItemDefinitionType::Number(definition)
    }
}

impl From<TableDefinition> for ItemDefinitionType {
    fn from(definition: TableDefinition) -> Self {
        ItemDefinitionType::Table(definition)
    }
}

impl ItemDefinitionType {
    /// Returns a new `ItemDefinitionType` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        match self {
            Self::Boolean(def) => Self::Boolean(def.launder(store)),
            Self::File(def) => Self::File(def.launder(store)),
            Self::Choice(def) => Self::Choice(def.launder(store)),
            Self::Map(def) => Self::Map(def.launder(store)),
            Self::Number(def) => Self::Number(def.launder(store)),
            Self::String(def) => Self::String(def.launder(store)),
            Self::Table(def) => Self::Table(def.launder(store)),
        }
    }
}

impl PartialEq<&ItemDefinitionType> for ItemDefinitionType {
    fn eq(&self, other: &&ItemDefinitionType) -> bool {
        self == *other
    }
}

impl PartialEq<ItemDefinitionType> for &ItemDefinitionType {
    fn eq(&self, other: &ItemDefinitionType) -> bool {
        *self == other
    }
}

impl TreePrint for ItemDefinitionType {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        match self {
            Self::Boolean(boolean) => boolean.tree_print(f, label, prefix, last),
            Self::String(basic) => basic.tree_print(f, label, prefix, last),
            Self::File(file) => file.tree_print(f, label, prefix, last),
            Self::Choice(choice) => choice.tree_print(f, label, prefix, last),
            Self::Number(number) => number.tree_print(f, label, prefix, last),
            Self::Table(table) => table.tree_print(f, label, prefix, last),
            Self::Map(map) => map.tree_print(f, label, prefix, last),
        }
    }
}
