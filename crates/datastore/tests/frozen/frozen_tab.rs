use datastore::{definition::TabDefinition, frozen::TabFrozen};

#[test]
fn test_frozen_tab() {
    let frozen = TabFrozen::new(TabDefinition::new("A tab"));

    assert_eq!(frozen.definition().description(), "A tab");
    assert_ne!(frozen.hash(), [0u8; 32]);
}
