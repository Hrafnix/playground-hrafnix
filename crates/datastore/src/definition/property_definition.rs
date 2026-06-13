use crate::definition::{BasicDefinition, MapDefinition, StructDefinition, TableDefinition};
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::sync::Arc;

/// The type of property definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyDefinitionType {
    /// A basic property (String, Number, etc.).
    Basic(BasicDefinition),
    /// A structured property.
    Struct(StructDefinition),
    /// A table property.
    Table(TableDefinition),
    /// A map property.
    Map(MapDefinition),
}

impl From<BasicDefinition> for PropertyDefinitionType {
    fn from(definition: BasicDefinition) -> Self {
        PropertyDefinitionType::Basic(definition)
    }
}

impl From<StructDefinition> for PropertyDefinitionType {
    fn from(definition: StructDefinition) -> Self {
        PropertyDefinitionType::Struct(definition)
    }
}

impl From<TableDefinition> for PropertyDefinitionType {
    fn from(definition: TableDefinition) -> Self {
        PropertyDefinitionType::Table(definition)
    }
}

impl From<MapDefinition> for PropertyDefinitionType {
    fn from(definition: MapDefinition) -> Self {
        PropertyDefinitionType::Map(definition)
    }
}

impl PropertyDefinitionType {
    /// Returns a new `PropertyDefinitionType` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        match self {
            Self::Basic(def) => Self::Basic(def.launder(store)),
            Self::Map(def) => Self::Map(def.launder(store)),
            Self::Struct(def) => Self::Struct(def.launder(store)),
            Self::Table(def) => Self::Table(def.launder(store)),
        }
    }
}

impl PartialEq<&PropertyDefinitionType> for PropertyDefinitionType {
    fn eq(&self, other: &&PropertyDefinitionType) -> bool {
        self == *other
    }
}

impl PartialEq<PropertyDefinitionType> for &PropertyDefinitionType {
    fn eq(&self, other: &PropertyDefinitionType) -> bool {
        *self == other
    }
}

/// Definition for a property, including its type and metadata like description and visibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyDefinition {
    description: ShareableString,
    item_type: Arc<PropertyDefinitionType>,
    gui_visibility: bool,
}

impl PropertyDefinition {
    /// Creates a new `PropertyDefinition` with a description and type.
    pub fn new<S: Into<ShareableString>, P: Into<PropertyDefinitionType>>(
        description: S,
        item_type: P,
    ) -> Self {
        Self {
            description: description.into(),
            item_type: Arc::new(item_type.into()),
            gui_visibility: true,
        }
    }

    /// Creates a new `PropertyDefinition` that is invisible in the GUI.
    pub fn new_gui_invisible<S: Into<ShareableString>, P: Into<PropertyDefinitionType>>(
        description: S,
        item_type: P,
    ) -> Self {
        Self {
            description: description.into(),
            item_type: Arc::new(item_type.into()),
            gui_visibility: false,
        }
    }

    /// Returns the description of the property.
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the type definition.
    pub fn item_type(&self) -> &PropertyDefinitionType {
        self.item_type.as_ref()
    }

    /// Returns whether the property is visible in the GUI.
    pub fn is_gui_visible(&self) -> bool {
        self.gui_visibility
    }

    /// Returns a reference to the description.
    pub fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns a new `PropertyDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            item_type: Arc::new(self.item_type.launder(store)),
            gui_visibility: self.gui_visibility,
        }
    }
}

impl PartialEq<&PropertyDefinition> for PropertyDefinition {
    fn eq(&self, other: &&PropertyDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<PropertyDefinition> for &PropertyDefinition {
    fn eq(&self, other: &PropertyDefinition) -> bool {
        *self == other
    }
}
