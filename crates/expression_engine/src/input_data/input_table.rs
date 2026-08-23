use datastore::definition::TableDefinition;
use shareable_string::ShareableString;

/// Represents input data for a table, including its definition and the associated data.
#[derive(Debug, Clone, PartialEq)]
pub struct TableInputData {
    /// The definition that describes the columns and constraints of this table.
    definition: TableDefinition,
    /// The name of the parameter variable that holds the row index during cell evaluation,
    /// or an empty string if no such parameter is used.
    parameter: ShareableString,
    /// The raw cell data, one inner `Vec<ShareableString>` per row.
    data: Vec<Vec<ShareableString>>,
}

impl TableInputData {
    /// Creates a new `TableInputData` with the given definition, parameter name, and raw data.
    pub(crate) const fn new(
        definition: TableDefinition,
        parameter: ShareableString,
        data: Vec<Vec<ShareableString>>,
    ) -> Self {
        Self {
            definition,
            parameter,
            data,
        }
    }

    /// Returns a reference to the definition of the table input data.
    #[must_use]
    pub const fn definition(&self) -> &TableDefinition {
        &self.definition
    }

    /// Returns a reference to the data of the table input data.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn data(&self) -> &[Vec<ShareableString>] {
        &self.data
    }

    /// Returns a reference to the parameter name of the table input data.
    ///
    /// When non-empty, this name is bound to the current row index while each
    /// row's cell expressions are evaluated.
    #[must_use]
    pub const fn parameter(&self) -> &ShareableString {
        &self.parameter
    }

    /// Returns a new `TableInputData` with strings laundered through the provided store.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn launder(&self, store: &shareable_string::SharedStringStore) -> Self {
        let laundered_definition = self.definition.launder(store);
        let laundered_data = self
            .data
            .iter()
            .map(|row| row.iter().map(|value| store.launder(value)).collect())
            .collect();

        Self {
            definition: laundered_definition,
            parameter: store.launder(&self.parameter),
            data: laundered_data,
        }
    }
}
