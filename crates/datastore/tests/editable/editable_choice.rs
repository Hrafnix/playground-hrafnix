use datastore::prelude::*;

#[test]
fn test_editable_choice_simple() {
    // Why: Editable choice should thaw from frozen, allow edits, and freeze back correctly.
    let frozen = ChoiceFrozen::new(ChoiceDefinition::new_with_default(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "a",
    ));
    let mut editable = frozen.thaw();
    assert_eq!(editable.value(), "a");

    editable.set("b");
    assert_eq!(editable.value(), "b");

    let frozen_2 = editable.freeze();
    assert_eq!(frozen_2.value(), "b");
    assert_ne!(frozen_2.hash(), frozen.hash());
}

#[test]
fn test_editable_choice_equality() {
    // Why: Editable Choice values with the same content should be equal.
    let frozen = ChoiceFrozen::new(ChoiceDefinition::new_with_default(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "a",
    ));
    let editable_1 = frozen.thaw();
    let mut editable_2 = frozen.thaw();
    assert_eq!(editable_1, editable_2);
    assert_eq!(&editable_1, editable_2);
    assert_eq!(editable_1, &editable_2);
    assert_eq!(&editable_1, &editable_2);

    editable_2.set("b");
    assert_ne!(editable_1, editable_2);
    assert_ne!(&editable_1, editable_2);
    assert_ne!(editable_1, &editable_2);
    assert_ne!(&editable_1, &editable_2);
}
