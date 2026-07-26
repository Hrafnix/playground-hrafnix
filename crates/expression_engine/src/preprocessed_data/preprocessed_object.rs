use crate::BasicDefinition;
use crate::preprocessed_data::preprocessed_basic::BasicPreprocessedData;
use crate::preprocessed_data::preprocessed_table::TablePreprocessedData;
use datastore::frozen::{
    GlobalObjectFrozen, ItemFrozen, MapItemFrozen, ParameterObjectFrozen, VariableObjectFrozen,
};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents a single item of preprocessed data within an object,
/// which can be either basic or table preprocessed data.
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectItemPreprocessedData {
    /// Basic preprocessed data item.
    Basic(BasicPreprocessedData),
    /// Table preprocessed data item.
    Table(TablePreprocessedData),
}

/// Converts a single `ItemFrozen` into one or more preprocessed data entries
/// and inserts them into `map`, keyed by `key`.
///
/// Most item kinds map to a single entry, but `Map` items are flattened:
/// each field of each entry becomes its own item, addressed by a
/// `key[entry].field` path.
fn item_to_preprocessed_data(
    map: &mut BTreeMap<ShareableString, ObjectItemPreprocessedData>,
    key: ShareableString,
    data: &ItemFrozen,
) {
    match data {
        ItemFrozen::Boolean(boolean) => {
            map.insert(
                key,
                ObjectItemPreprocessedData::Basic(BasicPreprocessedData::new(
                    BasicDefinition::Boolean(boolean.definition().clone()),
                    boolean.value(),
                )),
            );
        }
        ItemFrozen::Choice(choice) => {
            map.insert(
                key,
                ObjectItemPreprocessedData::Basic(BasicPreprocessedData::new(
                    BasicDefinition::Choice(choice.definition().clone()),
                    choice.value(),
                )),
            );
        }
        ItemFrozen::File(file) => {
            map.insert(
                key,
                ObjectItemPreprocessedData::Basic(BasicPreprocessedData::new(
                    BasicDefinition::File(file.definition().clone()),
                    file.value(),
                )),
            );
        }
        ItemFrozen::Map(item_map) => {
            // Maps are flattened: each field of each entry becomes its own
            // item, addressed by a `key[entry][field]` path.
            for (entry_key, entry) in item_map.iter() {
                for (item_key, map_item) in entry.iter() {
                    let path: ShareableString =
                        format!("{}[{}][{}]", key, entry_key, item_key).into();
                    let preprocessed_item = match map_item {
                        MapItemFrozen::Choice(choice) => {
                            ObjectItemPreprocessedData::Basic(BasicPreprocessedData::new(
                                BasicDefinition::Choice(choice.definition().clone()),
                                choice.value(),
                            ))
                        }
                        MapItemFrozen::File(file) => {
                            ObjectItemPreprocessedData::Basic(BasicPreprocessedData::new(
                                BasicDefinition::File(file.definition().clone()),
                                file.value(),
                            ))
                        }
                        MapItemFrozen::Number(number) => {
                            ObjectItemPreprocessedData::Basic(BasicPreprocessedData::new(
                                BasicDefinition::Number(number.definition().clone()),
                                number.value(),
                            ))
                        }
                        MapItemFrozen::String(string) => {
                            ObjectItemPreprocessedData::Basic(BasicPreprocessedData::new(
                                BasicDefinition::String(string.definition().clone()),
                                string.value(),
                            ))
                        }
                        MapItemFrozen::Table(table) => {
                            ObjectItemPreprocessedData::Table(TablePreprocessedData::new(
                                table.definition().clone(),
                                table.rows().to_vec(),
                            ))
                        }
                    };
                    map.insert(path, preprocessed_item);
                }
            }
        }
        ItemFrozen::Number(number) => {
            map.insert(
                key,
                ObjectItemPreprocessedData::Basic(BasicPreprocessedData::new(
                    BasicDefinition::Number(number.definition().clone()),
                    number.value(),
                )),
            );
        }
        ItemFrozen::String(string) => {
            map.insert(
                key,
                ObjectItemPreprocessedData::Basic(BasicPreprocessedData::new(
                    BasicDefinition::String(string.definition().clone()),
                    string.value(),
                )),
            );
        }
        ItemFrozen::Table(table) => {
            map.insert(
                key,
                ObjectItemPreprocessedData::Table(TablePreprocessedData::new(
                    table.definition().clone(),
                    table.rows().to_vec(),
                )),
            );
        }
    }
}

/// Represents preprocessed data for an object, mapping field names
/// to their corresponding preprocessed data items.
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalObjectPreprocessedData {
    data: BTreeMap<ShareableString, ObjectItemPreprocessedData>,
}

impl GlobalObjectPreprocessedData {
    /// Creates a new `GlobalObjectPreprocessedData` instance from the given `GlobalObjectFrozen`.
    pub fn new(frozen_data: GlobalObjectFrozen) -> Self {
        let mut data = BTreeMap::new();
        for (key, item) in frozen_data.iter() {
            item_to_preprocessed_data(&mut data, key.into(), item);
        }
        Self { data }
    }

    /// Returns a reference to the underlying `ObjectPreprocessedData`.
    pub fn data(&self) -> &BTreeMap<ShareableString, ObjectItemPreprocessedData> {
        &self.data
    }
}

/// Represents preprocessed data for an object, mapping field names
/// to their corresponding preprocessed data items.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterObjectPreprocessedData {
    data: BTreeMap<ShareableString, ObjectItemPreprocessedData>,
}

impl ParameterObjectPreprocessedData {
    /// Creates a new `ParameterObjectPreprocessedData` instance from the given `ParameterObjectFrozen`.
    pub fn new(frozen_data: ParameterObjectFrozen) -> Self {
        let mut data = BTreeMap::new();
        for (key, item) in frozen_data.iter() {
            item_to_preprocessed_data(&mut data, key.into(), item);
        }
        Self { data }
    }

    /// Returns a reference to the underlying `ObjectPreprocessedData`.
    pub fn data(&self) -> &BTreeMap<ShareableString, ObjectItemPreprocessedData> {
        &self.data
    }
}

/// Represents preprocessed data for an object, mapping field names
/// to their corresponding preprocessed data items.
#[derive(Debug, Clone, PartialEq)]
pub struct VariableObjectPreprocessedData {
    data: BTreeMap<ShareableString, ObjectItemPreprocessedData>,
}

impl VariableObjectPreprocessedData {
    /// Creates a new `VariableObjectPreprocessedData` instance from the given `VariableObjectFrozen`.
    pub fn new(frozen_data: VariableObjectFrozen) -> Self {
        let mut data = BTreeMap::new();
        for (key, item) in frozen_data.iter() {
            item_to_preprocessed_data(&mut data, key.into(), item);
        }
        Self { data }
    }

    /// Returns a reference to the underlying `ObjectPreprocessedData`.
    pub fn data(&self) -> &BTreeMap<ShareableString, ObjectItemPreprocessedData> {
        &self.data
    }
}
