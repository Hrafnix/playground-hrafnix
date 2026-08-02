use crate::StoreError;
use crate::definition::TableDefinition;
use crate::frozen::TableFrozen;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a table of data in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableEditable {
    definition: TableDefinition,
    parameter: ShareableString,
    rows: Vec<Vec<ShareableString>>,
}

impl TableEditable {
    /// Creates a new `TableEditable` from a `TableFrozen`.
    #[must_use]
    pub fn new(frozen_table: &TableFrozen) -> Self {
        Self {
            definition: frozen_table.definition().clone(),
            parameter: frozen_table.parameter().clone(),
            rows: frozen_table.rows().to_vec(),
        }
    }

    /// Converts this `TableEditable` into a `TableFrozen`.
    #[must_use]
    pub fn freeze(&self) -> TableFrozen {
        TableFrozen::new_from_editable(self)
    }

    /// Returns the value of a cell by row and column index.
    #[must_use]
    pub fn cell_by_index(&self, row: usize, column: usize) -> Option<&ShareableString> {
        self.rows.get(row)?.get(column)
    }

    /// Returns the value of a cell by row index and column name.
    pub fn cell_by_name<S: Into<ShareableString>>(
        &self,
        row: usize,
        column_name: S,
    ) -> Option<&ShareableString> {
        let column_index = self
            .definition
            .get_column_index_by_name(column_name.into())?;
        self.cell_by_index(row, column_index)
    }

    /// Returns the row at the specified index.
    #[must_use]
    pub fn row(&self, row: usize) -> Option<&Vec<ShareableString>> {
        self.rows.get(row)
    }

    /// Returns a reference to all rows in the table.
    #[must_use]
    pub fn rows(&self) -> &[Vec<ShareableString>] {
        &self.rows
    }

    /// Returns a reference to the table definition.
    #[must_use]
    pub fn definition(&self) -> &TableDefinition {
        &self.definition
    }

    /// Returns the number of rows in the table.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns the number of columns in the table.
    #[must_use]
    pub fn column_count(&self) -> usize {
        self.definition.count()
    }

    /// Sets the value of a cell and updates the hash.
    ///
    /// # Errors
    ///
    /// Returns `StoreError::KeyNotFound` if `column_name` does not match a column
    /// in the table's definition.
    pub fn set_cell<S: Into<ShareableString>, V: Into<ShareableString>>(
        &mut self,
        row: usize,
        column_name: S,
        value: V,
    ) -> Result<(), StoreError> {
        let col_name = column_name.into();
        let Some(column_index) = self.definition.get_column_index_by_name(col_name.clone()) else {
            return Err(StoreError::KeyNotFound);
        };

        if let Some(row_data) = self.rows.get_mut(row) {
            if let Some(column_data) = row_data.get_mut(column_index) {
                *column_data = value.into();
                Ok(())
            } else {
                Err(StoreError::IndexNotFound)
            }
        } else {
            Err(StoreError::IndexNotFound)
        }
    }

    /// Adds a new row and updates the hash.
    pub fn add_row(&mut self, row: usize) {
        let mut full_row = Vec::new();
        for (_, definition) in self.definition.iter() {
            full_row.push(definition.default_value().clone());
        }
        if row < self.rows.len() {
            self.rows.insert(row, full_row);
        } else {
            self.rows.push(full_row);
        }
    }

    /// Removes a row and updates the hash.
    pub fn remove_row(&mut self, row: usize) {
        if self.rows.is_empty() {
            return;
        }

        if row < self.rows.len() {
            self.rows.remove(row);
        } else {
            self.rows.pop();
        }
    }

    /// Set the parameter value for the table.
    pub fn set_parameter<S: Into<ShareableString>>(&mut self, parameter: S) {
        self.parameter = parameter.into();
    }

    /// Returns a reference to the parameter value for the table.
    #[must_use]
    pub fn parameter(&self) -> &ShareableString {
        &self.parameter
    }
}

impl PartialEq<&TableEditable> for TableEditable {
    fn eq(&self, other: &&TableEditable) -> bool {
        self == *other
    }
}

impl PartialEq<TableEditable> for &TableEditable {
    fn eq(&self, other: &TableEditable) -> bool {
        *self == other
    }
}

impl TreePrint for TableEditable {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Table {} rows",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.rows.len(),
        )?;

        let child_prefix = Self::child_prefix(prefix, last);

        writeln!(f, "{}{}data", child_prefix, Self::branch_char(false))?;

        let data_prefix = Self::child_prefix(&child_prefix, false);

        let mut rows_iter = self.rows.iter().enumerate().peekable();

        while let Some((i, row)) = rows_iter.next() {
            let is_last_row = rows_iter.peek().is_none();

            writeln!(
                f,
                "{}{}Row {}",
                data_prefix,
                Self::branch_char(is_last_row),
                i
            )?;

            let row_prefix = Self::child_prefix(&data_prefix, is_last_row);

            let mut column_iter = row.iter().enumerate().peekable();
            while let Some((j, value)) = column_iter.next() {
                let is_last_key = column_iter.peek().is_none();
                let key = match self.definition.keys().nth(j) {
                    Some(k) => k.as_str(),
                    None => "Unknown",
                };
                writeln!(
                    f,
                    "{}{}{} \"{}\"",
                    row_prefix,
                    Self::branch_char(is_last_key),
                    key,
                    value
                )?;
            }
        }

        writeln!(
            f,
            "{}{}Parameter \"{}\"",
            child_prefix,
            Self::branch_char(true),
            self.parameter
        )?;

        Ok(())
    }
}
