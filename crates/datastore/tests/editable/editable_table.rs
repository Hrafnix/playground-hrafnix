use datastore::prelude::*;

#[test]
fn test_editable_table_round_trip() {
    // Why: Editable table should thaw from frozen, allow row edits, and freeze back correctly.
    let frozen = TableFrozen::new(TableDefinition::new(
        "A table",
        vec![
            (store_key!("col1"), NumberDefinition::new("Column 1")),
            (
                store_key!("col2"),
                NumberDefinition::new_with_default("Column 2", "test"),
            ),
        ],
    ));

    let mut editable = frozen.thaw();
    assert_eq!(editable.row_count(), 0);
    assert_eq!(editable.column_count(), 2);

    editable.add_row(0);
    assert_eq!(editable.row_count(), 1);
    assert_eq!(
        editable.cell_by_name(0, "col2").map(AsRef::as_ref),
        Some("test")
    );

    editable.set_cell(0, "col1", "5").expect("set cell");
    assert_eq!(
        editable.cell_by_name(0, "col1").map(AsRef::as_ref),
        Some("5")
    );

    editable.add_row(1);
    editable.remove_row(0);
    assert_eq!(editable.row_count(), 1);

    let frozen_2 = editable.freeze();
    assert_eq!(frozen_2.row_count(), 1);
    assert_ne!(frozen_2.hash(), frozen.hash());
}

#[test]
fn test_editable_table_set_cell_invalid_column() {
    // Why: Setting a cell with an unknown column name should return an error.
    let frozen = TableFrozen::new(TableDefinition::new(
        "A table",
        vec![(store_key!("col1"), NumberDefinition::new("Column 1"))],
    ));

    let mut editable = frozen.thaw();
    editable.add_row(0);

    let result = editable.set_cell(0, "unknown", "5");
    assert_eq!(
        result
            .expect_err("unknown column should fail")
            .translate_data()
            .message_key()
            .as_str(),
        "datastore_key_not_found"
    );
}
