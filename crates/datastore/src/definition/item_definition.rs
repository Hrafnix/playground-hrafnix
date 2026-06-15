use crate::definition::{BasicDefinition, MapDefinition, StructDefinition, TableDefinition};
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::sync::Arc;

/// The type of item definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemDefinitionType {
    /// A basic item (String, Number, etc.).
    Basic(BasicDefinition),
    /// A structured item.
    Struct(StructDefinition),
    /// A table item.
    Table(TableDefinition),
    /// A map item.
    Map(MapDefinition),
}

impl From<BasicDefinition> for ItemDefinitionType {
    fn from(definition: BasicDefinition) -> Self {
        ItemDefinitionType::Basic(definition)
    }
}

impl From<StructDefinition> for ItemDefinitionType {
    fn from(definition: StructDefinition) -> Self {
        ItemDefinitionType::Struct(definition)
    }
}

impl From<TableDefinition> for ItemDefinitionType {
    fn from(definition: TableDefinition) -> Self {
        ItemDefinitionType::Table(definition)
    }
}

impl From<MapDefinition> for ItemDefinitionType {
    fn from(definition: MapDefinition) -> Self {
        ItemDefinitionType::Map(definition)
    }
}

impl ItemDefinitionType {
    /// Returns a new `ItemDefinitionType` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        match self {
            Self::Basic(def) => Self::Basic(def.launder(store)),
            Self::Map(def) => Self::Map(def.launder(store)),
            Self::Struct(def) => Self::Struct(def.launder(store)),
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

/// Definition for a item, including its type and metadata like description and visibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemDefinition {
    description: ShareableString,
    item_type: Arc<ItemDefinitionType>,
    gui_visibility: bool,
}

impl ItemDefinition {
    /// Creates a new `ItemDefinition` with a description and type.
    pub fn new<S: Into<ShareableString>, P: Into<ItemDefinitionType>>(
        description: S,
        item_type: P,
    ) -> Self {
        Self {
            description: description.into(),
            item_type: Arc::new(item_type.into()),
            gui_visibility: true,
        }
    }

    /// Creates a new `ItemDefinition` that is invisible in the GUI.
    pub fn new_gui_invisible<S: Into<ShareableString>, P: Into<ItemDefinitionType>>(
        description: S,
        item_type: P,
    ) -> Self {
        Self {
            description: description.into(),
            item_type: Arc::new(item_type.into()),
            gui_visibility: false,
        }
    }

    /// Returns the description of the item.
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the type definition.
    pub fn item_type(&self) -> &ItemDefinitionType {
        self.item_type.as_ref()
    }

    /// Returns whether the item is visible in the GUI.
    pub fn is_gui_visible(&self) -> bool {
        self.gui_visibility
    }

    /// Returns a reference to the description.
    pub fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns a new `ItemDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            item_type: Arc::new(self.item_type.launder(store)),
            gui_visibility: self.gui_visibility,
        }
    }
}

impl PartialEq<&ItemDefinition> for ItemDefinition {
    fn eq(&self, other: &&ItemDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<ItemDefinition> for &ItemDefinition {
    fn eq(&self, other: &ItemDefinition) -> bool {
        *self == other
    }
}
