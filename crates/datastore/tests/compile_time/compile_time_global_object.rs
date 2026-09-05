use datastore::prelude::*;
use units::{UnitFamilyId, UnitId};

#[test]
fn item_compile_time_supports_every_variant() {
    const CHOICES: &[ChoiceItemCompileTime] = &[const_choice_item!("one", "One")];
    const CHOICE: ChoiceCompileTime = const_choice!("Choice", CHOICES);
    const TABLE_COLUMNS: &[(ConstStoreKey, NumberCompileTime)] =
        &[(store_key!("number"), const_number!("Number"))];
    const TABLE: TableCompileTime = const_table!("Table", TABLE_COLUMNS);
    const UNIT_COLUMNS: &[(ConstStoreKey, NumberWithUnitsCompileTime)] = &[(
        store_key!("length"),
        const_number_with_units!("Length", UnitId::Length_Meter),
    )];
    const TABLE_WITH_UNITS: TableWithUnitsCompileTime =
        const_table_with_units!("Measurements", UNIT_COLUMNS);
    const MAP_ITEMS: &[(ConstStoreKey, MapItemCompileTime)] = &[(
        store_key!("name"),
        const_map_item!(string = const_string!("Name")),
    )];
    const MAP: MapCompileTime = const_map!("Map", MAP_ITEMS);

    let items = [
        const_item!(boolean = const_boolean!("Boolean")),
        const_item!(choice = CHOICE),
        const_item!(file = const_file!("File", "*", true)),
        const_item!(folder = const_folder!("Folder", true)),
        const_item!(integer = const_integer!("Integer")),
        const_item!(map = MAP),
        const_item!(number = const_number!("Number")),
        const_item!(number_with_units = const_number_with_units!("Length", UnitId::Length_Meter)),
        const_item!(string = const_string!("String")),
        const_item!(table = TABLE),
        const_item!(table_with_units = TABLE_WITH_UNITS),
        const_item!(unit = const_unit!("Unit", UnitFamilyId::Length)),
        const_item!(tab = const_tab!("Tab")),
        const_item!(separator = const_separator!("Separator")),
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
        const_item!(string = const_string!("Name")),
    )];
    const FROM_SLICE: GlobalObjectCompileTime = const_global_object!("Global", ITEMS);
    const FROM_LITERALS: GlobalObjectCompileTime = const_global_object!(
        "Global",
        [(
            "g_enabled",
            const_item!(boolean = const_boolean!("Enabled")),
        )],
    );

    assert_eq!(FROM_SLICE.description(), "Global");
    assert_eq!(FROM_SLICE.items(), ITEMS);
    assert_eq!(FROM_SLICE.count(), 1);
    assert!(FROM_SLICE.contains("g_name"));
    assert!(!FROM_SLICE.contains("g_missing"));
    assert!(matches!(
        FROM_SLICE.get("g_name"),
        Some(ItemCompileTime::String(_))
    ));
    assert_eq!(FROM_SLICE.get("g_missing"), None);
    assert_eq!(FROM_SLICE.iter().count(), 1);
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
