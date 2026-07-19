use crate::StoreError;
use crate::definition::TableDefinition;
use crate::frozen::TableFrozen;
use crate::key::StoreKey;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents a table of data in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableEditable {
    definition: TableDefinition,
    rows: Vec<BTreeMap<StoreKey, ShareableString>>,
}

impl TableEditable {
    /// Creates a new `TableEditable` from a `TableFrozen`.
    pub fn new(frozen_table: &TableFrozen) -> Self {
        Self {
            definition: frozen_table.definition().clone(),
            rows: frozen_table.rows().to_vec(),
        }
    }

    /// Converts this `TableEditable` into a `TableFrozen`.
    pub fn freeze(&self) -> TableFrozen {
        TableFrozen::new_from_editable(self)
    }

    /// Returns the value of a cell by row and column index.
    pub fn cell_by_index(&self, row: usize, column: usize) -> Option<&ShareableString> {
        self.rows
            .get(row)?
            .iter()
            .nth(column)
            .map(|(_, value)| value)
    }

    /// Returns the value of a cell by row index and column name.
    pub fn cell_by_name<S: Into<ShareableString>>(
        &self,
        row: usize,
        column_name: S,
    ) -> Option<&ShareableString> {
        self.rows.get(row)?.get(&column_name.into())
    }

    /// Returns the row at the specified index.
    pub fn row(&self, row: usize) -> Option<&BTreeMap<StoreKey, ShareableString>> {
        self.rows.get(row)
    }

    /// Returns a reference to all rows in the table.
    pub fn rows(&self) -> &[BTreeMap<StoreKey, ShareableString>] {
        &self.rows
    }

    /// Returns a reference to the table definition.
    pub fn definition(&self) -> &TableDefinition {
        &self.definition
    }

    /// Returns the number of rows in the table.
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns the number of columns in the table.
    pub fn column_count(&self) -> usize {
        self.definition.count()
    }

    /// Sets the value of a cell and updates the hash.
    pub fn set_cell<S: Into<ShareableString>, V: Into<ShareableString>>(
        &mut self,
        row: usize,
        column_name: S,
        value: V,
    ) -> Result<(), StoreError> {
        let col_name = column_name.into();
        if !self.definition.contains_key(col_name.clone()) {
            return Err(StoreError::KeyNotFound);
        }
        let col_key = StoreKey::new(col_name).map_err(|e| match e {
            StoreError::KeyEmpty => StoreError::KeyEmpty,
            StoreError::KeyInvalidCharacter(s) => StoreError::KeyInvalidCharacter(s),
            _ => unreachable!("StoreKey::new should only return KeyEmpty or KeyInvalidCharacter"),
        })?;
        if let Some(row_data) = self.rows.get_mut(row) {
            row_data.insert(col_key, value.into());
            Ok(())
        } else {
            Err(StoreError::IndexNotFound)
        }
    }

    /// Adds a new row and updates the hash.
    pub fn add_row(&mut self, row: usize) {
        let mut full_row = BTreeMap::new();
        for (key, definition) in self.definition.iter() {
            full_row.insert(key.clone(), definition.default_value().clone());
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

        let row_count = self.rows.len();
        let column_count = self.definition.count();

        for (i, row) in self.rows.iter().enumerate() {
            let is_last_row = i == row_count - 1;

            writeln!(
                f,
                "{}{}Row {}",
                child_prefix,
                Self::branch_char(is_last_row),
                i
            )?;

            let row_prefix = Self::child_prefix(&child_prefix, is_last_row);

            for (j, (key, value)) in row.iter().enumerate() {
                let is_last_key = j == column_count - 1;
                writeln!(
                    f,
                    "{}{}{} \"{}\"",
                    row_prefix,
                    Self::branch_char(is_last_key),
                    key.as_str(),
                    value
                )?;
            }
        }
        Ok(())
    }
}
