//! Integration tests for the static store implementation.
//!
//! Validates conversion of a dynamic [`Store`] into a [`StaticStore`] and back,
//! covering all container kinds (`StaticBasic`, `StaticStruct`, `StaticTable`)
//! and ensuring that data fidelity is maintained across the round-trip.
use datastore::definition::{
    BasicDefinition, MapDefinition, ObjectDefinition, PropertyDefinition, TableDefinition,
};
use datastore::path::StorePath;
use datastore::static_store::{StaticBasic, StaticStruct, StaticStructItem, StaticTable};
use datastore::store::Store;
use datastore::store_key;
use shareable_string::SharedStringStore;
use std::collections::BTreeMap;

#[test]
fn test_static_basic_creation() {
    let basic_def = BasicDefinition::new_string("Static data");
    let static_basic = StaticBasic::new(basic_def, "".into());

    assert_eq!(static_basic.value(), "");
    assert_eq!(static_basic.definition().description(), "Static data")
}

#[test]
fn test_static_table_creation() {
    let table_def = TableDefinition::new(
        "Static Table",
        vec![(
            store_key!("static_column"),
            BasicDefinition::new_string("Static value"),
        )],
    );
    let static_table = StaticTable::new(
        table_def,
        vec![BTreeMap::from([(
            store_key!("static_column").into(),
            "test".into(),
        )])],
    );

    assert_eq!(static_table.definition().description(), "Static Table");
    assert_eq!(
        *static_table.row(0).unwrap().get("static_column").unwrap(),
        "test"
    );
    assert_eq!(*static_table.cell_by_index(0, 0).unwrap(), "test");
    assert_eq!(
        *static_table.cell_by_name(0, "static_column").unwrap(),
        "test"
    );
}

#[test]
fn test_static_struct_creation() {
    let mut items = BTreeMap::new();
    items.insert(
        store_key!("static_field").into(),
        StaticStructItem::Basic(StaticBasic::new(
            BasicDefinition::new_string("Static value"),
            "test".into(),
        )),
    );
    let static_struct = StaticStruct::new("Static Struct", items).unwrap();

    assert_eq!(static_struct.definition().description(), "Static Struct");
    assert_eq!(
        static_struct
            .get("static_field")
            .unwrap()
            .get_basic()
            .unwrap()
            .value()
            .as_str(),
        "test"
    );
}

#[test]
fn test_store_to_static() {
    let store = Store::new(SharedStringStore::new());
    let obj_key = store_key!("my_object");
    let def = ObjectDefinition::builder("My Test Object")
        .with_inserted(
            store_key!("prop1"),
            PropertyDefinition::new("Property 1", BasicDefinition::new_string("A string")),
        )
        .finish();

    let _proxy = store.create_object(obj_key.clone(), &def).unwrap();
    let prop_path = StorePath::new(obj_key).with_segment(store_key!("prop1"));

    {
        let mut basic = store.basic(&prop_path).unwrap();
        basic.set_value("Static data");
        basic.push().unwrap();
    }

    let static_store = store.to_static().unwrap();

    // Verify hash consistency
    assert_eq!(store.get_blake3_hash(), static_store.get_blake3_hash());

    // Verify data access
    let obj = static_store
        .get("my_object")
        .expect("Object not found in static store");
    let prop = obj
        .get("prop1")
        .expect("Property not found in static store");

    if let Some(basic) = prop.get_basic() {
        assert_eq!(basic.value().as_str(), "Static data");
    } else {
        panic!("Property is not a StaticBasic");
    }

    // Verify tree print doesn't crash
    println!("{}", static_store);
}

#[test]
fn test_static_to_store_roundtrip() {
    let store = Store::new(SharedStringStore::new());
    let obj_key = store_key!("my_object");
    let def = ObjectDefinition::builder("My Test Object")
        .with_inserted(
            store_key!("prop1"),
            PropertyDefinition::new("Property 1", BasicDefinition::new_string("A string")),
        )
        .finish();

    let _proxy = store.create_object(obj_key.clone(), &def).unwrap();
    let prop_path = datastore::path::StorePath::new(obj_key).with_segment(store_key!("prop1"));

    {
        let mut basic = store.basic(&prop_path).unwrap();
        basic.set_value("Roundtrip data");
        basic.push().unwrap();
    }

    let static_store = store.to_static().unwrap();
    let restored_store = Store::new_from_static(&static_store);

    // Verify hash consistency
    assert_eq!(store.get_blake3_hash(), restored_store.get_blake3_hash());
    assert_eq!(
        static_store.get_blake3_hash(),
        restored_store.get_blake3_hash()
    );

    // Verify data access in restored store
    let basic_proxy = restored_store.basic(&prop_path).unwrap();
    assert_eq!(basic_proxy.value().as_str(), "Roundtrip data");
}

#[test]
fn test_update_from_static() {
    let string_store = SharedStringStore::new();
    let store = Store::new(string_store.clone());
    let obj_key = store_key!("my_object");
    let def = ObjectDefinition::builder("My Test Object")
        .with_inserted(
            store_key!("prop1"),
            PropertyDefinition::new("Property 1", BasicDefinition::new_string("Initial")),
        )
        .finish();

    let mut proxy = store.create_object(obj_key.clone(), &def).unwrap();
    {
        let mut basic = proxy.basic(store_key!("prop1")).unwrap();
        basic.set_value("Initial");
        basic.push().unwrap();
    }
    proxy.sync().unwrap();

    // Create static store from initial state
    let static_store = store.to_static().unwrap();

    // Modify the static store (in a real scenario this might come from another source)
    // Here we'll just modify the store and create a new static one to simulate an updated version.
    {
        let mut basic = store
            .basic(
                &datastore::path::StorePath::new(obj_key.clone()).with_segment(store_key!("prop1")),
            )
            .unwrap();
        basic.set_value("Updated");
        basic.push().unwrap();
    }
    let updated_static_store = store.to_static().unwrap();

    // Reset store to initial state (using the first static store)
    store.sync_from_static(&static_store).unwrap();
    proxy.sync().unwrap();
    assert_eq!(
        proxy.basic(store_key!("prop1")).unwrap().value().as_str(),
        "Initial"
    );

    // Now update from the "updated" static store
    store.sync_from_static(&updated_static_store).unwrap();
    proxy.sync().unwrap();

    // Verify that the proxy still works and reflects the update
    assert_eq!(
        proxy.basic(store_key!("prop1")).unwrap().value().as_str(),
        "Updated"
    );
    assert_eq!(
        store.get_blake3_hash(),
        updated_static_store.get_blake3_hash()
    );
}

#[test]
fn test_update_from_static_definition_mismatch() {
    let store = Store::new(SharedStringStore::new());
    let obj_key = store_key!("my_object");

    let def1 = ObjectDefinition::builder("Def 1")
        .with_inserted(
            store_key!("prop1"),
            PropertyDefinition::new("Property 1", BasicDefinition::new_string("Initial")),
        )
        .finish();

    let _proxy1 = store.create_object(obj_key.clone(), &def1).unwrap();

    let def2 = ObjectDefinition::builder("Def 2")
        .with_inserted(
            store_key!("prop2"),
            PropertyDefinition::new("Property 2", BasicDefinition::new_number("0")),
        )
        .finish();

    let other_store = Store::new(SharedStringStore::new());
    other_store.create_object(obj_key.clone(), &def2).unwrap();
    let static_store2 = other_store.to_static().unwrap();

    // Update store with a static store that has a different definition for the same key
    store.sync_from_static(&static_store2).unwrap();

    // Verify it was replaced
    let obj_keys = store.object_keys().unwrap();
    assert_eq!(obj_keys.len(), 1);
    assert_eq!(obj_keys[0].as_str(), obj_key.as_str());

    let mut proxy = store.object(obj_key.clone()).unwrap();
    assert!(proxy.basic(store_key!("prop2")).is_ok());
    assert!(proxy.basic(store_key!("prop1")).is_err());
}

#[test]
fn test_update_from_static_does_not_remove_missing_properties() {
    let store = Store::new(SharedStringStore::new());

    // Object: Has two properties
    let obj_key = store_key!("my_object");
    let def = ObjectDefinition::builder("My Object")
        .with_inserted(
            store_key!("prop1"),
            PropertyDefinition::new("Property 1", BasicDefinition::new_string("Initial")),
        )
        .with_inserted(
            store_key!("prop2"),
            PropertyDefinition::new("Property 2", BasicDefinition::new_string("Stay")),
        )
        .finish();
    store.create_object(obj_key.clone(), &def).unwrap();
    println!("{}", store);

    // Create a static store with the same object but ONLY prop1 (updated)
    let other_store = Store::new(SharedStringStore::new());
    let def_updated = ObjectDefinition::builder("My Object")
        .with_inserted(
            store_key!("prop1"),
            PropertyDefinition::new("Property 1", BasicDefinition::new_string("Updated")),
        )
        .finish();
    let mut other_proxy = other_store
        .create_object(obj_key.clone(), &def_updated)
        .unwrap();
    {
        let mut prop1_proxy = other_proxy.basic(store_key!("prop1")).unwrap();
        prop1_proxy.set_value("Updated");
        prop1_proxy.push().unwrap();
    }
    let static_store = other_store.to_static().unwrap();

    // Update original store from static store
    store.sync_from_static(&static_store).unwrap();
    println!("{}", store);

    // Verify prop1 was updated
    let mut proxy = store.object(obj_key.clone()).unwrap();
    let prop1_proxy = proxy.basic(store_key!("prop1")).unwrap();
    assert_eq!(prop1_proxy.value().as_str(), "Updated");

    // Verify prop2 still exists and was removed
    assert!(proxy.basic(store_key!("prop2")).is_err());
}

#[test]
fn test_update_from_static_does_not_remove_missing_objects() {
    let store = Store::new(SharedStringStore::new());

    // Object 1: Will be in both
    let obj_key1 = store_key!("object1");
    let def1 = ObjectDefinition::builder("Object 1")
        .with_inserted(
            store_key!("prop1"),
            PropertyDefinition::new("Property 1", BasicDefinition::new_string("Initial")),
        )
        .finish();
    store.create_object(obj_key1.clone(), &def1).unwrap();

    // Object 2: Only in original store, NOT in static store
    let obj_key2 = store_key!("object2");
    let def2 = ObjectDefinition::builder("Object 2")
        .with_inserted(
            store_key!("prop2"),
            PropertyDefinition::new("Property 2", BasicDefinition::new_string("Stay")),
        )
        .finish();
    store.create_object(obj_key2.clone(), &def2).unwrap();

    println!("{}", store);

    // Create a static store that only has object1 (updated)
    let other_store = Store::new(SharedStringStore::new());
    let def1_updated = ObjectDefinition::builder("Object 1")
        .with_inserted(
            store_key!("prop1"),
            PropertyDefinition::new("Property 1", BasicDefinition::new_string("Updated")),
        )
        .finish();
    let mut other_proxy = other_store
        .create_object(obj_key1.clone(), &def1_updated)
        .unwrap();
    let mut prop1_proxy = other_proxy.basic(store_key!("prop1")).unwrap();
    prop1_proxy.set_value("Updated");
    prop1_proxy.push().unwrap();
    let static_store = other_store.to_static().unwrap();
    println!("{}", static_store);

    // Update original store from static store
    store.merge_from_static(&static_store).unwrap();
    println!("{}", store);

    // Verify object1 was updated
    let mut proxy1 = store.object(obj_key1.clone()).unwrap();
    // No need to pull because we created a NEW proxy from the store
    let prop1_proxy = proxy1.basic(store_key!("prop1")).unwrap();
    assert_eq!(prop1_proxy.value().as_str(), "Updated");

    // Verify object2 still exists
    let mut proxy2 = store.object(obj_key2.clone()).unwrap();
    assert!(proxy2.basic(store_key!("prop2")).is_ok());

    // Verify both keys are present
    let obj_keys = store.object_keys().unwrap();
    assert_eq!(obj_keys.len(), 2);
    assert!(obj_keys.iter().any(|k| k.as_str() == "object1"));
    assert!(obj_keys.iter().any(|k| k.as_str() == "object2"));
}

#[test]
fn test_update_from_static_add_object() {
    let store = Store::new(SharedStringStore::new());

    let other_store = Store::new(SharedStringStore::new());
    let obj_key = store_key!("new_object");
    let def = ObjectDefinition::builder("New Object").finish();
    other_store.create_object(obj_key.clone(), &def).unwrap();
    let static_store = other_store.to_static().unwrap();

    // Update store (which is empty) from static store
    store.sync_from_static(&static_store).unwrap();

    // Verify it was added
    let obj_keys = store.object_keys().unwrap();
    assert_eq!(obj_keys.len(), 1);
    assert_eq!(obj_keys[0].as_str(), obj_key.as_str());
    assert!(store.object(obj_key.clone()).is_ok());
}

#[test]
fn test_static_map_with_structs() {
    use datastore::definition::StructDefinition;
    let store = Store::new(SharedStringStore::new());
    let obj_key = store_key!("my_object");

    let struct_def = StructDefinition::new(
        "A Struct",
        vec![(store_key!("s_prop"), BasicDefinition::new_string("s_val"))],
    );

    let map_def = MapDefinition::new("A Map", struct_def);

    let def = ObjectDefinition::builder("Object with Map")
        .with_inserted(
            store_key!("my_map"),
            PropertyDefinition::new("My Map", map_def),
        )
        .finish();

    let mut proxy = store.create_object(obj_key.clone(), &def).unwrap();

    {
        let map_proxy = proxy.container(store_key!("my_map")).unwrap();
        let s_proxy = map_proxy.insert_map_entry(store_key!("key1")).unwrap();
        let s_path = s_proxy.path();
        let mut b_proxy = store
            .basic(&s_path.clone().with_segment(store_key!("s_prop")))
            .unwrap();
        b_proxy.set_value("initial");
        b_proxy.push().unwrap();
    }

    let static_store = store.to_static().unwrap();

    // Verify static map access
    let obj = static_store.get("my_object").unwrap();
    let map_prop = obj.get("my_map").unwrap();
    let static_map = map_prop.get_map().unwrap();

    let static_struct = static_map.get("key1").unwrap();
    let basic_item = static_struct.get("s_prop").unwrap();
    if let StaticStructItem::Basic(b) = basic_item {
        assert_eq!(b.value().as_str(), "initial");
    } else {
        panic!("Expected basic item");
    }

    // Roundtrip update
    let other_store = Store::new(SharedStringStore::new());
    other_store.create_object(obj_key.clone(), &def).unwrap();

    other_store.sync_from_static(&static_store).unwrap();

    let s_path = datastore::path::StorePath::new(store_key!("my_object"))
        .with_segment(store_key!("my_map"))
        .with_segment(store_key!("key1"));
    let b_path = s_path.clone().with_segment(store_key!("s_prop"));
    let b_proxy = other_store.basic(&b_path).unwrap();
    assert_eq!(b_proxy.value().as_str(), "initial");
}

#[test]
fn test_static_store_all_types() {
    use datastore::definition::StructDefinition;

    let store = Store::new(SharedStringStore::new());
    let obj_key = store_key!("example_item");

    // Define Struct type
    let struct_def = StructDefinition::new(
        "A sample struct",
        vec![
            (
                store_key!("field_1"),
                BasicDefinition::new_string("Field 1"),
            ),
            (
                store_key!("field_2"),
                BasicDefinition::new_number("Field 2"),
            ),
        ],
    );

    // Define Table type
    let table_def = TableDefinition::new(
        "A sample table",
        vec![
            (store_key!("col_1"), BasicDefinition::new_string("Column 1")),
            (store_key!("col_2"), BasicDefinition::new_number("Column 2")),
        ],
    );

    // Define Map type
    let map_def = MapDefinition::new("A sample map", struct_def.clone());

    // Define Object structure
    let mut builder = ObjectDefinition::builder("Example Object");
    builder.insert(
        store_key!("basic_prop"),
        PropertyDefinition::new("Basic Property", BasicDefinition::new_string("Basic")),
    );
    builder.insert(
        store_key!("table_prop"),
        PropertyDefinition::new("Table Property", table_def.clone()),
    );
    builder.insert(
        store_key!("struct_prop"),
        PropertyDefinition::new("Struct Property", struct_def.clone()),
    );
    builder.insert(
        store_key!("map_prop"),
        PropertyDefinition::new("Map Property", map_def.clone()),
    );
    let object_def = builder.finish();

    // Create the object in the store
    store
        .create_object(obj_key.clone(), &object_def)
        .expect("Failed to create object");

    // Populate the data
    let mut object_proxy = store.object("example_item").unwrap();

    // Set Basic property
    {
        let mut basic = object_proxy.basic(store_key!("basic_prop")).unwrap();
        basic.set_value("Hello, Static Store!");
        basic.push().unwrap();
    }

    // Set Table property
    {
        let mut table = object_proxy.table(store_key!("table_prop")).unwrap();
        table.append_row();
        table.set_cell(0, "col_1", "Row 0, Col 1").unwrap();
        table.set_cell(0, "col_2", "42").unwrap();
        table.push().unwrap();
    }

    // Set Struct property
    {
        let struct_container = object_proxy.container(store_key!("struct_prop")).unwrap();
        let mut s_field_1 = store
            .basic(
                &struct_container
                    .path()
                    .clone()
                    .with_segment(store_key!("field_1")),
            )
            .unwrap();
        s_field_1.set_value("Struct Value");
        s_field_1.push().unwrap();

        let mut s_field_2 = store
            .basic(
                &struct_container
                    .path()
                    .clone()
                    .with_segment(store_key!("field_2")),
            )
            .unwrap();
        s_field_2.set_value("123");
        s_field_2.push().unwrap();
    }

    // Set Map property
    {
        let map_container = object_proxy.container(store_key!("map_prop")).unwrap();
        let entry_proxy = map_container
            .insert_map_entry(store_key!("entry_1"))
            .unwrap();
        let mut m_field_1 = store
            .basic(
                &entry_proxy
                    .path()
                    .clone()
                    .with_segment(store_key!("field_1")),
            )
            .unwrap();
        m_field_1.set_value("Map Entry Value");
        m_field_1.push().unwrap();

        let mut m_field_2 = store
            .basic(
                &entry_proxy
                    .path()
                    .clone()
                    .with_segment(store_key!("field_2")),
            )
            .unwrap();
        m_field_2.set_value("456");
        m_field_2.push().unwrap();
    }

    // Convert the store to a StaticStore
    let static_store = store.to_static().unwrap();

    // Print static store
    println!("{}", static_store);

    // Verify hash consistency
    assert_eq!(store.get_blake3_hash(), static_store.get_blake3_hash());

    // Verify StaticStore data access
    let obj = static_store.get("example_item").unwrap();

    // Basic
    assert_eq!(
        obj.get("basic_prop")
            .unwrap()
            .get_basic()
            .unwrap()
            .value()
            .as_str(),
        "Hello, Static Store!"
    );

    // Table
    let table = obj.get("table_prop").unwrap().get_table().unwrap();
    assert_eq!(
        table.cell_by_name(0, "col_1").unwrap().as_str(),
        "Row 0, Col 1"
    );
    assert_eq!(table.cell_by_name(0, "col_2").unwrap().as_str(), "42");

    // Struct
    let r_struct = obj.get("struct_prop").unwrap().get_struct().unwrap();
    if let StaticStructItem::Basic(b) = r_struct.get("field_1").unwrap() {
        assert_eq!(b.value().as_str(), "Struct Value");
    } else {
        panic!("Expected Basic for field_1");
    }

    // Map
    let map = obj.get("map_prop").unwrap().get_map().unwrap();
    let entry_struct = map.get("entry_1").unwrap();
    if let StaticStructItem::Basic(b) = entry_struct.get("field_2").unwrap() {
        assert_eq!(b.value().as_str(), "456");
    } else {
        panic!("Expected Basic for field_2 in map entry");
    }

    // Roundtrip back to a new Store
    let restored_store = Store::new_from_static(&static_store);
    assert_eq!(store.get_blake3_hash(), restored_store.get_blake3_hash());

    // Verify data in restored store
    let mut rest_obj_proxy = restored_store.object("example_item").unwrap();
    assert_eq!(
        rest_obj_proxy
            .basic(store_key!("basic_prop"))
            .unwrap()
            .value()
            .as_str(),
        "Hello, Static Store!"
    );

    let rest_table_proxy = rest_obj_proxy.table(store_key!("table_prop")).unwrap();
    assert_eq!(
        rest_table_proxy
            .row(0)
            .unwrap()
            .get("col_1")
            .unwrap()
            .as_str(),
        "Row 0, Col 1"
    );

    let rest_struct_container = rest_obj_proxy.container(store_key!("struct_prop")).unwrap();
    let rest_s_field_1 = restored_store
        .basic(
            &rest_struct_container
                .path()
                .clone()
                .with_segment(store_key!("field_1")),
        )
        .unwrap();
    assert_eq!(rest_s_field_1.value().as_str(), "Struct Value");

    let rest_map_container = rest_obj_proxy.container(store_key!("map_prop")).unwrap();
    let rest_m_path = rest_map_container
        .path()
        .clone()
        .with_segment(store_key!("entry_1"))
        .with_segment(store_key!("field_2"));
    let rest_m_field_2 = restored_store.basic(&rest_m_path).unwrap();
    assert_eq!(rest_m_field_2.value().as_str(), "456");
}
