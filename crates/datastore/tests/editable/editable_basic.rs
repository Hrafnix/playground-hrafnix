use datastore::prelude::*;

#[test]
fn test_editable_string_round_trip() {
    // Why: Editable string should thaw from frozen, allow edits, and freeze back correctly.
    let frozen = StringFrozen::new(StringDefinition::new_with_default(
        "A string parameter",
        "default value",
    ));
    let mut editable = frozen.thaw();
    assert_eq!(editable.value(), "default value");

    editable.set("new value");
    assert_eq!(editable.value(), "new value");

    let frozen_2 = editable.freeze();
    assert_eq!(frozen_2.value(), "new value");
    assert_eq!(frozen_2.definition(), frozen.definition());
    assert_ne!(frozen_2.hash(), frozen.hash());
}

#[test]
fn test_editable_number_round_trip() {
    // Why: Editable number should thaw from frozen, allow edits, and freeze back correctly.
    let frozen = NumberFrozen::new(NumberDefinition::new_with_default(
        "A number parameter",
        "1",
    ));
    let mut editable = frozen.thaw();
    assert_eq!(editable.value(), "1");

    editable.set("42");
    assert_eq!(editable.value(), "42");

    let frozen_2 = editable.freeze();
    assert_eq!(frozen_2.value(), "42");
    assert_ne!(frozen_2.hash(), frozen.hash());
}

#[test]
fn test_editable_file_round_trip() {
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
fn test_editable_basic_equality() {
    // Why: Editable values with the same content should be equal.
    let frozen = StringFrozen::new(StringDefinition::new("A string parameter"));
    let editable_1 = frozen.thaw();
    let editable_2 = frozen.thaw();
    assert_eq!(editable_1, editable_2);
    assert_eq!(&editable_1, editable_2);
    assert_eq!(editable_1, &editable_2);
    assert_eq!(&editable_1, &editable_2);
}
