use crate::compile_time::NumberWithUnitsCompileTime;
use crate::compile_time::compile_time_common::{assert_unique_keys, const_str_eq};
use crate::definition::{TableColumnRequirement, TableWithUnitsDefinition};
use hrafnix_keys::store_key::ConstStoreKey;

/// Compile-time representation of a table with units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TableWithUnitsCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Keyed columns contained in this compile-time table.
    columns: &'static [(ConstStoreKey, NumberWithUnitsCompileTime)],
    /// Adjacent-row requirements keyed by column; columns absent here have no requirement.
    column_requirements: &'static [(ConstStoreKey, TableColumnRequirement)],
}

impl TableWithUnitsCompileTime {
    /// Hidden backing constructor for `const_table_with_units!(description, columns)`.
    ///
    /// This is an implementation detail; call `const_table_with_units!` instead.
    /// `description` names the table and `columns` is the ordered slice of
    /// `(ConstStoreKey, NumberWithUnitsCompileTime)` column key/definition pairs, typically
    /// built with the `store_key!` macro and `const_number_with_units!`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(
        description: &'static str,
        columns: &'static [(ConstStoreKey, NumberWithUnitsCompileTime)],
    ) -> Self {
        Self::__new_with_column_requirements(description, columns, &[])
    }

    /// Hidden backing constructor for
    /// `const_table_with_units!(description, columns, column_requirements)`.
    ///
    /// This is an implementation detail; call `const_table_with_units!` instead. Every key in
    /// `column_requirements` must be unique and name a column in `columns`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_column_requirements(
        description: &'static str,
        columns: &'static [(ConstStoreKey, NumberWithUnitsCompileTime)],
        column_requirements: &'static [(ConstStoreKey, TableColumnRequirement)],
    ) -> Self {
        assert_unique_keys!(
            columns,
            "TableWithUnitsCompileTime column keys must be unique"
        );
        assert_unique_keys!(
            column_requirements,
            "TableWithUnitsCompileTime column requirement keys must be unique"
        );
        let mut unchecked = column_requirements;
        while let [(requirement_key, _), rest @ ..] = unchecked {
            let mut found = false;
            let mut candidates = columns;
            while let [(column_key, _), remaining @ ..] = candidates {
                if const_str_eq(requirement_key.as_str(), column_key.as_str()) {
                    found = true;
                }
                candidates = remaining;
            }
            assert!(
                found,
                "TableWithUnitsCompileTime column requirement keys must name an existing column"
            );
            unchecked = rest;
        }
        Self {
            description,
            columns,
            column_requirements,
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

    /// Returns the keyed column requirements.
    #[must_use]
    pub const fn column_requirements(&self) -> &'static [(ConstStoreKey, TableColumnRequirement)] {
        self.column_requirements
    }

    /// Returns the requirement for the given column, or `None` for an unknown column.
    ///
    /// A known column without a requirement returns `Some(TableColumnRequirement::None)`.
    #[must_use]
    pub fn column_requirement(&self, key: &str) -> Option<TableColumnRequirement> {
        self.contains_key(key).then(|| {
            self.column_requirements
                .iter()
                .find_map(|(requirement_key, requirement)| {
                    (requirement_key.as_str() == key).then_some(*requirement)
                })
                .unwrap_or(TableColumnRequirement::None)
        })
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
        TableWithUnitsDefinition::new_with_column_requirements(
            self.description,
            self.columns
                .iter()
                .map(|(key, column)| {
                    (
                        *key,
                        column.into_definition(),
                        self.column_requirement(key.as_str())
                            .unwrap_or(TableColumnRequirement::None),
                    )
                })
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
/// const_table_with_units!(description, columns)
/// const_table_with_units!(description, columns, column_requirements)
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the table.
/// - `columns`: `&'static [(ConstStoreKey, NumberWithUnitsCompileTime)]` ordered slice of
///   column key/definition pairs, typically built with the `store_key!` macro and
///   `const_number_with_units!`.
/// - `column_requirements` (optional): `&'static [(ConstStoreKey, TableColumnRequirement)]`
///   adjacent-row requirements keyed by column. Keys must be unique and name a column in
///   `columns`; omitted columns have no requirement.
///
/// # Examples
/// ```rust
/// use hrafnix_datastore::compile_time::{NumberWithUnitsCompileTime, TableWithUnitsCompileTime};
/// use hrafnix_datastore::prelude::*;
/// use hrafnix_units::UnitId;
///
/// const COLUMNS: &[(ConstStoreKey, NumberWithUnitsCompileTime)] = &[
///     (
///         store_key!("length"),
///         const_number_with_units!("Length", UnitId::Length_Meter),
///     ),
///     (
///         store_key!("area"),
///         const_number_with_units!("Area", UnitId::Area_SquareMeter),
///     ),
/// ];
/// const MEASUREMENTS: TableWithUnitsCompileTime =
///     const_table_with_units!("Measurements", COLUMNS);
/// assert_eq!(MEASUREMENTS.count(), 2);
///
/// let _definition = MEASUREMENTS.into_definition();
/// ```
#[macro_export]
macro_rules! const_table_with_units {
    ($description:expr, $columns:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::TableWithUnitsCompileTime::__new($description, $columns)
        }
    };
    ($description:expr, $columns:expr, $column_requirements:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::TableWithUnitsCompileTime::__new_with_column_requirements(
                $description,
                $columns,
                $column_requirements,
            )
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{const_number_with_units, store_key};
    use hrafnix_units::UnitId;

    #[test]
    #[allow(clippy::disallowed_methods)]
    fn hidden_constructor_runs_at_runtime() {
        const COLUMNS: &[(ConstStoreKey, NumberWithUnitsCompileTime)] = &[(
            store_key!("length"),
            const_number_with_units!("Length", UnitId::Length_Meter),
        )];
        let table = TableWithUnitsCompileTime::__new(std::hint::black_box("Measurements"), COLUMNS);

        assert_eq!(table.description(), "Measurements");
        assert_eq!(table.columns(), COLUMNS);
    }

    #[test]
    #[should_panic(expected = "TableWithUnitsCompileTime column keys must be unique")]
    fn table_with_units_compile_time_rejects_duplicate_keys() {
        const DUPLICATES: &[(ConstStoreKey, NumberWithUnitsCompileTime)] = &[
            (
                store_key!("duplicate"),
                const_number_with_units!("First", UnitId::Length_Meter),
            ),
            (
                store_key!("duplicate"),
                const_number_with_units!("Second", UnitId::Length_Meter),
            ),
        ];
        #[allow(clippy::disallowed_methods)]
        let _ = TableWithUnitsCompileTime::__new(std::hint::black_box("Duplicates"), DUPLICATES);
    }

    #[test]
    fn column_requirements_are_applied_to_definition() {
        const COLUMNS: &[(ConstStoreKey, NumberWithUnitsCompileTime)] = &[
            (
                store_key!("length"),
                const_number_with_units!("Length", UnitId::Length_Meter),
            ),
            (
                store_key!("other"),
                const_number_with_units!("Length", UnitId::Length_Meter),
            ),
        ];
        const REQUIREMENTS: &[(ConstStoreKey, TableColumnRequirement)] =
            &[(store_key!("length"), TableColumnRequirement::Increasing)];
        const TABLE: TableWithUnitsCompileTime =
            const_table_with_units!("Table", COLUMNS, REQUIREMENTS);

        assert_eq!(
            TABLE.column_requirement("length"),
            Some(TableColumnRequirement::Increasing)
        );
        assert_eq!(
            TABLE.column_requirement("other"),
            Some(TableColumnRequirement::None)
        );
        assert_eq!(TABLE.column_requirement("missing"), None);

        let definition = TABLE.into_definition();
        assert_eq!(
            definition.column_requirement("length"),
            Some(TableColumnRequirement::Increasing)
        );
        assert_eq!(
            definition.column_requirement("other"),
            Some(TableColumnRequirement::None)
        );
    }

    #[test]
    #[should_panic(expected = "column requirement keys must name an existing column")]
    fn rejects_requirement_for_unknown_column() {
        const COLUMNS: &[(ConstStoreKey, NumberWithUnitsCompileTime)] = &[(
            store_key!("length"),
            const_number_with_units!("Length", UnitId::Length_Meter),
        )];
        const REQUIREMENTS: &[(ConstStoreKey, TableColumnRequirement)] =
            &[(store_key!("missing"), TableColumnRequirement::Increasing)];
        #[allow(clippy::disallowed_methods)]
        let _ = TableWithUnitsCompileTime::__new_with_column_requirements(
            std::hint::black_box("Table"),
            COLUMNS,
            REQUIREMENTS,
        );
    }

    #[test]
    #[should_panic(expected = "column requirement keys must be unique")]
    fn rejects_duplicate_requirement_keys() {
        const COLUMNS: &[(ConstStoreKey, NumberWithUnitsCompileTime)] = &[(
            store_key!("length"),
            const_number_with_units!("Length", UnitId::Length_Meter),
        )];
        const REQUIREMENTS: &[(ConstStoreKey, TableColumnRequirement)] = &[
            (store_key!("length"), TableColumnRequirement::Increasing),
            (store_key!("length"), TableColumnRequirement::Decreasing),
        ];
        #[allow(clippy::disallowed_methods)]
        let _ = TableWithUnitsCompileTime::__new_with_column_requirements(
            std::hint::black_box("Table"),
            COLUMNS,
            REQUIREMENTS,
        );
    }
}
