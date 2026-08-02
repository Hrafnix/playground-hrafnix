use datastore::definition::{ChoiceDefinition, ChoiceItemDefinition};
use datastore::store_key;

#[test]
fn test_definition_choice() {
    // Why: Test choice definition creation and definition.
    let def = ChoiceDefinition::new(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
    );

    // Check the various data items of the choice definition.
    assert_eq!(def.description(), "A choice parameter");
    assert_eq!(def.description_ref(), "A choice parameter");
    let choices = def.choices();
    assert_eq!(choices.len(), 2);
    assert_eq!(choices[0].id(), "a");
    assert_eq!(choices[0].description(), "A");
    assert_eq!(choices[1].id(), "b");
    assert_eq!(choices[1].description(), "B");
    assert_eq!(def.default_value(), "");
    assert_eq!(def.default_value_ref(), "");
    assert!(def.contains("a"));
    assert!(!def.contains("c"));
    assert_eq!(def.ids(), vec!["a", "b"]);
    assert_eq!(def.descriptions(), vec!["A", "B"]);
}

#[test]
fn test_definition_choice_items() {
    // Why: Test choice definition creation and definition.
    let def = ChoiceDefinition::new(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
    );

    // Check the various child items of the choice definition.
    let choices = def.choices();
    assert_eq!(choices.len(), 2);

    assert_eq!(choices[0].id(), "a");
    assert_eq!(choices[0].description(), "A");
    assert_eq!(choices[0].description_ref(), "A");

    assert_eq!(choices[1].id(), "b");
    assert_eq!(choices[1].description(), "B");
    assert_eq!(choices[1].description_ref(), "B");
}

#[test]
fn test_definition_choice_with_default() {
    // Why: Test choice definition creation with a default value.
    let def = ChoiceDefinition::new_with_default(
        "A Default choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "a",
    );

    // Check the various data items of the choice definition.
    assert_eq!(def.description(), "A Default choice parameter");
    assert_eq!(def.description_ref(), "A Default choice parameter");
    let choices = def.choices();
    assert_eq!(choices.len(), 2);
    assert_eq!(choices[0].id(), "a");
    assert_eq!(choices[0].description(), "A");
    assert_eq!(choices[1].id(), "b");
    assert_eq!(choices[1].description(), "B");
    assert_eq!(def.default_value(), "a");
    assert_eq!(def.default_value_ref(), "a");
    assert!(def.contains("a"));
    assert!(!def.contains("c"));
    assert_eq!(def.ids(), vec!["a", "b"]);
    assert_eq!(def.descriptions(), vec!["A", "B"]);
}

#[test]
fn test_definition_choice_equality() {
    // Why: Test that two choice definitions with the same data items are considered equal.
    let def_1 = ChoiceDefinition::new_with_default(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "a",
    );
    let def_2 = ChoiceDefinition::new_with_default(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "a",
    );
    let def_3 = ChoiceDefinition::new_with_default(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "b",
    );

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(&def_1, def_2);
    assert_ne!(def_1, &def_3);
}
