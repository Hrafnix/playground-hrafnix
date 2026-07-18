use crate::definition::TableDefinition;
use crate::key::StoreKey;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents a table of data in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableFrozen {
    definition: TableDefinition,
    rows: Vec<BTreeMap<StoreKey, ShareableString>>,
    hash: [u8; 32],
}

impl TableFrozen {
    /// Creates a new `TableFrozen` with a definition.
    pub fn new(definition: TableDefinition) -> Self {
        let mut s = Self {
            definition,
            rows: Vec::new(),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `TableFrozen` with a definition and rows.
    pub fn new_from_rows(
        definition: TableDefinition,
        rows: Vec<BTreeMap<StoreKey, ShareableString>>,
    ) -> Self {
        let mut s = Self {
            definition,
            rows,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"Table");

        h.update(&(self.rows.len() as u64).to_le_bytes());
        for row in &self.rows {
            h.update(&(row.len() as u64).to_le_bytes());
            for (key, value) in row {
                h.update(&key.current_blake3_hash());
                h.update(&value.current_blake3_hash());
            }
        }

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
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

    /// Returns the pre-calculated BLAKE3 hash of the table.
    pub fn hash(&self) -> [u8; 32] {
        self.hash
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
}

impl PartialEq<&TableFrozen> for TableFrozen {
    fn eq(&self, other: &&TableFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<TableFrozen> for &TableFrozen {
    fn eq(&self, other: &TableFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for TableFrozen {
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
