//! Integration tests for store serialization and deserialization (save / load).
//!
//! Verifies that a [`Store`] can be serialized to JSON and reloaded without
//! data loss, covering basic parameters, nested structs, maps, tables, choice
//! definitions, and file references.  Also checks that the BLAKE3 content hash
//! is stable across a round-trip.

use datastore::prelude::*;
use std::fs;

#[test]
fn test_save_load_file() {
    let store = Store::new(SharedStringStore::new());
    let obj_key = store_key!("my_object");
    let def = ObjectDefinition::builder("My Test Object")
        .with_parameter_inserted(
            parameter_key!("p_prop1"),
            ItemDefinition::new("parameter 1", BasicDefinition::new_string("A string")),
        )
        .finish();

    let _proxy = store.create_object(obj_key.clone(), &def).unwrap();
    let prop_path = StorePath::new(obj_key).with_segment(store_key!("p_prop1"));

    {
        let mut basic = store.basic(&prop_path).unwrap();
        basic.set_value("Saved data");
        basic.push().unwrap();
    }

    let original_hash = store.get_blake3_hash();

    let temp_file = "test_store_save_load.json";
    let json = store.to_json().expect("Failed to serialize store");
    fs::write(temp_file, json).expect("Failed to write to file");

    // Verify JSON content doesn't have redundant definitions
    let json_content = fs::read_to_string(temp_file).unwrap();
    // In "items", we should not find "definition"
    // The items are under "my_object" (the 2nd element of the tuple in "objects")
    // The structure is now: "objects": { "my_object": [ { "description": ... }, { "items": { ... } } ] }
    assert!(
        !json_content.contains(r#""definition": {"#),
        "JSON still contains redundant definitions: {}",
        json_content
    );
    assert!(
        !json_content.contains(r#""string_store""#),
        "JSON still contains string_store: {}",
        json_content
    );

    let loaded_store = Store::from_json(&json_content).expect("Failed to load from JSON");

    // Check data consistency
    let loaded_basic = loaded_store
        .basic(&prop_path)
        .expect("Could not find parameter in loaded store");
    assert_eq!(loaded_basic.value().as_str(), "Saved data");

    // Check hash consistency
    let loaded_hash = loaded_store.get_blake3_hash();
    assert_eq!(original_hash, loaded_hash);

    // Cleanup
    fs::remove_file(temp_file).ok();
}

#[test]
fn test_save_load_comprehensive() {
    let store = Store::new(SharedStringStore::new());

    // 1. Define nested structures
    let table_def = TableDefinition::new(
        "Table",
        vec![
            (
                store_key!("col_str"),
                BasicDefinition::new_string("String col"),
            ),
            (
                store_key!("col_num"),
                BasicDefinition::new_number("Number col"),
            ),
        ],
    );

    let struct_def = StructDefinition::new(
        "Struct",
        vec![
            (
                store_key!("s_basic"),
                StructItemDefinition::Basic(BasicDefinition::new_string("Struct basic")),
            ),
            (
                store_key!("s_table"),
                StructItemDefinition::Table(table_def.clone()),
            ),
        ],
    );

    let map_def = MapDefinition::new("Map", struct_def.clone());

    // 2. Define Object with all types
    let obj_def = ObjectDefinition::builder("Comprehensive Object")
        .with_parameter_inserted(
            parameter_key!("p_string"),
            ItemDefinition::new("String", BasicDefinition::new_string("S")),
        )
        .with_parameter_inserted(
            parameter_key!("p_number"),
            ItemDefinition::new("Number", BasicDefinition::new_number("N")),
        )
        .with_parameter_inserted(
            parameter_key!("p_file"),
            ItemDefinition::new(
                "File",
                BasicDefinition::new_file("F", FileDefinition::new("*.txt")),
            ),
        )
        .with_parameter_inserted(
            parameter_key!("p_choice"),
            ItemDefinition::new(
                "Choice",
                BasicDefinition::new_choice(
                    "C",
                    ChoiceDefinition::new(vec!["A".into(), "B".into()]),
                ),
            ),
        )
        .with_parameter_inserted(
            parameter_key!("p_struct"),
            ItemDefinition::new("Struct", struct_def),
        )
        .with_parameter_inserted(
            parameter_key!("p_table"),
            ItemDefinition::new("Table", table_def),
        )
        .with_parameter_inserted(parameter_key!("p_map"), ItemDefinition::new("Map", map_def))
        .finish();

    let obj_key: datastore::key::StoreKey = store_key!("comp_obj").into();
    let mut obj_proxy = store.create_object(obj_key.clone(), &obj_def).unwrap();

    // 3. Populate data
    // p_string
    {
        let mut p = obj_proxy.parameter_basic(store_key!("p_string")).unwrap();
        p.set_value("Hello String");
        p.push().unwrap();
    }
    // p_number
    {
        let mut p = obj_proxy.parameter_basic(store_key!("p_number")).unwrap();
        p.set_value("12345");
        p.push().unwrap();
    }
    // p_file
    {
        let mut p = obj_proxy.parameter_basic(store_key!("p_file")).unwrap();
        p.set_value("test.txt");
        p.push().unwrap();
    }
    // p_choice
    {
        let mut p = obj_proxy.parameter_basic(store_key!("p_choice")).unwrap();
        p.set_value("B");
        p.push().unwrap();
    }
    // p_struct -> s_basic
    {
        let path = obj_proxy
            .parameter_container(store_key!("p_struct"))
            .unwrap()
            .path()
            .clone()
            .with_segment(store_key!("s_basic"));
        let mut p = store.basic(&path).unwrap();
        p.set_value("Struct Value");
        p.push().unwrap();
    }
    // p_struct -> s_table
    {
        let path = obj_proxy
            .parameter_container(store_key!("p_struct"))
            .unwrap()
            .path()
            .clone()
            .with_segment(store_key!("s_table"));
        let mut p = store.table(&path).unwrap();
        p.append_row();
        p.set_cell(0, "col_str", "Row 0").unwrap();
        p.set_cell(0, "col_num", "10").unwrap();
        p.push().unwrap();
    }
    // p_table
    {
        let mut p = obj_proxy.parameter_table(store_key!("p_table")).unwrap();
        p.append_row();
        p.set_cell(0, "col_str", "Table Row").unwrap();
        p.push().unwrap();
    }
    // p_map
    {
        let map_container = obj_proxy.parameter_container(store_key!("p_map")).unwrap();
        let item_key = store_key!("entry_1");
        let entry_proxy = map_container.insert_map_entry(item_key).unwrap();
        let path = entry_proxy.path();
        // Entry in the map is a Struct.
        let basic_path = path.clone().with_segment(store_key!("s_basic"));
        let mut p = store.basic(&basic_path).unwrap();
        p.set_value("Map Struct Value");
        p.push().unwrap();
    }

    let original_hash = store.get_blake3_hash();
    let temp_file = "test_comprehensive_save_load.json";
    let json = store.to_json().expect("Failed to save");
    fs::write(temp_file, json).expect("Failed to write");

    // 4. Load and Verify
    let json_content = fs::read_to_string(temp_file).expect("Failed to read");
    let loaded_store = Store::from_json(&json_content).expect("Failed to load");

    let mut loaded_obj = loaded_store.object(obj_key.clone()).unwrap();

    assert_eq!(
        loaded_obj
            .parameter_basic(store_key!("p_string"))
            .unwrap()
            .value()
            .as_ref(),
        "Hello String"
    );
    assert_eq!(
        loaded_obj
            .parameter_basic(store_key!("p_number"))
            .unwrap()
            .value()
            .as_ref(),
        "12345"
    );
    assert_eq!(
        loaded_obj
            .parameter_basic(store_key!("p_file"))
            .unwrap()
            .value()
            .as_ref(),
        "test.txt"
    );
    assert_eq!(
        loaded_obj
            .parameter_basic(store_key!("p_choice"))
            .unwrap()
            .value()
            .as_ref(),
        "B"
    );

    // Verify Struct
    let s_basic_path = StorePath::new(obj_key.clone())
        .with_segment(store_key!("p_struct"))
        .with_segment(store_key!("s_basic"));
    assert_eq!(
        loaded_store.basic(&s_basic_path).unwrap().value().as_ref(),
        "Struct Value"
    );

    // Verify Map
    let m_basic_path = StorePath::new(obj_key.clone())
        .with_segment(store_key!("p_map"))
        .with_segment(store_key!("entry_1"))
        .with_segment(store_key!("s_basic"));
    assert_eq!(
        loaded_store.basic(&m_basic_path).unwrap().value().as_ref(),
        "Map Struct Value"
    );

    // Verify Table
    let table_path = StorePath::new(obj_key.clone()).with_segment(store_key!("p_table"));
    let loaded_table = loaded_store.table(&table_path).unwrap();
    assert_eq!(loaded_table.row_count(), 1);
    assert_eq!(
        loaded_table
            .row(0)
            .unwrap()
            .get("col_str")
            .unwrap()
            .as_ref(),
        "Table Row"
    );

    let loaded_hash = loaded_store.get_blake3_hash();
    assert_eq!(original_hash, loaded_hash, "Hashes should match after load");

    fs::remove_file(temp_file).ok();
}

#[test]
fn test_launder_consistency_after_load() {
    let store = Store::new(SharedStringStore::new());
    let obj_key = store_key!("test_obj");
    let def = ObjectDefinition::builder("Test").finish();
    store.create_object(obj_key, &def).unwrap();

    let temp_file = "test_launder.json";
    let json = store.to_json().unwrap();
    let loaded_store = Store::from_json(&json).unwrap();

    let laundered1 = loaded_store.launder_string("hello".into());
    let laundered2 = loaded_store.launder_string("hello".into());

    assert!(laundered1.ptr_eq(&laundered2));

    fs::remove_file(temp_file).ok();
}
