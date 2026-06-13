use datastore::definition::{
    BasicDefinition, MapDefinition, StructDefinition, StructItemDefinition, TableDefinition,
};
use datastore::key::StoreKey;
use datastore::store_key;

#[test]
fn test_map_definition() {
    // Why: Test map definition creation and properties.
    let struct_def = StructDefinition::new(
        "Item struct",
        Vec::<(StoreKey, StructItemDefinition)>::new(),
    );
    let map_def = MapDefinition::new("A map", struct_def.clone());

    // Check the various properties of the map definition.
    assert_eq!(map_def.description().as_ref(), "A map");
    assert_eq!(map_def.item_type().description().as_ref(), "Item struct");

    let item_def = map_def.item_type();
    assert_eq!(item_def.description().as_ref(), "Item struct");
    assert_eq!(item_def.count(), 0);
}

#[test]
fn test_complex_map_definition() {
    // Why: Test complex map definition creation and properties.
    let struct_item_def_1 = StructItemDefinition::Basic(BasicDefinition::new_string("Field 1"));
    let struct_item_def_2 = StructItemDefinition::Table(TableDefinition::new(
        "Table field",
        Vec::<(StoreKey, BasicDefinition)>::new(),
    ));
    let struct_def = StructDefinition::new(
        "Item struct",
        vec![
            (store_key!("field1"), struct_item_def_1.clone()),
            (store_key!("field2"), struct_item_def_2.clone()),
        ],
    );
    let map_def = MapDefinition::new("A map", struct_def.clone());

    // Check the various properties of the map definition.
    assert_eq!(map_def.description().as_ref(), "A map");
    assert_eq!(map_def.item_type().description().as_ref(), "Item struct");

    let item_def = map_def.item_type();
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
}

#[test]
fn test_map_definition_equality() {
    // Why: Test that two map definitions with the same properties are considered equal and ref equal.
    let map_def_1 = MapDefinition::new(
        "A map",
        StructDefinition::new(
            "Item struct",
            vec![
                (store_key!("field1"), BasicDefinition::new_string("Field 1")),
                (store_key!("field2"), BasicDefinition::new_string("Field 2")),
            ],
        ),
    );
    let map_def_2 = MapDefinition::new(
        "A map",
        StructDefinition::new(
            "Item struct",
            vec![
                (store_key!("field1"), BasicDefinition::new_string("Field 1")),
                (store_key!("field2"), BasicDefinition::new_string("Field 2")),
            ],
        ),
    );
    let map_def_3 = MapDefinition::new(
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
    );

    assert_eq!(map_def_1, map_def_2);
    assert_ne!(map_def_1, map_def_3);
    assert_eq!(&map_def_1, map_def_2);
    assert_ne!(map_def_1, &map_def_3);
}
