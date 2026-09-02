use datastore::prelude::*;
use std::collections::BTreeMap;

#[test]
fn test_map_frozen() {
    // Why: Test map frozen object creation and definition.
    let map_frozen = MapFrozen::new(MapDefinition::new(
        "A map",
        Vec::<(StoreKey, MapItemDefinition)>::new(),
    ));

    // Check the various parameters of the map definition.
    assert_eq!(map_frozen.definition().description().as_ref(), "A map");
    assert_eq!(map_frozen.definition().count(), 0);

    assert!(map_frozen.get(store_key!("field1")).is_none());
    assert_eq!(map_frozen.count(), 0);
    assert_ne!(map_frozen.hash(), [0u8; 32]);
}

#[test]
fn test_complex_map_frozen() {
    // Why: Test complex map frozen object creation and definition.
    let map_item_def_1 = MapItemDefinition::String(StringDefinition::new("Field 1"));
    let map_item_def_2 = MapItemDefinition::Table(TableDefinition::new(
        "Table field",
        Vec::<(StoreKey, NumberDefinition)>::new(),
    ));

    let map_frozen = MapFrozen::new(MapDefinition::new(
        "A map",
        vec![
            (store_key!("field1"), map_item_def_1.clone()),
            (store_key!("field2"), map_item_def_2.clone()),
        ],
    ));

    // Check the various parameters of the map definition.
    assert_eq!(map_frozen.definition().description().as_ref(), "A map");
    assert_eq!(map_frozen.definition().count(), 2);
    assert_eq!(
        map_frozen.definition().get(store_key!("field1")).unwrap(),
        &map_item_def_1
    );
    assert_eq!(
        map_frozen.definition().get(store_key!("field2")).unwrap(),
        &map_item_def_2
    );

    assert!(map_frozen.get("f").is_none());
    assert!(map_frozen.get("field1").is_none());
    assert!(map_frozen.get("field2").is_none());
    assert_eq!(map_frozen.count(), 0);
    assert_ne!(map_frozen.hash(), [0u8; 32]);
}

#[test]
fn test_map_frozen_equality() {
    // Why: Test that two map frozen objects with the same parameters are considered equal.
    let map_frozen_1 = MapFrozen::new(MapDefinition::new(
        "A map",
        vec![
            (store_key!("field1"), StringDefinition::new("Field 1")),
            (store_key!("field2"), StringDefinition::new("Field 2")),
        ],
    ));
    let map_frozen_2 = MapFrozen::new(MapDefinition::new(
        "A map",
        vec![
            (store_key!("field1"), StringDefinition::new("Field 1")),
            (store_key!("field2"), StringDefinition::new("Field 2")),
        ],
    ));
    let map_frozen_3 = MapFrozen::new(MapDefinition::new(
        "A map",
        vec![
            (store_key!("field1"), StringDefinition::new("New Field 1")),
            (store_key!("field2"), StringDefinition::new("New Field 2")),
        ],
    ));

    assert_eq!(map_frozen_1, map_frozen_2);
    assert_ne!(map_frozen_1, map_frozen_3);
    assert_eq!(&map_frozen_1, map_frozen_2);
    assert_ne!(map_frozen_1, &map_frozen_3);
}

#[test]
fn test_map_entry_all_basic_frozen() {
    // Why: Test map entry frozen creation with all basic definitions.
    let item_type: BTreeMap<StoreKey, MapItemDefinition> = vec![
        (
            store_key!("field1").into(),
            MapItemDefinition::String(StringDefinition::new("Field 1")),
        ),
        (
            store_key!("field2").into(),
            MapItemDefinition::String(StringDefinition::new_with_default(
                "Field 2",
                "Default value",
            )),
        ),
    ]
    .into_iter()
    .collect();
    let entry = MapEntryFrozen::new(&item_type);

    let t1 = entry.get("field1").unwrap().get_string().unwrap();
    let t2 = entry.get_string("field2").unwrap();
    assert_ne!(t1, t2);
    assert_eq!(t1.value(), "");
    assert_eq!(t2.value(), "Default value");

    assert_eq!(entry.get("field3"), None);
    assert_eq!(entry.get_table("field3"), None);
    assert_eq!(entry.get_table("field1"), None);
    assert_eq!(entry.get_table("field2"), None);
    assert_ne!(entry.hash(), [0u8; 32]);
}

#[test]
fn test_map_entry_all_table_frozen() {
    // Why: Test map entry frozen creation with all table definitions.
    let item_type: BTreeMap<StoreKey, MapItemDefinition> = vec![
        (
            store_key!("field1").into(),
            MapItemDefinition::Table(TableDefinition::new(
                "Table field 1",
                Vec::<(StoreKey, NumberDefinition)>::new(),
            )),
        ),
        (
            store_key!("field2").into(),
            MapItemDefinition::Table(TableDefinition::new(
                "Table field 2",
                Vec::<(StoreKey, NumberDefinition)>::new(),
            )),
        ),
    ]
    .into_iter()
    .collect();
    let entry = MapEntryFrozen::new(&item_type);

    let t1 = entry.get("field1").unwrap().get_table().unwrap();
    let t2 = entry.get_table("field2").unwrap();
    assert_ne!(t1, t2);
    assert_eq!(t1.row_count(), 0);
    assert_eq!(t1.column_count(), 0);
    assert_eq!(t2.row_count(), 0);
    assert_eq!(t2.column_count(), 0);

    assert_eq!(entry.get("field3"), None);
    assert_eq!(entry.get_string("field3"), None);
    assert_eq!(entry.get_string("field1"), None);
    assert_eq!(entry.get_string("field2"), None);
    assert_ne!(entry.hash(), [0u8; 32]);
}

#[test]
fn test_map_entry_mixed_frozen() {
    // Why: Test map entry frozen creation with mixed field types.
    let item_type: BTreeMap<StoreKey, MapItemDefinition> = vec![
        (
            store_key!("field1").into(),
            MapItemDefinition::String(StringDefinition::new("Field 1")),
        ),
        (
            store_key!("field2").into(),
            MapItemDefinition::Table(TableDefinition::new(
                "Table field",
                Vec::<(StoreKey, NumberDefinition)>::new(),
            )),
        ),
    ]
    .into_iter()
    .collect();
    let entry = MapEntryFrozen::new(&item_type);

    let t1 = entry.get("field1").unwrap().get_string().unwrap();
    let t2 = entry.get_table("field2").unwrap();
    assert_eq!(t1.value(), "");
    assert_eq!(t2.row_count(), 0);
    assert_eq!(t2.column_count(), 0);

    assert_eq!(entry.get("field3"), None);
    assert_eq!(entry.get_string("field3"), None);
    assert_eq!(entry.get_table("field1"), None);
    assert_eq!(entry.get_string("field2"), None);
    assert_ne!(entry.hash(), [0u8; 32]);
}

#[test]
fn test_map_entry_frozen_equality() {
    // Why: Test that two map entry frozen objects with the same items are considered equal.
    let item_type: BTreeMap<StoreKey, MapItemDefinition> = vec![
        (
            store_key!("field1").into(),
            MapItemDefinition::String(StringDefinition::new("Field 1")),
        ),
        (
            store_key!("field2").into(),
            MapItemDefinition::String(StringDefinition::new("Field 2")),
        ),
    ]
    .into_iter()
    .collect();
    let item_type_3: BTreeMap<StoreKey, MapItemDefinition> = vec![
        (
            store_key!("field1").into(),
            MapItemDefinition::String(StringDefinition::new("New Field 1")),
        ),
        (
            store_key!("field2").into(),
            MapItemDefinition::String(StringDefinition::new("New Field 2")),
        ),
    ]
    .into_iter()
    .collect();

    let entry_1 = MapEntryFrozen::new(&item_type);
    let entry_2 = MapEntryFrozen::new(&item_type);
    let entry_3 = MapEntryFrozen::new(&item_type_3);

    assert_eq!(entry_1, entry_2);
    assert_ne!(entry_1, entry_3);
    assert_eq!(&entry_1, &entry_2);
    assert_ne!(&entry_1, &entry_3);

    let item_1 = entry_1.get(store_key!("field1")).unwrap();
    let item_2 = entry_2.get(store_key!("field1")).unwrap();
    assert_eq!(item_1, item_2);
}

#[test]
fn test_map_frozen_new_from_items() {
    // Why: Test constructing a `MapFrozen` from existing entries, inferring the schema.
    let item_type: BTreeMap<StoreKey, MapItemDefinition> = vec![(
        store_key!("field1").into(),
        MapItemDefinition::String(StringDefinition::new("Field 1")),
    )]
    .into_iter()
    .collect();

    let mut entries = BTreeMap::new();
    entries.insert(store_key!("entry1").into(), MapEntryFrozen::new(&item_type));
    entries.insert(store_key!("entry2").into(), MapEntryFrozen::new(&item_type));

    let map_frozen = MapFrozen::new_from_items("A map", entries).unwrap();
    assert_eq!(map_frozen.definition().description().as_ref(), "A map");
    assert_eq!(map_frozen.definition().count(), 1);
    assert_eq!(map_frozen.count(), 2);
}

#[test]
fn test_map_frozen_new_from_items_empty_error() {
    // Why: An empty item set cannot be used to infer the map's entry schema.
    let entries: BTreeMap<StoreKey, MapEntryFrozen> = BTreeMap::new();
    assert!(MapFrozen::new_from_items("A map", entries).is_err());
}

#[test]
fn test_map_frozen_new_from_items_schema_mismatch_error() {
    // Why: Entries with differing schemas cannot form a valid map.
    let item_type_1: BTreeMap<StoreKey, MapItemDefinition> = vec![(
        store_key!("field1").into(),
        MapItemDefinition::String(StringDefinition::new("Field 1")),
    )]
    .into_iter()
    .collect();
    let item_type_2: BTreeMap<StoreKey, MapItemDefinition> = vec![(
        store_key!("field1").into(),
        MapItemDefinition::Table(TableDefinition::new(
            "Table field",
            Vec::<(StoreKey, NumberDefinition)>::new(),
        )),
    )]
    .into_iter()
    .collect();

    let mut entries = BTreeMap::new();
    entries.insert(
        store_key!("entry1").into(),
        MapEntryFrozen::new(&item_type_1),
    );
    entries.insert(
        store_key!("entry2").into(),
        MapEntryFrozen::new(&item_type_2),
    );

    assert!(MapFrozen::new_from_items("A map", entries).is_err());
}

#[test]
fn test_map_frozen_materializes_definition_defaults() {
    let table_definition = TableDefinition::new(
        "Values",
        vec![(store_key!("value"), NumberDefinition::new("Value"))],
    );
    let defaults = BTreeMap::from([(
        store_key!("first").into(),
        vec![
            MapItemDefault::scalar("Default name"),
            MapItemDefault::scalar("schema default"),
            MapItemDefault::table(vec![vec!["42"]]),
        ],
    )]);
    let definition = MapDefinition::new_with_default(
        "A map",
        vec![
            (
                store_key!("name"),
                MapItemDefinition::String(StringDefinition::new("Name")),
            ),
            (
                store_key!("inherited"),
                MapItemDefinition::String(StringDefinition::new_with_default(
                    "Inherited",
                    "schema default",
                )),
            ),
            (
                store_key!("values"),
                MapItemDefinition::Table(table_definition),
            ),
        ],
        defaults,
    );

    let map = MapFrozen::new(definition);
    let entry = map.get("first").unwrap();
    assert_eq!(entry.get_string("name").unwrap().value(), "Default name");
    assert_eq!(
        entry.get_string("inherited").unwrap().value(),
        "schema default"
    );
    assert_eq!(
        entry
            .get_table("values")
            .unwrap()
            .cell_by_name(0, "value")
            .unwrap()
            .as_ref(),
        "42"
    );
}
