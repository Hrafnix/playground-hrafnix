use datastore::prelude::*;

#[test]
fn test_definition_integer() {
    // Why: Test integer definition creation and definition.
    let def = IntegerDefinition::new("A integer parameter");

    // Check the various data items of the integer definition.
    assert_eq!(def.description(), "A integer parameter");
    assert_eq!(def.description_ref(), "A integer parameter");
    assert_eq!(def.constraint(), IntegerConstraintEnum::None);
    assert_eq!(def.constraint_ref(), &IntegerConstraintEnum::None);
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
}

#[test]
fn test_definition_integer_with_default() {
    // Why: Test integer definition creation with a default value.
    let def = IntegerDefinition::new_with_default("A integer parameter", "5");

    // Check the various data items of the integer definition.
    assert_eq!(def.description(), "A integer parameter");
    assert_eq!(def.description_ref(), "A integer parameter");
    assert_eq!(def.constraint(), IntegerConstraintEnum::None);
    assert_eq!(def.constraint_ref(), &IntegerConstraintEnum::None);
    assert_eq!(def.default_value(), "5");
    assert_eq!(def.default_value_ref(), "5");
}

#[test]
fn test_definition_integer_with_min_constraint() {
    // Why: Test integer definition creation with a minimum constraint.
    let def = IntegerDefinition::new_with_constraint(
        "A integer parameter",
        IntegerConstraint::min(0, true),
    );

    // Check the various data items of the integer definition.
    assert_eq!(def.description(), "A integer parameter");
    assert_eq!(def.description_ref(), "A integer parameter");
    assert_eq!(
        def.constraint(),
        IntegerConstraintEnum::Min {
            min: 0,
            inclusive: true
        }
    );
    assert_eq!(
        def.constraint_ref(),
        &IntegerConstraintEnum::Min {
            min: 0,
            inclusive: true
        }
    );
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
}

#[test]
fn test_definition_integer_with_max_constraint() {
    // Why: Test integer definition creation with a maximum constraint.
    let def = IntegerDefinition::new_with_constraint(
        "A integer parameter",
        IntegerConstraint::max(10, true),
    );

    // Check the various data items of the integer definition.
    assert_eq!(def.description(), "A integer parameter");
    assert_eq!(def.description_ref(), "A integer parameter");
    assert_eq!(
        def.constraint(),
        IntegerConstraintEnum::Max {
            max: 10,
            inclusive: true
        }
    );
    assert_eq!(
        def.constraint_ref(),
        &IntegerConstraintEnum::Max {
            max: 10,
            inclusive: true
        }
    );
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
}

#[test]
fn test_definition_integer_with_range_constraint() {
    // Why: Test integer definition creation with a range constraint.
    let def = IntegerDefinition::new_with_constraint(
        "A integer parameter",
        IntegerConstraint::range(0, 10, true, true),
    );

    // Check the various data items of the integer definition.
    assert_eq!(def.description(), "A integer parameter");
    assert_eq!(def.description_ref(), "A integer parameter");
    assert_eq!(
        def.constraint(),
        IntegerConstraintEnum::Range {
            min: 0,
            max: 10,
            min_inclusive: true,
            max_inclusive: true
        }
    );
    assert_eq!(
        def.constraint_ref(),
        &IntegerConstraintEnum::Range {
            min: 0,
            max: 10,
            min_inclusive: true,
            max_inclusive: true
        }
    );
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
}

#[test]
fn test_definition_integer_with_swap_range_constraint() {
    // Why: Test integer definition creation with a swapped range constraint.
    let def = IntegerDefinition::new_with_constraint(
        "A integer parameter",
        IntegerConstraint::range(10, 0, true, true),
    );

    // Check the various data items of the integer definition.
    assert_eq!(def.description(), "A integer parameter");
    assert_eq!(def.description_ref(), "A integer parameter");
    assert_eq!(
        def.constraint(),
        IntegerConstraintEnum::Range {
            min: 0,
            max: 10,
            min_inclusive: true,
            max_inclusive: true
        }
    );
    assert_eq!(
        def.constraint_ref(),
        &IntegerConstraintEnum::Range {
            min: 0,
            max: 10,
            min_inclusive: true,
            max_inclusive: true
        }
    );
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
}

#[test]
fn test_definition_integer_with_equal_value_range_constraint() {
    // Why: When both range values are equal, the constraint should always be
    // inclusive on both ends (regardless of the passed inclusivity flags), so it
    // represents exactly that single value rather than a contradictory,
    // unsatisfiable range.
    let def = IntegerDefinition::new_with_constraint(
        "A integer parameter",
        IntegerConstraint::range(5, 5, false, false),
    );

    assert_eq!(
        def.constraint(),
        IntegerConstraintEnum::Range {
            min: 5,
            max: 5,
            min_inclusive: true,
            max_inclusive: true
        }
    );

    // Same result regardless of which flag combination is passed in.
    assert_eq!(
        IntegerConstraint::range(5, 5, true, false),
        IntegerConstraint::range(5, 5, false, true)
    );
}

#[test]
fn test_integer_constraint_deserialize_normalizes_swapped_range() {
    // Why: `IntegerConstraint::range` swaps `min`/`max` when `min > max`, but that
    // guard must also hold when a constraint is deserialized directly (e.g. from a
    // saved definition file), not just when constructed via the `range` function.
    let json = serde_json::json!({
        "constraint_enum": {
            "Range": {
                "min": 10,
                "max": 0,
                "min_inclusive": true,
                "max_inclusive": false
            }
        }
    });

    let constraint: IntegerConstraint = serde_json::from_value(json).unwrap();
    let expected = IntegerConstraint::range(10, 0, true, false);

    assert_eq!(constraint, expected);
}

#[test]
fn test_definition_integer_with_constraint_and_default() {
    // Why: Test integer definition creation with a constraint and a default value.
    let def = IntegerDefinition::new_with_constraint_and_default(
        "A integer parameter",
        IntegerConstraint::max(10, true),
        "5",
    );

    // Check the various data items of the integer definition.
    assert_eq!(def.description(), "A integer parameter");
    assert_eq!(def.description_ref(), "A integer parameter");
    assert_eq!(
        def.constraint(),
        IntegerConstraintEnum::Max {
            max: 10,
            inclusive: true
        }
    );
    assert_eq!(
        def.constraint_ref(),
        &IntegerConstraintEnum::Max {
            max: 10,
            inclusive: true
        }
    );
    assert_eq!(def.default_value(), "5");
    assert_eq!(def.default_value_ref(), "5");
}

#[test]
fn test_definition_integer_equality() {
    // Why: Test integer definition equality.
    let def_1 = IntegerDefinition::new_with_constraint_and_default(
        "A integer parameter",
        IntegerConstraint::max(10, true),
        "5",
    );
    let def_2 = IntegerDefinition::new_with_constraint_and_default(
        "A integer parameter",
        IntegerConstraint::max(10, true),
        "5",
    );
    let def_3 = IntegerDefinition::new_with_constraint_and_default(
        "A integer parameter",
        IntegerConstraint::max(10, true),
        "6",
    );

    // Check equality of the three integer definitions.
    assert_eq!(def_1, def_2);
    assert_eq!(def_1, &def_2);
    assert_eq!(&def_1, def_2);
    assert_eq!(&def_1, &def_2);

    assert_ne!(def_1, def_3);
    assert_ne!(&def_1, def_3);
    assert_ne!(def_1, &def_3);
    assert_ne!(&def_1, &def_3);
}
