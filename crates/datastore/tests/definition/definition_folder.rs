use datastore::definition::FolderDefinition;

#[test]
fn test_definition_folder() {
    let def = FolderDefinition::new("A folder parameter", false);

    assert_eq!(def.description(), "A folder parameter");
    assert_eq!(def.description_ref(), "A folder parameter");
    assert!(!def.is_input());
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
}

#[test]
fn test_definition_folder_with_default() {
    let def = FolderDefinition::new_with_default("A default folder parameter", true, "input/files");

    assert_eq!(def.description(), "A default folder parameter");
    assert_eq!(def.description_ref(), "A default folder parameter");
    assert!(def.is_input());
    assert_eq!(def.default_value(), "input/files");
    assert_eq!(def.default_value_ref(), "input/files");
}

#[test]
fn test_definition_folder_equality() {
    let def_1 = FolderDefinition::new_with_default("A folder parameter", true, "input/files");
    let def_2 = FolderDefinition::new_with_default("A folder parameter", true, "input/files");
    let def_3 = FolderDefinition::new_with_default("A folder parameter", true, "other/files");
    let def_4 = FolderDefinition::new_with_default("A folder parameter", false, "input/files");
    let def_5 = FolderDefinition::new_with_default("A new folder parameter", true, "input/files");

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_ne!(def_1, def_4);
    assert_ne!(def_1, def_5);
    assert_eq!(def_1, &def_2);
    assert_ne!(&def_1, def_3);
    assert_ne!(def_1, &def_4);
    assert_ne!(&def_1, &def_5);
}
