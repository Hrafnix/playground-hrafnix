//! Integration tests for the proxy-based store access layer.
//!
//! Exercises reading and mutating store data through [`BasicProxy`],
//! [`ObjectProxy`], [`StructProxy`], and [`TableProxy`], as well as proxy
//! validity, change-tracking (`has_changed` / `pull`), and push semantics.

use datastore::prelude::*;
use std::collections::BTreeMap;

#[test]
fn test_complex_proxy_structure() {
    let store = Store::new(SharedStringStore::new());

    // 1. Create a Table Definition
    let table_def = TableDefinition::new(
        "Nested Table",
        vec![(store_key!("col1"), BasicDefinition::new_string("default"))],
    );

    // 2. Create a Struct Definition containing the Table and a Basic parameter
    let struct_def = StructDefinition::new(
        "Nested Struct",
        vec![
            (store_key!("table"), StructItemDefinition::Table(table_def)),
            (
                store_key!("inner_basic"),
                StructItemDefinition::Basic(BasicDefinition::new_number_with_default(
                    "Inner Basic",
                    "42",
                )),
            ),
        ],
    );

    // 3. Create an Object Definition containing the Struct
    let mut builder = ObjectDefinition::builder("Complex Object");
    builder.insert_parameter(
        parameter_key!("p_outer_struct"),
        ItemDefinition::new("Outer Struct", struct_def),
    );
    let obj_def = builder.finish();

    // 4. Create Object in Store
    let obj_key = store_key!("complex_obj");
    let mut obj_proxy = store.create_object(obj_key, &obj_def).unwrap();

    // 5. Access Struct Container Proxy
    let struct_proxy = obj_proxy
        .parameter_container(store_key!("p_outer_struct"))
        .unwrap();
    assert_eq!(struct_proxy.description().as_ref(), "Nested Struct");

    // 6. Access Table Proxy from Struct
    // Since ContainerProxy doesn't have table, we need to use store with path
    let table_path = struct_proxy
        .path()
        .clone()
        .with_segment(store_key!("table"));
    let mut table_proxy = store.table(&table_path).unwrap();

    // 7. Access Basic Proxy from Struct
    let basic_path = struct_proxy
        .path()
        .clone()
        .with_segment(store_key!("inner_basic"));
    let mut basic_proxy = store.basic(&basic_path).unwrap();

    // 8. Modify values at deep levels
    assert_eq!(basic_proxy.value().as_ref(), "42");
    basic_proxy.set_value("100");

    table_proxy.append_row();
    table_proxy.set_cell(0, "col1", "new_value").unwrap();

    table_proxy.append_row();
    table_proxy
        .set_row(1, BTreeMap::from([("col1", "new_value")]))
        .unwrap();

    // 9. Check change detection
    assert!(basic_proxy.has_changed());
    assert!(table_proxy.has_changed());

    // 10. Push changes
    basic_proxy.push().unwrap();
    table_proxy.push().unwrap();

    assert!(!basic_proxy.has_changed());
    assert!(!table_proxy.has_changed());

    // 11. Verify changes via new proxies
    let _obj_proxy2 = store.object(obj_proxy.path().object_key().clone()).unwrap();
    let basic_proxy2 = store.basic(&basic_path).unwrap();
    assert_eq!(basic_proxy2.value().as_ref(), "100");

    let table_proxy2 = store.table(&table_path).unwrap();
    assert_eq!(table_proxy2.row_count(), 2);
    assert_eq!(
        table_proxy2.row(0).unwrap().get("col1").unwrap().as_ref(),
        "new_value"
    );
    assert_eq!(
        table_proxy2.row(1).unwrap().get("col1").unwrap().as_ref(),
        "new_value"
    );

    // 12. Check parent synchronization
    assert!(obj_proxy.has_changed());
    obj_proxy.pull().unwrap();
    assert!(!obj_proxy.has_changed());
}

#[test]
fn test_proxy_basic_operations() {
    let store = Store::new(SharedStringStore::new());

    // 1. Create Object Definition
    let mut builder = ObjectDefinition::builder("Test Object");
    builder.insert_parameter(
        parameter_key!("p_name"),
        ItemDefinition::new("Name", BasicDefinition::new_string("The name")),
    );
    let obj_def = builder.finish();

    // 2. Create Object in Store
    let obj_key = store_key!("my_object");
    let mut obj_proxy = store.create_object(obj_key, &obj_def).unwrap();

    assert_eq!(obj_proxy.description().as_ref(), "Test Object");

    // 3. Get Basic parameter Proxy
    let mut name_proxy = obj_proxy.parameter_basic(store_key!("p_name")).unwrap();
    assert_eq!(name_proxy.value().as_ref(), "");

    // 4. Set Value and Push
    name_proxy.set_value("Junie");
    assert!(name_proxy.has_changed());
    name_proxy.push().unwrap();
    assert!(!name_proxy.has_changed());

    // 5. Verify in store (via another proxy)
    let mut obj_proxy2 = store.object(obj_proxy.path().object_key().clone()).unwrap();
    let name_proxy2 = obj_proxy2.parameter_basic(store_key!("p_name")).unwrap();
    assert_eq!(name_proxy2.value().as_ref(), "Junie");

    // 6. Test Pull
    name_proxy.set_value("Something else");
    // Before pushing name_proxy, name_proxy2 still has "Junie"
    assert_eq!(name_proxy2.value().as_ref(), "Junie");

    name_proxy.push().unwrap();

    // name_proxy2 should now detect change
    assert!(name_proxy2.has_changed());
    let mut name_proxy2_mut = name_proxy2;
    name_proxy2_mut.pull().unwrap();
    assert_eq!(name_proxy2_mut.value().as_ref(), "Something else");
}

#[test]
fn test_proxy_deleted_object() {
    let store = Store::new(SharedStringStore::new());

    // 1. Create Object Definition
    let mut builder = ObjectDefinition::builder("Test Object");
    builder.insert_parameter(
        parameter_key!("p_name"),
        ItemDefinition::new("Name", BasicDefinition::new_string("The name")),
    );
    let obj_def = builder.finish();

    // 2. Create Object in Store
    let obj_key: datastore::key::StoreKey = store_key!("my_object").into();
    let mut obj_proxy = store.create_object(obj_key.clone(), &obj_def).unwrap();

    assert!(obj_proxy.is_valid());

    // 3. Delete Object from Store
    store.delete_object(obj_key).unwrap();

    // 4. Verify Proxy is now invalid
    assert!(!obj_proxy.is_valid());

    // 5. Verify operations on invalid proxy fail
    assert!(obj_proxy.parameter_basic(store_key!("p_name")).is_err());
    assert!(obj_proxy.pull().is_err());
}

#[test]
fn test_proxy_multiple_parameters() {
    let store = Store::new(SharedStringStore::new());

    let mut builder = ObjectDefinition::builder("Multi Prop Object");
    builder.insert_parameter(
        parameter_key!("p_name"),
        ItemDefinition::new("Name", BasicDefinition::new_string("The name")),
    );
    builder.insert_parameter(
        parameter_key!("p_age"),
        ItemDefinition::new("Age", BasicDefinition::new_string("The age")),
    );
    let obj_def = builder.finish();

    let obj_key = store_key!("user_1");
    let mut obj_proxy = store.create_object(obj_key, &obj_def).unwrap();

    let mut name_proxy = obj_proxy.parameter_basic(store_key!("p_name")).unwrap();
    let mut age_proxy = obj_proxy.parameter_basic(store_key!("p_age")).unwrap();

    name_proxy.set_value("Alice");
    age_proxy.set_value("30");

    name_proxy.push().unwrap();
    age_proxy.push().unwrap();

    // Verify both are updated
    let mut obj_proxy2 = store.object(obj_proxy.path().object_key().clone()).unwrap();
    assert_eq!(
        obj_proxy2
            .parameter_basic(store_key!("p_name"))
            .unwrap()
            .value()
            .as_ref(),
        "Alice"
    );
    assert_eq!(
        obj_proxy2
            .parameter_basic(store_key!("p_age"))
            .unwrap()
            .value()
            .as_ref(),
        "30"
    );
}

#[test]
fn test_proxy_sync_from_store() {
    let store = Store::new(SharedStringStore::new());

    let mut builder = ObjectDefinition::builder("Sync Object");
    builder.insert_parameter(
        parameter_key!("p_name"),
        ItemDefinition::new("Name", BasicDefinition::new_string("The name")),
    );
    let obj_def = builder.finish();

    let obj_key = store_key!("user_2");
    let mut proxy1 = store.create_object(obj_key, &obj_def).unwrap();
    let mut proxy2 = store.object(obj_key).unwrap();

    // Modify via proxy1
    let mut name_proxy1 = proxy1.parameter_basic(store_key!("p_name")).unwrap();
    name_proxy1.set_value("Bob");
    name_proxy1.push().unwrap();

    // proxy2 still has old value until pull
    let name_proxy2 = proxy2.parameter_basic(store_key!("p_name")).unwrap();
    // It seems proxy2 already sees "Bob" because they might share the same underlying Basic object
    // if object doesn't deep clone. Let's check.
    assert_eq!(name_proxy2.value().as_ref(), "Bob");

    // However, last_sync_hash in proxy2 should still be old
    assert!(proxy2.has_changed());

    proxy2.pull().unwrap();
    assert!(!proxy2.has_changed());
}

#[test]
fn test_proxy_is_valid_initially() {
    let store = Store::new(SharedStringStore::new());
    let mut builder = ObjectDefinition::builder("Test Object");
    builder.insert_parameter(
        parameter_key!("p_name"),
        ItemDefinition::new("Name", BasicDefinition::new_string("")),
    );
    let obj_def = builder.finish();
    let obj_key = store_key!("valid_obj");
    let obj_proxy = store.create_object(obj_key, &obj_def).unwrap();

    assert!(obj_proxy.is_valid());
}

#[test]
fn test_proxy_get_object() {
    let store = Store::new(SharedStringStore::new());

    // 1. Create Object Definition
    let mut builder = ObjectDefinition::builder("Test Object");
    builder.insert_parameter(
        parameter_key!("p_name"),
        ItemDefinition::new("Name", BasicDefinition::new_string("The name")),
    );
    let obj_def = builder.finish();

    // 2. Create Object in Store
    let obj_key = store_key!("my_object");
    let mut obj_proxy = store.create_object(obj_key, &obj_def).unwrap();

    // 3. Get Basic parameter Proxy
    let name_proxy = obj_proxy.parameter_basic(store_key!("p_name")).unwrap();

    // 4. Get object from name_proxy
    let obj_proxy_from_name = name_proxy.object().unwrap();
    assert_eq!(
        obj_proxy_from_name.path().object_key().as_ref(),
        "my_object"
    );
    assert_eq!(obj_proxy_from_name.description().as_ref(), "Test Object");

    // 5. Get object from obj_proxy itself
    let obj_proxy_from_itself = obj_proxy.object().unwrap();
    assert_eq!(
        obj_proxy_from_itself.path().object_key().as_ref(),
        "my_object"
    );
}
