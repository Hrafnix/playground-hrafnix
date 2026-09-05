use datastore::prelude::*;

#[test]
fn test_basic_frozen_file() {
    let frozen = FileFrozen::new(FileDefinition::new("A file parameter", "txt", false));

    assert_eq!(frozen.definition().description(), "A file parameter");
    assert_eq!(frozen.definition().extension_filter(), "txt");
    assert!(!frozen.definition().is_input());
    assert_eq!(frozen.definition().default_value(), "");
    assert_eq!(frozen.value(), "");
    assert_ne!(frozen.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_file_with_default() {
    let frozen = FileFrozen::new(FileDefinition::new_with_default(
        "A file parameter",
        "txt",
        true,
        "test.txt",
    ));

    assert_eq!(frozen.definition().description(), "A file parameter");
    assert_eq!(frozen.definition().extension_filter(), "txt");
    assert!(frozen.definition().is_input());
    assert_eq!(frozen.definition().default_value(), "test.txt");
    assert_eq!(frozen.value(), "test.txt");
    assert_ne!(frozen.hash(), [0u8; 32]);
}
