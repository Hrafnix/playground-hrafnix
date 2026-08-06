use shareable_string::ShareableString;
use std::fmt;

/// Represents a computed table, consisting of column keys and rows of numeric values.
#[derive(Debug, Clone, PartialEq)]
pub struct ComputedTable {
    keys: Vec<ShareableString>,
    rows: Vec<Vec<f64>>,
}

impl ComputedTable {
    pub(crate) const fn new(keys: Vec<ShareableString>, rows: Vec<Vec<f64>>) -> Self {
        Self { keys, rows }
    }

    /// Returns a reference to the keys of the computed table.
    #[must_use]
    pub fn keys(&self) -> &[ShareableString] {
        &self.keys
    }

    /// Returns a reference to the rows of the computed table.
    #[must_use]
    pub fn rows(&self) -> &[Vec<f64>] {
        &self.rows
    }

    /// Returns the value of a cell by row and column index.
    #[must_use]
    pub fn get_cell(&self, row_index: usize, column_index: usize) -> Option<f64> {
        if let Some(row) = self.rows.get(row_index) {
            if let Some(&value) = row.get(column_index) {
                return Some(value);
            }
        }

        None
    }

    /// Returns the value of a cell by row index and column name.
    pub fn get_cell_by_name<S: Into<ShareableString>>(
        &self,
        row_index: usize,
        column_name: S,
    ) -> Option<f64> {
        let column_name = column_name.into();
        if let Some(column_index) = self.keys.iter().position(|key| key == &column_name) {
            self.get_cell(row_index, column_index)
        } else {
            None
        }
    }

    /// Returns the number of rows in the computed table.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns the number of columns in the computed table.
    #[must_use]
    pub fn column_count(&self) -> usize {
        self.keys.len()
    }
}

/// Represents a computed item that can be a float, string, or table.
#[derive(Debug, Clone, PartialEq)]
pub enum ComputedItem {
    /// A boolean value.
    Boolean(bool),
    /// An integer value.
    Integer(i64),
    /// A floating-point number.
    Float(f64),
    /// A String value.
    String(ShareableString),
    /// An Identifier value.
    Identifier(ShareableString),
    /// Path to a file.
    File(ShareableString),
    /// A table represented as a `ComputedTable`.
    Table(ComputedTable),
}

impl fmt::Display for ComputedItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComputedItem::Boolean(value) => write!(f, "{value}"),
            ComputedItem::Integer(value) => write!(f, "{value}"),
            ComputedItem::Float(value) => write!(f, "{value}"),
            ComputedItem::String(value)
            | ComputedItem::File(value)
            | ComputedItem::Identifier(value) => write!(f, "{value}"),
            ComputedItem::Table(_) => write!(f, "{self:?}"),
        }
    }
}
