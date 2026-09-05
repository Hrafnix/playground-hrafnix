use datastore::prelude::*;
use units::UnitId;

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
