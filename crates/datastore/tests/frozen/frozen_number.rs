use datastore::prelude::*;

#[test]
fn test_basic_frozen_number() {
    let frozen = NumberFrozen::new(NumberDefinition::new("A number parameter"));

    assert_eq!(frozen.definition().description(), "A number parameter");
    assert_eq!(frozen.definition().default_value(), "");
    assert_eq!(frozen.value(), "");
    assert_ne!(frozen.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_number_with_default() {
    let frozen = NumberFrozen::new(NumberDefinition::new_with_default(
        "A number parameter",
        "5.0",
    ));

    assert_eq!(frozen.definition().description(), "A number parameter");
    assert_eq!(frozen.definition().default_value(), "5.0");
    assert_eq!(frozen.value(), "5.0");
    assert_ne!(frozen.hash(), [0u8; 32]);
}
