use datastore::definition::{BasicDefinition, ObjectDefinition, PropertyDefinition};
use datastore::store_key;

#[test]
fn test_object_definition_basic() {
    // Why: Test object definition creation and properties.
    let mut builder = ObjectDefinition::builder("Test Object");
    builder.insert(
        store_key!("prop1"),
        PropertyDefinition::new("P1", BasicDefinition::new_string("D1")),
    );
    let obj_def = builder.finish();

    assert_eq!(obj_def.description().as_ref(), "Test Object");
    assert_eq!(obj_def.count(), 1);
    assert!(obj_def.contains_key(store_key!("prop1")));
    assert!(obj_def.contains_key_str("prop1"));
}

#[test]
fn test_object_definition_equality() {
    // Why: Test that two object definitions with the same properties are considered equal and ref equal.
    let def_1 = ObjectDefinition::builder("Test Object")
        .with_inserted(
            store_key!("prop1"),
            PropertyDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .finish();
    let def_2 = ObjectDefinition::builder("Test Object")
        .with_inserted(
            store_key!("prop1"),
            PropertyDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .finish();
    let def_3 = ObjectDefinition::builder("Test Object")
        .with_inserted(
            store_key!("prop1"),
            PropertyDefinition::new("P1", BasicDefinition::new_string("D2")),
        )
        .finish();

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(&def_1, def_2);
    assert_ne!(def_1, &def_3);
}
