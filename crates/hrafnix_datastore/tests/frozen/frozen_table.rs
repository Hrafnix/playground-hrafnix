use hrafnix_datastore::prelude::*;

#[test]
fn table_frozen_from_rows_normalizes_row_widths() {
    let rows = vec![
        vec![],
        vec!["1".into()],
        vec!["2".into(), "3".into()],
        vec!["4".into(), "5".into(), "discarded".into()],
    ];
    let expected_cases: [Vec<Vec<ShareableString>>; 3] = [
        vec![vec![]; 4],
        vec![
            vec!["".into()],
            vec!["1".into()],
            vec!["2".into()],
            vec!["4".into()],
        ],
        vec![
            vec!["".into(), "".into()],
            vec!["1".into(), "".into()],
            vec!["2".into(), "3".into()],
            vec!["4".into(), "5".into()],
        ],
    ];
    for (column_count, expected_rows) in expected_cases.into_iter().enumerate() {
        let definition = TableDefinition::new(
            "Table",
            [
                (
                    store_key!("z"),
                    NumberDefinition::new_with_default("", "99"),
                ),
                (
                    store_key!("a"),
                    NumberDefinition::new_with_default("", "99"),
                ),
            ]
            .into_iter()
            .take(column_count)
            .collect(),
        );
        let table = TableFrozen::new_from_rows(definition.clone(), rows.clone());
        let expected = TableFrozen::new_from_rows(definition.clone(), expected_rows.clone());

        assert_eq!(table.rows(), expected_rows);
        assert_eq!(table.row_count(), 4);
        assert_eq!(table.column_count(), column_count);
        assert_eq!(table, expected);
        assert_eq!(table.hash(), expected.hash());
        assert_eq!(table.thaw().freeze(), table);
        assert_eq!(table.launder(&SharedStringStore::new()), table);
        assert_eq!(
            TableFrozen::new_from_rows(definition, vec![]).row_count(),
            0
        );
    }
}

#[test]
fn test_table_frozen() {
    // Why: Test table frozen object creation and parameters.
    let table_frozen = TableFrozen::new(TableDefinition::new(
        "A table",
        vec![
            (store_key!("col1"), NumberDefinition::new("Column 1")),
            (
                store_key!("col2"),
                NumberDefinition::new_with_default("Column 2", "test"),
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
            (store_key!("col1"), NumberDefinition::new("Column 1")),
            (
                store_key!("col2"),
                NumberDefinition::new_with_default("Column 2", "test"),
            ),
        ],
    ));

    let table_frozen_2 = TableFrozen::new(TableDefinition::new(
        "A table",
        vec![
            (store_key!("col1"), NumberDefinition::new("Column 1")),
            (
                store_key!("col2"),
                NumberDefinition::new_with_default("Column 2", "test"),
            ),
        ],
    ));

    let table_frozen_3 = TableFrozen::new(TableDefinition::new(
        "A new table",
        vec![
            (store_key!("col1"), NumberDefinition::new("New Column 1")),
            (
                store_key!("col2"),
                NumberDefinition::new_with_default("New Column 2", "test"),
            ),
        ],
    ));

    assert_eq!(table_frozen_1, table_frozen_2);
    assert_ne!(table_frozen_1, table_frozen_3);
    assert_eq!(&table_frozen_1, table_frozen_2);
    assert_ne!(table_frozen_1, &table_frozen_3);
}

#[test]
fn test_table_frozen_uses_definition_default_table() {
    let table = TableFrozen::new(TableDefinition::new_with_default(
        "A table",
        vec![(store_key!("col1"), NumberDefinition::new("Column 1"))],
        vec![vec!["42"]],
    ));

    assert_eq!(table.cell_by_index(0, 0).map(AsRef::as_ref), Some("42"));
}
