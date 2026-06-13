use datastore::definition::{
    BasicDefinition, BasicDefinitionType, ChoiceDefinition, FileDefinition,
};

#[test]
fn test_basic_definition_string() {
    // Why: Test basic string definition creation and properties.
    let def = BasicDefinition::new_string("A string property");

    // Check the various properties of the string definition.
    assert_eq!(def.description().as_ref(), "A string property");
    assert!(matches!(def.type_definition(), BasicDefinitionType::String));
    assert_eq!(def.default_value().as_ref(), "");
}

#[test]
fn test_basic_definition_string_with_default() {
    // Why: Test basic string definition creation with a default value.
    let def = BasicDefinition::new_string_with_default("A string property", "default value");

    // Check the various properties of the string definition.
    assert_eq!(def.description().as_ref(), "A string property");
    assert!(matches!(def.type_definition(), BasicDefinitionType::String));
    assert_eq!(def.default_value().as_ref(), "default value");
}

#[test]
fn test_basic_definition_number() {
    // Why: Test basic number definition creation and properties.
    let def = BasicDefinition::new_number("A number property");

    // Check the various properties of the number definition.
    assert_eq!(def.description().as_ref(), "A number property");
    assert!(matches!(def.type_definition(), BasicDefinitionType::Number));
    assert_eq!(def.default_value().as_ref(), "");
}

#[test]
fn test_basic_definition_number_with_default() {
    // Why: Test basic number definition creation with a default value.
    let def = BasicDefinition::new_number_with_default("A number property", "5.0");

    // Check the various properties of the number definition.
    assert_eq!(def.description().as_ref(), "A number property");
    assert!(matches!(def.type_definition(), BasicDefinitionType::Number));
    assert_eq!(def.default_value().as_ref(), "5.0");
}

#[test]
fn test_basic_definition_file() {
    // Why: Test basic file definition creation and properties.
    let file_def = FileDefinition::new("txt");
    let def = BasicDefinition::new_file("A file property", file_def.clone());

    // Check the various properties of the file definition.
    assert_eq!(def.description().as_ref(), "A file property");
    if let BasicDefinitionType::File(f) = def.type_definition() {
        assert_eq!(f.extension_filter().as_ref(), "txt");
    } else {
        panic!("Expected File type");
    }
    assert_eq!(def.default_value().as_ref(), "");
}

#[test]
fn test_basic_definition_file_with_default() {
    // Why: Test basic file definition creation and properties with a default value.
    let file_def = FileDefinition::new("txt");
    let def =
        BasicDefinition::new_file_with_default("A file property", file_def.clone(), "test.txt");

    // Check the various properties of the file definition.
    assert_eq!(def.description().as_ref(), "A file property");
    if let BasicDefinitionType::File(f) = def.type_definition() {
        assert_eq!(f.extension_filter().as_ref(), "txt");
    } else {
        panic!("Expected File type");
    }
    assert_eq!(def.default_value().as_ref(), "test.txt");
}

#[test]
fn test_basic_definition_choice() {
    // Why: Test basic choice definition creation and properties.
    let choice_def = ChoiceDefinition::new(vec!["A".into(), "B".into()]);
    let def = BasicDefinition::new_choice("A choice property", choice_def.clone());

    // Check the various properties of the choice definition.
    assert_eq!(def.description().as_ref(), "A choice property");
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
    // Why: Test basic choice definition creation and properties with a default value.
    let choice_def = ChoiceDefinition::new(vec!["A".into(), "B".into()]);
    let def =
        BasicDefinition::new_choice_with_default("A choice property", choice_def.clone(), "A");

    // Check the various properties of the choice definition.
    assert_eq!(def.description().as_ref(), "A choice property");
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
    // Why: Test that two basic definitions with the same properties are considered equal and ref equal.
    let choice_def = ChoiceDefinition::new(vec!["A".into(), "B".into()]);
    let def_1 =
        BasicDefinition::new_choice_with_default("A choice property", choice_def.clone(), "A");
    let def_2 =
        BasicDefinition::new_choice_with_default("A choice property", choice_def.clone(), "A");
    let def_3 =
        BasicDefinition::new_choice_with_default("A choice property", choice_def.clone(), "B");

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(&def_1, def_2);
    assert_ne!(def_1, &def_3);
}
