use datastore::prelude::*;

#[test]
fn test_editable_boolean_simple() {
    // Why: Editable boolean should thaw from frozen, allow edits, and freeze back correctly.
    let frozen = BooleanFrozen::new(BooleanDefinition::new_with_default(
        "A boolean parameter",
        true,
    ));
    let mut editable = frozen.thaw();
    assert_eq!(editable.value(), "true");

    editable.set("false");
    assert_eq!(editable.value(), "false");

    let frozen_2 = editable.freeze();
    assert_eq!(frozen_2.value(), "false");
    assert_ne!(frozen_2.hash(), frozen.hash());
}

#[test]
fn test_editable_boolean_equality() {
    // Why: Editable Boolean values with the same content should be equal.
    let frozen = BooleanFrozen::new(BooleanDefinition::new("A boolean parameter"));
    let editable_1 = frozen.thaw();
    let mut editable_2 = frozen.thaw();
    assert_eq!(editable_1, editable_2);
    assert_eq!(&editable_1, editable_2);
    assert_eq!(editable_1, &editable_2);
    assert_eq!(&editable_1, &editable_2);

    editable_2.set("true");
    assert_ne!(editable_1, editable_2);
    assert_ne!(&editable_1, editable_2);
    assert_ne!(editable_1, &editable_2);
    assert_ne!(&editable_1, &editable_2);
}
