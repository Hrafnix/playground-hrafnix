//! Integration tests for adding and copying objects between stores.
//!
//! Verifies that [`Store::copy_object`] correctly duplicates an object from one
//! store into another, that the copied data matches the source, and that
//! later mutations in either store do not affect the other.
use datastore::definition::{BasicDefinition, ItemDefinition, ObjectDefinition};
use datastore::path::StorePath;
use datastore::store::Store;
use datastore::{parameter_key, store_key};
use shareable_string::SharedStringStore;

#[test]
fn test_add_object_from_another_store() {
    let store1 = Store::new(SharedStringStore::new());
    let store2 = Store::new(SharedStringStore::new());

    let obj_key1: datastore::key::StoreKey = store_key!("object1").into();
    let def = ObjectDefinition::builder("Test Object")
        .with_parameter_inserted(
            parameter_key!("p_prop1"),
            ItemDefinition::new(
                "parameter 1",
                BasicDefinition::new_string("String parameter"),
            ),
        )
        .finish();

    let _proxy1 = store1.create_object(obj_key1.clone(), &def).unwrap();
    let prop_path = StorePath::new(obj_key1.clone()).with_segment(store_key!("p_prop1"));
    let mut basic1 = store1.basic(&prop_path).unwrap();
    basic1.set_value("Hello from Store 1");
    basic1.push().unwrap();

    // Add an object from store1 to store2
    let obj_key2: datastore::key::StoreKey = store_key!("object2").into();
    let proxy2 = store2
        .copy_object(obj_key2.clone(), &store1, obj_key1.clone())
        .unwrap();

    assert_eq!(proxy2.description().as_str(), "Test Object");

    let prop_path2 = StorePath::new(obj_key2.clone()).with_segment(store_key!("p_prop1"));
    let basic2 = store2.basic(&prop_path2).unwrap();
    assert_eq!(basic2.value().as_str(), "Hello from Store 1");

    // Verify they are independent
    let mut basic2_mut = store2.basic(&prop_path2).unwrap();
    basic2_mut.set_value("Changed in Store 2");

    let basic1_after = store1.basic(&prop_path).unwrap();
    assert_eq!(basic1_after.value().as_str(), "Hello from Store 1");
    assert_eq!(basic2_mut.value().as_str(), "Changed in Store 2");
}
