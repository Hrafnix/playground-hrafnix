use datastore::definition::{BasicDefinition, TableDefinition};
use datastore::store_key;

#[test]
fn test_table_definition() {
    // Why: Test table definition creation and properties.
    let table_def = TableDefinition::new(
        "A table",
        vec![
            (store_key!("col1"), BasicDefinition::new_string("Column 1")),
            (
                store_key!("col2"),
                BasicDefinition::new_number_with_default("Column 2", "test"),
            ),
        ],
    );

    // Check the various properties of the table definition.
    assert_eq!(table_def.description().as_ref(), "A table");
    assert_eq!(table_def.count(), 2);
    assert!(table_def.contains_key(store_key!("col1")));
    assert!(table_def.contains_key(store_key!("col2")));
    assert!(!table_def.contains_key(store_key!("col3")));

    let col1 = table_def.get(store_key!("col1")).unwrap();
    assert_eq!(col1.description().as_ref(), "Column 1");
    assert_eq!(col1.default_value().as_ref(), "");

    let col2 = table_def.get(store_key!("col2")).unwrap();
    assert_eq!(col2.description().as_ref(), "Column 2");
    assert_eq!(col2.default_value().as_ref(), "test");
}

#[test]
fn test_table_definition_equality() {
    // Why: Test that two table definitions with the same properties are considered equal and ref equal.
    let table_def_1 = TableDefinition::new(
        "A table",
        vec![
            (store_key!("col1"), BasicDefinition::new_string("Column 1")),
            (
                store_key!("col2"),
                BasicDefinition::new_number_with_default("Column 2", "test"),
            ),
        ],
    );

    let table_def_2 = TableDefinition::new(
        "A table",
        vec![
            (store_key!("col1"), BasicDefinition::new_string("Column 1")),
            (
                store_key!("col2"),
                BasicDefinition::new_number_with_default("Column 2", "test"),
            ),
        ],
    );

    let table_def_3 = TableDefinition::new(
        "A new table",
        vec![
            (
                store_key!("col1"),
                BasicDefinition::new_string("New Column 1"),
            ),
            (
                store_key!("col2"),
                BasicDefinition::new_number_with_default("New Column 2", "test"),
            ),
        ],
    );

    assert_eq!(table_def_1, table_def_2);
    assert_ne!(table_def_1, table_def_3);
    assert_eq!(&table_def_1, table_def_2);
    assert_ne!(table_def_1, &table_def_3);
}
