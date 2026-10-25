use hrafnix_datastore::prelude::*;
use hrafnix_units::UnitId;

const COLUMNS: &[(ConstStoreKey, NumberWithUnitsCompileTime)] = &[
    (
        store_key!("length"),
        const_number_with_units!("Length", UnitId::Length_Meter, default = "1"),
    ),
    (
        store_key!("area"),
        const_number_with_units!("Area", UnitId::Area_SquareMeter),
    ),
];
const TABLE: TableWithUnitsCompileTime = const_table_with_units!("Measurements", COLUMNS);

#[test]
fn table_with_units_compile_time_preserves_columns_and_order() {
    assert_eq!(TABLE.description(), "Measurements");
    assert_eq!(TABLE.columns(), COLUMNS);
    assert_eq!(TABLE.count(), 2);
    assert!(TABLE.contains_key("length"));
    assert!(!TABLE.contains_key("missing"));
    assert_eq!(
        TABLE
            .get("area")
            .map(NumberWithUnitsCompileTime::description),
        Some("Area")
    );
    assert_eq!(TABLE.get("missing"), None);
    assert_eq!(
        TABLE
            .get_by_index(1)
            .map(NumberWithUnitsCompileTime::description),
        Some("Area")
    );
    assert_eq!(TABLE.get_by_index(2), None);
    assert_eq!(TABLE.get_column_index_by_name("area"), Some(1));
    assert_eq!(TABLE.get_column_index_by_name("missing"), None);
    assert_eq!(
        TABLE.keys().map(|key| key.to_string()).collect::<Vec<_>>(),
        ["length", "area"]
    );
    assert_eq!(TABLE.iter().count(), 2);
    assert_eq!(
        TABLE
            .into_definition()
            .keys()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["length", "area"]
    );
}

const REQUIREMENTS: &[(ConstStoreKey, TableColumnRequirement)] = &[(
    store_key!("area"),
    TableColumnRequirement::ConstantOrDecreasing,
)];
const TABLE_WITH_REQUIREMENTS: TableWithUnitsCompileTime =
    const_table_with_units!("Measurements", COLUMNS, REQUIREMENTS);

#[test]
fn table_with_units_compile_time_without_requirements_has_none() {
    assert_eq!(TABLE.column_requirements(), &[]);
    assert_eq!(
        TABLE.column_requirement("length"),
        Some(TableColumnRequirement::None)
    );
    assert_eq!(TABLE.column_requirement("missing"), None);
}

#[test]
fn table_with_units_compile_time_preserves_column_requirements() {
    assert_eq!(TABLE_WITH_REQUIREMENTS.column_requirements(), REQUIREMENTS);
    assert_eq!(
        TABLE_WITH_REQUIREMENTS.column_requirement("area"),
        Some(TableColumnRequirement::ConstantOrDecreasing)
    );
    assert_eq!(
        TABLE_WITH_REQUIREMENTS.column_requirement("length"),
        Some(TableColumnRequirement::None)
    );
    assert_eq!(TABLE_WITH_REQUIREMENTS.column_requirement("missing"), None);

    let definition = TABLE_WITH_REQUIREMENTS.into_definition();
    assert_eq!(
        definition
            .column_requirements()
            .map(|(key, requirement)| (key.to_string(), requirement))
            .collect::<Vec<_>>(),
        [
            ("length".to_string(), TableColumnRequirement::None),
            (
                "area".to_string(),
                TableColumnRequirement::ConstantOrDecreasing
            ),
        ]
    );
}
