use datastore::prelude::*;
use units::UnitId;

#[test]
fn test_frozen_number_with_units() {
    let frozen = NumberWithUnitsFrozen::new(NumberWithUnitsDefinition::new(
        "A number parameter",
        UnitId::Length_Meter,
    ));

    assert_eq!(frozen.definition().description(), "A number parameter");
    assert_eq!(frozen.value(), "");
    assert_eq!(frozen.units(), UnitId::Length_Meter.string_id().as_str());
    assert_ne!(frozen.hash(), [0u8; 32]);
}

#[test]
fn test_frozen_number_with_units_with_default() {
    let frozen = NumberWithUnitsFrozen::new(NumberWithUnitsDefinition::new_with_default(
        "A number parameter",
        "5.0",
        UnitId::Length_Meter,
    ));

    assert_eq!(frozen.definition().description(), "A number parameter");
    assert_eq!(frozen.definition().default_value(), "5.0");
    assert_eq!(frozen.value(), "5.0");
    assert_eq!(frozen.units(), UnitId::Length_Meter.string_id().as_str());
    assert_ne!(frozen.hash(), [0u8; 32]);
}
