use datastore::prelude::*;

#[test]
fn test_editable_file_simple() {
    // Why: Editable file should thaw from frozen, allow edits, and freeze back correctly.
    let frozen = FileFrozen::new(FileDefinition::new("A file parameter", "txt", false));
    let mut editable = frozen.thaw();
    assert_eq!(editable.value(), "");

    editable.set("test.txt");
    assert_eq!(editable.value(), "test.txt");

    let frozen_2 = editable.freeze();
    assert_eq!(frozen_2.value(), "test.txt");
    assert_ne!(frozen_2.hash(), frozen.hash());
}

#[test]
fn test_editable_file_equality() {
    // Why: Editable File values with the same content should be equal.
    let frozen = FileFrozen::new(FileDefinition::new("A file parameter", "txt", false));
    let editable_1 = frozen.thaw();
    let mut editable_2 = frozen.thaw();
    assert_eq!(editable_1, editable_2);
    assert_eq!(&editable_1, editable_2);
    assert_eq!(editable_1, &editable_2);
    assert_eq!(&editable_1, &editable_2);

    editable_2.set("test2.txt");
    assert_ne!(editable_1, editable_2);
    assert_ne!(&editable_1, editable_2);
    assert_ne!(editable_1, &editable_2);
    assert_ne!(&editable_1, &editable_2);
}
