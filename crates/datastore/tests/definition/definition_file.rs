use datastore::prelude::*;

#[test]
fn test_definition_file() {
    // Why: Test file definition creation and definition.
    let def = FileDefinition::new("A file parameter", "txt", false);

    // Check the various data items of the file definition.
    assert_eq!(def.description(), "A file parameter");
    assert_eq!(def.description_ref(), "A file parameter");
    assert_eq!(def.extension_filter(), "txt");
    assert_eq!(def.extension_filter_ref(), "txt");
    assert!(!def.is_input());
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
}

#[test]
fn test_definition_file_with_default() {
    // Why: Test file definition creation with a default value.
    let def = FileDefinition::new_with_default("A Default file parameter", "exe", true, "test.exe");

    // Check the various data items of the file definition.
    assert_eq!(def.description(), "A Default file parameter");
    assert_eq!(def.description_ref(), "A Default file parameter");
    assert_eq!(def.extension_filter(), "exe");
    assert_eq!(def.extension_filter_ref(), "exe");
    assert!(def.is_input());
    assert_eq!(def.default_value(), "test.exe");
    assert_eq!(def.default_value_ref(), "test.exe");
}

#[test]
fn test_definition_file_equality() {
    // Why: Test file definition equality.
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
