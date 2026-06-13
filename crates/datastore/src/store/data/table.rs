use crate::StoreError;
use crate::definition::TableDefinition;
use crate::key::StoreKey;
use crate::static_store::data::StaticTable;
use crate::store::{CommonStoreTraitInternal, StoreHashContainer, TreePrint};
use shareable_string::{ShareableString, SharedStringStore};
use std::collections::BTreeMap;

/// Represents a table of data in the store.
#[derive(Debug, Clone)]
pub(crate) struct Table {
    definition: TableDefinition,
    rows: Vec<BTreeMap<StoreKey, ShareableString>>,
    current_hash: [u8; 32],
    shared_hash: StoreHashContainer,
}

impl Table {
    /// Creates a new `Table` instance with the given definition.
    pub(crate) fn new(definition: TableDefinition) -> Self {
        let mut s = Self {
            definition,
            rows: Vec::new(),
            current_hash: [0u8; 32],
            shared_hash: StoreHashContainer::new(),
        };
        s.update_current_hash();
        s.update_shared_hash();
        s
    }

    /// Returns a new `Table` instance with strings laundered through the provided store.
    pub(crate) fn launder(&self, store: &SharedStringStore) -> Self {
        let mut s = Self {
            definition: self.definition.launder(store),
            rows: self
                .rows
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|(k, v)| (k.launder(store), store.launder(v)))
                        .collect()
                })
                .collect(),
            current_hash: self.current_hash,
            shared_hash: StoreHashContainer::new(),
        };
        s.update_shared_hash();
        s
    }

    /// Returns a reference to the table definition.
    pub(crate) fn definition(&self) -> &TableDefinition {
        &self.definition
    }

    /// Returns the number of rows in the table.
    pub(crate) fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns the number of columns in the table.
    pub(crate) fn column_count(&self) -> usize {
        self.definition.count()
    }

    /// Appends a new row with default values to the table.
    pub(crate) fn append_row(&mut self) {
        let default_row = self.default_row();
        self.rows.push(default_row);
        self.update_current_hash();
    }

    /// Inserts a new row with default values at the specified index.
    pub(crate) fn insert_row(&mut self, index: usize) {
        if index >= self.rows.len() {
            self.append_row();
            return;
        }

        let default_row = self.default_row();
        self.rows.insert(index, default_row);
        self.update_current_hash();
    }

    /// Removes the row at the specified index.
    pub(crate) fn remove_row(&mut self, index: usize) -> Result<(), StoreError> {
        if index >= self.rows.len() {
            return Err(StoreError::IndexNotFound);
        }

        self.rows.remove(index);
        self.update_current_hash();
        Ok(())
    }

    /// Returns a reference to the row at the specified index.
    pub(crate) fn row(&self, index: usize) -> Option<&BTreeMap<StoreKey, ShareableString>> {
        self.rows.get(index)
    }

    /// Sets the value of a cell in the table.
    pub(crate) fn set_cell<K: AsRef<str>>(
        &mut self,
        row_index: usize,
        column_key: K,
        value: ShareableString,
    ) -> Result<(), StoreError> {
        let column_key = column_key.as_ref();
        if let Some(row) = self.rows.get_mut(row_index) {
            if let Some(cell) = row.get_mut(column_key) {
                *cell = value;
                self.update_current_hash();
                Ok(())
            } else {
                Err(StoreError::KeyNotFound)
            }
        } else {
            Err(StoreError::IndexNotFound)
        }
    }

    /// Sets the values of a row in the table.
    pub(crate) fn set_row(
        &mut self,
        row_index: usize,
        values: BTreeMap<ShareableString, ShareableString>,
    ) -> Result<(), StoreError> {
        if let Some(row) = self.rows.get_mut(row_index) {
            for (key, value) in values {
                if let Some(cell) = row.get_mut(key.as_str()) {
                    *cell = value;
                }
            }
            self.update_current_hash();
            Ok(())
        } else {
            Err(StoreError::IndexNotFound)
        }
    }

    /// Returns a new row populated with default values according to the table definition.
    fn default_row(&self) -> BTreeMap<StoreKey, ShareableString> {
        self.definition
            .iter()
            .map(|column| (column.0.clone(), column.1.default_value().clone()))
            .collect()
    }

    /// Updates this `Table` from a [`StaticTable`], replacing all rows and syncing the shared hash.
    pub(crate) fn update_from_static(&mut self, static_table: &StaticTable) {
        self.rows = static_table.rows().clone();
        self.shared_hash.set(static_table.hash());
    }
}

impl From<&StaticTable> for Table {
    fn from(static_table: &StaticTable) -> Self {
        let hash = static_table.hash();
        let s = Self {
            definition: static_table.definition().clone(),
            rows: static_table.rows().clone(),
            current_hash: hash,
            shared_hash: StoreHashContainer::new(),
        };
        s.shared_hash.set(hash);
        s
    }
}

impl CommonStoreTraitInternal for Table {
    fn current_shared_hash(&self) -> [u8; 32] {
        self.shared_hash.get()
    }

    fn update_current_hash(&mut self) {
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

        self.current_hash = *h.finalize().as_bytes()
    }

    fn update_shared_hash(&mut self) {
        self.shared_hash.set(self.current_hash);
    }

    fn clear_shared_hash(&mut self) {
        self.shared_hash.clear()
    }

    fn has_changed(&self) -> bool {
        self.current_hash != self.shared_hash.get()
    }

    fn is_valid(&self) -> bool {
        self.shared_hash.get() != [0u8; 32]
    }
}

impl TreePrint for Table {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{}: [Table, {} rows] ({})",
            prefix,
            Self::branch_char(prefix, last),
            label,
            self.rows.len(),
            self.definition.description()
        )?;
        let next_prefix = Self::next_prefix(prefix, last);
        for (i, row) in self.rows.iter().enumerate() {
            let row_last = i == self.rows.len() - 1;
            writeln!(
                f,
                "{}{}Row {}",
                next_prefix,
                Self::branch_char(&next_prefix, row_last),
                i
            )?;
            let row_prefix = Self::next_prefix(&next_prefix, row_last);
            let keys: Vec<_> = row.keys().collect();
            for (j, key) in keys.iter().enumerate() {
                let cell_last = j == keys.len() - 1;
                writeln!(
                    f,
                    "{}{}{}: {}",
                    row_prefix,
                    Self::branch_char(&row_prefix, cell_last),
                    key,
                    row.get(*key)
                        .expect("key was taken from row keys, so it must exist")
                )?;
            }
        }
        Ok(())
    }
}
