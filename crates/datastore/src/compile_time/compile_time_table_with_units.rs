use crate::compile_time::NumberWithUnitsCompileTime;
use crate::compile_time::compile_time_common::assert_unique_keys;
use crate::definition::TableWithUnitsDefinition;
use keys::store_key::ConstStoreKey;

/// Compile-time representation of a table with units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TableWithUnitsCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Keyed columns contained in this compile-time table.
    columns: &'static [(ConstStoreKey, NumberWithUnitsCompileTime)],
}

impl TableWithUnitsCompileTime {
    /// Hidden backing constructor for `table_with_units_compile_time!(description, columns)`.
    ///
    /// This is an implementation detail; call `table_with_units_compile_time!` instead.
    /// `description` names the table and `columns` is the ordered slice of
    /// `(ConstStoreKey, NumberWithUnitsCompileTime)` column key/definition pairs, typically
    /// built with the `store_key!` macro and `number_with_units_compile_time!`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(
        description: &'static str,
        columns: &'static [(ConstStoreKey, NumberWithUnitsCompileTime)],
    ) -> Self {
        assert_unique_keys!(
            columns,
            "TableWithUnitsCompileTime column keys must be unique"
        );
        Self {
            description,
            columns,
        }
    }

    /// Returns the description.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Returns the keyed columns.
    #[must_use]
    pub const fn columns(&self) -> &'static [(ConstStoreKey, NumberWithUnitsCompileTime)] {
        self.columns
    }

    /// Returns the number of entries.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.columns.len()
    }

    /// Returns true if the given key is present.
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Returns the value associated with the given key.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&NumberWithUnitsCompileTime> {
        self.columns
            .iter()
            .find_map(|(column_key, column)| (column_key.as_str() == key).then_some(column))
    }

    /// Returns the value at the given index.
    #[must_use]
    pub fn get_by_index(&self, index: usize) -> Option<&NumberWithUnitsCompileTime> {
        match self.columns.get(index) {
            Some((_, column)) => Some(column),
            None => None,
        }
    }

    /// Returns the index of the column with the given key.
    #[must_use]
    pub fn get_column_index_by_name(&self, key: &str) -> Option<usize> {
        self.columns
            .iter()
            .position(|(column_key, _)| column_key.as_str() == key)
    }

    /// Returns an iterator over the keys.
    pub fn keys(&self) -> impl Iterator<Item = ConstStoreKey> + '_ {
        self.columns.iter().map(|(key, _)| *key)
    }

    /// Returns an iterator over the entries.
    pub fn iter(&self) -> impl Iterator<Item = &(ConstStoreKey, NumberWithUnitsCompileTime)> + '_ {
        self.columns.iter()
    }

    /// Converts this compile-time table-with-units into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> TableWithUnitsDefinition {
        TableWithUnitsDefinition::new(
            self.description,
            self.columns
                .iter()
                .map(|(key, column)| (*key, column.into_definition()))
                .collect(),
        )
    }
}

/// Creates a [`TableWithUnitsCompileTime`], the compile-time metadata for a table
/// parameter made of numeric columns that each carry their own preferred unit, keyed by
/// [`ConstStoreKey`].
///
/// Declaration order of `columns` is preserved by [`TableWithUnitsCompileTime::keys`],
/// [`TableWithUnitsCompileTime::iter`], and [`TableWithUnitsCompileTime::into_definition`],
/// and matches the indices used by [`TableWithUnitsCompileTime::get_by_index`].
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// table_with_units_compile_time!(description, columns)
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the table.
/// - `columns`: `&'static [(ConstStoreKey, NumberWithUnitsCompileTime)]` ordered slice of
///   column key/definition pairs, typically built with the `store_key!` macro and
///   `number_with_units_compile_time!`.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::{NumberWithUnitsCompileTime, TableWithUnitsCompileTime};
/// use datastore::prelude::*;
/// use units::UnitId;
///
/// const COLUMNS: &[(ConstStoreKey, NumberWithUnitsCompileTime)] = &[
///     (
///         store_key!("length"),
///         number_with_units_compile_time!("Length", UnitId::Length_Meter),
///     ),
///     (
///         store_key!("area"),
///         number_with_units_compile_time!("Area", UnitId::Area_SquareMeter),
///     ),
/// ];
/// const MEASUREMENTS: TableWithUnitsCompileTime =
///     table_with_units_compile_time!("Measurements", COLUMNS);
/// assert_eq!(MEASUREMENTS.count(), 2);
///
/// let _definition = MEASUREMENTS.into_definition();
/// ```
#[macro_export]
macro_rules! table_with_units_compile_time {
    ($description:expr, $columns:expr) => {
        const { $crate::compile_time::TableWithUnitsCompileTime::__new($description, $columns) }
    };
}
