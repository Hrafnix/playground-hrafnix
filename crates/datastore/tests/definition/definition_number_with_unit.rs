use datastore::prelude::*;
use units::UnitId;

#[test]
fn test_definition_number() {
    // Why: Test number definition creation and definition.
    let def = NumberWithUnitsDefinition::new("A number parameter", UnitId::None);

    // Check the various data items of the number definition.
    assert_eq!(def.description(), "A number parameter");
    assert_eq!(def.description_ref(), "A number parameter");
    assert_eq!(def.constraint(), NumberConstraintEnum::None);
    assert_eq!(def.constraint_ref(), &NumberConstraintEnum::None);
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
    assert_eq!(def.preferred_units(), UnitId::None);
    assert_eq!(def.preferred_units_ref(), &UnitId::None);
    let unit_keys: Vec<ShareableString> = def.unit_keys();
    assert_eq!(unit_keys.len(), 1);
    assert_eq!(unit_keys[0], "u_none");
    let unit_descriptions: Vec<ShareableString> = def.unit_descriptions();
    assert_eq!(unit_descriptions.len(), 1);
    assert_eq!(unit_descriptions[0], "");
}

#[test]
fn test_definition_number_with_default() {
    // Why: Test number definition creation with a default value.
    let def = NumberWithUnitsDefinition::new_with_default(
        "A Default number parameter",
        "5.0",
        UnitId::Length_Meter,
    );

    // Check the various data items of the number definition.
    assert_eq!(def.description(), "A Default number parameter");
    assert_eq!(def.description_ref(), "A Default number parameter");
    assert_eq!(def.constraint(), NumberConstraintEnum::None);
    assert_eq!(def.constraint_ref(), &NumberConstraintEnum::None);
    assert_eq!(def.default_value(), "5.0");
    assert_eq!(def.default_value_ref(), "5.0");
    assert_eq!(def.preferred_units(), UnitId::Length_Meter);
    assert_eq!(def.preferred_units_ref(), &UnitId::Length_Meter);
    let unit_keys: Vec<ShareableString> = def.unit_keys();
    assert_eq!(unit_keys.len(), 8);
    assert_eq!(unit_keys[0], "u_length_meter");
    assert_eq!(unit_keys[1], "u_length_kilometer");
    assert_eq!(unit_keys[2], "u_length_centimeter");
    assert_eq!(unit_keys[3], "u_length_millimeter");
    assert_eq!(unit_keys[4], "u_length_foot");
    assert_eq!(unit_keys[5], "u_length_inch");
    assert_eq!(unit_keys[6], "u_length_yard");
    assert_eq!(unit_keys[7], "u_length_mile");
    let unit_descriptions: Vec<ShareableString> = def.unit_descriptions();
    assert_eq!(unit_descriptions.len(), 8);
    assert_eq!(unit_descriptions[0], "m");
    assert_eq!(unit_descriptions[1], "km");
    assert_eq!(unit_descriptions[2], "cm");
    assert_eq!(unit_descriptions[3], "mm");
    assert_eq!(unit_descriptions[4], "ft");
    assert_eq!(unit_descriptions[5], "in");
    assert_eq!(unit_descriptions[6], "yd");
    assert_eq!(unit_descriptions[7], "mi");
}

#[test]
fn test_definition_number_with_min_constraint() {
    // Why: Test number definition creation with a minimum constraint.
    let def = NumberWithUnitsDefinition::new_with_constraint(
        "A number parameter",
        NumberConstraint::min(0.0, true),
        UnitId::Temperature_Celsius,
    );

    // Check the various data items of the number definition.
    assert_eq!(def.description(), "A number parameter");
    assert_eq!(def.description_ref(), "A number parameter");
    assert_eq!(
        def.constraint(),
        NumberConstraintEnum::Min {
            min: 0.0,
            inclusive: true
        }
    );
    assert_eq!(
        def.constraint_ref(),
        &NumberConstraintEnum::Min {
            min: 0.0,
            inclusive: true
        }
    );
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
    assert_eq!(def.preferred_units(), UnitId::Temperature_Celsius);
    assert_eq!(def.preferred_units_ref(), &UnitId::Temperature_Celsius);
    let unit_keys: Vec<ShareableString> = def.unit_keys();
    assert_eq!(unit_keys.len(), 3);
    assert_eq!(unit_keys[0], "u_temperature_celsius");
    assert_eq!(unit_keys[1], "u_temperature_fahrenheit");
    assert_eq!(unit_keys[2], "u_temperature_kelvin");
    let unit_descriptions: Vec<ShareableString> = def.unit_descriptions();
    assert_eq!(unit_descriptions.len(), 3);
    assert_eq!(unit_descriptions[0], "°C");
    assert_eq!(unit_descriptions[1], "°F");
    assert_eq!(unit_descriptions[2], "K");
}

#[test]
fn test_definition_number_with_max_constraint() {
    // Why: Test number definition creation with a maximum constraint.
    let def = NumberWithUnitsDefinition::new_with_constraint(
        "A number parameter",
        NumberConstraint::max(10.0, true),
        UnitId::LuminousIntensity_InternationalCandle,
    );

    // Check the various data items of the number definition.
    assert_eq!(def.description(), "A number parameter");
    assert_eq!(def.description_ref(), "A number parameter");
    assert_eq!(
        def.constraint(),
        NumberConstraintEnum::Max {
            max: 10.0,
            inclusive: true
        }
    );
    assert_eq!(
        def.constraint_ref(),
        &NumberConstraintEnum::Max {
            max: 10.0,
            inclusive: true
        }
    );
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
    assert_eq!(
        def.preferred_units(),
        UnitId::LuminousIntensity_InternationalCandle
    );
    assert_eq!(
        def.preferred_units_ref(),
        &UnitId::LuminousIntensity_InternationalCandle
    );
    let unit_keys: Vec<ShareableString> = def.unit_keys();
    assert_eq!(unit_keys.len(), 6);
    assert_eq!(unit_keys[0], "u_luminous_intensity_candela");
    assert_eq!(unit_keys[1], "u_luminous_intensity_millicandela");
    assert_eq!(unit_keys[2], "u_luminous_intensity_kilocandela");
    assert_eq!(unit_keys[3], "u_luminous_intensity_hefnerkerze");
    assert_eq!(unit_keys[4], "u_luminous_intensity_international_candle");
    assert_eq!(unit_keys[5], "u_luminous_intensity_decimal_candle");
    let unit_descriptions: Vec<ShareableString> = def.unit_descriptions();
    assert_eq!(unit_descriptions.len(), 6);
    assert_eq!(unit_descriptions[0], "cd");
    assert_eq!(unit_descriptions[1], "mcd");
    assert_eq!(unit_descriptions[2], "kcd");
    assert_eq!(unit_descriptions[3], "hk");
    assert_eq!(unit_descriptions[4], "ic");
    assert_eq!(unit_descriptions[5], "dc");
}

#[test]
fn test_definition_number_with_range_constraint() {
    // Why: Test number definition creation with a range constraint.
    let def = NumberWithUnitsDefinition::new_with_constraint(
        "A number parameter",
        NumberConstraint::range(0.0, 10.0, true, true),
        UnitId::Mass_Gram,
    );

    // Check the various data items of the number definition.
    assert_eq!(def.description(), "A number parameter");
    assert_eq!(def.description_ref(), "A number parameter");
    assert_eq!(
        def.constraint(),
        NumberConstraintEnum::Range {
            min: 0.0,
            max: 10.0,
            min_inclusive: true,
            max_inclusive: true
        }
    );
    assert_eq!(
        def.constraint_ref(),
        &NumberConstraintEnum::Range {
            min: 0.0,
            max: 10.0,
            min_inclusive: true,
            max_inclusive: true
        }
    );
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
    assert_eq!(def.preferred_units(), UnitId::Mass_Gram);
    assert_eq!(def.preferred_units_ref(), &UnitId::Mass_Gram);
    let unit_keys: Vec<ShareableString> = def.unit_keys();
    assert_eq!(unit_keys.len(), 6);
    assert_eq!(unit_keys[0], "u_mass_kilogram");
    assert_eq!(unit_keys[1], "u_mass_gram");
    assert_eq!(unit_keys[2], "u_mass_pound");
    assert_eq!(unit_keys[3], "u_mass_ounce");
    assert_eq!(unit_keys[4], "u_mass_tonne");
    assert_eq!(unit_keys[5], "u_mass_stone");
    let unit_descriptions: Vec<ShareableString> = def.unit_descriptions();
    assert_eq!(unit_descriptions.len(), 6);
    assert_eq!(unit_descriptions[0], "kg");
    assert_eq!(unit_descriptions[1], "g");
    assert_eq!(unit_descriptions[2], "lb");
    assert_eq!(unit_descriptions[3], "oz");
    assert_eq!(unit_descriptions[4], "t");
    assert_eq!(unit_descriptions[5], "st");
}

#[test]
fn test_definition_number_with_swap_range_constraint() {
    // Why: Test number definition creation with a swapped range constraint.
    let def = NumberWithUnitsDefinition::new_with_constraint(
        "A number parameter",
        NumberConstraint::range(10.0, 0.0, true, true),
        UnitId::Amount_Millimole,
    );

    // Check the various data items of the number definition.
    assert_eq!(def.description(), "A number parameter");
    assert_eq!(def.description_ref(), "A number parameter");
    assert_eq!(
        def.constraint(),
        NumberConstraintEnum::Range {
            min: 0.0,
            max: 10.0,
            min_inclusive: true,
            max_inclusive: true
        }
    );
    assert_eq!(
        def.constraint_ref(),
        &NumberConstraintEnum::Range {
            min: 0.0,
            max: 10.0,
            min_inclusive: true,
            max_inclusive: true
        }
    );
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
    assert_eq!(def.preferred_units(), UnitId::Amount_Millimole);
    assert_eq!(def.preferred_units_ref(), &UnitId::Amount_Millimole);
    let unit_keys: Vec<ShareableString> = def.unit_keys();
    assert_eq!(unit_keys.len(), 6);
    assert_eq!(unit_keys[0], "u_amount_mole");
    assert_eq!(unit_keys[1], "u_amount_millimole");
    assert_eq!(unit_keys[2], "u_amount_micromole");
    assert_eq!(unit_keys[3], "u_amount_nanomole");
    assert_eq!(unit_keys[4], "u_amount_picomole");
    assert_eq!(unit_keys[5], "u_amount_kilomole");
    let unit_descriptions: Vec<ShareableString> = def.unit_descriptions();
    assert_eq!(unit_descriptions.len(), 6);
    assert_eq!(unit_descriptions[0], "mol");
    assert_eq!(unit_descriptions[1], "mmol");
    assert_eq!(unit_descriptions[2], "μmol");
    assert_eq!(unit_descriptions[3], "nmol");
    assert_eq!(unit_descriptions[4], "pmol");
    assert_eq!(unit_descriptions[5], "kmol");
}

#[test]
fn test_definition_number_with_degenerate_range_constraint() {
    // Why: `NumberConstraint::range` must widen a degenerate (zero-width) range
    // by `f64::EPSILON` on each side, rather than allowing `min == max`.
    let def = NumberWithUnitsDefinition::new_with_constraint(
        "A number parameter",
        NumberConstraint::range(5.0, 5.0, true, true),
        UnitId::Time_Second,
    );

    assert_eq!(
        def.constraint(),
        NumberConstraintEnum::Range {
            min: 5.0 - f64::EPSILON,
            max: 5.0 + f64::EPSILON,
            min_inclusive: true,
            max_inclusive: true
        }
    );
}

#[test]
fn test_number_constraint_deserialize_normalizes_swapped_range() {
    // Why: `NumberConstraint::range` swaps `min`/`max` when `min > max`, but that
    // guard must also hold when a constraint is deserialized directly (e.g. from a
    // saved definition file), not just when constructed via the `range` function.
    let json = serde_json::json!({
        "constraint_enum": {
            "Range": {
                "min": 10.0,
                "max": 0.0,
                "min_inclusive": true,
                "max_inclusive": false
            }
        }
    });

    let constraint: NumberConstraint = serde_json::from_value(json).unwrap();
    let expected = NumberConstraint::range(10.0, 0.0, true, false);

    assert_eq!(constraint, expected);
}

#[test]
fn test_definition_number_with_constraint_and_default() {
    // Why: Test number definition creation with a constraint and a default value.
    let def = NumberWithUnitsDefinition::new_with_constraint_and_default(
        "A number parameter",
        NumberConstraint::max(10.0, true),
        "5.0",
        UnitId::Time_Second,
    );

    // Check the various data items of the number definition.
    assert_eq!(def.description(), "A number parameter");
    assert_eq!(
        def.constraint(),
        NumberConstraintEnum::Max {
            max: 10.0,
            inclusive: true
        }
    );
    assert_eq!(def.default_value(), "5.0");
    assert_eq!(def.preferred_units(), UnitId::Time_Second);
    assert_eq!(def.preferred_units_ref(), &UnitId::Time_Second);
    let unit_keys: Vec<ShareableString> = def.unit_keys();
    assert_eq!(unit_keys.len(), 9);
    assert_eq!(unit_keys[0], "u_time_second");
    assert_eq!(unit_keys[1], "u_time_minute");
    assert_eq!(unit_keys[2], "u_time_hour");
    assert_eq!(unit_keys[3], "u_time_day");
    assert_eq!(unit_keys[4], "u_time_week");
    assert_eq!(unit_keys[5], "u_time_year");
    assert_eq!(unit_keys[6], "u_time_millisecond");
    assert_eq!(unit_keys[7], "u_time_microsecond");
    assert_eq!(unit_keys[8], "u_time_nanosecond");
    let unit_descriptions: Vec<ShareableString> = def.unit_descriptions();
    assert_eq!(unit_descriptions.len(), 9);
    assert_eq!(unit_descriptions[0], "s");
    assert_eq!(unit_descriptions[1], "min");
    assert_eq!(unit_descriptions[2], "h");
    assert_eq!(unit_descriptions[3], "day");
    assert_eq!(unit_descriptions[4], "week");
    assert_eq!(unit_descriptions[5], "year");
    assert_eq!(unit_descriptions[6], "ms");
    assert_eq!(unit_descriptions[7], "μs");
    assert_eq!(unit_descriptions[8], "ns");
}

#[test]
fn test_definition_number_equality() {
    // Why: Test number definition equality.
    let def_1 = NumberWithUnitsDefinition::new_with_constraint_and_default(
        "A number parameter",
        NumberConstraint::max(10.0, true),
        "5",
        UnitId::None,
    );
    let def_2 = NumberWithUnitsDefinition::new_with_constraint_and_default(
        "A number parameter",
        NumberConstraint::max(10.0, true),
        "5",
        UnitId::None,
    );
    let def_3 = NumberWithUnitsDefinition::new_with_constraint_and_default(
        "A number parameter",
        NumberConstraint::max(10.0, true),
        "6",
        UnitId::None,
    );

    // Check equality of the three number definitions.
    assert_eq!(def_1, def_2);
    assert_eq!(def_1, &def_2);
    assert_eq!(&def_1, def_2);
    assert_eq!(&def_1, &def_2);

    assert_ne!(def_1, def_3);
    assert_ne!(&def_1, def_3);
    assert_ne!(def_1, &def_3);
    assert_ne!(&def_1, &def_3);
}
