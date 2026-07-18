use datastore::prelude::*;

#[test]
fn test_object_definition_basic() {
    // Why: Test object definition creation and items.
    let mut builder = ObjectDefinition::builder("Test Object");
    builder.insert(
        StoreKey::new("p1".into()).unwrap(),
        ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
    );
    let obj_def = builder.finish();

    assert_eq!(obj_def.description().as_ref(), "Test Object");
    assert_eq!(obj_def.count(), 1);
    assert!(obj_def.contains("p1"));
    assert!(obj_def.contains_str("p1"));
}

#[test]
fn test_object_definition_equality() {
    // Why: Test that two object definitions with the same items are considered equal.
    let def_1 = ObjectDefinition::builder("Test Object")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .finish();
    let def_2 = ObjectDefinition::builder("Test Object")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .finish();
    let def_3 = ObjectDefinition::builder("Test Object")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D2")),
        )
        .finish();

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(&def_1, def_2);
    assert_ne!(def_1, &def_3);
}
