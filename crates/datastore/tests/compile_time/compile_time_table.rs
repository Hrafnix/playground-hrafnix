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
    assert_eq!(TABLE.count(), 2);
    assert!(TABLE.contains_key("height"));
    assert_eq!(TABLE.get_column_index_by_name("height"), Some(1));
    assert_eq!(
        TABLE.keys().map(|key| key.to_string()).collect::<Vec<_>>(),
        ["width", "height"]
    );
    assert_eq!(
        TABLE
            .into_definition()
            .keys()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["width", "height"]
    );
}
