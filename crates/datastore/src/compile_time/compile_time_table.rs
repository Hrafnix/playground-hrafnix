use crate::compile_time::NumberCompileTime;
use crate::compile_time::compile_time_common::assert_unique_keys;
use crate::definition::TableDefinition;
use keys::store_key::ConstStoreKey;

/// Compile-time representation of a table.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TableCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Keyed columns contained in this compile-time table.
    columns: &'static [(ConstStoreKey, NumberCompileTime)],
}

impl TableCompileTime {
    /// Hidden backing constructor for `const_table!(description, columns)`.
    ///
    /// This is an implementation detail; call `const_table!` instead.
    /// `description` names the table and `columns` is the ordered slice of
    /// `(ConstStoreKey, NumberCompileTime)` column key/definition pairs, typically built
    /// with the `store_key!` macro and `const_number!`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(
        description: &'static str,
        columns: &'static [(ConstStoreKey, NumberCompileTime)],
    ) -> Self {
        assert_unique_keys!(columns, "TableCompileTime column keys must be unique");
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
    pub const fn columns(&self) -> &'static [(ConstStoreKey, NumberCompileTime)] {
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
    pub fn get(&self, key: &str) -> Option<&NumberCompileTime> {
        self.columns
            .iter()
            .find_map(|(column_key, column)| (column_key.as_str() == key).then_some(column))
    }

    /// Returns the value at the given index.
    #[must_use]
    pub fn get_by_index(&self, index: usize) -> Option<&NumberCompileTime> {
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
    pub fn iter(&self) -> impl Iterator<Item = &(ConstStoreKey, NumberCompileTime)> + '_ {
        self.columns.iter()
    }

    /// Converts this compile-time table into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> TableDefinition {
        TableDefinition::new(
            self.description,
            self.columns
                .iter()
                .map(|(key, column)| (*key, column.into_definition()))
                .collect(),
        )
    }
}

/// Creates a [`TableCompileTime`], the compile-time metadata for a table parameter made of
/// unit-less numeric columns, keyed by [`ConstStoreKey`].
///
/// Declaration order of `columns` is preserved by [`TableCompileTime::keys`],
/// [`TableCompileTime::iter`], and [`TableCompileTime::into_definition`], and matches the
/// indices used by [`TableCompileTime::get_by_index`].
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// const_table!(description, columns)
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the table.
/// - `columns`: `&'static [(ConstStoreKey, NumberCompileTime)]` ordered slice of column
///   key/definition pairs, typically built with the `store_key!` macro and
///   `const_number!`.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::{NumberCompileTime, TableCompileTime};
/// use datastore::prelude::*;
///
/// const COLUMNS: &[(ConstStoreKey, NumberCompileTime)] = &[
///     (
///         store_key!("width"),
///         const_number!("Width", default = "10"),
///     ),
///     (
///         store_key!("height"),
///         const_number!("Height", default = "20"),
///     ),
/// ];
/// const DIMENSIONS: TableCompileTime = const_table!("Dimensions", COLUMNS);
/// assert_eq!(DIMENSIONS.count(), 2);
///
/// let _definition = DIMENSIONS.into_definition();
/// ```
#[macro_export]
macro_rules! const_table {
    ($description:expr, $columns:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::TableCompileTime::__new($description, $columns)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{const_number, store_key};

    #[test]
    #[allow(clippy::disallowed_methods)]
    fn hidden_constructor_runs_at_runtime() {
        const COLUMNS: &[(ConstStoreKey, NumberCompileTime)] =
            &[(store_key!("width"), const_number!("Width"))];
        let table = TableCompileTime::__new(std::hint::black_box("Dimensions"), COLUMNS);

        assert_eq!(table.description(), "Dimensions");
        assert_eq!(table.columns(), COLUMNS);
    }

    #[test]
    #[should_panic(expected = "TableCompileTime column keys must be unique")]
    fn table_compile_time_rejects_duplicate_keys() {
        const DUPLICATES: &[(ConstStoreKey, NumberCompileTime)] = &[
            (store_key!("duplicate"), const_number!("First")),
            (store_key!("duplicate"), const_number!("Second")),
        ];
        #[allow(clippy::disallowed_methods)]
        let _ = TableCompileTime::__new(std::hint::black_box("Duplicates"), DUPLICATES);
    }
}
