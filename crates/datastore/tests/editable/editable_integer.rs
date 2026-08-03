use datastore::prelude::*;

#[test]
fn test_editable_integer_simple() {
    // Why: Editable integer should thaw from frozen, allow edits, and freeze back correctly.
    let frozen = IntegerFrozen::new(IntegerDefinition::new_with_default(
        "An integer parameter",
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
fn test_editable_integer_equality() {
    // Why: Editable integer values with the same content should be equal.
    let frozen = IntegerFrozen::new(IntegerDefinition::new_with_default(
        "An integer parameter",
        "1",
    ));
    let editable_1 = frozen.thaw();
    let mut editable_2 = frozen.thaw();
    assert_eq!(editable_1, editable_2);
    assert_eq!(&editable_1, editable_2);
    assert_eq!(editable_1, &editable_2);
    assert_eq!(&editable_1, &editable_2);

    editable_2.set("42");
    assert_ne!(editable_1, editable_2);
    assert_ne!(&editable_1, editable_2);
    assert_ne!(editable_1, &editable_2);
    assert_ne!(&editable_1, &editable_2);
}
