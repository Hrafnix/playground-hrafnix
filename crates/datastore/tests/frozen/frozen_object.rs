use datastore::definition::{BasicDefinition, ItemDefinition, ObjectDefinition};
use datastore::frozen::ObjectFrozen;
use datastore::key::StoreKey;

#[test]
fn test_object_frozen_basic() {
    // Why: Test frozen object creation and items.
    let frozen_1 = ObjectFrozen::new(
        ObjectDefinition::builder("Test Object")
            .with(
                StoreKey::new("p1".into()).unwrap(),
                ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
            )
            .finish(),
    );

    assert_eq!(frozen_1.definition().description().as_ref(), "Test Object");
    assert_eq!(frozen_1.definition().count(), 1);
    assert!(frozen_1.definition().contains("p1"));
    assert!(frozen_1.definition().contains_str("p1"));
    assert_ne!(frozen_1.hash(), [0u8; 32]);
}

#[test]
fn test_object_frozen_equality() {
    // Why: Test that two frozen objects with the same items are considered equal.
    let frozen_1 = ObjectFrozen::new(
        ObjectDefinition::builder("Test Object")
            .with(
                StoreKey::new("p1".into()).unwrap(),
                ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
            )
            .finish(),
    );
    let frozen_2 = ObjectFrozen::new(
        ObjectDefinition::builder("Test Object")
            .with(
                StoreKey::new("p1".into()).unwrap(),
                ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
            )
            .finish(),
    );
    let frozen_3 = ObjectFrozen::new(
        ObjectDefinition::builder("Test Object")
            .with(
                StoreKey::new("p1".into()).unwrap(),
                ItemDefinition::new("P1", BasicDefinition::new_string("D2")),
            )
            .finish(),
    );

    assert_eq!(frozen_1, frozen_2);
    assert_ne!(frozen_1, frozen_3);
    assert_eq!(&frozen_1, frozen_2);
    assert_ne!(frozen_1, &frozen_3);
}
