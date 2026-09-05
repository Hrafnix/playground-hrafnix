use datastore::prelude::*;
use units::{UnitFamilyId, UnitId};

const CHOICES: &[ChoiceItemCompileTime] = &[const_choice_item!("one", "One")];
const CHOICE: ChoiceCompileTime = const_choice!("Choice", CHOICES);
const TABLE_COLUMNS: &[(ConstStoreKey, NumberCompileTime)] =
    &[(store_key!("width"), const_number!("Width"))];
const TABLE: TableCompileTime = const_table!("Dimensions", TABLE_COLUMNS);
const UNIT_COLUMNS: &[(ConstStoreKey, NumberWithUnitsCompileTime)] = &[(
    store_key!("length"),
    const_number_with_units!("Length", UnitId::Length_Meter),
)];
const TABLE_WITH_UNITS: TableWithUnitsCompileTime =
    const_table_with_units!("Measurements", UNIT_COLUMNS);

#[test]
fn map_item_compile_time_supports_every_variant() {
    let items = [
        const_map_item!(boolean = const_boolean!("Boolean")),
        const_map_item!(choice = CHOICE),
        const_map_item!(file = const_file!("File", "*", true)),
        const_map_item!(integer = const_integer!("Integer")),
        const_map_item!(number = const_number!("Number")),
        const_map_item!(
            number_with_units = const_number_with_units!("Length", UnitId::Length_Meter)
        ),
        const_map_item!(string = const_string!("String")),
        const_map_item!(table = TABLE),
        const_map_item!(table_with_units = TABLE_WITH_UNITS),
        const_map_item!(unit = const_unit!("Unit", UnitFamilyId::Length)),
    ];

    assert!(matches!(
        items[0].into_definition(),
        MapItemDefinition::Boolean(_)
    ));
    assert!(matches!(
        items[1].into_definition(),
        MapItemDefinition::Choice(_)
    ));
    assert!(matches!(
        items[2].into_definition(),
        MapItemDefinition::File(_)
    ));
    assert!(matches!(
        items[3].into_definition(),
        MapItemDefinition::Integer(_)
    ));
    assert!(matches!(
        items[4].into_definition(),
        MapItemDefinition::Number(_)
    ));
    assert!(matches!(
        items[5].into_definition(),
        MapItemDefinition::NumberWithUnits(_)
    ));
    assert!(matches!(
        items[6].into_definition(),
        MapItemDefinition::String(_)
    ));
    assert!(matches!(
        items[7].into_definition(),
        MapItemDefinition::Table(_)
    ));
    assert!(matches!(
        items[8].into_definition(),
        MapItemDefinition::TableWithUnits(_)
    ));
    assert!(matches!(
        items[9].into_definition(),
        MapItemDefinition::Unit(_)
    ));
}

#[test]
fn map_compile_time_preserves_items_and_order() {
    const ITEMS: &[(ConstStoreKey, MapItemCompileTime)] = &[
        (
            store_key!("name"),
            const_map_item!(string = const_string!("Name")),
        ),
        (store_key!("dimensions"), const_map_item!(table = TABLE)),
    ];
    const MAP: MapCompileTime = const_map!("Shapes", ITEMS);

    assert_eq!(MAP.count(), 2);
    assert!(MAP.get("dimensions").is_some());
    assert_eq!(
        MAP.into_definition()
            .keys()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["name", "dimensions"]
    );
}
