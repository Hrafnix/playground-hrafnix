use datastore::{definition::FolderDefinition, frozen::FolderFrozen};

#[test]
fn test_frozen_folder() {
    let frozen = FolderFrozen::new(FolderDefinition::new("A folder parameter", false));

    assert_eq!(frozen.definition().description(), "A folder parameter");
    assert!(!frozen.definition().is_input());
    assert_eq!(frozen.definition().default_value(), "");
    assert_eq!(frozen.value(), "");
    assert_ne!(frozen.hash(), [0u8; 32]);
}

#[test]
fn test_frozen_folder_with_default() {
    let frozen = FolderFrozen::new(FolderDefinition::new_with_default(
        "A folder parameter",
        true,
        "input/files",
    ));

    assert_eq!(frozen.definition().description(), "A folder parameter");
    assert!(frozen.definition().is_input());
    assert_eq!(frozen.definition().default_value(), "input/files");
    assert_eq!(frozen.value(), "input/files");
    assert_ne!(frozen.hash(), [0u8; 32]);
}
