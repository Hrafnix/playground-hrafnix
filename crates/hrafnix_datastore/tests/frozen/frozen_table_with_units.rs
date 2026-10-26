use hrafnix_datastore::prelude::*;
use hrafnix_units::UnitId;

#[test]
fn table_with_units_frozen_from_rows_normalizes_row_widths() {
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
        let definition = TableWithUnitsDefinition::new(
            "Table",
            [
                (
                    store_key!("z"),
                    NumberWithUnitsDefinition::new_with_default("", "99", UnitId::Length_Meter),
                ),
                (
                    store_key!("a"),
                    NumberWithUnitsDefinition::new_with_default("", "99", UnitId::Time_Second),
                ),
            ]
            .into_iter()
            .take(column_count)
            .collect(),
        );
        let units: Vec<ShareableString> = [UnitId::Length_Centimeter, UnitId::Time_Second]
            .into_iter()
            .take(column_count)
            .map(|unit| unit.string_id().into())
            .collect();
        let table =
            TableWithUnitsFrozen::new_from_rows(definition.clone(), rows.clone(), units.clone());
        let expected = TableWithUnitsFrozen::new_from_rows(
            definition.clone(),
            expected_rows.clone(),
            units.clone(),
        );

        assert_eq!(table.rows(), expected_rows);
        assert_eq!(table.row_count(), 4);
        assert_eq!(table.column_count(), column_count);
        assert_eq!(table.units(), units);
        assert_eq!(table, expected);
        assert_eq!(table.hash(), expected.hash());
        assert_eq!(table.thaw().freeze(), table);
        assert_eq!(table.launder(&SharedStringStore::new()), table);
        assert_eq!(
            TableWithUnitsFrozen::new_from_rows(definition, vec![], units).row_count(),
            0
        );
    }
}

#[test]
fn test_table_with_units_frozen() {
    let table = TableWithUnitsFrozen::new(TableWithUnitsDefinition::new(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberWithUnitsDefinition::new_with_default("Length", "1.0", UnitId::Length_Meter),
            ),
            (
                store_key!("duration"),
                NumberWithUnitsDefinition::new("Duration", UnitId::Time_Second),
            ),
        ],
    ));

    assert_eq!(table.definition().description().as_ref(), "Measurements");
    assert_eq!(table.row_count(), 0);
    assert_eq!(table.column_count(), 2);
    assert_ne!(table.hash(), [0u8; 32]);
    assert_eq!(table.parameter().as_ref(), "");
    assert_eq!(
        table.unit_by_name("length").expect("length unit").as_ref(),
        UnitId::Length_Meter.string_id().as_str()
    );
    assert_eq!(
        table.unit_by_index(1).expect("duration unit").as_ref(),
        UnitId::Time_Second.string_id().as_str()
    );
}

#[test]
fn test_table_with_units_frozen_from_rows_hashes_values_and_units() {
    let definition = TableWithUnitsDefinition::new(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberWithUnitsDefinition::new_with_default("Length", "1.0", UnitId::Length_Meter),
            ),
            (
                store_key!("duration"),
                NumberWithUnitsDefinition::new("Duration", UnitId::Time_Second),
            ),
        ],
    );
    let table = TableWithUnitsFrozen::new_from_rows(
        definition.clone(),
        vec![vec!["3.0".into(), "4.0".into()]],
        vec![
            UnitId::Length_Meter.string_id().into(),
            UnitId::Time_Second.string_id().into(),
        ],
    );
    let different_units = TableWithUnitsFrozen::new_from_rows(
        definition,
        vec![vec!["3.0".into(), "4.0".into()]],
        vec![
            UnitId::Length_Centimeter.string_id().into(),
            UnitId::Time_Second.string_id().into(),
        ],
    );

    assert_eq!(table.row_count(), 1);
    assert_eq!(table.rows().len(), 1);
    assert_eq!(
        table
            .cell_by_name(0, "length")
            .expect("length cell")
            .as_ref(),
        "3.0"
    );
    assert_eq!(
        table
            .unit_by_name("duration")
            .expect("duration unit")
            .as_ref(),
        UnitId::Time_Second.string_id().as_str()
    );
    assert_ne!(table.hash(), different_units.hash());
}

#[test]
fn test_table_with_units_frozen_equality() {
    let table_one = TableWithUnitsFrozen::new(TableWithUnitsDefinition::new(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberWithUnitsDefinition::new_with_default("Length", "1.0", UnitId::Length_Meter),
            ),
            (
                store_key!("duration"),
                NumberWithUnitsDefinition::new("Duration", UnitId::Time_Second),
            ),
        ],
    ));
    let table_two = TableWithUnitsFrozen::new(TableWithUnitsDefinition::new(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberWithUnitsDefinition::new_with_default("Length", "1.0", UnitId::Length_Meter),
            ),
            (
                store_key!("duration"),
                NumberWithUnitsDefinition::new("Duration", UnitId::Time_Second),
            ),
        ],
    ));
    let different = TableWithUnitsFrozen::new(TableWithUnitsDefinition::new(
        "Other measurements",
        vec![(
            store_key!("length"),
            NumberWithUnitsDefinition::new("Length", UnitId::Length_Meter),
        )],
    ));

    assert_eq!(table_one, table_two);
    assert_eq!(&table_one, table_two);
    assert_ne!(table_one, different);
    assert_ne!(table_one, &different);
}

#[test]
fn test_table_with_units_frozen_uses_definition_default_table() {
    let table = TableWithUnitsFrozen::new(TableWithUnitsDefinition::new_with_default(
        "Measurements",
        vec![(
            store_key!("length"),
            NumberWithUnitsDefinition::new("Length", UnitId::Length_Meter),
        )],
        vec![vec!["12"]],
    ));

    assert_eq!(table.cell_by_index(0, 0).map(AsRef::as_ref), Some("12"));
}
