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
            value: 0.0,
            inclusive: true,
        },
    );

    // Check the various data items of the number definition.
    assert_eq!(def.description(), "A number parameter");
    assert_eq!(
        def.constraint(),
        NumberConstraint::Min {
            value: 0.0,
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
            value: 10.0,
            inclusive: true,
        },
        "5.0",
    );

    // Check the various data items of the number definition.
    assert_eq!(def.description(), "A number parameter");
    assert_eq!(
        def.constraint(),
        NumberConstraint::Max {
            value: 10.0,
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
    assert_eq!(def.extension_filter(), "txt");
    assert!(!def.bundle_on_archive());
    assert_eq!(def.default_value(), "");
}

#[test]
fn test_basic_definition_file_with_default() {
    // Why: Test basic file definition creation with a default value.
    let def = FileDefinition::new_with_default("A file parameter", "txt", true, "test.txt");

    // Check the various data items of the file definition.
    assert_eq!(def.description(), "A file parameter");
    assert_eq!(def.extension_filter(), "txt");
    assert!(def.bundle_on_archive());
    assert_eq!(def.default_value(), "test.txt");
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
    assert_eq!(&def_1, &def_2);
    assert_ne!(&def_1, &def_3);
    assert_ne!(&def_1, &def_4);
    assert_ne!(&def_1, &def_5);
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
    let choices = def.choices();
    assert_eq!(choices.len(), 2);
    assert_eq!(choices[0].id(), "a");
    assert_eq!(choices[0].description(), "A");
    assert_eq!(choices[1].id(), "b");
    assert_eq!(choices[1].description(), "B");
    assert_eq!(def.default_value(), "");
}

#[test]
fn test_basic_definition_choice_with_default() {
    // Why: Test basic choice definition creation with a default value.
    let def = ChoiceDefinition::new_with_default(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "a",
    );

    // Check the various data items of the choice definition.
    assert_eq!(def.description(), "A choice parameter");
    let choices = def.choices();
    assert_eq!(choices.len(), 2);
    assert_eq!(choices[0].id(), "a");
    assert_eq!(choices[0].description(), "A");
    assert_eq!(choices[1].id(), "b");
    assert_eq!(choices[1].description(), "B");
    assert_eq!(def.default_value(), "a");
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
