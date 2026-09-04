use datastore::prelude::*;

const TABLE_COLUMNS: &[(ConstStoreKey, NumberCompileTime)] = &[
    (
        store_key!("width"),
        number_compile_time!("Width", default = "10"),
    ),
    (
        store_key!("height"),
        number_compile_time!("Height", default = "20"),
    ),
];
const TABLE: TableCompileTime = table_compile_time!("Dimensions", TABLE_COLUMNS);
const MAP_ITEMS: &[(ConstStoreKey, MapItemCompileTime)] = &[
    (
        store_key!("name"),
        map_item_compile_time!(string = string_compile_time!("Name")),
    ),
    (
        store_key!("dimensions"),
        map_item_compile_time!(table = TABLE),
    ),
];
const MAP: MapCompileTime = map_compile_time!("Shapes", MAP_ITEMS);
const OBJECT: GlobalObjectCompileTime = global_object_compile_time!(
    "Settings",
    [
        (
            "g_heading",
            item_compile_time!(tab = tab_compile_time!("General")),
        ),
        ("g_shapes", item_compile_time!(map = MAP)),
        (
            "g_divider",
            item_compile_time!(separator = separator_compile_time!("Advanced")),
        ),
    ],
);

#[test]
fn nested_compile_time_definitions_convert_in_declaration_order() {
    let definition = OBJECT.into_definition();
    assert_eq!(
        definition
            .keys()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["g_heading", "g_shapes", "g_divider"]
    );

    let ItemDefinitionType::Map(map) = definition.get("g_shapes").unwrap() else {
        panic!("expected nested map definition");
    };
    assert_eq!(
        map.keys().map(ToString::to_string).collect::<Vec<_>>(),
        ["name", "dimensions"]
    );

    let MapItemDefinition::Table(table) = map.get("dimensions").unwrap() else {
        panic!("expected nested table definition");
    };
    assert_eq!(
        table.keys().map(ToString::to_string).collect::<Vec<_>>(),
        ["width", "height"]
    );
}
