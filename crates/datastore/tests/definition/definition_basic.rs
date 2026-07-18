use datastore::prelude::*;

#[test]
fn test_basic_definition_string() {
    // Why: Test basic string definition creation and definition.
    let def = BasicDefinition::new_string("A string parameter");

    // Check the various data items of the string definition.
    assert_eq!(def.description().as_ref(), "A string parameter");
    assert!(matches!(def.type_definition(), BasicDefinitionType::String));
    assert_eq!(def.default_value().as_ref(), "");
}

#[test]
fn test_basic_definition_string_with_default() {
    // Why: Test basic string definition creation with a default value.
    let def = BasicDefinition::new_string_with_default("A string parameter", "default value");

    // Check the various data items of the string definition.
    assert_eq!(def.description().as_ref(), "A string parameter");
    assert!(matches!(def.type_definition(), BasicDefinitionType::String));
    assert_eq!(def.default_value().as_ref(), "default value");
}

#[test]
fn test_basic_definition_number() {
    // Why: Test basic number definition creation and definition.
    let def = BasicDefinition::new_number("A number parameter");

    // Check the various data items of the number definition.
    assert_eq!(def.description().as_ref(), "A number parameter");
    assert!(matches!(def.type_definition(), BasicDefinitionType::Number));
    assert_eq!(def.default_value().as_ref(), "");
}

#[test]
fn test_basic_definition_number_with_default() {
    // Why: Test basic number definition creation with a default value.
    let def = BasicDefinition::new_number_with_default("A number parameter", "5.0");

    // Check the various data items of the number definition.
    assert_eq!(def.description().as_ref(), "A number parameter");
    assert!(matches!(def.type_definition(), BasicDefinitionType::Number));
    assert_eq!(def.default_value().as_ref(), "5.0");
}

#[test]
fn test_basic_definition_file() {
    // Why: Test basic file definition creation and definition.
    let file_def = FileDefinition::new("txt", false);
    let def = BasicDefinition::new_file("A file parameter", file_def.clone());

    // Check the various data items of the file definition.
    assert_eq!(def.description().as_ref(), "A file parameter");
    if let BasicDefinitionType::File(f) = def.type_definition() {
        assert_eq!(f.extension_filter().as_ref(), "txt");
        assert!(!f.bundle_on_archive());
    } else {
        panic!("Expected File type");
    }
    assert_eq!(def.default_value().as_ref(), "");
}

#[test]
fn test_basic_definition_file_with_default() {
    // Why: Test basic file definition creation with a default value.
    let file_def = FileDefinition::new("txt", true);
    let def =
        BasicDefinition::new_file_with_default("A file parameter", file_def.clone(), "test.txt");

    // Check the various data items of the file definition.
    assert_eq!(def.description().as_ref(), "A file parameter");
    if let BasicDefinitionType::File(f) = def.type_definition() {
        assert_eq!(f.extension_filter().as_ref(), "txt");
        assert!(f.bundle_on_archive());
    } else {
        panic!("Expected File type");
    }
    assert_eq!(def.default_value().as_ref(), "test.txt");
}

#[test]
fn test_basic_definition_choice() {
    // Why: Test basic choice definition creation and definition.
    let choice_def = ChoiceDefinition::new(vec!["A".into(), "B".into()]);
    let def = BasicDefinition::new_choice("A choice parameter", choice_def.clone());

    // Check the various data items of the choice definition.
    assert_eq!(def.description().as_ref(), "A choice parameter");
    if let BasicDefinitionType::Choice(c) = def.type_definition() {
        assert_eq!(c.choices().len(), 2);
        assert_eq!(c.choices()[0].as_ref(), "A");
        assert_eq!(c.choices()[1].as_ref(), "B");
    } else {
        panic!("Expected Choice type");
    }
    assert_eq!(def.default_value().as_ref(), "");
}

#[test]
fn test_basic_definition_choice_with_default() {
    // Why: Test basic choice definition creation with a default value.
    let choice_def = ChoiceDefinition::new(vec!["A".into(), "B".into()]);
    let def =
        BasicDefinition::new_choice_with_default("A choice parameter", choice_def.clone(), "A");

    // Check the various data items of the choice definition.
    assert_eq!(def.description().as_ref(), "A choice parameter");
    if let BasicDefinitionType::Choice(c) = def.type_definition() {
        assert_eq!(c.choices().len(), 2);
        assert_eq!(c.choices()[0].as_ref(), "A");
        assert_eq!(c.choices()[1].as_ref(), "B");
    } else {
        panic!("Expected Choice type");
    }
    assert_eq!(def.default_value().as_ref(), "A");
}

#[test]
fn test_basic_definition_equality() {
    // Why: Test that two basic definitions with the same data items are considered equal.
    let choice_def = ChoiceDefinition::new(vec!["A".into(), "B".into()]);
    let def_1 =
        BasicDefinition::new_choice_with_default("A choice parameter", choice_def.clone(), "A");
    let def_2 =
        BasicDefinition::new_choice_with_default("A choice parameter", choice_def.clone(), "A");
    let def_3 =
        BasicDefinition::new_choice_with_default("A choice parameter", choice_def.clone(), "B");

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(&def_1, def_2);
    assert_ne!(def_1, &def_3);
}
