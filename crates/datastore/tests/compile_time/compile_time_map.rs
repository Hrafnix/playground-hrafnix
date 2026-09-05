use datastore::prelude::*;
use units::{UnitFamilyId, UnitId};

const CHOICES: &[ChoiceItemCompileTime] = &[choice_item_compile_time!("one", "One")];
const CHOICE: ChoiceCompileTime = choice_compile_time!("Choice", CHOICES);
const TABLE_COLUMNS: &[(ConstStoreKey, NumberCompileTime)] =
    &[(store_key!("width"), number_compile_time!("Width"))];
const TABLE: TableCompileTime = table_compile_time!("Dimensions", TABLE_COLUMNS);
const UNIT_COLUMNS: &[(ConstStoreKey, NumberWithUnitsCompileTime)] = &[(
    store_key!("length"),
    number_with_units_compile_time!("Length", UnitId::Length_Meter),
)];
const TABLE_WITH_UNITS: TableWithUnitsCompileTime =
    table_with_units_compile_time!("Measurements", UNIT_COLUMNS);

#[test]
fn map_item_compile_time_supports_every_variant() {
    let items = [
        map_item_compile_time!(boolean = boolean_compile_time!("Boolean")),
        map_item_compile_time!(choice = CHOICE),
        map_item_compile_time!(file = file_compile_time!("File", "*", true)),
        map_item_compile_time!(integer = integer_compile_time!("Integer")),
        map_item_compile_time!(number = number_compile_time!("Number")),
        map_item_compile_time!(
            number_with_units = number_with_units_compile_time!("Length", UnitId::Length_Meter)
        ),
        map_item_compile_time!(string = string_compile_time!("String")),
        map_item_compile_time!(table = TABLE),
        map_item_compile_time!(table_with_units = TABLE_WITH_UNITS),
        map_item_compile_time!(unit = unit_compile_time!("Unit", UnitFamilyId::Length)),
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
            map_item_compile_time!(string = string_compile_time!("Name")),
        ),
        (
            store_key!("dimensions"),
            map_item_compile_time!(table = TABLE),
        ),
    ];
    const MAP: MapCompileTime = map_compile_time!("Shapes", ITEMS);

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

#[test]
#[should_panic(expected = "MapCompileTime item keys must be unique")]
fn map_compile_time_rejects_duplicate_keys() {
    const DUPLICATES: &[(ConstStoreKey, MapItemCompileTime)] = &[
        (
            store_key!("duplicate"),
            map_item_compile_time!(string = string_compile_time!("First")),
        ),
        (
            store_key!("duplicate"),
            map_item_compile_time!(string = string_compile_time!("Second")),
        ),
    ];
    #[allow(clippy::disallowed_methods)]
    let _ = MapCompileTime::__new(std::hint::black_box("Duplicates"), DUPLICATES);
}
