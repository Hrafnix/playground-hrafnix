use hrafnix_datastore::prelude::*;
use hrafnix_units::UnitId;

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
    assert_eq!(unit_keys.len(), 11);
    assert_eq!(unit_keys[0], "u_length_meter");
    assert_eq!(unit_keys[1], "u_length_picometer");
    assert_eq!(unit_keys[2], "u_length_nanometer");
    assert_eq!(unit_keys[3], "u_length_micrometer");
    assert_eq!(unit_keys[4], "u_length_millimeter");
    assert_eq!(unit_keys[5], "u_length_centimeter");
    assert_eq!(unit_keys[6], "u_length_kilometer");
    assert_eq!(unit_keys[7], "u_length_foot");
    assert_eq!(unit_keys[8], "u_length_inch");
    assert_eq!(unit_keys[9], "u_length_yard");
    assert_eq!(unit_keys[10], "u_length_mile");
    let unit_descriptions: Vec<ShareableString> = def.unit_descriptions();
    assert_eq!(unit_descriptions.len(), 11);
    assert_eq!(unit_descriptions[0], "m");
    assert_eq!(unit_descriptions[1], "pm");
    assert_eq!(unit_descriptions[2], "nm");
    assert_eq!(unit_descriptions[3], "μm");
    assert_eq!(unit_descriptions[4], "mm");
    assert_eq!(unit_descriptions[5], "cm");
    assert_eq!(unit_descriptions[6], "km");
    assert_eq!(unit_descriptions[7], "ft");
    assert_eq!(unit_descriptions[8], "in");
    assert_eq!(unit_descriptions[9], "yd");
    assert_eq!(unit_descriptions[10], "mi");
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
    assert_eq!(unit_keys.len(), 7);
    assert_eq!(unit_keys[0], "u_temperature_kelvin");
    assert_eq!(unit_keys[1], "u_temperature_nanokelvin");
    assert_eq!(unit_keys[2], "u_temperature_microkelvin");
    assert_eq!(unit_keys[3], "u_temperature_millikelvin");
    assert_eq!(unit_keys[4], "u_temperature_kilokelvin");
    assert_eq!(unit_keys[5], "u_temperature_celsius");
    assert_eq!(unit_keys[6], "u_temperature_fahrenheit");
    let unit_descriptions: Vec<ShareableString> = def.unit_descriptions();
    assert_eq!(unit_descriptions.len(), 7);
    assert_eq!(unit_descriptions[0], "K");
    assert_eq!(unit_descriptions[1], "nK");
    assert_eq!(unit_descriptions[2], "μK");
    assert_eq!(unit_descriptions[3], "mK");
    assert_eq!(unit_descriptions[4], "kK");
    assert_eq!(unit_descriptions[5], "°C");
    assert_eq!(unit_descriptions[6], "°F");
}

#[test]
fn test_definition_number_with_max_constraint() {
    // Why: Test number definition creation with a maximum constraint.
    let def = NumberWithUnitsDefinition::new_with_constraint(
        "A number parameter",
        NumberConstraint::max(10.0, true),
        UnitId::Current_Ampere,
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
    assert_eq!(def.preferred_units(), UnitId::Current_Ampere);
    assert_eq!(def.preferred_units_ref(), &UnitId::Current_Ampere);
    let unit_keys: Vec<ShareableString> = def.unit_keys();
    assert_eq!(unit_keys.len(), 6);
    assert_eq!(unit_keys[0], "u_current_ampere");
    assert_eq!(unit_keys[1], "u_current_nanoampere");
    assert_eq!(unit_keys[2], "u_current_microampere");
    assert_eq!(unit_keys[3], "u_current_milliampere");
    assert_eq!(unit_keys[4], "u_current_kiloampere");
    assert_eq!(unit_keys[5], "u_current_megaampere");
    let unit_descriptions: Vec<ShareableString> = def.unit_descriptions();
    assert_eq!(unit_descriptions.len(), 6);
    assert_eq!(unit_descriptions[0], "A");
    assert_eq!(unit_descriptions[1], "nA");
    assert_eq!(unit_descriptions[2], "μA");
    assert_eq!(unit_descriptions[3], "mA");
    assert_eq!(unit_descriptions[4], "kA");
    assert_eq!(unit_descriptions[5], "MA");
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
    assert_eq!(unit_keys.len(), 11);
    assert_eq!(unit_keys[0], "u_mass_kilogram");
    assert_eq!(unit_keys[1], "u_mass_gram");
    assert_eq!(unit_keys[2], "u_mass_picogram");
    assert_eq!(unit_keys[3], "u_mass_nanogram");
    assert_eq!(unit_keys[4], "u_mass_microgram");
    assert_eq!(unit_keys[5], "u_mass_milligram");
    assert_eq!(unit_keys[6], "u_mass_megagram");
    assert_eq!(unit_keys[7], "u_mass_tonne");
    assert_eq!(unit_keys[8], "u_mass_pound");
    assert_eq!(unit_keys[9], "u_mass_ounce");
    assert_eq!(unit_keys[10], "u_mass_stone");
    let unit_descriptions: Vec<ShareableString> = def.unit_descriptions();
    assert_eq!(unit_descriptions.len(), 11);
    assert_eq!(unit_descriptions[0], "kg");
    assert_eq!(unit_descriptions[1], "g");
    assert_eq!(unit_descriptions[2], "pg");
    assert_eq!(unit_descriptions[3], "ng");
    assert_eq!(unit_descriptions[4], "μg");
    assert_eq!(unit_descriptions[5], "mg");
    assert_eq!(unit_descriptions[6], "Mg");
    assert_eq!(unit_descriptions[7], "t");
    assert_eq!(unit_descriptions[8], "lb");
    assert_eq!(unit_descriptions[9], "oz");
    assert_eq!(unit_descriptions[10], "st");
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
    assert_eq!(unit_keys[1], "u_amount_picomole");
    assert_eq!(unit_keys[2], "u_amount_nanomole");
    assert_eq!(unit_keys[3], "u_amount_micromole");
    assert_eq!(unit_keys[4], "u_amount_millimole");
    assert_eq!(unit_keys[5], "u_amount_kilomole");
    let unit_descriptions: Vec<ShareableString> = def.unit_descriptions();
    assert_eq!(unit_descriptions.len(), 6);
    assert_eq!(unit_descriptions[0], "mol");
    assert_eq!(unit_descriptions[1], "pmol");
    assert_eq!(unit_descriptions[2], "nmol");
    assert_eq!(unit_descriptions[3], "μmol");
    assert_eq!(unit_descriptions[4], "mmol");
    assert_eq!(unit_descriptions[5], "kmol");
}

#[test]
fn test_definition_number_with_degenerate_range_constraint() {
    // Why: An inclusive single-value range `[5.0, 5.0]` is valid and must be kept as-is.
    let def = NumberWithUnitsDefinition::new_with_constraint(
        "A number parameter",
        NumberConstraint::range(5.0, 5.0, true, true),
        UnitId::Time_Second,
    );

    assert_eq!(
        def.constraint(),
        NumberConstraintEnum::Range {
            min: 5.0,
            max: 5.0,
            min_inclusive: true,
            max_inclusive: true
        }
    );
}

#[test]
fn test_definition_number_with_empty_range_constraint_becomes_inclusive() {
    // Why: Ranges containing no representable `f64` (equal bounds with an exclusive end,
    // or exclusive adjacent floats) must be made inclusive on both ends so they stay satisfiable.
    let next = f64::from_bits(5.0_f64.to_bits() + 1);
    for (a, b, a_inclusive, b_inclusive) in [
        (5.0, 5.0, false, true),
        (5.0, 5.0, true, false),
        (5.0, 5.0, false, false),
        (5.0, next, false, false),
    ] {
        let def = NumberWithUnitsDefinition::new_with_constraint(
            "A number parameter",
            NumberConstraint::range(a, b, a_inclusive, b_inclusive),
            UnitId::Time_Second,
        );
        assert_eq!(
            def.constraint(),
            NumberConstraintEnum::Range {
                min: a,
                max: b,
                min_inclusive: true,
                max_inclusive: true
            }
        );
    }
}

#[test]
fn test_definition_number_with_non_empty_exclusive_range_constraint_is_kept() {
    // Why: An exclusive range with at least one float strictly between the bounds is valid.
    let two_up = f64::from_bits(5.0_f64.to_bits() + 2);
    let def = NumberWithUnitsDefinition::new_with_constraint(
        "A number parameter",
        NumberConstraint::range(5.0, two_up, false, false),
        UnitId::Time_Second,
    );
    assert_eq!(
        def.constraint(),
        NumberConstraintEnum::Range {
            min: 5.0,
            max: two_up,
            min_inclusive: false,
            max_inclusive: false
        }
    );
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
    assert_eq!(unit_keys.len(), 11);
    assert_eq!(unit_keys[0], "u_time_second");
    assert_eq!(unit_keys[1], "u_time_picosecond");
    assert_eq!(unit_keys[2], "u_time_nanosecond");
    assert_eq!(unit_keys[3], "u_time_microsecond");
    assert_eq!(unit_keys[4], "u_time_millisecond");
    assert_eq!(unit_keys[5], "u_time_kilosecond");
    assert_eq!(unit_keys[6], "u_time_minute");
    assert_eq!(unit_keys[7], "u_time_hour");
    assert_eq!(unit_keys[8], "u_time_day");
    assert_eq!(unit_keys[9], "u_time_week");
    assert_eq!(unit_keys[10], "u_time_year");
    let unit_descriptions: Vec<ShareableString> = def.unit_descriptions();
    assert_eq!(unit_descriptions.len(), 11);
    assert_eq!(unit_descriptions[0], "s");
    assert_eq!(unit_descriptions[1], "ps");
    assert_eq!(unit_descriptions[2], "ns");
    assert_eq!(unit_descriptions[3], "μs");
    assert_eq!(unit_descriptions[4], "ms");
    assert_eq!(unit_descriptions[5], "ks");
    assert_eq!(unit_descriptions[6], "min");
    assert_eq!(unit_descriptions[7], "h");
    assert_eq!(unit_descriptions[8], "day");
    assert_eq!(unit_descriptions[9], "week");
    assert_eq!(unit_descriptions[10], "year");
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
