use hrafnix_datastore::prelude::*;

#[test]
fn tree_print_includes_column_requirements_and_numeric_bounds() {
    let unit = hrafnix_units::UnitId::Length_Meter;
    for (requirement, suffix) in [
        (TableColumnRequirement::None, ""),
        (
            TableColumnRequirement::ConstantOrIncreasing,
            " [Constant Or Increasing]",
        ),
        (TableColumnRequirement::Increasing, " [Increasing]"),
        (
            TableColumnRequirement::ConstantOrDecreasing,
            " [Constant Or Decreasing]",
        ),
        (TableColumnRequirement::Decreasing, " [Decreasing]"),
        (TableColumnRequirement::Constant, " [Constant]"),
    ] {
        let table = TableWithUnitsDefinition::new_with_column_requirements(
            "table",
            vec![
                (
                    store_key!("a"),
                    NumberWithUnitsDefinition::new_with_constraint(
                        "bounded",
                        NumberConstraint::max(10.0, true),
                        unit,
                    ),
                    requirement,
                ),
                (
                    store_key!("b"),
                    NumberWithUnitsDefinition::new("plain", unit),
                    TableColumnRequirement::None,
                ),
            ],
        );
        let definition = ParameterObjectDefinition::builder("Test")
            .with(ParameterKey::new("p_table".into()).unwrap(), table)
            .finish();
        assert_eq!(
            format!("{definition}"),
            format!(
                concat!(
                    "Parameter Object Definition (Test)\n",
                    "    └── p_table (table) Table with units\n",
                    "        ├── a{} (bounded) Number - unit: {} - default: \"\" [Max(10.0, inclusive)]\n",
                    "        └── b (plain) Number - unit: {} - default: \"\"\n",
                ),
                suffix,
                unit.description(),
                unit.description(),
            )
        );
    }
}

#[test]
fn column_requirements_are_independent_and_preserved() {
    let original = TableWithUnitsDefinition::new_with_default(
        "table",
        vec![
            (
                store_key!("z"),
                NumberWithUnitsDefinition::new_with_constraint(
                    "",
                    NumberConstraint::max(10.0, true),
                    hrafnix_units::UnitId::Length_Meter,
                ),
            ),
            (
                store_key!("a"),
                NumberWithUnitsDefinition::new("", hrafnix_units::UnitId::None),
            ),
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
        let definition = TableWithUnitsDefinition::new_with_column_requirements_and_default(
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
        assert_eq!(
            definition.is_merge_compatible(&original),
            requirement == TableColumnRequirement::None
        );
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
                NumberWithUnitsDefinition::new("initial", hrafnix_units::UnitId::Length_Meter),
                TableColumnRequirement::Constant,
            ),
            (
                store_key!("a"),
                NumberWithUnitsDefinition::new("other", hrafnix_units::UnitId::Time_Second),
                TableColumnRequirement::ConstantOrDecreasing,
            ),
            (
                store_key!("z"),
                NumberWithUnitsDefinition::new("replacement", hrafnix_units::UnitId::Length_Foot),
                requirement,
            ),
        ];
        let definition =
            TableWithUnitsDefinition::new_with_column_requirements("table", columns.clone());
        assert_eq!(definition.default_table(), None);
        assert_eq!(
            definition.keys().map(StoreKey::as_str).collect::<Vec<_>>(),
            vec!["a", "z"]
        );
        let column = definition.get_by_index(1).unwrap();
        assert_eq!(column.description(), "replacement");
        assert_eq!(column.preferred_units(), hrafnix_units::UnitId::Length_Foot);
        assert_eq!(definition.column_requirement_by_index(1), Some(requirement));
        assert_eq!(
            definition.column_requirement_by_index(0),
            Some(TableColumnRequirement::ConstantOrDecreasing)
        );
        let with_defaults = TableWithUnitsDefinition::new_with_column_requirements_and_default(
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
fn test_table_with_units_definition() {
    let table_def = TableWithUnitsDefinition::new(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberWithUnitsDefinition::new_with_default(
                    "Length",
                    "1.0",
                    hrafnix_units::UnitId::Length_Meter,
                ),
            ),
            (
                store_key!("duration"),
                NumberWithUnitsDefinition::new("Duration", hrafnix_units::UnitId::Time_Second),
            ),
        ],
    );

    assert_eq!(table_def.description(), "Measurements");
    assert_eq!(table_def.count(), 2);
    assert!(table_def.contains_key(store_key!("length")));
    assert!(table_def.contains_key_str("duration"));
    assert_eq!(
        table_def
            .get_by_index(0)
            .map(NumberWithUnitsDefinition::description),
        Some("Length".into())
    );
    assert_eq!(
        table_def
            .get_str("duration")
            .map(NumberWithUnitsDefinition::preferred_units),
        Some(hrafnix_units::UnitId::Time_Second)
    );
    assert_eq!(
        table_def.get_column_index_by_name(store_key!("duration")),
        Some(1)
    );
}

#[test]
fn test_table_with_units_definition_deduplicates_column_keys() {
    let table_def = TableWithUnitsDefinition::new(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberWithUnitsDefinition::new(
                    "Initial length",
                    hrafnix_units::UnitId::Length_Meter,
                ),
            ),
            (
                store_key!("duration"),
                NumberWithUnitsDefinition::new("Duration", hrafnix_units::UnitId::Time_Second),
            ),
            (
                store_key!("length"),
                NumberWithUnitsDefinition::new(
                    "Replacement length",
                    hrafnix_units::UnitId::Length_Foot,
                ),
            ),
        ],
    );

    assert_eq!(table_def.count(), 2);
    assert_eq!(
        table_def.keys().map(StoreKey::as_str).collect::<Vec<_>>(),
        vec!["duration", "length"]
    );
    assert_eq!(
        table_def
            .get_by_index(1)
            .map(NumberWithUnitsDefinition::description),
        Some("Replacement length".into())
    );
    assert_eq!(
        table_def
            .get_by_index(1)
            .map(NumberWithUnitsDefinition::preferred_units),
        Some(hrafnix_units::UnitId::Length_Foot)
    );
}

#[test]
fn test_table_with_units_definition_with_default() {
    let table_def = TableWithUnitsDefinition::new_with_default(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberWithUnitsDefinition::new("Length", hrafnix_units::UnitId::Length_Meter),
            ),
            (
                store_key!("duration"),
                NumberWithUnitsDefinition::new("Duration", hrafnix_units::UnitId::Time_Second),
            ),
        ],
        vec![vec!["12"], vec!["3", "4", "5"]],
    );

    let default_table = table_def.default_table().expect("default table");
    assert_eq!(default_table[0][0].as_ref(), "12");
    assert_eq!(default_table[0][1].as_ref(), "");
    assert_eq!(default_table[1].len(), 2);
    assert_eq!(default_table[1][1].as_ref(), "4");
}
