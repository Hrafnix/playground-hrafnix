use datastore::prelude::*;
use units::UnitFamilyId;

#[test]
fn unit_compile_time_preserves_family_and_default() {
    let unit = const_unit!("Unit", UnitFamilyId::Length);
    let default = const_unit!(
        "Unit default",
        UnitFamilyId::Length,
        default = "u_length_meter"
    );

    assert_eq!(unit.description(), "Unit");
    assert_eq!(unit.unit_family(), UnitFamilyId::Length);
    assert_eq!(unit.default_value(), "");
    assert_eq!(unit.into_definition().default_value(), "");
    assert_eq!(default.default_value(), "u_length_meter");
    assert_eq!(
        default.into_definition().unit_family(),
        UnitFamilyId::Length
    );
}
