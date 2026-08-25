use expression_engine::ComputedItem;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use units::UnitId;

/// A validated numeric table used at the simulation boundary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeTable {
    /// Column names in stable display order.
    columns: Vec<ShareableString>,
    /// Optional canonical unit for every column.
    column_units: Option<Vec<UnitId>>,
    /// Finite numeric rows.
    rows: Vec<Vec<f64>>,
}

impl RuntimeTable {
    /// Returns column names in stable order.
    #[must_use]
    pub fn columns(&self) -> &[ShareableString] {
        &self.columns
    }

    /// Returns canonical column units when the source table carries units.
    #[must_use]
    pub fn column_units(&self) -> Option<&[UnitId]> {
        self.column_units.as_deref()
    }

    /// Returns the finite numeric rows.
    #[must_use]
    pub fn rows(&self) -> &[Vec<f64>] {
        &self.rows
    }
}

/// A value validated for use at simulation configuration and port boundaries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuntimeValue {
    /// Boolean value.
    Boolean(bool),
    /// Integer value.
    Integer(i64),
    /// Finite unitless scalar.
    Scalar(f64),
    /// Finite scalar expressed in a concrete unit.
    ScalarWithUnit {
        /// Numeric value.
        value: f64,
        /// Unit associated with `value`.
        unit: UnitId,
    },
    /// Text value.
    String(ShareableString),
    /// Identifier value.
    Identifier(ShareableString),
    /// File or folder path value.
    Path(ShareableString),
    /// Numeric table.
    Table(RuntimeTable),
    /// Unit selection.
    Unit(UnitId),
}

/// Failure while adapting an expression result into a runtime value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueAdaptationError {
    /// A scalar or table cell was NaN or infinite.
    NonFiniteNumber,
    /// A table row did not contain one cell per declared column.
    InconsistentTableWidth,
    /// A unit-bearing table did not contain one unit per declared column.
    InconsistentTableUnits,
}

impl TryFrom<&ComputedItem> for RuntimeValue {
    type Error = ValueAdaptationError;

    fn try_from(item: &ComputedItem) -> Result<Self, Self::Error> {
        match item {
            ComputedItem::Boolean(value) => Ok(Self::Boolean(*value)),
            ComputedItem::Integer(value) => Ok(Self::Integer(*value)),
            ComputedItem::Float(value) => finite(*value).map(Self::Scalar),
            ComputedItem::FloatWithUnit { value, unit } => {
                finite(*value).map(|value| Self::ScalarWithUnit { value, unit: *unit })
            }
            ComputedItem::String(value) => Ok(Self::String(value.clone())),
            ComputedItem::Identifier(value) => Ok(Self::Identifier(value.clone())),
            ComputedItem::Path(value) => Ok(Self::Path(value.clone())),
            ComputedItem::Table(table) => adapt_table(table.keys(), None, table.rows()),
            ComputedItem::TableWithUnits(table) => {
                adapt_table(table.keys(), Some(table.units()), table.rows())
            }
            ComputedItem::Unit(unit) => Ok(Self::Unit(*unit)),
        }
    }
}

/// Rejects nonfinite values before they enter runtime storage.
fn finite(value: f64) -> Result<f64, ValueAdaptationError> {
    value
        .is_finite()
        .then_some(value)
        .ok_or(ValueAdaptationError::NonFiniteNumber)
}

/// Copies and validates table data from the expression layer.
fn adapt_table(
    columns: &[ShareableString],
    units: Option<&[UnitId]>,
    rows: &[Vec<f64>],
) -> Result<RuntimeValue, ValueAdaptationError> {
    if units.is_some_and(|values| values.len() != columns.len()) {
        return Err(ValueAdaptationError::InconsistentTableUnits);
    }
    if rows.iter().any(|row| row.len() != columns.len()) {
        return Err(ValueAdaptationError::InconsistentTableWidth);
    }
    if rows.iter().flatten().any(|value| !value.is_finite()) {
        return Err(ValueAdaptationError::NonFiniteNumber);
    }

    Ok(RuntimeValue::Table(RuntimeTable {
        columns: columns.to_vec(),
        column_units: units.map(<[UnitId]>::to_vec),
        rows: rows.to_vec(),
    }))
}

#[cfg(test)]
mod tests {
    use super::{RuntimeValue, ValueAdaptationError};
    use expression_engine::ComputedItem;
    use units::UnitId;

    #[test]
    fn adapts_scalar_with_unit_without_losing_metadata() {
        let computed = ComputedItem::FloatWithUnit {
            value: 2.5,
            unit: UnitId::Time_Second,
        };

        assert_eq!(
            RuntimeValue::try_from(&computed),
            Ok(RuntimeValue::ScalarWithUnit {
                value: 2.5,
                unit: UnitId::Time_Second,
            })
        );
    }

    #[test]
    fn rejects_nonfinite_scalar() {
        assert_eq!(
            RuntimeValue::try_from(&ComputedItem::Float(f64::NAN)),
            Err(ValueAdaptationError::NonFiniteNumber)
        );
    }
}
