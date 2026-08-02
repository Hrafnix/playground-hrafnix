use datastore::prelude::*;

#[test]
fn test_definition_string() {
    // Why: Test string definition creation and definition.
    let def = StringDefinition::new("A string parameter");

    // Check the various data items of the string definition.
    assert_eq!(def.description(), "A string parameter");
    assert_eq!(def.default_value(), "");
}

#[test]
fn test_definition_string_with_default() {
    // Why: Test string definition creation with a default value.
    let def = StringDefinition::new_with_default("A string parameter", "default value");

    // Check the various data items of the string definition.
    assert_eq!(def.description(), "A string parameter");
    assert_eq!(def.default_value(), "default value");
}

#[test]
fn test_definition_string_equality() {
    // Why: Test string definition equality.
    let def_1 = StringDefinition::new_with_default("A string parameter", "default value");
    let def_2 = StringDefinition::new_with_default("A string parameter", "default value");
    let def_3 = StringDefinition::new_with_default("A string parameter", "different value");

    // Check equality of the three string definitions.
    assert_eq!(def_1, def_2);
    assert_eq!(def_1, &def_2);
    assert_eq!(&def_1, def_2);
    assert_eq!(&def_1, &def_2);

    assert_ne!(def_1, def_3);
    assert_ne!(&def_1, def_3);
    assert_ne!(def_1, &def_3);
    assert_ne!(&def_1, &def_3);
}
