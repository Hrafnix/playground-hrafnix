use datastore::prelude::*;

const TABLE_COLUMNS: &[(ConstStoreKey, NumberCompileTime)] = &[
    (store_key!("width"), const_number!("Width", default = "10")),
    (
        store_key!("height"),
        const_number!("Height", default = "20"),
    ),
];
const TABLE: TableCompileTime = const_table!("Dimensions", TABLE_COLUMNS);
const MAP_ITEMS: &[(ConstStoreKey, MapItemCompileTime)] = &[
    (
        store_key!("name"),
        const_map_item!(string = const_string!("Name")),
    ),
    (store_key!("dimensions"), const_map_item!(table = TABLE)),
];
const MAP: MapCompileTime = const_map!("Shapes", MAP_ITEMS);
const OBJECT: GlobalObjectCompileTime = const_global_object!(
    "Settings",
    [
        ("g_heading", const_item!(tab = const_tab!("General")),),
        ("g_shapes", const_item!(map = MAP)),
        (
            "g_divider",
            const_item!(separator = const_separator!("Advanced")),
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
