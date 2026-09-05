use crate::compile_time::compile_time_common::assert_unique_keys;
use crate::compile_time::{
    BooleanCompileTime, ChoiceCompileTime, FileCompileTime, IntegerCompileTime, NumberCompileTime,
    NumberWithUnitsCompileTime, StringCompileTime, TableCompileTime, TableWithUnitsCompileTime,
    UnitCompileTime,
};
use crate::definition::{MapDefinition, MapItemDefinition};
use keys::store_key::ConstStoreKey;

/// Compile-time representation of a map item. Use the `const_map_item!` macro to
/// construct values; Rust enum variants remain public for matching.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapItemCompileTime {
    /// Boolean map item variant.
    Boolean(BooleanCompileTime),
    /// Choice map item variant.
    Choice(ChoiceCompileTime),
    /// File map item variant.
    File(FileCompileTime),
    /// Integer map item variant.
    Integer(IntegerCompileTime),
    /// Number map item variant.
    Number(NumberCompileTime),
    /// Number-with-units map item variant.
    NumberWithUnits(NumberWithUnitsCompileTime),
    /// String map item variant.
    String(StringCompileTime),
    /// Table map item variant.
    Table(TableCompileTime),
    /// Table-with-units map item variant.
    TableWithUnits(TableWithUnitsCompileTime),
    /// Unit map item variant.
    Unit(UnitCompileTime),
}

/// Helper macro for converting compile-time map item types into `MapItemCompileTime`.
macro_rules! map_item_from {
    ($type:ty, $variant:ident) => {
        impl From<$type> for MapItemCompileTime {
            fn from(value: $type) -> Self {
                Self::$variant(value)
            }
        }
    };
}
map_item_from!(BooleanCompileTime, Boolean);
map_item_from!(ChoiceCompileTime, Choice);
map_item_from!(FileCompileTime, File);
map_item_from!(IntegerCompileTime, Integer);
map_item_from!(NumberCompileTime, Number);
map_item_from!(NumberWithUnitsCompileTime, NumberWithUnits);
map_item_from!(StringCompileTime, String);
map_item_from!(TableCompileTime, Table);
map_item_from!(TableWithUnitsCompileTime, TableWithUnits);
map_item_from!(UnitCompileTime, Unit);

impl MapItemCompileTime {
    /// Hidden wrapper for the `const_map_item!(boolean = value)` arm.
    ///
    /// This is an implementation detail; call `const_map_item!` instead.
    /// Wraps a [`BooleanCompileTime`] (a `true`/`false` toggle) as a `MapItemCompileTime`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __boolean(value: BooleanCompileTime) -> Self {
        Self::Boolean(value)
    }

    /// Hidden wrapper for the `const_map_item!(choice = value)` arm.
    ///
    /// This is an implementation detail; call `const_map_item!` instead.
    /// Wraps a [`ChoiceCompileTime`] (a single-select value chosen from a fixed list) as a
    /// `MapItemCompileTime`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __choice(value: ChoiceCompileTime) -> Self {
        Self::Choice(value)
    }

    /// Hidden wrapper for the `const_map_item!(file = value)` arm.
    ///
    /// This is an implementation detail; call `const_map_item!` instead.
    /// Wraps a [`FileCompileTime`] (a file-picker parameter) as a `MapItemCompileTime`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __file(value: FileCompileTime) -> Self {
        Self::File(value)
    }

    /// Hidden wrapper for the `const_map_item!(integer = value)` arm.
    ///
    /// This is an implementation detail; call `const_map_item!` instead.
    /// Wraps an [`IntegerCompileTime`] (an integer value with an optional constraint) as a
    /// `MapItemCompileTime`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __integer(value: IntegerCompileTime) -> Self {
        Self::Integer(value)
    }

    /// Hidden wrapper for the `const_map_item!(number = value)` arm.
    ///
    /// This is an implementation detail; call `const_map_item!` instead.
    /// Wraps a [`NumberCompileTime`] (an `f64` value with an optional constraint) as a
    /// `MapItemCompileTime`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __number(value: NumberCompileTime) -> Self {
        Self::Number(value)
    }

    /// Hidden wrapper for the `const_map_item!(number_with_units = value)` arm.
    ///
    /// This is an implementation detail; call `const_map_item!` instead.
    /// Wraps a [`NumberWithUnitsCompileTime`] (an `f64` value with a preferred unit and an
    /// optional constraint) as a `MapItemCompileTime`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __number_with_units(value: NumberWithUnitsCompileTime) -> Self {
        Self::NumberWithUnits(value)
    }

    /// Hidden wrapper for the `const_map_item!(string = value)` arm.
    ///
    /// This is an implementation detail; call `const_map_item!` instead.
    /// Wraps a [`StringCompileTime`] (a free-form text value) as a `MapItemCompileTime`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __string(value: StringCompileTime) -> Self {
        Self::String(value)
    }

    /// Hidden wrapper for the `const_map_item!(table = value)` arm.
    ///
    /// This is an implementation detail; call `const_map_item!` instead.
    /// Wraps a [`TableCompileTime`] (a table of unit-less numeric columns) as a
    /// `MapItemCompileTime`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __table(value: TableCompileTime) -> Self {
        Self::Table(value)
    }

    /// Hidden wrapper for the `const_map_item!(table_with_units = value)` arm.
    ///
    /// This is an implementation detail; call `const_map_item!` instead.
    /// Wraps a [`TableWithUnitsCompileTime`] (a table of numeric columns, each with its own
    /// preferred unit) as a `MapItemCompileTime`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __table_with_units(value: TableWithUnitsCompileTime) -> Self {
        Self::TableWithUnits(value)
    }

    /// Hidden wrapper for the `const_map_item!(unit = value)` arm.
    ///
    /// This is an implementation detail; call `const_map_item!` instead.
    /// Wraps a [`UnitCompileTime`] (a value chosen from the units of a unit family) as a
    /// `MapItemCompileTime`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __unit(value: UnitCompileTime) -> Self {
        Self::Unit(value)
    }

    /// Converts this compile-time map item into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> MapItemDefinition {
        match self {
            MapItemCompileTime::Boolean(value) => {
                MapItemDefinition::Boolean(value.into_definition())
            }
            MapItemCompileTime::Choice(value) => MapItemDefinition::Choice(value.into_definition()),
            MapItemCompileTime::File(value) => MapItemDefinition::File(value.into_definition()),
            MapItemCompileTime::Integer(value) => {
                MapItemDefinition::Integer(value.into_definition())
            }
            MapItemCompileTime::Number(value) => MapItemDefinition::Number(value.into_definition()),
            MapItemCompileTime::NumberWithUnits(value) => {
                MapItemDefinition::NumberWithUnits(value.into_definition())
            }
            MapItemCompileTime::String(value) => MapItemDefinition::String(value.into_definition()),
            MapItemCompileTime::Table(value) => MapItemDefinition::Table(value.into_definition()),
            MapItemCompileTime::TableWithUnits(value) => {
                MapItemDefinition::TableWithUnits(value.into_definition())
            }
            MapItemCompileTime::Unit(value) => MapItemDefinition::Unit(value.into_definition()),
        }
    }
}

/// Compile-time representation of a map.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Keyed items contained in this compile-time container.
    items: &'static [(ConstStoreKey, MapItemCompileTime)],
}

impl MapCompileTime {
    /// Hidden backing constructor for `const_map!(description, items)`.
    ///
    /// This is an implementation detail; call `const_map!` instead.
    /// `description` names the map and `items` is the ordered slice of
    /// `(ConstStoreKey, MapItemCompileTime)` key/item pairs, typically built with the
    /// `store_key!` macro and `const_map_item!`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(
        description: &'static str,
        items: &'static [(ConstStoreKey, MapItemCompileTime)],
    ) -> Self {
        assert_unique_keys!(items, "MapCompileTime item keys must be unique");
        Self { description, items }
    }

    /// Returns the description.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Returns the keyed items.
    #[must_use]
    pub const fn items(&self) -> &'static [(ConstStoreKey, MapItemCompileTime)] {
        self.items
    }

    /// Returns the number of entries.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the given key is present.
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Returns the value associated with the given key.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&MapItemCompileTime> {
        self.items
            .iter()
            .find_map(|(item_key, item)| (item_key.as_str() == key).then_some(item))
    }

    /// Returns an iterator over the keys.
    pub fn keys(&self) -> impl Iterator<Item = ConstStoreKey> + '_ {
        self.items.iter().map(|(key, _)| *key)
    }

    /// Returns an iterator over the entries.
    pub fn iter(&self) -> impl Iterator<Item = &(ConstStoreKey, MapItemCompileTime)> + '_ {
        self.items.iter()
    }

    /// Converts this compile-time map into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> MapDefinition {
        MapDefinition::new(
            self.description,
            self.items
                .iter()
                .map(|(key, item)| (*key, item.into_definition()))
                .collect(),
        )
    }
}

/// Wraps a compile-time value as a [`MapItemCompileTime`] for use inside
/// `const_map!` entry lists.
///
/// Maps only accept leaf value kinds; the container and layout-only kinds available to
/// `const_item!` (`folder`, `map`, `tab`, `separator`) are not accepted here.
///
/// Expansion is wrapped in a `const` block, so `value` must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// const_map_item!(boolean = value)
/// const_map_item!(choice = value)
/// const_map_item!(file = value)
/// const_map_item!(integer = value)
/// const_map_item!(number = value)
/// const_map_item!(number_with_units = value)
/// const_map_item!(string = value)
/// const_map_item!(table = value)
/// const_map_item!(table_with_units = value)
/// const_map_item!(unit = value)
/// ```
///
/// # Arguments
/// Each arm takes a single `value` of the matching compile-time type:
/// - `boolean`: [`BooleanCompileTime`] — a `true`/`false` toggle.
/// - `choice`: [`ChoiceCompileTime`] — a single-select value chosen from a fixed list.
/// - `file`: [`FileCompileTime`] — a file-picker parameter.
/// - `integer`: [`IntegerCompileTime`] — an integer value with an optional constraint.
/// - `number`: [`NumberCompileTime`] — an `f64` value with an optional constraint.
/// - `number_with_units`: [`NumberWithUnitsCompileTime`] — an `f64` value with a preferred
///   unit and an optional constraint.
/// - `string`: [`StringCompileTime`] — a free-form text value.
/// - `table`: [`TableCompileTime`] — a table of unit-less numeric columns.
/// - `table_with_units`: [`TableWithUnitsCompileTime`] — a table of numeric columns, each
///   with its own preferred unit.
/// - `unit`: [`UnitCompileTime`] — a value chosen from the units of a unit family.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::MapItemCompileTime;
/// use datastore::prelude::*;
///
/// const NAME: MapItemCompileTime = const_map_item!(string = const_string!("Name"));
/// let _definition = NAME.into_definition();
/// ```
#[macro_export]
macro_rules! const_map_item {
    (boolean = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::MapItemCompileTime::__boolean($value)
        }
    };
    (choice = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::MapItemCompileTime::__choice($value)
        }
    };
    (file = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::MapItemCompileTime::__file($value)
        }
    };
    (integer = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::MapItemCompileTime::__integer($value)
        }
    };
    (number = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::MapItemCompileTime::__number($value)
        }
    };
    (number_with_units = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::MapItemCompileTime::__number_with_units($value)
        }
    };
    (string = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::MapItemCompileTime::__string($value)
        }
    };
    (table = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::MapItemCompileTime::__table($value)
        }
    };
    (table_with_units = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::MapItemCompileTime::__table_with_units($value)
        }
    };
    (unit = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::MapItemCompileTime::__unit($value)
        }
    };
}

/// Creates a [`MapCompileTime`], the compile-time metadata for a dynamically keyed
/// collection of [`MapItemCompileTime`] entries.
///
/// Declaration order of `items` is preserved by [`MapCompileTime::keys`],
/// [`MapCompileTime::iter`], and [`MapCompileTime::into_definition`].
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// const_map!(description, items)
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the map.
/// - `items`: `&'static [(ConstStoreKey, MapItemCompileTime)]` ordered slice of key/item
///   pairs, typically built with the `store_key!` macro and `const_map_item!`.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::{MapCompileTime, MapItemCompileTime};
/// use datastore::prelude::*;
///
/// const SHAPE_ITEMS: &[(ConstStoreKey, MapItemCompileTime)] = &[
///     (
///         store_key!("name"),
///         const_map_item!(string = const_string!("Name")),
///     ),
///     (
///         store_key!("width"),
///         const_map_item!(number = const_number!("Width", default = "10")),
///     ),
/// ];
/// const SHAPES: MapCompileTime = const_map!("Shapes", SHAPE_ITEMS);
/// assert_eq!(SHAPES.count(), 2);
///
/// let _definition = SHAPES.into_definition();
/// ```
#[macro_export]
macro_rules! const_map {
    ($description:expr, $items:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::MapCompileTime::__new($description, $items)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{const_string, store_key};

    #[test]
    #[should_panic(expected = "MapCompileTime item keys must be unique")]
    fn map_compile_time_rejects_duplicate_keys() {
        const DUPLICATES: &[(ConstStoreKey, MapItemCompileTime)] = &[
            (
                store_key!("duplicate"),
                const_map_item!(string = const_string!("First")),
            ),
            (
                store_key!("duplicate"),
                const_map_item!(string = const_string!("Second")),
            ),
        ];
        #[allow(clippy::disallowed_methods)]
        let _ = MapCompileTime::__new(std::hint::black_box("Duplicates"), DUPLICATES);
    }
}
