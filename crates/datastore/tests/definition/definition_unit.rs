use datastore::prelude::*;
use units::{UnitFamilyId, UnitId};

#[test]
fn test_definition_unit() {
    let definition = UnitDefinition::new("Length unit", UnitFamilyId::Length);

    assert_eq!(definition.description(), "Length unit");
    assert_eq!(definition.description_ref(), "Length unit");
    assert_eq!(definition.unit_family(), UnitFamilyId::Length);
    assert_eq!(definition.default_value(), "");
    assert!(definition.contains(UnitId::Length_Meter.string_id().as_str()));
    assert!(!definition.contains(UnitId::Time_Second.string_id().as_str()));
    assert_eq!(definition.ids(), UnitFamilyId::Length.unit_ids());
    assert_eq!(
        definition.keys(),
        UnitFamilyId::Length
            .unit_ids()
            .iter()
            .map(|unit_id| unit_id.string_id().into())
            .collect::<Vec<UnitKey>>()
    );
}

#[test]
fn test_definition_unit_with_default() {
    let definition = UnitDefinition::new_with_default(
        "Length unit",
        UnitFamilyId::Length,
        UnitId::Length_Meter.string_id().as_str(),
    );

    assert_eq!(
        definition.default_value(),
        UnitId::Length_Meter.string_id().as_str()
    );
    assert_eq!(
        definition.default_value_ref(),
        UnitId::Length_Meter.string_id().as_str()
    );
}
