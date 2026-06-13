//! Demonstrates advanced definition types: [`StructDefinition`], [`MapDefinition`],
//! and [`TableDefinition`].
//!
//! Shows how to define reusable struct types, key-value maps, and row-based
//! tables, then create and populate them in a [`Store`] using proxy-based access
//! and [`StorePath`] navigation.
use datastore::prelude::*;
use std::collections::BTreeMap;

fn main() {
    // 1. Create a Store
    let store = Store::new(Default::default());

    // 2. Define the Struct
    // A Struct is a reusable and nested structure with named fields.
    let address_def = StructDefinition::new(
        "Address",
        vec![
            (store_key!("street"), BasicDefinition::new_string("")),
            (store_key!("city"), BasicDefinition::new_string("")),
        ],
    );

    // 3. Define the Map
    // A Map stores multiple instances of a Struct, keyed by strings.
    let contacts_def = MapDefinition::new("Contacts", address_def.clone());

    // 4. Define the Table
    // A Table is a collection of rows with fixed columns.
    let inventory_def = TableDefinition::new(
        "Inventory",
        vec![
            (store_key!("item_id"), BasicDefinition::new_string("")),
            (store_key!("quantity"), BasicDefinition::new_number("0")),
        ],
    );

    // 5. Create an Object with these components
    let builder = ObjectDefinition::builder("Company Data");
    let company_def = builder
        .with_inserted(
            store_key!("hq_address"),
            PropertyDefinition::new("Headquarters", address_def),
        )
        .with_inserted(
            store_key!("branches"),
            PropertyDefinition::new("Branch Offices", contacts_def),
        )
        .with_inserted(
            store_key!("stock"),
            PropertyDefinition::new("Warehouse Stock", inventory_def),
        )
        .finish();

    store
        .create_object(store_key!("my_company"), &company_def)
        .unwrap();

    let mut company_proxy = store.object("my_company").unwrap();
    let company_key = "my_company";

    // Access the 'hq_address' struct
    // We can now use the 'path!' macro for more ergonomic path construction.
    let street_path = path!(company_key / "hq_address" / "street");

    let mut street_proxy = store.basic(&street_path).unwrap();
    street_proxy.set_value("123 Main St");
    street_proxy.push().unwrap();

    println!("HQ Street: {}", street_proxy.value().as_str());

    // 7. Interact with the Map
    // Maps allows inserting new entries that follow the defined Struct schema.
    let branches_proxy = company_proxy.container(store_key!("branches")).unwrap();

    // Insert a new branch "london"
    branches_proxy
        .insert_map_entry(store_key!("london"))
        .unwrap();

    // Now we can access the 'london' branch fields using a tuple for ergonomics.
    let london_city_path: StorePath = (
        store_key!("my_company"),
        store_key!("branches"),
        store_key!("london"),
        store_key!("city"),
    )
        .into();

    let mut london_city_proxy = store.basic(&london_city_path).unwrap();
    london_city_proxy.set_value("London");
    london_city_proxy.push().unwrap();

    println!(
        "Branch 'london' city: {}",
        london_city_proxy.value().as_str()
    );

    // 8. Interact with the Table
    let mut table_proxy = company_proxy.table(store_key!("stock")).unwrap();

    // Add some rows
    table_proxy.append_row();
    table_proxy.set_cell(0, "item_id", "widget_a").unwrap();
    table_proxy.set_cell(0, "quantity", "50").unwrap();

    table_proxy.append_row();
    table_proxy
        .set_row(
            1,
            BTreeMap::from([("item_id", "widget_b"), ("quantity", "25")]),
        )
        .unwrap();

    table_proxy.push().unwrap();

    println!("Stock table rows: {}", table_proxy.row_count());
    for i in 0..table_proxy.row_count() {
        let row = table_proxy.row(i).unwrap();
        println!(
            "  Row {}: ID={}, Qty={}",
            i, row["item_id"], row["quantity"]
        );
    }

    println!("\nFull Store Tree Print:\n{}", store);
}
