use datastore::prelude::*;

const COLUMNS: &[(ConstStoreKey, NumberCompileTime)] = &[
    (store_key!("width"), const_number!("Width", default = "10")),
    (
        store_key!("height"),
        const_number!("Height", default = "20"),
    ),
];
const TABLE: TableCompileTime = const_table!("Dimensions", COLUMNS);

#[test]
fn table_compile_time_preserves_columns_and_order() {
    assert_eq!(TABLE.description(), "Dimensions");
    assert_eq!(TABLE.columns(), COLUMNS);
    assert_eq!(TABLE.count(), 2);
    assert!(TABLE.contains_key("height"));
    assert!(!TABLE.contains_key("missing"));
    assert_eq!(
        TABLE.get("height").map(NumberCompileTime::description),
        Some("Height")
    );
    assert_eq!(TABLE.get("missing"), None);
    assert_eq!(
        TABLE.get_by_index(0).map(NumberCompileTime::description),
        Some("Width")
    );
    assert_eq!(TABLE.get_by_index(2), None);
    assert_eq!(TABLE.get_column_index_by_name("height"), Some(1));
    assert_eq!(TABLE.get_column_index_by_name("missing"), None);
    assert_eq!(
        TABLE.keys().map(|key| key.to_string()).collect::<Vec<_>>(),
        ["width", "height"]
    );
    assert_eq!(TABLE.iter().count(), 2);
    assert_eq!(
        TABLE
            .into_definition()
            .keys()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["width", "height"]
    );
}
