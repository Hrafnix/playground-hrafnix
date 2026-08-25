use datastore::definition::ItemDefinitionType;
use serde::{Deserialize, Serialize};
use units::{UnitFamilyId, UnitId};

/// Simulation-facing value shape derived from a datastore parameter definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterValueType {
    /// Boolean parameter.
    Boolean,
    /// Choice parameter represented by an identifier.
    Choice,
    /// File path parameter.
    File,
    /// Folder path parameter.
    Folder,
    /// Integer parameter.
    Integer,
    /// Unitless scalar parameter.
    Scalar,
    /// Scalar parameter with a preferred unit.
    ScalarWithUnit(UnitId),
    /// String parameter.
    String,
    /// Unitless numeric table.
    Table,
    /// Numeric table with canonical units in column order.
    TableWithUnits(Vec<UnitId>),
    /// Unit selector constrained to one family.
    Unit(UnitFamilyId),
}

/// Indicates that a datastore item is structural or unsupported as a runtime parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnsupportedParameterDefinition;

impl TryFrom<&ItemDefinitionType> for ParameterValueType {
    type Error = UnsupportedParameterDefinition;

    fn try_from(definition: &ItemDefinitionType) -> Result<Self, Self::Error> {
        match definition {
            ItemDefinitionType::Boolean(_) => Ok(Self::Boolean),
            ItemDefinitionType::Choice(_) => Ok(Self::Choice),
            ItemDefinitionType::File(_) => Ok(Self::File),
            ItemDefinitionType::Folder(_) => Ok(Self::Folder),
            ItemDefinitionType::Integer(_) => Ok(Self::Integer),
            ItemDefinitionType::Number(_) => Ok(Self::Scalar),
            ItemDefinitionType::NumberWithUnits(value) => {
                Ok(Self::ScalarWithUnit(value.preferred_units()))
            }
            ItemDefinitionType::String(_) => Ok(Self::String),
            ItemDefinitionType::Table(_) => Ok(Self::Table),
            ItemDefinitionType::TableWithUnits(table) => Ok(Self::TableWithUnits(
                table
                    .iter()
                    .map(|(_, column)| column.preferred_units())
                    .collect(),
            )),
            ItemDefinitionType::Unit(value) => Ok(Self::Unit(value.unit_family())),
            ItemDefinitionType::Map(_)
            | ItemDefinitionType::Tab(_)
            | ItemDefinitionType::Separator(_) => Err(UnsupportedParameterDefinition),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ParameterValueType, UnsupportedParameterDefinition};
    use datastore::definition::{
        ItemDefinitionType, NumberWithUnitsDefinition, SeparatorDefinition,
    };
    use units::UnitId;

    #[test]
    fn preserves_preferred_unit_from_datastore_definition() {
        let definition = ItemDefinitionType::NumberWithUnits(NumberWithUnitsDefinition::new(
            "duration",
            UnitId::Time_Second,
        ));

        assert_eq!(
            ParameterValueType::try_from(&definition),
            Ok(ParameterValueType::ScalarWithUnit(UnitId::Time_Second))
        );
    }

    #[test]
    fn rejects_structural_items_as_parameters() {
        let definition = ItemDefinitionType::Separator(SeparatorDefinition::new("layout"));

        assert_eq!(
            ParameterValueType::try_from(&definition),
            Err(UnsupportedParameterDefinition)
        );
    }
}
