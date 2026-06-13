use crate::definition::TableDefinition;
use crate::key::StoreKey;
use crate::store::data::Table;
use crate::store::{CommonStoreTraitInternal, TreePrint};
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents a table of data in the static store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticTable {
    definition: TableDefinition,
    rows: Vec<BTreeMap<StoreKey, ShareableString>>,
    hash: [u8; 32],
}

impl StaticTable {
    /// Creates a new `StaticTable` with a definition and rows.
    pub fn new(
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
    pub fn rows(&self) -> &Vec<BTreeMap<StoreKey, ShareableString>> {
        &self.rows
    }

    /// Returns a reference to the table definition.
    pub fn definition(&self) -> &TableDefinition {
        &self.definition
    }
}

impl From<&Table> for StaticTable {
    fn from(table: &Table) -> Self {
        let mut rows = Vec::new();
        for i in 0..table.row_count() {
            if let Some(row) = table.row(i) {
                rows.push(row.clone());
            }
        }
        Self {
            definition: table.definition().clone(),
            rows,
            hash: table.current_shared_hash(),
        }
    }
}

impl TreePrint for StaticTable {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{}: Table ({} rows) - {}",
            prefix,
            Self::branch_char(prefix, last),
            label,
            self.rows.len(),
            self.definition.description()
        )?;
        let next_prefix = Self::next_prefix(prefix, last);
        for (i, row) in self.rows.iter().enumerate() {
            let is_last_row = i == self.rows.len() - 1;
            writeln!(
                f,
                "{}{}Row {}",
                next_prefix,
                Self::branch_char(&next_prefix, is_last_row),
                i
            )?;
            let row_prefix = Self::next_prefix(&next_prefix, is_last_row);
            let entries: Vec<_> = row.iter().collect();
            for (j, (key, value)) in entries.iter().enumerate() {
                let is_last_key = j == entries.len() - 1;
                writeln!(
                    f,
                    "{}{}{}: {}",
                    row_prefix,
                    Self::branch_char(&row_prefix, is_last_key),
                    key.as_str(),
                    value
                )?;
            }
        }
        Ok(())
    }
}
