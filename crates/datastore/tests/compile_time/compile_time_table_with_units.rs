use datastore::prelude::*;
use units::UnitId;

const COLUMNS: &[(ConstStoreKey, NumberWithUnitsCompileTime)] = &[
    (
        store_key!("length"),
        number_with_units_compile_time!("Length", UnitId::Length_Meter, default = "1"),
    ),
    (
        store_key!("area"),
        number_with_units_compile_time!("Area", UnitId::Area_SquareMeter),
    ),
];
const TABLE: TableWithUnitsCompileTime = table_with_units_compile_time!("Measurements", COLUMNS);

#[test]
fn table_with_units_compile_time_preserves_columns_and_order() {
    assert_eq!(
        TABLE
            .get_by_index(1)
            .map(NumberWithUnitsCompileTime::description),
        Some("Area")
    );
    assert_eq!(
        TABLE
            .into_definition()
            .keys()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["length", "area"]
    );
}

#[test]
#[should_panic(expected = "TableWithUnitsCompileTime column keys must be unique")]
fn table_with_units_compile_time_rejects_duplicate_keys() {
    const DUPLICATES: &[(ConstStoreKey, NumberWithUnitsCompileTime)] = &[
        (
            store_key!("duplicate"),
            number_with_units_compile_time!("First", UnitId::Length_Meter),
        ),
        (
            store_key!("duplicate"),
            number_with_units_compile_time!("Second", UnitId::Length_Meter),
        ),
    ];
    let _ = TableWithUnitsCompileTime::__new(std::hint::black_box("Duplicates"), DUPLICATES);
}
