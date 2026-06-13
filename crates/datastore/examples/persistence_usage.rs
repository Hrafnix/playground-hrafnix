//! Demonstrates store persistence: serializing a [`Store`] to JSON and
//! reloading it from disk.
//!
//! Populates a store with an object, saves it to a temporary JSON file via
//! [`Store::to_json`], then reconstructs the store with [`Store::from_json`]
//! and verifies that all values are intact.
use datastore::prelude::*;
use std::fs;

fn main() {
    let path = "example_store.json";

    // 1. Create a Store and Populate it
    let store = Store::new(Default::default());
    let mut builder = ObjectDefinition::builder("Settings Object");
    builder.insert(
        store_key!("theme"),
        PropertyDefinition::new("App theme", BasicDefinition::new_string("light")),
    );
    let def = builder.finish();

    store
        .create_object(store_key!("app_settings"), &def)
        .unwrap();

    // 2. Save the Store to a File
    // This will serialize the entire store state (objects, definitions, and shared strings).
    let json = store.to_json().expect("Failed to serialize store");
    fs::write(path, json).expect("Failed to save store");
    println!("Store saved to {}", path);

    // 3. Load the Store from the File
    // This creates a new store instance with the same state.
    let loaded_json = fs::read_to_string(path).expect("Failed to read store file");
    let loaded_store = Store::from_json(&loaded_json).expect("Failed to load store");
    println!("Store loaded from {}", path);

    // 4. Verify Loaded Data
    let mut keys = loaded_store.object_keys().unwrap();
    keys.sort();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].as_str(), "app_settings");

    // Clean up
    fs::remove_file(path).unwrap();
}
