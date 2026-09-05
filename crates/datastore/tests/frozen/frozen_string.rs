use datastore::prelude::*;

#[test]
fn test_basic_frozen_string() {
    let frozen = StringFrozen::new(StringDefinition::new("A string parameter"));

    assert_eq!(frozen.definition().description(), "A string parameter");
    assert_eq!(frozen.definition().default_value(), "");
    assert_eq!(frozen.value(), "");
    assert_ne!(frozen.hash(), [0u8; 32]);
}

#[test]
fn test_basic_frozen_string_with_default() {
    let frozen = StringFrozen::new(StringDefinition::new_with_default(
        "A string parameter",
        "default value",
    ));

    assert_eq!(frozen.definition().description(), "A string parameter");
    assert_eq!(frozen.definition().default_value(), "default value");
    assert_eq!(frozen.value(), "default value");
    assert_ne!(frozen.hash(), [0u8; 32]);
}
