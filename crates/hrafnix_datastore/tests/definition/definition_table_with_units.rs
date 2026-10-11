use hrafnix_datastore::prelude::*;

#[test]
fn column_requirements_are_independent_and_preserved() {
    let mut definition = TableWithUnitsDefinition::new_with_default(
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
    let original = definition.clone();
    assert_eq!(
        definition.column_requirement("z"),
        Some(TableColumnRequirement::None)
    );
    assert_eq!(definition.column_requirement("missing"), None);
    assert_eq!(definition.column_requirement_by_index(2), None);
    assert!(
        !definition.set_column_requirement(store_key!("missing"), TableColumnRequirement::Constant)
    );
    for requirement in [
        TableColumnRequirement::Increasing { strict: true },
        TableColumnRequirement::Decreasing { strict: false },
        TableColumnRequirement::Constant,
    ] {
        assert!(definition.set_column_requirement(store_key!("z"), requirement));
        assert_eq!(definition.column_requirement("z"), Some(requirement));
        assert_eq!(definition.column_requirement_by_index(0), Some(requirement));
        assert_eq!(
            definition.column_requirement_by_index(1),
            Some(TableColumnRequirement::None)
        );
        assert_eq!(definition.launder(&SharedStringStore::new()), definition);
        assert_eq!(definition.get("z").unwrap(), original.get("z").unwrap());
        assert!(definition.is_merge_compatible(&original));
    }
    assert!(definition.set_column_requirement(store_key!("z"), TableColumnRequirement::None));
    assert_eq!(definition, original);
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
