use datastore::prelude::*;
use units::{UnitFamilyId, UnitId};

#[test]
fn test_frozen_unit() {
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
