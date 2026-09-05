use datastore::prelude::*;

const COLUMNS: &[(ConstStoreKey, NumberCompileTime)] = &[
    (
        store_key!("width"),
        number_compile_time!("Width", default = "10"),
    ),
    (
        store_key!("height"),
        number_compile_time!("Height", default = "20"),
    ),
];
const TABLE: TableCompileTime = table_compile_time!("Dimensions", COLUMNS);

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

#[test]
#[should_panic(expected = "TableCompileTime column keys must be unique")]
fn table_compile_time_rejects_duplicate_keys() {
    const DUPLICATES: &[(ConstStoreKey, NumberCompileTime)] = &[
        (store_key!("duplicate"), number_compile_time!("First")),
        (store_key!("duplicate"), number_compile_time!("Second")),
    ];
    #[allow(clippy::disallowed_methods)]
    let _ = TableCompileTime::__new(std::hint::black_box("Duplicates"), DUPLICATES);
}
