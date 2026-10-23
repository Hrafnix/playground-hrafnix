use hrafnix_datastore::prelude::*;

#[test]
fn column_requirements_are_independent_and_preserved() {
    let original = TableDefinition::new_with_default(
        "table",
        vec![
            (
                store_key!("z"),
                NumberDefinition::new_with_constraint("", NumberConstraint::max(10.0, true)),
            ),
            (store_key!("a"), NumberDefinition::new("")),
        ],
        vec![vec!["1", "2"]],
    );
    assert_eq!(
        original.column_requirement("z"),
        Some(TableColumnRequirement::None)
    );
    for requirement in [
        TableColumnRequirement::None,
        TableColumnRequirement::Increasing,
        TableColumnRequirement::ConstantOrDecreasing,
        TableColumnRequirement::Constant,
    ] {
        let definition = TableDefinition::new_with_column_requirements_and_default(
            "table",
            vec![
                (
                    store_key!("z"),
                    original.get("z").unwrap().clone(),
                    requirement,
                ),
                (
                    store_key!("a"),
                    original.get("a").unwrap().clone(),
                    TableColumnRequirement::None,
                ),
            ],
            vec![vec!["1", "2"]],
        );
        assert_eq!(definition.column_requirement("missing"), None);
        assert_eq!(definition.column_requirement_by_index(2), None);
        assert_eq!(definition.column_requirement("z"), Some(requirement));
        assert_eq!(definition.column_requirement_by_index(0), Some(requirement));
        assert_eq!(
            definition.column_requirement_by_index(1),
            Some(TableColumnRequirement::None)
        );
        assert_eq!(definition.launder(&SharedStringStore::new()), definition);
        assert_eq!(definition.get("z").unwrap(), original.get("z").unwrap());
        assert_eq!(definition.default_table(), original.default_table());
        assert!(definition.is_merge_compatible(&original));
        assert_eq!(
            definition == original,
            requirement == TableColumnRequirement::None
        );
    }
}

#[test]
fn column_requirements_use_last_duplicate_and_normalize_defaults() {
    for requirement in [
        TableColumnRequirement::None,
        TableColumnRequirement::Increasing,
    ] {
        let columns = vec![
            (
                store_key!("z"),
                NumberDefinition::new("initial"),
                TableColumnRequirement::Constant,
            ),
            (
                store_key!("a"),
                NumberDefinition::new("other"),
                TableColumnRequirement::ConstantOrDecreasing,
            ),
            (
                store_key!("z"),
                NumberDefinition::new("replacement"),
                requirement,
            ),
        ];
        let definition = TableDefinition::new_with_column_requirements("table", columns.clone());
        assert_eq!(definition.default_table(), None);
        assert_eq!(
            definition.keys().map(StoreKey::as_str).collect::<Vec<_>>(),
            vec!["a", "z"]
        );
        assert_eq!(
            definition.get_by_index(1).unwrap().description(),
            "replacement"
        );
        assert_eq!(definition.column_requirement_by_index(1), Some(requirement));
        assert_eq!(
            definition.column_requirement_by_index(0),
            Some(TableColumnRequirement::ConstantOrDecreasing)
        );
        let with_defaults = TableDefinition::new_with_column_requirements_and_default(
            "table",
            columns,
            vec![vec!["1"], vec!["2", "3", "4"]],
        );
        let expected: Vec<Vec<ShareableString>> =
            vec![vec!["1".into(), "".into()], vec!["2".into(), "3".into()]];
        assert_eq!(with_defaults.default_table().unwrap(), expected.as_slice());
        assert_eq!(with_defaults.column_requirement("z"), Some(requirement));
    }
}

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
