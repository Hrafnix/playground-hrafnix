use datastore::prelude::*;
use units::{UnitFamilyId, UnitId};

#[test]
fn item_compile_time_supports_every_variant() {
    const CHOICES: &[ChoiceItemCompileTime] = &[choice_item_compile_time!("one", "One")];
    const CHOICE: ChoiceCompileTime = choice_compile_time!("Choice", CHOICES);
    const TABLE_COLUMNS: &[(ConstStoreKey, NumberCompileTime)] =
        &[(store_key!("number"), number_compile_time!("Number"))];
    const TABLE: TableCompileTime = table_compile_time!("Table", TABLE_COLUMNS);
    const UNIT_COLUMNS: &[(ConstStoreKey, NumberWithUnitsCompileTime)] = &[(
        store_key!("length"),
        number_with_units_compile_time!("Length", UnitId::Length_Meter),
    )];
    const TABLE_WITH_UNITS: TableWithUnitsCompileTime =
        table_with_units_compile_time!("Measurements", UNIT_COLUMNS);
    const MAP_ITEMS: &[(ConstStoreKey, MapItemCompileTime)] = &[(
        store_key!("name"),
        map_item_compile_time!(string = string_compile_time!("Name")),
    )];
    const MAP: MapCompileTime = map_compile_time!("Map", MAP_ITEMS);

    let items = [
        item_compile_time!(boolean = boolean_compile_time!("Boolean")),
        item_compile_time!(choice = CHOICE),
        item_compile_time!(file = file_compile_time!("File", "*", true)),
        item_compile_time!(folder = folder_compile_time!("Folder", true)),
        item_compile_time!(integer = integer_compile_time!("Integer")),
        item_compile_time!(map = MAP),
        item_compile_time!(number = number_compile_time!("Number")),
        item_compile_time!(
            number_with_units = number_with_units_compile_time!("Length", UnitId::Length_Meter)
        ),
        item_compile_time!(string = string_compile_time!("String")),
        item_compile_time!(table = TABLE),
        item_compile_time!(table_with_units = TABLE_WITH_UNITS),
        item_compile_time!(unit = unit_compile_time!("Unit", UnitFamilyId::Length)),
        item_compile_time!(tab = tab_compile_time!("Tab")),
        item_compile_time!(separator = separator_compile_time!("Separator")),
    ];

    assert!(matches!(
        items[0].into_definition(),
        ItemDefinitionType::Boolean(_)
    ));
    assert!(matches!(
        items[1].into_definition(),
        ItemDefinitionType::Choice(_)
    ));
    assert!(matches!(
        items[2].into_definition(),
        ItemDefinitionType::File(_)
    ));
    assert!(matches!(
        items[3].into_definition(),
        ItemDefinitionType::Folder(_)
    ));
    assert!(matches!(
        items[4].into_definition(),
        ItemDefinitionType::Integer(_)
    ));
    assert!(matches!(
        items[5].into_definition(),
        ItemDefinitionType::Map(_)
    ));
    assert!(matches!(
        items[6].into_definition(),
        ItemDefinitionType::Number(_)
    ));
    assert!(matches!(
        items[7].into_definition(),
        ItemDefinitionType::NumberWithUnits(_)
    ));
    assert!(matches!(
        items[8].into_definition(),
        ItemDefinitionType::String(_)
    ));
    assert!(matches!(
        items[9].into_definition(),
        ItemDefinitionType::Table(_)
    ));
    assert!(matches!(
        items[10].into_definition(),
        ItemDefinitionType::TableWithUnits(_)
    ));
    assert!(matches!(
        items[11].into_definition(),
        ItemDefinitionType::Unit(_)
    ));
    assert!(matches!(
        items[12].into_definition(),
        ItemDefinitionType::Tab(_)
    ));
    assert!(matches!(
        items[13].into_definition(),
        ItemDefinitionType::Separator(_)
    ));
}

#[test]
fn global_object_compile_time_converts_both_macro_forms() {
    const ITEMS: &[(ConstGlobalKey, ItemCompileTime)] = &[(
        global_key!("g_name"),
        item_compile_time!(string = string_compile_time!("Name")),
    )];
    const FROM_SLICE: GlobalObjectCompileTime = global_object_compile_time!("Global", ITEMS);
    const FROM_LITERALS: GlobalObjectCompileTime = global_object_compile_time!(
        "Global",
        [(
            "g_enabled",
            item_compile_time!(boolean = boolean_compile_time!("Enabled")),
        )],
    );

    assert_eq!(
        FROM_SLICE
            .keys()
            .map(|key| key.to_string())
            .collect::<Vec<_>>(),
        ["g_name"]
    );
    assert_eq!(
        FROM_LITERALS
            .into_definition()
            .keys()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["g_enabled"]
    );
}

#[test]
#[should_panic(expected = "GlobalObjectCompileTime item keys must be unique")]
fn global_object_compile_time_rejects_duplicate_keys() {
    const DUPLICATES: &[(ConstGlobalKey, ItemCompileTime)] = &[
        (
            global_key!("g_duplicate"),
            item_compile_time!(string = string_compile_time!("First")),
        ),
        (
            global_key!("g_duplicate"),
            item_compile_time!(string = string_compile_time!("Second")),
        ),
    ];
    let _ = GlobalObjectCompileTime::__new(std::hint::black_box("Duplicates"), DUPLICATES);
}
