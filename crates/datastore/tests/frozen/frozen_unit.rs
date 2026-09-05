use datastore::prelude::*;
use units::{UnitFamilyId, UnitId};

#[test]
fn test_frozen_unit() {
    let frozen = UnitFrozen::new(UnitDefinition::new("Length unit", UnitFamilyId::Length));

    assert_eq!(frozen.definition().description(), "Length unit");
    assert_eq!(frozen.definition().unit_family(), UnitFamilyId::Length);
    assert_eq!(frozen.value(), "");
    assert_ne!(frozen.hash(), [0u8; 32]);
}

#[test]
fn test_frozen_unit_with_default() {
    let definition = UnitDefinition::new_with_default(
        "Length unit",
        UnitFamilyId::Length,
        UnitId::Length_Meter.string_id().as_str(),
    );
    let frozen = UnitFrozen::new(definition.clone());

    assert_eq!(frozen.definition(), &definition);
    assert_eq!(frozen.value(), UnitId::Length_Meter.string_id().as_str());
    assert_ne!(frozen.hash(), [0; 32]);

    let item = ItemFrozen::Unit(frozen.clone());
    assert_eq!(item.get_unit(), Some(frozen));
}
