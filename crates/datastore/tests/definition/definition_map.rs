use datastore::prelude::*;
use std::collections::BTreeMap;

#[test]
fn test_map_definition() {
    // Why: Test map definition creation and definition.
    let map_def = MapDefinition::new("A map", Vec::<(StoreKey, MapItemDefinition)>::new());

    // Check the various data items of the map definition.
    assert_eq!(map_def.description().as_ref(), "A map");
    assert_eq!(map_def.count(), 0);
}

#[test]
fn test_complex_map_definition() {
    // Why: Test complex map definition creation and definition.
    let map_item_def_1 = MapItemDefinition::String(StringDefinition::new("Field 1"));
    let map_item_def_2 = MapItemDefinition::Table(TableDefinition::new(
        "Table field",
        Vec::<(StoreKey, NumberDefinition)>::new(),
    ));
    let map_def = MapDefinition::new(
        "A map",
        vec![
            (store_key!("field1"), map_item_def_1.clone()),
            (store_key!("field2"), map_item_def_2.clone()),
        ],
    );

    // Check the various data items of the map definition.
    assert_eq!(map_def.description().as_ref(), "A map");
    assert_eq!(map_def.count(), 2);

    let mut keys: Vec<String> = map_def.keys().map(|k| k.as_ref().to_string()).collect();
    keys.sort();
    assert_eq!(keys, vec!["field1", "field2"]);

    assert_eq!(map_def.get(store_key!("field1")).unwrap(), &map_item_def_1);
    assert_eq!(map_def.get(store_key!("field2")).unwrap(), &map_item_def_2);
}

#[test]
fn test_map_definition_all_basic_definition() {
    // Why: Test map definition creation with all definitions.
    let map_def = MapDefinition::new(
        "A map",
        vec![
            (store_key!("field1"), StringDefinition::new("Field 1")),
            (store_key!("field2"), StringDefinition::new("Field 2")),
        ],
    );

    assert_eq!(map_def.description().as_ref(), "A map");
    assert_eq!(map_def.count(), 2);

    let item1 = map_def.get(store_key!("field1")).unwrap();
    if let MapItemDefinition::String(def) = item1 {
        assert_eq!(def.description().as_ref(), "Field 1");
        assert_eq!(def.default_value().as_ref(), "");
    } else {
        panic!("Expected item1 to be MapItemDefinition::String, but got {item1:?}");
    }

    let item2 = map_def.get(store_key!("field2")).unwrap();
    if let MapItemDefinition::String(def) = item2 {
        assert_eq!(def.description().as_ref(), "Field 2");
        assert_eq!(def.default_value().as_ref(), "");
    } else {
        panic!("Expected item2 to be MapItemDefinition::String, but got {item2:?}");
    }
}

#[test]
fn test_map_definition_equality() {
    // Why: Test that two map definitions with the same data items are considered equal.
    let map_def_1 = MapDefinition::new(
        "A map",
        vec![
            (store_key!("field1"), StringDefinition::new("Field 1")),
            (store_key!("field2"), StringDefinition::new("Field 2")),
        ],
    );
    let map_def_2 = MapDefinition::new(
        "A map",
        vec![
            (store_key!("field1"), StringDefinition::new("Field 1")),
            (store_key!("field2"), StringDefinition::new("Field 2")),
        ],
    );
    let map_def_3 = MapDefinition::new(
        "A map",
        vec![
            (store_key!("field1"), StringDefinition::new("New Field 1")),
            (store_key!("field2"), StringDefinition::new("New Field 2")),
        ],
    );

    assert_eq!(map_def_1, map_def_2);
    assert_ne!(map_def_1, map_def_3);
    assert_eq!(&map_def_1, map_def_2);
    assert_ne!(map_def_1, &map_def_3);

    let item_1 = map_def_1.get(store_key!("field1")).unwrap();
    let item_2 = map_def_2.get(store_key!("field1")).unwrap();
    let item_3 = map_def_3.get(store_key!("field1")).unwrap();
    assert_eq!(item_1, item_2);
    assert_ne!(*item_1, *item_3);
    assert_eq!(*item_1, item_2);
    assert_ne!(item_1, *item_3);
}

#[test]
fn test_map_definition_deduplicates_column_keys() {
    let map_def = MapDefinition::new(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberDefinition::new("Initial length"),
            ),
            (store_key!("duration"), NumberDefinition::new("Duration")),
            (
                store_key!("length"),
                NumberDefinition::new("Replacement length"),
            ),
        ],
    );

    assert_eq!(map_def.count(), 2);
    assert_eq!(
        map_def.keys().map(StoreKey::as_str).collect::<Vec<_>>(),
        vec!["duration", "length"]
    );
    if let Some(MapItemDefinition::Number(def)) = map_def.get(store_key!("length")) {
        assert_eq!(def.description().as_ref(), "Replacement length");
    } else {
        panic!("Expected map item definition for 'length' to be a NumberDefinition");
    }
}

#[test]
fn test_map_definition_with_defaults() {
    let defaults = BTreeMap::from([(
        store_key!("first").into(),
        vec![MapItemDefault::scalar("Default name")],
    )]);
    let definition = MapDefinition::new_with_default(
        "A map",
        vec![(store_key!("name"), StringDefinition::new("Name"))],
        defaults,
    );

    assert_eq!(definition.default_map().unwrap().len(), 1);
}

#[test]
fn test_map_definition_accepts_unmatched_defaults() {
    let too_many_items = BTreeMap::from([(
        store_key!("first").into(),
        vec![
            MapItemDefault::scalar("value"),
            MapItemDefault::scalar("extra"),
        ],
    )]);
    let definition = MapDefinition::new_with_default(
        "A map",
        vec![(store_key!("name"), StringDefinition::new("Name"))],
        too_many_items,
    );
    assert_eq!(definition.default_map().unwrap()["first"].len(), 2);

    let wrong_type = BTreeMap::from([(
        store_key!("first").into(),
        vec![MapItemDefault::table(vec![vec!["value"]])],
    )]);
    let definition = MapDefinition::new_with_default(
        "A map",
        vec![(store_key!("name"), StringDefinition::new("Name"))],
        wrong_type,
    );
    assert_eq!(definition.default_map().unwrap()["first"].len(), 1);
}
