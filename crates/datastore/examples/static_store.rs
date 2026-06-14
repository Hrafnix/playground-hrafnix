//! Demonstrates the static store: converting a dynamic [`Store`] into a
//! read-optimized [`StaticStore`].
//!
//! Builds a store containing objects with structs, maps, and tables, converts
//! it to a static representation, and shows how to traverse the static data
//! hierarchy for fast, allocation-free reads.
use datastore::prelude::*;

fn main() {
    // 1. Initialize the shared string store and the main store.
    let string_store = Default::default();
    let store = Store::new(string_store);

    // 2. Define our data structure.
    // We'll create an object that contains one of each item type.

    // Define a Struct type
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

    // Define a Table type
    let table_def = TableDefinition::new(
        "A sample table",
        vec![
            (store_key!("col_1"), BasicDefinition::new_string("Column 1")),
            (store_key!("col_2"), BasicDefinition::new_number("Column 2")),
        ],
    );

    // Define a Map type (maps strings to our struct_def)
    let map_def = MapDefinition::new("A sample map", struct_def.clone());

    // Define the main Object structure
    let mut builder = ObjectDefinition::builder("Example Object");
    builder.insert_parameter(
        parameter_key!("p_basic_prop"),
        ItemDefinition::new("Basic parameter", BasicDefinition::new_string("Basic")),
    );
    builder.insert_parameter(
        parameter_key!("p_table_prop"),
        ItemDefinition::new("Table parameter", table_def.clone()),
    );
    builder.insert_parameter(
        parameter_key!("p_struct_prop"),
        ItemDefinition::new("Struct parameter", struct_def.clone()),
    );
    builder.insert_parameter(
        parameter_key!("p_map_prop"),
        ItemDefinition::new("Map parameter", map_def.clone()),
    );
    let object_def = builder.finish();

    // 3. Create the object in the store.
    store
        .create_object(store_key!("example_item"), &object_def)
        .expect("Failed to create object");

    // 4. Populate the data.
    let mut object_proxy = store.object("example_item").unwrap();

    // Set Basic parameter
    let mut basic = object_proxy
        .parameter_basic(store_key!("p_basic_prop"))
        .unwrap();
    basic.set_value("Hello, Static Store!");
    basic.push().unwrap();

    // Set Table parameter
    let mut table = object_proxy
        .parameter_table(store_key!("p_table_prop"))
        .unwrap();
    table.append_row();
    table.set_cell(0, "col_1", "Row 0, Col 1").unwrap();
    table.set_cell(0, "col_2", "42").unwrap();
    table.push().unwrap();

    // Set Struct parameter
    let struct_container = object_proxy
        .parameter_container(store_key!("p_struct_prop"))
        .unwrap();
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

    // Set Map parameter
    let map_container = object_proxy
        .parameter_container(store_key!("p_map_prop"))
        .unwrap();
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

    // 5. Convert the store to a StaticStore.
    // A StaticStore is a read-only, serializable snapshot of the store.
    let static_store = store.to_static().expect("Failed to create static store");

    // 6. Demonstrate StaticStore functionality.
    println!("--- Static Store Tree View ---\n{}", static_store);

    // 7. Accessing data in StaticStore
    if let Some(obj) = static_store.get("example_item") {
        if let Some(prop) = obj.get_parameter("p_basic_prop") {
            if let Some(basic) = prop.get_basic() {
                println!("\nDirect access to basic_prop: {}", basic.value().as_str());
            }
        }
    }
}
