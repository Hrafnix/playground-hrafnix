use datastore::prelude::*;

#[test]
fn test_definition_number() {
    // Why: Test number definition creation and definition.
    let def = NumberDefinition::new("A number parameter");

    // Check the various data items of the number definition.
    assert_eq!(def.description(), "A number parameter");
    assert_eq!(def.description_ref(), "A number parameter");
    assert_eq!(def.constraint(), NumberConstraintEnum::None);
    assert_eq!(def.constraint_ref(), &NumberConstraintEnum::None);
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
}

#[test]
fn test_definition_number_with_default() {
    // Why: Test number definition creation with a default value.
    let def = NumberDefinition::new_with_default("A Default number parameter", "5.0");

    // Check the various data items of the number definition.
    assert_eq!(def.description(), "A Default number parameter");
    assert_eq!(def.description_ref(), "A Default number parameter");
    assert_eq!(def.constraint(), NumberConstraintEnum::None);
    assert_eq!(def.constraint_ref(), &NumberConstraintEnum::None);
    assert_eq!(def.default_value(), "5.0");
    assert_eq!(def.default_value_ref(), "5.0");
}

#[test]
fn test_definition_number_with_min_constraint() {
    // Why: Test number definition creation with a minimum constraint.
    let def = NumberDefinition::new_with_constraint(
        "A number parameter",
        NumberConstraint::min(0.0, true),
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
}

#[test]
fn test_definition_number_with_max_constraint() {
    // Why: Test number definition creation with a maximum constraint.
    let def = NumberDefinition::new_with_constraint(
        "A number parameter",
        NumberConstraint::max(10.0, true),
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
}

#[test]
fn test_definition_number_with_range_constraint() {
    // Why: Test number definition creation with a range constraint.
    let def = NumberDefinition::new_with_constraint(
        "A number parameter",
        NumberConstraint::range(0.0, 10.0, true, true),
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
}

#[test]
fn test_definition_number_with_swap_range_constraint() {
    // Why: Test number definition creation with a swapped range constraint.
    let def = NumberDefinition::new_with_constraint(
        "A number parameter",
        NumberConstraint::range(10.0, 0.0, true, true),
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
    let def = NumberDefinition::new_with_constraint_and_default(
        "A number parameter",
        NumberConstraint::max(10.0, true),
        "5.0",
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
}

#[test]
fn test_definition_number_equality() {
    // Why: Test number definition equality.
    let def_1 = NumberDefinition::new_with_constraint_and_default(
        "A number parameter",
        NumberConstraint::max(10.0, true),
        "5",
    );
    let def_2 = NumberDefinition::new_with_constraint_and_default(
        "A number parameter",
        NumberConstraint::max(10.0, true),
        "5",
    );
    let def_3 = NumberDefinition::new_with_constraint_and_default(
        "A number parameter",
        NumberConstraint::max(10.0, true),
        "6",
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
