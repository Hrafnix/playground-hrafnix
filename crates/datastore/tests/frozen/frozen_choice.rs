use datastore::prelude::*;

#[test]
fn test_basic_frozen_choice() {
    let frozen = ChoiceFrozen::new(ChoiceDefinition::new(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
    ));

    assert_eq!(frozen.definition().description(), "A choice parameter");
    let choices = frozen.definition().choices();
    assert_eq!(choices.len(), 2);
    assert_eq!(choices[0].id(), "a");
    assert_eq!(choices[0].description(), "A");
    assert_eq!(choices[1].id(), "b");
    assert_eq!(choices[1].description(), "B");
    assert_eq!(frozen.definition().default_value(), "");
    assert_eq!(frozen.value(), "");
    assert_ne!(frozen.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_choice_with_default() {
    let frozen = ChoiceFrozen::new(ChoiceDefinition::new_with_default(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "a",
    ));

    assert_eq!(frozen.definition().description(), "A choice parameter");
    let choices = frozen.definition().choices();
    assert_eq!(choices.len(), 2);
    assert_eq!(choices[0].id(), "a");
    assert_eq!(choices[0].description(), "A");
    assert_eq!(choices[1].id(), "b");
    assert_eq!(choices[1].description(), "B");
    assert_eq!(frozen.definition().default_value(), "a");
    assert_eq!(frozen.value(), "a");
    assert_ne!(frozen.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_equality() {
    let frozen_1 = ChoiceDefinition::new_with_default(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "a",
    );
    let frozen_2 = ChoiceDefinition::new_with_default(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "a",
    );
    let frozen_3 = ChoiceDefinition::new_with_default(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "b",
    );

    assert_eq!(frozen_1, frozen_2);
    assert_ne!(frozen_1, frozen_3);
    assert_eq!(&frozen_1, frozen_2);
    assert_ne!(frozen_1, &frozen_3);
}
