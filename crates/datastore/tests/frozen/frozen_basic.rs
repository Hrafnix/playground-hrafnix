use datastore::prelude::*;

#[test]
fn test_basic_frozen_string() {
    // Why: Test frozen basic string creation and definition.
    let frozen_basic = StringFrozen::new(StringDefinition::new("A string parameter"));

    // Check the various parameters of the string definition.
    assert_eq!(
        frozen_basic.definition().description(),
        "A string parameter"
    );
    assert_eq!(frozen_basic.definition().default_value(), "");
    assert_eq!(frozen_basic.value(), "");
    assert_ne!(frozen_basic.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_string_with_default() {
    // Why: Test frozen basic string creation with a default value.
    let frozen_basic = StringFrozen::new(StringDefinition::new_with_default(
        "A string parameter",
        "default value",
    ));

    // Check the frozen string object.
    assert_eq!(
        frozen_basic.definition().description(),
        "A string parameter"
    );
    assert_eq!(frozen_basic.definition().default_value(), "default value");
    assert_eq!(frozen_basic.value(), "default value");
    assert_ne!(frozen_basic.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_number() {
    // Why: Test frozen basic number creation and definition.
    let frozen_basic = NumberFrozen::new(NumberDefinition::new("A number parameter"));

    // Check the frozen number object.
    assert_eq!(
        frozen_basic.definition().description(),
        "A number parameter"
    );
    assert_eq!(frozen_basic.definition().default_value(), "");
    assert_eq!(frozen_basic.value(), "");
    assert_ne!(frozen_basic.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_number_with_default() {
    // Why: Test frozen basic number creation with a default value.
    let frozen_basic = NumberFrozen::new(NumberDefinition::new_with_default(
        "A number parameter",
        "5.0",
    ));

    // Check the frozen number object.
    assert_eq!(
        frozen_basic.definition().description(),
        "A number parameter"
    );
    assert_eq!(frozen_basic.definition().default_value(), "5.0");
    assert_eq!(frozen_basic.value(), "5.0");
    assert_ne!(frozen_basic.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_file() {
    // Why: Test frozen basic file creation and definition.
    let frozen_basic = FileFrozen::new(FileDefinition::new("A file parameter", "txt", false));

    // Check the frozen file object.
    assert_eq!(frozen_basic.definition().description(), "A file parameter");
    assert_eq!(frozen_basic.definition().extension_filter(), "txt");
    assert!(!frozen_basic.definition().bundle_on_archive());
    assert_eq!(frozen_basic.definition().default_value(), "");
    assert_eq!(frozen_basic.value(), "");
    assert_ne!(frozen_basic.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_file_with_default() {
    // Why: Test frozen basic file creation with a default value.
    let frozen_basic = FileFrozen::new(FileDefinition::new_with_default(
        "A file parameter",
        "txt",
        true,
        "test.txt",
    ));

    // Check the frozen file object.
    assert_eq!(frozen_basic.definition().description(), "A file parameter");
    assert_eq!(frozen_basic.definition().extension_filter(), "txt");
    assert!(frozen_basic.definition().bundle_on_archive());
    assert_eq!(frozen_basic.definition().default_value(), "test.txt");
    assert_eq!(frozen_basic.value(), "test.txt");
    assert_ne!(frozen_basic.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_choice() {
    // Why: Test frozen basic choice creation and definition.
    let frozen_basic = ChoiceFrozen::new(ChoiceDefinition::new(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
    ));

    // Check the frozen choice object.
    assert_eq!(
        frozen_basic.definition().description(),
        "A choice parameter"
    );

    let choices = frozen_basic.definition().choices();
    assert_eq!(choices.len(), 2);
    assert_eq!(choices[0].id(), "a");
    assert_eq!(choices[0].description(), "A");
    assert_eq!(choices[1].id(), "b");
    assert_eq!(choices[1].description(), "B");
    assert_eq!(frozen_basic.definition().default_value(), "");
    assert_eq!(frozen_basic.value(), "");
    assert_ne!(frozen_basic.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_choice_with_default() {
    // Why: Test frozen basic choice creation with a default value.
    let frozen_basic = ChoiceFrozen::new(ChoiceDefinition::new_with_default(
        "A choice parameter",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "a",
    ));

    // Check the frozen choice object with a default value.
    assert_eq!(
        frozen_basic.definition().description(),
        "A choice parameter"
    );
    let choices = frozen_basic.definition().choices();
    assert_eq!(choices.len(), 2);
    assert_eq!(choices[0].id(), "a");
    assert_eq!(choices[0].description(), "A");
    assert_eq!(choices[1].id(), "b");
    assert_eq!(choices[1].description(), "B");
    assert_eq!(frozen_basic.definition().default_value(), "a");
    assert_eq!(frozen_basic.value(), "a");
    assert_ne!(frozen_basic.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_equality() {
    // Why: Test that two frozen basic choices with the same parameters are considered equal.
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

    // Check equality.
    assert_eq!(frozen_1, frozen_2);
    assert_ne!(frozen_1, frozen_3);
    assert_eq!(&frozen_1, frozen_2);
    assert_ne!(frozen_1, &frozen_3);
}
