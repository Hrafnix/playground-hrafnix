use datastore::definition::TableDefinition;
use shareable_string::ShareableString;

/// Represents preprocessed data for a table, including its definition and the associated data.
#[derive(Debug, Clone, PartialEq)]
pub struct TablePreprocessedData {
    definition: TableDefinition,
    data: Vec<Vec<ShareableString>>,
}

impl TablePreprocessedData {
    pub(crate) fn new(definition: TableDefinition, data: Vec<Vec<ShareableString>>) -> Self {
        Self { definition, data }
    }

    /// Returns a reference to the definition of the table preprocessed data.
    pub fn definition(&self) -> &TableDefinition {
        &self.definition
    }

    /// Returns a reference to the data of the table preprocessed data.
    pub fn data(&self) -> &[Vec<ShareableString>] {
        &self.data
    }

    /// Returns a new `TablePreprocessedData` with strings laundered through the provided store.
    pub fn launder(&self, store: &shareable_string::SharedStringStore) -> Self {
        let laundered_definition = self.definition.launder(store);
        let laundered_data = self
            .data
            .iter()
            .map(|row| row.iter().map(|value| store.launder(value)).collect())
            .collect();

        Self {
            definition: laundered_definition,
            data: laundered_data,
        }
    }
}
