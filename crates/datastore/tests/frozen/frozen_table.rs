use datastore::definition::{BasicDefinition, TableDefinition};
use datastore::prelude::TableFrozen;
use datastore::store_key;

#[test]
fn test_table_frozen() {
    // Why: Test table frozen object creation and parameters.
    let table_frozen = TableFrozen::new(TableDefinition::new(
        "A table",
        vec![
            (store_key!("col1"), BasicDefinition::new_string("Column 1")),
            (
                store_key!("col2"),
                BasicDefinition::new_number_with_default("Column 2", "test"),
            ),
        ],
    ));

    // Check the various parameters of the table definition.
    assert_eq!(table_frozen.definition().description().as_ref(), "A table");
    assert_eq!(table_frozen.definition().count(), 2);
    assert!(table_frozen.definition().contains_key(store_key!("col1")));
    assert!(table_frozen.definition().contains_key(store_key!("col2")));
    assert!(!table_frozen.definition().contains_key(store_key!("col3")));

    let col1 = table_frozen.definition().get(store_key!("col1")).unwrap();
    assert_eq!(col1.description().as_ref(), "Column 1");
    assert_eq!(col1.default_value().as_ref(), "");

    let col2 = table_frozen.definition().get(store_key!("col2")).unwrap();
    assert_eq!(col2.description().as_ref(), "Column 2");
    assert_eq!(col2.default_value().as_ref(), "test");

    assert_eq!(table_frozen.row_count(), 0);
    assert_eq!(table_frozen.column_count(), 2);
    assert_ne!(table_frozen.hash(), [0u8; 32]);
}

#[test]
fn test_table_frozen_equality() {
    // Why: Test that two table definitions with the same parameters are considered equal.
    let table_frozen_1 = TableFrozen::new(TableDefinition::new(
        "A table",
        vec![
            (store_key!("col1"), BasicDefinition::new_string("Column 1")),
            (
                store_key!("col2"),
                BasicDefinition::new_number_with_default("Column 2", "test"),
            ),
        ],
    ));

    let table_frozen_2 = TableFrozen::new(TableDefinition::new(
        "A table",
        vec![
            (store_key!("col1"), BasicDefinition::new_string("Column 1")),
            (
                store_key!("col2"),
                BasicDefinition::new_number_with_default("Column 2", "test"),
            ),
        ],
    ));

    let table_frozen_3 = TableFrozen::new(TableDefinition::new(
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
    ));

    assert_eq!(table_frozen_1, table_frozen_2);
    assert_ne!(table_frozen_1, table_frozen_3);
    assert_eq!(&table_frozen_1, table_frozen_2);
    assert_ne!(table_frozen_1, &table_frozen_3);
}
