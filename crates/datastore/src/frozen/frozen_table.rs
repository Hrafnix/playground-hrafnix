use crate::definition::TableDefinition;
use crate::editable::TableEditable;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a table of data in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableFrozen {
    /// Definition metadata for this table value.
    definition: TableDefinition,
    /// Row data; each inner `Vec` holds one value per column.
    rows: Vec<Vec<ShareableString>>,
    /// Parameter key associated with this table instance.
    parameter: ShareableString,
    /// Pre-computed BLAKE3 hash of all rows for fast diffing.
    hash: [u8; 32],
}

impl TableFrozen {
    /// Creates a new `TableFrozen` with a definition.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new(definition: TableDefinition) -> Self {
        let mut s = Self {
            definition,
            rows: Vec::new(),
            parameter: ShareableString::new(""),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `TableFrozen` with a definition and rows.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new_from_rows(definition: TableDefinition, rows: Vec<Vec<ShareableString>>) -> Self {
        let mut s = Self {
            definition,
            rows,
            parameter: ShareableString::new(""),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `TableFrozen` from a `TableEditable`.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new_from_editable(editable_table: &TableEditable) -> Self {
        let mut s = Self {
            definition: editable_table.definition().clone(),
            rows: editable_table.rows().to_vec(),
            parameter: editable_table.parameter().clone(),
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Converts the current `TableFrozen` instance into a `TableEditable` instance.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn thaw(&self) -> TableEditable {
        TableEditable::new(self)
    }

    /// Recomputes and stores the BLAKE3 hash of all rows.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"Table");

        h.update(
            &u64::try_from(self.rows.len())
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );

        for row in &self.rows {
            h.update(&u64::try_from(row.len()).unwrap_or(u64::MAX).to_le_bytes());
            for value in row {
                h.update(&value.current_blake3_hash());
            }
        }

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the value of a cell by row and column index.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn cell_by_index(&self, row: usize, column: usize) -> Option<&ShareableString> {
        self.rows.get(row)?.get(column)
    }

    /// Returns the value of a cell by row index and column name.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
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
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn row(&self, row: usize) -> Option<&Vec<ShareableString>> {
        self.rows.get(row)
    }

    /// Returns the pre-calculated BLAKE3 hash of the table.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns a reference to all rows in the table.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn rows(&self) -> &[Vec<ShareableString>] {
        &self.rows
    }

    /// Returns a reference to the table definition.
    #[must_use]
    pub const fn definition(&self) -> &TableDefinition {
        &self.definition
    }

    /// Returns the number of rows in the table.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns the number of columns in the table.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn column_count(&self) -> usize {
        self.definition.count()
    }

    /// Returns a reference to the parameter value for the table.
    #[must_use]
    pub const fn parameter(&self) -> &ShareableString {
        &self.parameter
    }
}

impl PartialEq<&TableFrozen> for TableFrozen {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&TableFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<TableFrozen> for &TableFrozen {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &TableFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for TableFrozen {
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

        let mut row_iter = self.rows.iter().enumerate().peekable();

        while let Some((i, row)) = row_iter.next() {
            let is_last_row = row_iter.peek().is_none();

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
