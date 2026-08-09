use datastore::prelude::*;
use units::{UnitFamilyId, UnitId};

#[test]
fn test_editable_unit_round_trip() {
    let frozen = UnitFrozen::new(UnitDefinition::new_with_default(
        "Length unit",
        UnitFamilyId::Length,
        UnitId::Length_Meter.string_id().as_str(),
    ));
    let mut editable = frozen.thaw();

    editable.set(UnitId::Length_Foot.string_id().as_str());
    let refrozen = editable.freeze();

    assert_eq!(refrozen.value(), UnitId::Length_Foot.string_id().as_str());
    assert_ne!(refrozen.hash(), frozen.hash());

    let mut item = ItemEditable::Unit(refrozen.thaw());
    assert_eq!(
        item.get_unit().map(UnitEditable::value),
        Some(UnitId::Length_Foot.string_id().into())
    );
    item.get_mut_unit()
        .expect("unit item")
        .set(UnitId::Length_Meter.string_id().as_str());
    assert_eq!(
        item.get_unit().map(UnitEditable::value),
        Some(UnitId::Length_Meter.string_id().into())
    );
}
