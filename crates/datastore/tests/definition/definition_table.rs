use datastore::prelude::*;

#[test]
fn test_table_definition() {
    // Why: Test table definition creation and parameters.
    let table_def = TableDefinition::new(
        "A table",
        vec![
            (store_key!("col1"), NumberDefinition::new("Column 1")),
            (
                store_key!("col2"),
                NumberDefinition::new_with_default("Column 2", "test"),
            ),
        ],
    );

    // Check the various data items of the table definition.
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
    // Why: Test that two table definitions with the same data items are considered equal.
    let table_def_1 = TableDefinition::new(
        "A table",
        vec![
            (store_key!("col1"), NumberDefinition::new("Column 1")),
            (
                store_key!("col2"),
                NumberDefinition::new_with_default("Column 2", "test"),
            ),
        ],
    );

    let table_def_2 = TableDefinition::new(
        "A table",
        vec![
            (store_key!("col1"), NumberDefinition::new("Column 1")),
            (
                store_key!("col2"),
                NumberDefinition::new_with_default("Column 2", "test"),
            ),
        ],
    );

    let table_def_3 = TableDefinition::new(
        "A new table",
        vec![
            (store_key!("col1"), NumberDefinition::new("New Column 1")),
            (
                store_key!("col2"),
                NumberDefinition::new_with_default("New Column 2", "test"),
            ),
        ],
    );

    assert_eq!(table_def_1, table_def_2);
    assert_ne!(table_def_1, table_def_3);
    assert_eq!(&table_def_1, table_def_2);
    assert_ne!(table_def_1, &table_def_3);
}

#[test]
fn test_table_definition_deduplicates_column_keys() {
    let table_def = TableDefinition::new(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberDefinition::new("Initial length"),
            ),
            (store_key!("duration"), NumberDefinition::new("Duration")),
            (
                store_key!("length"),
                NumberDefinition::new("Replacement length"),
            ),
        ],
    );

    assert_eq!(table_def.count(), 2);
    assert_eq!(
        table_def.keys().map(StoreKey::as_str).collect::<Vec<_>>(),
        vec!["duration", "length"]
    );
    assert_eq!(
        table_def.get_by_index(1).map(NumberDefinition::description),
        Some("Replacement length".into())
    );
}

#[test]
fn test_table_definition_with_default() {
    let table_def = TableDefinition::new_with_default(
        "A table",
        vec![
            (store_key!("col1"), NumberDefinition::new("Column 1")),
            (store_key!("col2"), NumberDefinition::new("Column 2")),
        ],
        vec![vec!["42"], vec!["1", "2", "3"]],
    );

    let default_table = table_def.default_table().expect("default table");
    assert_eq!(default_table[0][0].as_ref(), "42");
    assert_eq!(default_table[0][1].as_ref(), "");
    assert_eq!(default_table[1].len(), 2);
    assert_eq!(default_table[1][1].as_ref(), "2");
}
