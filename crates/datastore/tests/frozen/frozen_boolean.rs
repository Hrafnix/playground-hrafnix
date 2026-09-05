use datastore::prelude::*;

#[test]
fn test_basic_frozen_boolean() {
    let frozen = BooleanFrozen::new(BooleanDefinition::new("A boolean parameter"));

    assert_eq!(frozen.definition().description(), "A boolean parameter");
    assert_eq!(frozen.definition().default_value(), "");
    assert_eq!(frozen.value(), "");
    assert_ne!(frozen.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_boolean_with_default() {
    let frozen = BooleanFrozen::new(BooleanDefinition::new_with_default(
        "A boolean parameter",
        true,
    ));

    assert_eq!(frozen.definition().description(), "A boolean parameter");
    assert_eq!(frozen.definition().default_value(), "true");
    assert_eq!(frozen.value(), "true");
    assert_ne!(frozen.hash(), [0u8; 32]);
}
