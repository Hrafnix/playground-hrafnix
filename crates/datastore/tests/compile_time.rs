//! Static compile-time definition conversion tests.

use datastore::compile_time::{
    ChoiceCompileTime, ChoiceItemCompileTime, GlobalObjectCompileTime, ItemCompileTimeType,
    MapCompileTime, MapItemCompileTime, NumberCompileTime, StringCompileTime, TableCompileTime,
};
use datastore::definition::{
    ChoiceDefinition, GlobalObjectDefinition, MapDefinition, TableDefinition,
};
use datastore::prelude::{
    ConstGlobalKey, ConstStoreKey, choice_compile_time, choice_item_compile_time, global_key,
    global_object_compile_time, item_compile_time, map_compile_time, map_item_compile_time,
    number_compile_time, store_key, string_compile_time, table_compile_time,
};

const CHOICES: &[ChoiceItemCompileTime] = &[
    choice_item_compile_time!("first", "First"),
    choice_item_compile_time!("second", "Second"),
];
const DUPLICATE_CHOICES: &[ChoiceItemCompileTime] = &[
    choice_item_compile_time!("duplicate", "First"),
    choice_item_compile_time!("duplicate", "Second"),
];
const CHOICE: ChoiceCompileTime = choice_compile_time!("Choice", CHOICES, default = "second");

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
    (store_key!("size"), map_item_compile_time!(table = TABLE)),
];
const MAP: MapCompileTime = map_compile_time!("Shapes", MAP_ITEMS);

const OBJECT: GlobalObjectCompileTime = global_object_compile_time!(
    "Shape settings",
    [
        (
            "g_shape_name",
            item_compile_time!(string = string_compile_time!("Shape name")),
        ),
        ("g_shape_choice", item_compile_time!(choice = CHOICE)),
        ("g_shape_map", item_compile_time!(map = MAP)),
    ],
);
const DUPLICATE_GLOBAL_ITEMS: &[(ConstGlobalKey, ItemCompileTimeType)] = &[
    (
        global_key!("g_duplicate"),
        item_compile_time!(string = string_compile_time!("First")),
    ),
    (
        global_key!("g_duplicate"),
        item_compile_time!(string = string_compile_time!("Second")),
    ),
];

#[test]
fn static_compile_time_values_convert_in_declaration_order() {
    let local: StringCompileTime = string_compile_time!("Local binding");
    assert_eq!(local.description(), "Local binding");

    let choice: ChoiceDefinition = CHOICE.into_definition();
    assert_eq!(choice.default_value(), "second");
    assert_eq!(
        choice
            .ids()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["first", "second"]
    );

    let table: TableDefinition = TABLE.into_definition();
    assert_eq!(
        table.keys().map(ToString::to_string).collect::<Vec<_>>(),
        ["width", "height"]
    );

    let map: MapDefinition = MAP.into_definition();
    assert_eq!(
        map.keys().map(ToString::to_string).collect::<Vec<_>>(),
        ["name", "size"]
    );

    let object: GlobalObjectDefinition = OBJECT.into_definition();
    assert_eq!(
        object.keys().map(ToString::to_string).collect::<Vec<_>>(),
        ["g_shape_name", "g_shape_choice", "g_shape_map"]
    );
}

#[test]
#[should_panic(expected = "GlobalObjectCompileTime item keys must be unique")]
fn global_object_compile_time_rejects_duplicate_item_keys() {
    let _ =
        GlobalObjectCompileTime::__new(std::hint::black_box("Duplicates"), DUPLICATE_GLOBAL_ITEMS);
}

#[test]
#[should_panic(expected = "ChoiceCompileTime choice ids must be unique")]
fn choice_compile_time_rejects_duplicate_ids() {
    let _ = ChoiceCompileTime::__new(std::hint::black_box("Duplicates"), DUPLICATE_CHOICES);
}
