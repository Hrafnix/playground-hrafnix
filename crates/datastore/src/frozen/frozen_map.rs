use crate::definition::{MapDefinition, MapItemDefault, MapItemDefinition};
use crate::editable::{MapEditable, MapEntryEditable, MapItemEditable};
use crate::frozen::{
    BooleanFrozen, ChoiceFrozen, FileFrozen, IntegerFrozen, NumberFrozen, NumberWithUnitsFrozen,
    StringFrozen, TableFrozen, TableWithUnitsFrozen, UnitFrozen,
};
use crate::traits::TreePrint;
use keys::store_key::StoreKey;
use message::message::{Message, MessageCategory};
use shareable_string::{ShareableString, SharedStringStore};
use std::collections::BTreeMap;

/// Represents an item within a frozen map entry.
#[derive(Debug, Clone, PartialEq)]
pub enum MapItemFrozen {
    /// A boolean value.
    Boolean(BooleanFrozen),
    /// A choice value.
    Choice(ChoiceFrozen),
    /// A file value.
    File(FileFrozen),
    /// An integer value.
    Integer(IntegerFrozen),
    /// A number value.
    Number(NumberFrozen),
    /// A number value with associated units.
    NumberWithUnits(NumberWithUnitsFrozen),
    /// A string value.
    String(StringFrozen),
    /// A table value.
    Table(TableFrozen),
    /// A table value with associated units.
    TableWithUnits(TableWithUnitsFrozen),
    /// A unit value.
    Unit(UnitFrozen),
}

impl MapItemFrozen {
    /// Creates a map item initialized from its definition's default value.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new(definition: &MapItemDefinition) -> Self {
        match definition {
            MapItemDefinition::Boolean(value) => Self::Boolean(BooleanFrozen::new(value.clone())),
            MapItemDefinition::Choice(value) => Self::Choice(ChoiceFrozen::new(value.clone())),
            MapItemDefinition::File(value) => Self::File(FileFrozen::new(value.clone())),
            MapItemDefinition::Integer(value) => Self::Integer(IntegerFrozen::new(value.clone())),
            MapItemDefinition::Number(value) => Self::Number(NumberFrozen::new(value.clone())),
            MapItemDefinition::NumberWithUnits(value) => {
                Self::NumberWithUnits(NumberWithUnitsFrozen::new(value.clone()))
            }
            MapItemDefinition::String(value) => Self::String(StringFrozen::new(value.clone())),
            MapItemDefinition::Table(value) => Self::Table(TableFrozen::new(value.clone())),
            MapItemDefinition::TableWithUnits(value) => {
                Self::TableWithUnits(TableWithUnitsFrozen::new(value.clone()))
            }
            MapItemDefinition::Unit(value) => Self::Unit(UnitFrozen::new(value.clone())),
        }
    }

    /// Creates a map item initialized from an entry-specific default value.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn new_with_default(definition: &MapItemDefinition, default: &MapItemDefault) -> Self {
        match (definition, default) {
            (MapItemDefinition::Boolean(definition), MapItemDefault::Scalar(value)) => {
                Self::Boolean(BooleanFrozen::new_with_value(
                    definition.clone(),
                    value.clone(),
                ))
            }
            (MapItemDefinition::Choice(definition), MapItemDefault::Scalar(value)) => Self::Choice(
                ChoiceFrozen::new_with_value(definition.clone(), value.clone()),
            ),
            (MapItemDefinition::File(definition), MapItemDefault::Scalar(value)) => Self::File(
                FileFrozen::new_with_value(definition.clone(), value.clone()),
            ),
            (MapItemDefinition::Integer(definition), MapItemDefault::Scalar(value)) => {
                Self::Integer(IntegerFrozen::new_with_value(
                    definition.clone(),
                    value.clone(),
                ))
            }
            (MapItemDefinition::Number(definition), MapItemDefault::Scalar(value)) => Self::Number(
                NumberFrozen::new_with_value(definition.clone(), value.clone()),
            ),
            (MapItemDefinition::NumberWithUnits(definition), MapItemDefault::Scalar(value)) => {
                Self::NumberWithUnits(NumberWithUnitsFrozen::new_with_value(
                    definition.clone(),
                    value.clone(),
                    definition.preferred_units().string_id().into(),
                ))
            }
            (MapItemDefinition::String(definition), MapItemDefault::Scalar(value)) => Self::String(
                StringFrozen::new_with_value(definition.clone(), value.clone()),
            ),
            (MapItemDefinition::Table(definition), MapItemDefault::Table(rows)) => {
                Self::Table(TableFrozen::new_from_rows(definition.clone(), rows.clone()))
            }
            (MapItemDefinition::TableWithUnits(definition), MapItemDefault::Table(rows)) => {
                let units = definition
                    .iter()
                    .map(|(_, column)| column.preferred_units().string_id().into())
                    .collect();
                Self::TableWithUnits(TableWithUnitsFrozen::new_from_rows(
                    definition.clone(),
                    rows.clone(),
                    units,
                ))
            }
            (MapItemDefinition::Unit(definition), MapItemDefault::Scalar(value)) => Self::Unit(
                UnitFrozen::new_with_value(definition.clone(), value.clone()),
            ),
            _ => Self::new(definition),
        }
    }

    /// Returns a copy whose strings are interned in `store`.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        match self {
            Self::Boolean(value) => Self::Boolean(value.launder(store)),
            Self::Choice(value) => Self::Choice(value.launder(store)),
            Self::File(value) => Self::File(value.launder(store)),
            Self::Integer(value) => Self::Integer(value.launder(store)),
            Self::Number(value) => Self::Number(value.launder(store)),
            Self::NumberWithUnits(value) => Self::NumberWithUnits(value.launder(store)),
            Self::String(value) => Self::String(value.launder(store)),
            Self::Table(value) => Self::Table(value.launder(store)),
            Self::TableWithUnits(value) => Self::TableWithUnits(value.launder(store)),
            Self::Unit(value) => Self::Unit(value.launder(store)),
        }
    }

    /// Returns the string value if this item is a string value.
    #[must_use]
    pub const fn get_string(&self) -> Option<&StringFrozen> {
        match self {
            MapItemFrozen::String(string) => Some(string),
            MapItemFrozen::Boolean(_)
            | MapItemFrozen::Choice(_)
            | MapItemFrozen::File(_)
            | MapItemFrozen::Integer(_)
            | MapItemFrozen::Number(_)
            | MapItemFrozen::NumberWithUnits(_)
            | MapItemFrozen::Table(_)
            | MapItemFrozen::TableWithUnits(_)
            | MapItemFrozen::Unit(_) => None,
        }
    }

    /// Returns the table value if this item is a table value.
    #[must_use]
    pub const fn get_table(&self) -> Option<&TableFrozen> {
        match self {
            MapItemFrozen::Table(table) => Some(table),
            MapItemFrozen::Boolean(_)
            | MapItemFrozen::Choice(_)
            | MapItemFrozen::File(_)
            | MapItemFrozen::Integer(_)
            | MapItemFrozen::Number(_)
            | MapItemFrozen::NumberWithUnits(_)
            | MapItemFrozen::String(_)
            | MapItemFrozen::TableWithUnits(_)
            | MapItemFrozen::Unit(_) => None,
        }
    }

    /// Returns the unit value if this item is a unit value.
    #[must_use]
    pub const fn get_unit(&self) -> Option<&UnitFrozen> {
        match self {
            MapItemFrozen::Unit(unit) => Some(unit),
            MapItemFrozen::Boolean(_)
            | MapItemFrozen::Choice(_)
            | MapItemFrozen::File(_)
            | MapItemFrozen::Integer(_)
            | MapItemFrozen::Number(_)
            | MapItemFrozen::NumberWithUnits(_)
            | MapItemFrozen::String(_)
            | MapItemFrozen::Table(_)
            | MapItemFrozen::TableWithUnits(_) => None,
        }
    }

    /// Returns the map item definition.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn definition(&self) -> MapItemDefinition {
        match self {
            MapItemFrozen::Boolean(boolean) => {
                MapItemDefinition::Boolean(boolean.definition().clone())
            }
            MapItemFrozen::Choice(choice) => MapItemDefinition::Choice(choice.definition().clone()),
            MapItemFrozen::File(file) => MapItemDefinition::File(file.definition().clone()),
            MapItemFrozen::Integer(integer) => {
                MapItemDefinition::Integer(integer.definition().clone())
            }
            MapItemFrozen::Number(number) => MapItemDefinition::Number(number.definition().clone()),
            MapItemFrozen::NumberWithUnits(number_with_units) => {
                MapItemDefinition::NumberWithUnits(number_with_units.definition().clone())
            }
            MapItemFrozen::String(basic) => MapItemDefinition::String(basic.definition().clone()),
            MapItemFrozen::Table(table) => MapItemDefinition::Table(table.definition().clone()),
            MapItemFrozen::TableWithUnits(table_with_units) => {
                MapItemDefinition::TableWithUnits(table_with_units.definition().clone())
            }
            MapItemFrozen::Unit(unit) => MapItemDefinition::Unit(unit.definition().clone()),
        }
    }

    /// Returns the pre-calculated BLAKE3 hash of the item.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        match self {
            MapItemFrozen::Boolean(boolean) => boolean.hash(),
            MapItemFrozen::Choice(choice) => choice.hash(),
            MapItemFrozen::File(file) => file.hash(),
            MapItemFrozen::Integer(integer) => integer.hash(),
            MapItemFrozen::Number(number) => number.hash(),
            MapItemFrozen::NumberWithUnits(number_with_units) => number_with_units.hash(),
            MapItemFrozen::String(basic) => basic.hash(),
            MapItemFrozen::Table(table) => table.hash(),
            MapItemFrozen::TableWithUnits(table_with_units) => table_with_units.hash(),
            MapItemFrozen::Unit(unit) => unit.hash(),
        }
    }

    /// Creates a new `MapItemFrozen` instance from a given `MapItemEditable` value.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new_from_editable(item: &MapItemEditable) -> Self {
        match item {
            MapItemEditable::Boolean(boolean) => {
                MapItemFrozen::Boolean(BooleanFrozen::new_from_editable(boolean))
            }
            MapItemEditable::Choice(choice) => {
                MapItemFrozen::Choice(ChoiceFrozen::new_from_editable(choice))
            }
            MapItemEditable::File(file) => MapItemFrozen::File(FileFrozen::new_from_editable(file)),
            MapItemEditable::Integer(integer) => {
                MapItemFrozen::Integer(IntegerFrozen::new_from_editable(integer))
            }
            MapItemEditable::Number(number) => {
                MapItemFrozen::Number(NumberFrozen::new_from_editable(number))
            }
            MapItemEditable::NumberWithUnits(number_with_units) => MapItemFrozen::NumberWithUnits(
                NumberWithUnitsFrozen::new_from_editable(number_with_units),
            ),
            MapItemEditable::String(basic) => {
                MapItemFrozen::String(StringFrozen::new_from_editable(basic))
            }
            MapItemEditable::Table(table) => {
                MapItemFrozen::Table(TableFrozen::new_from_editable(table))
            }
            MapItemEditable::TableWithUnits(table_with_units) => MapItemFrozen::TableWithUnits(
                TableWithUnitsFrozen::new_from_editable(table_with_units),
            ),
            MapItemEditable::Unit(unit) => MapItemFrozen::Unit(UnitFrozen::new_from_editable(unit)),
        }
    }

    /// Converts the current `MapItemFrozen` instance into a `MapItemEditable` instance.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn thaw(&self) -> MapItemEditable {
        MapItemEditable::new(self)
    }
}

impl PartialEq<&MapItemFrozen> for MapItemFrozen {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&MapItemFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<MapItemFrozen> for &MapItemFrozen {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &MapItemFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for MapItemFrozen {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        match self {
            MapItemFrozen::Boolean(boolean) => boolean.tree_print(f, label, prefix, last),
            MapItemFrozen::Choice(choice) => choice.tree_print(f, label, prefix, last),
            MapItemFrozen::File(file) => file.tree_print(f, label, prefix, last),
            MapItemFrozen::Integer(integer) => integer.tree_print(f, label, prefix, last),
            MapItemFrozen::Number(number) => number.tree_print(f, label, prefix, last),
            MapItemFrozen::NumberWithUnits(number_with_units) => {
                number_with_units.tree_print(f, label, prefix, last)
            }
            MapItemFrozen::String(basic) => basic.tree_print(f, label, prefix, last),
            MapItemFrozen::Table(table) => table.tree_print(f, label, prefix, last),
            MapItemFrozen::TableWithUnits(table_with_units) => {
                table_with_units.tree_print(f, label, prefix, last)
            }
            MapItemFrozen::Unit(unit) => unit.tree_print(f, label, prefix, last),
        }
    }
}

/// Represents a single entry's value within a frozen map, following the map's entry schema.
#[derive(Debug, Clone, PartialEq)]
pub struct MapEntryFrozen {
    /// The items in the map entry.
    items: BTreeMap<StoreKey, MapItemFrozen>,
    /// The pre-calculated BLAKE3 hash of the entry's content.
    hash: [u8; 32],
}

impl MapEntryFrozen {
    /// Creates a new `MapEntryFrozen` from the map's entry schema.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new(item_type: &BTreeMap<StoreKey, MapItemDefinition>) -> Self {
        Self::new_from_items(
            item_type
                .iter()
                .map(|(key, definition)| (key.clone(), MapItemFrozen::new(definition)))
                .collect(),
        )
    }

    /// Creates a new map entry with entry-specific defaults overlaid on schema defaults.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn new_with_defaults(definition: &MapDefinition, defaults: &[MapItemDefault]) -> Self {
        Self::new_from_items(
            definition
                .iter()
                .enumerate()
                .map(|(index, (key, item_definition))| {
                    let item = defaults.get(index).map_or_else(
                        || MapItemFrozen::new(item_definition),
                        |default| MapItemFrozen::new_with_default(item_definition, default),
                    );
                    (key.clone(), item)
                })
                .collect(),
        )
    }

    /// Creates a new `MapEntryFrozen` from a set of items.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new_from_items(items: BTreeMap<StoreKey, MapItemFrozen>) -> Self {
        let mut s = Self {
            items,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `MapEntryFrozen` from a given `MapEntryEditable` value.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new_from_editable(editable_entry: &MapEntryEditable) -> Self {
        let items = editable_entry
            .iter()
            .map(|(key, value)| (key.clone(), MapItemFrozen::new_from_editable(value)))
            .collect();
        Self::new_from_items(items)
    }

    /// Converts the current `MapEntryFrozen` instance into a `MapEntryEditable` instance.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn thaw(&self) -> MapEntryEditable {
        MapEntryEditable::new(self)
    }

    /// Returns a copy whose strings are interned in `store`.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self::new_from_items(
            self.items
                .iter()
                .map(|(key, value)| (key.launder(store), value.launder(store)))
                .collect(),
        )
    }

    /// Recomputes and stores the BLAKE3 hash of all items in this map entry.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        h.update(&[0x01]);
        h.update(b"MapEntry");

        h.update(
            &u64::try_from(self.items.len())
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );

        for (key, item) in &self.items {
            h.update(&key.current_blake3_hash());
            h.update(&item.hash());
        }

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the pre-calculated BLAKE3 hash of the entry.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns a reference to the item with the specified key if it exists.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&MapItemFrozen> {
        self.items.get(&key.into())
    }

    /// Return the string value if this item is a string value.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn get_string<S: Into<ShareableString>>(&self, key: S) -> Option<&StringFrozen> {
        if let Some(item) = self.get(key) {
            item.get_string()
        } else {
            None
        }
    }

    /// Return the table value if this item is a table value.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn get_table<S: Into<ShareableString>>(&self, key: S) -> Option<&TableFrozen> {
        if let Some(item) = self.get(key) {
            item.get_table()
        } else {
            None
        }
    }

    /// Return the unit value if this item is a unit value.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn get_unit<S: Into<ShareableString>>(&self, key: S) -> Option<&UnitFrozen> {
        self.get(key).and_then(MapItemFrozen::get_unit)
    }

    /// Returns an iterator over the key-item pairs in the entry.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &MapItemFrozen)> {
        self.items.iter()
    }

    /// Returns the schema of this entry, derived from its current items.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn definition(&self) -> BTreeMap<StoreKey, MapItemDefinition> {
        self.items
            .iter()
            .map(|(k, v)| (k.clone(), v.definition()))
            .collect()
    }
}

impl PartialEq<&MapEntryFrozen> for MapEntryFrozen {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&MapEntryFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<MapEntryFrozen> for &MapEntryFrozen {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &MapEntryFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for MapEntryFrozen {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(f, "{}{}{}", prefix, Self::branch_char(last), label)?;

        let child_prefix = Self::child_prefix(prefix, last);

        let mut item_iter = self.items.iter().peekable();

        while let Some((key, item)) = item_iter.next() {
            let is_last = item_iter.peek().is_none();
            item.tree_print(f, key.as_str(), &child_prefix, is_last)?;
        }

        Ok(())
    }
}

/// Represents a map of parameter in the frozen data.
#[derive(Debug, Clone, PartialEq)]
pub struct MapFrozen {
    /// The definition of the map.
    definition: MapDefinition,
    /// The items in the map.
    items: BTreeMap<StoreKey, MapEntryFrozen>,
    /// The pre-calculated BLAKE3 hash of the map's content.
    hash: [u8; 32],
}

impl MapFrozen {
    /// Creates a new `MapFrozen` with a definition.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new(definition: MapDefinition) -> Self {
        let items = definition
            .default_map()
            .map(|entries| {
                entries
                    .iter()
                    .map(|(key, defaults)| {
                        (
                            key.clone(),
                            MapEntryFrozen::new_with_defaults(&definition, defaults),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
        let mut s = Self {
            definition,
            items,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `MapFrozen` with a description and items.
    ///
    /// # Errors
    ///
    /// Returns an error message if the items do not all share the same entry schema
    /// or if `items` is empty.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new_from_items<S: Into<ShareableString>>(
        description: S,
        items: BTreeMap<StoreKey, MapEntryFrozen>,
    ) -> Result<Self, Message> {
        let item_schema = if let Some(first_item) = items.values().next() {
            let first_schema = first_item.definition();
            for item in items.values().skip(1) {
                let schema = item.definition();
                if first_schema != schema {
                    return Err(Message::error(
                        MessageCategory::Datastore,
                        "datastore_schema_mismatch",
                    ));
                }
            }
            first_schema
        } else {
            return Err(Message::error(
                MessageCategory::Datastore,
                "datastore_missing_schema",
            ));
        };

        let definition = MapDefinition::new(description, item_schema.into_iter().collect());
        let mut s = Self {
            definition,
            items,
            hash: [0u8; 32],
        };
        s.update_hash();
        Ok(s)
    }

    /// Creates a new `MapFrozen` from a given `MapEditable` value.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new_from_editable(editable_map: &MapEditable) -> Self {
        let definition = editable_map.definition().clone();
        let items = editable_map
            .iter()
            .map(|(key, value)| (key.clone(), MapEntryFrozen::new_from_editable(value)))
            .collect();
        let mut s = Self {
            definition,
            items,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Converts the current `MapFrozen` instance into a `MapEditable` instance.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn thaw(&self) -> MapEditable {
        MapEditable::new(self)
    }

    /// Returns a copy whose strings are interned in `store`.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        let mut map = Self {
            definition: self.definition.launder(store),
            items: self
                .items
                .iter()
                .map(|(key, value)| (key.launder(store), value.launder(store)))
                .collect(),
            hash: [0u8; 32],
        };
        map.update_hash();
        map
    }

    /// Recomputes and stores the BLAKE3 hash of all entries in this map.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        h.update(&[0x01]);
        h.update(b"Map");

        h.update(
            &u64::try_from(self.items.len())
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );

        for (key, item) in &self.items {
            h.update(&key.current_blake3_hash());
            h.update(&item.hash());
        }

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the pre-calculated BLAKE3 hash of the map.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns a reference to the item with the specified key if it exists.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&MapEntryFrozen> {
        self.items.get(&key.into())
    }

    /// Returns an iterator over the key-item pairs in the map.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &MapEntryFrozen)> {
        self.items.iter()
    }

    /// Returns a reference to the map definition.
    #[must_use]
    pub const fn definition(&self) -> &MapDefinition {
        &self.definition
    }

    /// Returns the number of items in the map.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn count(&self) -> usize {
        self.items.len()
    }
}

impl PartialEq<&MapFrozen> for MapFrozen {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&MapFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<MapFrozen> for &MapFrozen {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &MapFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for MapFrozen {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Map",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
        )?;

        let child_prefix = Self::child_prefix(prefix, last);

        let mut item_iter = self.items.iter().peekable();

        while let Some((key, item)) = item_iter.next() {
            let is_last = item_iter.peek().is_none();
            item.tree_print(f, key.as_str(), &child_prefix, is_last)?;
        }

        Ok(())
    }
}
