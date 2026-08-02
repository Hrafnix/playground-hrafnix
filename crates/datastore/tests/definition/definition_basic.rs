use datastore::prelude::*;

#[test]
fn test_basic_definition_string() {
    // Why: Test basic string definition creation and definition.
    let def = StringDefinition::new("A string parameter");

    // Check the various data items of the string definition.
    assert_eq!(def.description(), "A string parameter");
    assert_eq!(def.default_value(), "");
}

#[test]
fn test_basic_definition_string_with_default() {
    // Why: Test basic string definition creation with a default value.
    let def = StringDefinition::new_with_default("A string parameter", "default value");

    // Check the various data items of the string definition.
    assert_eq!(def.description(), "A string parameter");
    assert_eq!(def.default_value(), "default value");
}

#[test]
fn test_basic_definition_boolean() {
    // Why: Test basic boolean definition creation and definition.
    let def_1 = BooleanDefinition::new("A Boolean parameter");

    // Check the various data items of the boolean definition.
    assert_eq!(def_1.description(), "A Boolean parameter");
    assert_eq!(def_1.description_ref(), "A Boolean parameter");
    assert_eq!(def_1.default_value(), "");
    assert_eq!(def_1.default_value_ref(), "");
    assert_eq!(def_1.ids(), vec!["true", "false"]);
    assert_eq!(def_1.descriptions(), vec!["True", "False"]);
}

#[test]
fn test_basic_definition_boolean_with_default() {
    // Why: Test basic boolean definition creation with a default value.
    let def = BooleanDefinition::new_with_default("A Default Boolean parameter", true);

    // Check the various data items of the boolean definition.
    assert_eq!(def.description(), "A Default Boolean parameter");
    assert_eq!(def.description_ref(), "A Default Boolean parameter");
    assert_eq!(def.default_value(), "true");
    assert_eq!(def.default_value_ref(), "true");
    assert_eq!(def.ids(), vec!["true", "false"]);
    assert_eq!(def.descriptions(), vec!["True", "False"]);
}

#[test]
fn test_basic_definition_boolean_equality() {
    // Why: Test basic boolean definition equality.
    let def_1 = BooleanDefinition::new_with_default("A Default Boolean parameter", true);
    let def_2 = BooleanDefinition::new_with_default("A Default Boolean parameter", true);
    let def_3 = BooleanDefinition::new_with_default("A Default Boolean parameter", false);

    // Check equality of the three boolean definitions.
    assert_eq!(def_1, def_2);
    assert_eq!(def_1, &def_2);
    assert_eq!(&def_1, def_2);
    assert_eq!(&def_1, &def_2);

    assert_ne!(def_1, def_3);
    assert_ne!(&def_1, def_3);
    assert_ne!(def_1, &def_3);
    assert_ne!(&def_1, &def_3);
}

#[test]
fn test_basic_definition_integer() {
    // Why: Test basic integer definition creation and definition.
    let def = IntegerDefinition::new("A integer parameter");

    // Check the various data items of the integer definition.
    assert_eq!(def.description(), "A integer parameter");
    assert_eq!(def.description_ref(), "A integer parameter");
    assert_eq!(def.constraint(), IntegerConstraint::None);
    assert_eq!(def.constraint_ref(), &IntegerConstraint::None);
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
}

#[test]
fn test_basic_definition_integer_with_default() {
    // Why: Test basic integer definition creation with a default value.
    let def = IntegerDefinition::new_with_default("A integer parameter", "5");

    // Check the various data items of the integer definition.
    assert_eq!(def.description(), "A integer parameter");
    assert_eq!(def.description_ref(), "A integer parameter");
    assert_eq!(def.constraint(), IntegerConstraint::None);
    assert_eq!(def.constraint_ref(), &IntegerConstraint::None);
    assert_eq!(def.default_value(), "5");
    assert_eq!(def.default_value_ref(), "5");
}

#[test]
fn test_basic_definition_integer_with_constraint() {
    // Why: Test basic integer definition creation with a constraint.
    let def = IntegerDefinition::new_with_constraint(
        "A integer parameter",
        IntegerConstraint::Min {
            min: 0,
            inclusive: true,
        },
    );

    // Check the various data items of the integer definition.
    assert_eq!(def.description(), "A integer parameter");
    assert_eq!(def.description_ref(), "A integer parameter");
    assert_eq!(
        def.constraint(),
        IntegerConstraint::Min {
            min: 0,
            inclusive: true
        }
    );
    assert_eq!(def.constraint_ref(), &IntegerConstraint::Min {
        min: 0,
        inclusive: true
    });
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
}

#[test]
fn test_basic_definition_integer_with_constraint_and_default() {
    // Why: Test basic integer definition creation with a constraint and a default value.
    let def = IntegerDefinition::new_with_constraint_and_default(
        "A integer parameter",
        IntegerConstraint::Max {
            max: 10,
            inclusive: true,
        },
        "5",
    );

    // Check the various data items of the integer definition.
    assert_eq!(def.description(), "A integer parameter");
    assert_eq!(def.description_ref(), "A integer parameter");
    assert_eq!(
        def.constraint(),
        IntegerConstraint::Max {
            max: 10,
            inclusive: true
        }
    );
    assert_eq!(def.constraint_ref(), &IntegerConstraint::Max {
        max: 10,
        inclusive: true
    });
    assert_eq!(def.default_value(), "5");
    assert_eq!(def.default_value_ref(), "5");
}

#[test]
fn test_basic_definition_integer_equality() {
    // Why: Test basic integer definition equality.
    let def_1 = IntegerDefinition::new_with_constraint_and_default(
        "A integer parameter",
        IntegerConstraint::Max {
            max: 10,
            inclusive: true,
        },
        "5",
    );
    let def_2 = IntegerDefinition::new_with_constraint_and_default(
        "A integer parameter",
        IntegerConstraint::Max {
            max: 10,
            inclusive: true,
        },
        "5",
    );
    let def_3 = IntegerDefinition::new_with_constraint_and_default(
        "A integer parameter",
        IntegerConstraint::Max {
            max: 10,
            inclusive: true,
        },
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

#[test]
fn test_basic_definition_number() {
    // Why: Test basic number definition creation and definition.
    let def = NumberDefinition::new("A number parameter");

    // Check the various data items of the number definition.
    assert_eq!(def.description(), "A number parameter");
    assert_eq!(def.constraint(), NumberConstraint::None);
    assert_eq!(def.default_value(), "");
}

#[test]
fn test_basic_definition_number_with_default() {
    // Why: Test basic number definition creation with a default value.
    let def = NumberDefinition::new_with_default("A number parameter", "5.0");

    // Check the various data items of the number definition.
    assert_eq!(def.description(), "A number parameter");
    assert_eq!(def.constraint(), NumberConstraint::None);
    assert_eq!(def.default_value(), "5.0");
}

#[test]
fn test_basic_definition_number_with_constraint() {
    // Why: Test basic number definition creation with a constraint.
    let def = NumberDefinition::new_with_constraint(
        "A number parameter",
        NumberConstraint::Min {
            min: 0.0,
            inclusive: true,
        },
    );

    // Check the various data items of the number definition.
    assert_eq!(def.description(), "A number parameter");
    assert_eq!(
        def.constraint(),
        NumberConstraint::Min {
            min: 0.0,
            inclusive: true
        }
    );
    assert_eq!(def.default_value(), "");
}

#[test]
fn test_basic_definition_number_with_constraint_and_default() {
    // Why: Test basic number definition creation with a constraint and a default value.
    let def = NumberDefinition::new_with_constraint_and_default(
        "A number parameter",
        NumberConstraint::Max {
            max: 10.0,
            inclusive: true,
        },
        "5.0",
    );

    // Check the various data items of the number definition.
    assert_eq!(def.description(), "A number parameter");
    assert_eq!(
        def.constraint(),
        NumberConstraint::Max {
            max: 10.0,
            inclusive: true
        }
    );
    assert_eq!(def.default_value(), "5.0");
}

#[test]
fn test_basic_definition_file() {
    // Why: Test basic file definition creation and definition.
    let def = FileDefinition::new("A file parameter", "txt", false);

    // Check the various data items of the file definition.
    assert_eq!(def.description(), "A file parameter");
    assert_eq!(def.description_ref(), "A file parameter");
    assert_eq!(def.extension_filter(), "txt");
    assert_eq!(def.extension_filter_ref(), "txt");
    assert!(!def.bundle_on_archive());
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
}

#[test]
fn test_basic_definition_file_with_default() {
    // Why: Test basic file definition creation with a default value.
    let def = FileDefinition::new_with_default("A Default file parameter", "exe", true, "test.exe");

    // Check the various data items of the file definition.
    assert_eq!(def.description(), "A Default file parameter");
    assert_eq!(def.description_ref(), "A Default file parameter");
    assert_eq!(def.extension_filter(), "exe");
    assert_eq!(def.extension_filter_ref(), "exe");
    assert!(def.bundle_on_archive());
    assert_eq!(def.default_value(), "test.exe");
    assert_eq!(def.default_value_ref(), "test.exe");
}

#[test]
fn test_basic_definition_file_equality() {
    // Why: Test basic file definition equality.
    let def_1 = FileDefinition::new_with_default("A file parameter", "txt", true, "test.txt");
    let def_2 = FileDefinition::new_with_default("A file parameter", "txt", true, "test.txt");
    let def_3 = FileDefinition::new_with_default("A file parameter", "txt", true, "test2.txt");
    let def_4 = FileDefinition::new_with_default("A file parameter", "txt", false, "test.txt");
    let def_5 = FileDefinition::new_with_default("A file parameter", "exe", true, "test.txt");
    let def_6 = FileDefinition::new_with_default("A new file parameter", "txt", true, "test.txt");

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_ne!(def_1, def_4);
    assert_ne!(def_1, def_5);
    assert_ne!(def_1, def_6);
    assert_eq!(def_1, &def_2);
    assert_ne!(&def_1, def_3);
    assert_ne!(def_1, &def_4);
    assert_ne!(&def_1, def_5);
    assert_ne!(&def_1, &def_6);
}

#[test]
fn test_basic_definition_choice() {
    // Why: Test basic choice definition creation and definition.
    let def = ChoiceDefinition::new(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
    );

    // Check the various data items of the choice definition.
    assert_eq!(def.description(), "A choice parameter");
    assert_eq!(def.description_ref(), "A choice parameter");
    let choices = def.choices();
    assert_eq!(choices.len(), 2);
    assert_eq!(choices[0].id(), "a");
    assert_eq!(choices[0].description(), "A");
    assert_eq!(choices[1].id(), "b");
    assert_eq!(choices[1].description(), "B");
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
    assert!(def.contains("a"));
    assert!(!def.contains("c"));
    assert_eq!(def.ids(), vec!["a", "b"]);
    assert_eq!(def.descriptions(), vec!["A", "B"]);
}

#[test]
fn test_basic_definition_choice_items() {
    // Why: Test basic choice definition creation and definition.
    let def = ChoiceDefinition::new(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
    );

    // Check the various child items of the choice definition.
    let choices = def.choices();
    assert_eq!(choices.len(), 2);

    assert_eq!(choices[0].id(), "a");
    assert_eq!(choices[0].description(), "A");
    assert_eq!(choices[0].description_ref(), "A");

    assert_eq!(choices[1].id(), "b");
    assert_eq!(choices[1].description(), "B");
    assert_eq!(choices[1].description_ref(), "B");
}

#[test]
fn test_basic_definition_choice_with_default() {
    // Why: Test basic choice definition creation with a default value.
    let def = ChoiceDefinition::new_with_default(
        "A Default choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "a",
    );

    // Check the various data items of the choice definition.
    assert_eq!(def.description(), "A Default choice parameter");
    assert_eq!(def.description_ref(), "A Default choice parameter");
    let choices = def.choices();
    assert_eq!(choices.len(), 2);
    assert_eq!(choices[0].id(), "a");
    assert_eq!(choices[0].description(), "A");
    assert_eq!(choices[1].id(), "b");
    assert_eq!(choices[1].description(), "B");
    assert_eq!(def.default_value(), "a");
    assert_eq!(def.default_value_ref(), "a");
    assert!(def.contains("a"));
    assert!(!def.contains("c"));
    assert_eq!(def.ids(), vec!["a", "b"]);
    assert_eq!(def.descriptions(), vec!["A", "B"]);
}

#[test]
fn test_basic_definition_equality() {
    // Why: Test that two basic definitions with the same data items are considered equal.
    let def_1 = ChoiceDefinition::new_with_default(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "a",
    );
    let def_2 = ChoiceDefinition::new_with_default(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "a",
    );
    let def_3 = ChoiceDefinition::new_with_default(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "b",
    );

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(&def_1, def_2);
    assert_ne!(def_1, &def_3);
}
