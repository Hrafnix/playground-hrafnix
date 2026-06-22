use datastore::definition::{
    BasicDefinition, MapDefinition, StructDefinition, StructItemDefinition, TableDefinition,
};
use datastore::frozen::MapFrozen;
use datastore::key::StoreKey;
use datastore::store_key;

#[test]
fn test_map_frozen() {
    // Why: Test map frozen object creation and definition.
    let map_frozen = MapFrozen::new(MapDefinition::new(
        "A map",
        StructDefinition::new(
            "Item struct",
            Vec::<(StoreKey, StructItemDefinition)>::new(),
        ),
    ));

    // Check the various parameters of the map definition.
    assert_eq!(map_frozen.definition().description().as_ref(), "A map");
    assert_eq!(
        map_frozen.definition().item_type().description().as_ref(),
        "Item struct"
    );

    let item_def = map_frozen.definition().item_type();
    assert_eq!(item_def.description().as_ref(), "Item struct");
    assert_eq!(item_def.count(), 0);

    assert_eq!(map_frozen.get(&store_key!("field1")).is_none(), true);
    assert_eq!(map_frozen.count(), 0);
    assert_ne!(map_frozen.hash(), [0u8; 32]);
}

#[test]
fn test_complex_map_frozen() {
    // Why: Test complex map frozen object creation and definition.
    let struct_item_def_1 = StructItemDefinition::Basic(BasicDefinition::new_string("Field 1"));
    let struct_item_def_2 = StructItemDefinition::Table(TableDefinition::new(
        "Table field",
        Vec::<(StoreKey, BasicDefinition)>::new(),
    ));

    let map_frozen = MapFrozen::new(MapDefinition::new(
        "A map",
        StructDefinition::new(
            "Item struct",
            vec![
                (store_key!("field1"), struct_item_def_1.clone()),
                (store_key!("field2"), struct_item_def_2.clone()),
            ],
        ),
    ));

    // Check the various parameters of the map definition.
    assert_eq!(map_frozen.definition().description().as_ref(), "A map");
    assert_eq!(
        map_frozen.definition().item_type().description().as_ref(),
        "Item struct"
    );

    let item_def = map_frozen.definition().item_type();
    assert_eq!(item_def.description().as_ref(), "Item struct");
    assert_eq!(item_def.count(), 2);
    assert_eq!(
        item_def.get(&store_key!("field1")).unwrap(),
        &struct_item_def_1
    );
    assert_eq!(
        item_def.get(&store_key!("field2")).unwrap(),
        &struct_item_def_2
    );

    assert_eq!(map_frozen.get("f").is_none(), true);
    assert_eq!(map_frozen.get("field1").is_none(), true);
    assert_eq!(map_frozen.get("field2").is_none(), true);
    assert_eq!(map_frozen.count(), 0);
    assert_ne!(map_frozen.hash(), [0u8; 32]);
}

#[test]
fn test_map_frozen_equality() {
    // Why: Test that two map frozen objects with the same parameters are considered equal.
    let map_frozen_1 = MapFrozen::new(MapDefinition::new(
        "A map",
        StructDefinition::new(
            "Item struct",
            vec![
                (store_key!("field1"), BasicDefinition::new_string("Field 1")),
                (store_key!("field2"), BasicDefinition::new_string("Field 2")),
            ],
        ),
    ));
    let map_frozen_2 = MapFrozen::new(MapDefinition::new(
        "A map",
        StructDefinition::new(
            "Item struct",
            vec![
                (store_key!("field1"), BasicDefinition::new_string("Field 1")),
                (store_key!("field2"), BasicDefinition::new_string("Field 2")),
            ],
        ),
    ));
    let map_frozen_3 = MapFrozen::new(MapDefinition::new(
        "A map",
        StructDefinition::new(
            "Item struct",
            vec![
                (
                    store_key!("field1"),
                    BasicDefinition::new_string("New Field 1"),
                ),
                (
                    store_key!("field2"),
                    BasicDefinition::new_string("New Field 2"),
                ),
            ],
        ),
    ));

    assert_eq!(map_frozen_1, map_frozen_2);
    assert_ne!(map_frozen_1, map_frozen_3);
    assert_eq!(&map_frozen_1, map_frozen_2);
    assert_ne!(map_frozen_1, &map_frozen_3);
}
