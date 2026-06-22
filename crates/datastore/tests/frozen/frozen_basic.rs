use datastore::definition::{
    BasicDefinition, BasicDefinitionType, ChoiceDefinition, FileDefinition,
};
use datastore::frozen::BasicFrozen;

#[test]
fn test_basic_frozen_string() {
    // Why: Test frozen basic string creation and definition.
    let frozen_basic = BasicFrozen::new(BasicDefinition::new_string("A string parameter"));

    // Check the various parameters of the string definition.
    assert_eq!(
        frozen_basic.definition().description(),
        "A string parameter"
    );
    assert!(matches!(
        frozen_basic.definition().type_definition(),
        BasicDefinitionType::String
    ));
    assert_eq!(frozen_basic.definition().default_value(), "");
    assert_eq!(frozen_basic.value(), "");
    assert_ne!(frozen_basic.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_string_with_default() {
    // Why: Test frozen basic string creation with a default value.
    let frozen_basic = BasicFrozen::new(BasicDefinition::new_string_with_default(
        "A string parameter",
        "default value",
    ));

    // Check the frozen string object.
    assert_eq!(
        frozen_basic.definition().description(),
        "A string parameter"
    );
    assert!(matches!(
        frozen_basic.definition().type_definition(),
        BasicDefinitionType::String
    ));
    assert_eq!(frozen_basic.definition().default_value(), "default value");
    assert_eq!(frozen_basic.value(), "default value");
    assert_ne!(frozen_basic.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_number() {
    // Why: Test frozen basic number creation and definition.
    let frozen_basic = BasicFrozen::new(BasicDefinition::new_number("A number parameter"));

    // Check the frozen number object.
    assert_eq!(
        frozen_basic.definition().description(),
        "A number parameter"
    );
    assert!(matches!(
        frozen_basic.definition().type_definition(),
        BasicDefinitionType::Number
    ));
    assert_eq!(frozen_basic.definition().default_value(), "");
    assert_eq!(frozen_basic.value(), "");
    assert_ne!(frozen_basic.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_number_with_default() {
    // Why: Test frozen basic number creation with a default value.
    let frozen_basic = BasicFrozen::new(BasicDefinition::new_number_with_default(
        "A number parameter",
        "5.0",
    ));

    // Check the frozen number object.
    assert_eq!(
        frozen_basic.definition().description(),
        "A number parameter"
    );
    assert!(matches!(
        frozen_basic.definition().type_definition(),
        BasicDefinitionType::Number
    ));
    assert_eq!(frozen_basic.definition().default_value(), "5.0");
    assert_eq!(frozen_basic.value(), "5.0");
    assert_ne!(frozen_basic.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_file() {
    // Why: Test frozen basic file creation and definition.
    let frozen_basic = BasicFrozen::new(BasicDefinition::new_file(
        "A file parameter",
        FileDefinition::new("txt", false),
    ));

    // Check the frozen file object.
    assert_eq!(frozen_basic.definition().description(), "A file parameter");
    if let BasicDefinitionType::File(f) = frozen_basic.definition().type_definition() {
        assert_eq!(f.extension_filter(), "txt");
        assert!(!f.bundle_on_archive());
    } else {
        panic!("Expected File type");
    }
    assert_eq!(frozen_basic.definition().default_value(), "");
    assert_eq!(frozen_basic.value(), "");
    assert_ne!(frozen_basic.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_file_with_default() {
    // Why: Test frozen basic file creation with a default value.
    let frozen_basic = BasicFrozen::new(BasicDefinition::new_file_with_default(
        "A file parameter",
        FileDefinition::new("txt", true),
        "test.txt",
    ));

    // Check the frozen file object.
    assert_eq!(frozen_basic.definition().description(), "A file parameter");
    if let BasicDefinitionType::File(f) = frozen_basic.definition().type_definition() {
        assert_eq!(f.extension_filter(), "txt");
        assert!(f.bundle_on_archive());
    } else {
        panic!("Expected File type");
    }
    assert_eq!(frozen_basic.definition().default_value(), "test.txt");
    assert_eq!(frozen_basic.value(), "test.txt");
    assert_ne!(frozen_basic.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_choice() {
    // Why: Test frozen basic choice creation and definition.
    let frozen_basic = BasicFrozen::new(BasicDefinition::new_choice(
        "A choice parameter",
        ChoiceDefinition::new(vec!["A".into(), "B".into()]),
    ));

    // Check the frozen choice object.
    assert_eq!(
        frozen_basic.definition().description(),
        "A choice parameter"
    );
    if let BasicDefinitionType::Choice(c) = frozen_basic.definition().type_definition() {
        assert_eq!(c.choices().len(), 2);
        assert_eq!(c.choices()[0], "A");
        assert_eq!(c.choices()[1], "B");
    } else {
        panic!("Expected Choice type");
    }
    assert_eq!(frozen_basic.definition().default_value(), "");
    assert_eq!(frozen_basic.value(), "");
    assert_ne!(frozen_basic.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_choice_with_default() {
    // Why: Test frozen basic choice creation with a default value.
    let frozen_basic = BasicFrozen::new(BasicDefinition::new_choice_with_default(
        "A choice parameter",
        ChoiceDefinition::new(vec!["A".into(), "B".into()]),
        "A",
    ));

    // Check the frozen choice object with a default value.
    assert_eq!(
        frozen_basic.definition().description(),
        "A choice parameter"
    );
    if let BasicDefinitionType::Choice(c) = frozen_basic.definition().type_definition() {
        assert_eq!(c.choices().len(), 2);
        assert_eq!(c.choices()[0], "A");
        assert_eq!(c.choices()[1], "B");
    } else {
        panic!("Expected Choice type");
    }
    assert_eq!(frozen_basic.definition().default_value(), "A");
    assert_eq!(frozen_basic.value(), "A");
    assert_ne!(frozen_basic.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_equality() {
    // Why: Test that two frozen basic choices with the same parameters are considered equal.
    let choice_def = ChoiceDefinition::new(vec!["A".into(), "B".into()]);
    let frozen_1 = BasicFrozen::new(BasicDefinition::new_choice_with_default(
        "A choice parameter",
        choice_def.clone(),
        "A",
    ));
    let frozen_2 = BasicFrozen::new(BasicDefinition::new_choice_with_default(
        "A choice parameter",
        choice_def.clone(),
        "A",
    ));
    let frozen_3 = BasicFrozen::new(BasicDefinition::new_choice_with_default(
        "A choice parameter",
        choice_def.clone(),
        "B",
    ));

    // Check equality.
    assert_eq!(frozen_1, frozen_2);
    assert_ne!(frozen_1, frozen_3);
    assert_eq!(&frozen_1, frozen_2);
    assert_ne!(frozen_1, &frozen_3);
}
