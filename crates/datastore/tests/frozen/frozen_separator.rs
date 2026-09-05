use datastore::{definition::SeparatorDefinition, frozen::SeparatorFrozen};

#[test]
fn test_frozen_separator() {
    let frozen = SeparatorFrozen::new(SeparatorDefinition::new("A separator"));

    assert_eq!(frozen.definition().description(), "A separator");
    assert_ne!(frozen.hash(), [0u8; 32]);
}
