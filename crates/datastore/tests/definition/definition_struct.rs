use datastore::definition::{
    BasicDefinition, BasicDefinitionType, StructDefinition, StructItemDefinition, TableDefinition,
};
use datastore::key::StoreKey;
use datastore::store_key;

#[test]
fn test_struct_all_basic_definition() {
    // Why: Test struct definition creation and properties.
    let struct_def = StructDefinition::new(
        "A struct",
        vec![
            (store_key!("field1"), BasicDefinition::new_string("Field 1")),
            (store_key!("field2"), BasicDefinition::new_string("Field 2")),
        ],
    );

    // Check the various properties of the struct definition.
    assert_eq!(struct_def.description().as_ref(), "A struct");
    assert_eq!(struct_def.count(), 2);

    let mut keys: Vec<String> = struct_def.keys().map(|k| k.as_ref().to_string()).collect();
    keys.sort();
    assert_eq!(keys, vec!["field1", "field2"]);

    let item1 = struct_def.get(&store_key!("field1")).unwrap();
    if let StructItemDefinition::Basic(def) = item1 {
        assert_eq!(def.description().as_ref(), "Field 1");
        assert!(matches!(def.type_definition(), BasicDefinitionType::String));
        assert_eq!(def.default_value().as_ref(), "");
    } else {
        panic!(
            "Expected item1 to be StructItemDefinition::Basic, but got {:?}",
            item1
        );
    }

    let item2 = struct_def.get(&store_key!("field2")).unwrap();
    if let StructItemDefinition::Basic(def) = item2 {
        assert_eq!(def.description().as_ref(), "Field 2");
        assert!(matches!(def.type_definition(), BasicDefinitionType::String));
        assert_eq!(def.default_value().as_ref(), "");
    } else {
        panic!(
            "Expected item1 to be StructItemDefinition::Table, but got {:?}",
            item1
        );
    }
}

#[test]
fn test_struct_all_table_definition() {
    // Why: Test struct definition creation and properties.
    let struct_def = StructDefinition::new(
        "A struct",
        vec![
            (
                store_key!("field1"),
                TableDefinition::new("Table field 1", Vec::<(StoreKey, BasicDefinition)>::new()),
            ),
            (
                store_key!("field2"),
                TableDefinition::new("Table field 2", Vec::<(StoreKey, BasicDefinition)>::new()),
            ),
        ],
    );

    // Check the various properties of the struct definition.
    assert_eq!(struct_def.description().as_ref(), "A struct");
    assert_eq!(struct_def.count(), 2);

    let mut keys: Vec<String> = struct_def.keys().map(|k| k.as_ref().to_string()).collect();
    keys.sort();
    assert_eq!(keys, vec!["field1", "field2"]);

    let item1 = struct_def.get(&store_key!("field1")).unwrap();
    if let StructItemDefinition::Table(def) = item1 {
        assert_eq!(def.description().as_ref(), "Table field 1");
        assert_eq!(def.count(), 0);
    } else {
        panic!(
            "Expected item1 to be StructItemDefinition::Basic, but got {:?}",
            item1
        );
    }

    let item2 = struct_def.get(&store_key!("field2")).unwrap();
    if let StructItemDefinition::Table(def) = item2 {
        assert_eq!(def.description().as_ref(), "Table field 2");
        assert_eq!(def.count(), 0);
    } else {
        panic!(
            "Expected item1 to be StructItemDefinition::Table, but got {:?}",
            item1
        );
    }
}

#[test]
fn test_struct_mixed_definition() {
    // Why: Test struct definition creation and properties.
    let struct_def = StructDefinition::new(
        "A struct",
        vec![
            (
                store_key!("field1"),
                StructItemDefinition::Basic(BasicDefinition::new_string("Field 1")),
            ),
            (
                store_key!("field2"),
                StructItemDefinition::Table(TableDefinition::new(
                    "Table field",
                    Vec::<(StoreKey, BasicDefinition)>::new(),
                )),
            ),
        ],
    );

    // Check the various properties of the struct definition.
    assert_eq!(struct_def.description().as_ref(), "A struct");
    assert_eq!(struct_def.count(), 2);

    let mut keys: Vec<String> = struct_def.keys().map(|k| k.as_ref().to_string()).collect();
    keys.sort();
    assert_eq!(keys, vec!["field1", "field2"]);

    let item1 = struct_def.get(&store_key!("field1")).unwrap();
    if let StructItemDefinition::Basic(def) = item1 {
        assert_eq!(def.description().as_ref(), "Field 1");
        assert!(matches!(def.type_definition(), BasicDefinitionType::String));
        assert_eq!(def.default_value().as_ref(), "");
    } else {
        panic!(
            "Expected item1 to be StructItemDefinition::Basic, but got {:?}",
            item1
        );
    }

    let item2 = struct_def.get(&store_key!("field2")).unwrap();
    if let StructItemDefinition::Table(def) = item2 {
        assert_eq!(def.description().as_ref(), "Table field");
        assert_eq!(def.count(), 0);
    } else {
        panic!(
            "Expected item1 to be StructItemDefinition::Table, but got {:?}",
            item1
        );
    }
}

#[test]
fn test_struct_definition_equality() {
    // Why: Test that two struct definitions with the same properties are considered equal and ref equal.
    let struct_def_1 = StructDefinition::new(
        "A struct",
        vec![
            (store_key!("field1"), BasicDefinition::new_string("Field 1")),
            (store_key!("field2"), BasicDefinition::new_string("Field 2")),
        ],
    );
    let struct_def_2 = StructDefinition::new(
        "A struct",
        vec![
            (store_key!("field1"), BasicDefinition::new_string("Field 1")),
            (store_key!("field2"), BasicDefinition::new_string("Field 2")),
        ],
    );
    let struct_def_3 = StructDefinition::new(
        "A struct",
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
    );

    assert_eq!(struct_def_1, struct_def_2);
    assert_ne!(struct_def_1, struct_def_3);
    assert_eq!(&struct_def_1, struct_def_2);
    assert_ne!(struct_def_1, &struct_def_3);

    let struct_item_1 = struct_def_1.get(&store_key!("field1")).unwrap();
    let struct_item_2 = struct_def_2.get(&store_key!("field1")).unwrap();
    let struct_item_3 = struct_def_3.get(&store_key!("field1")).unwrap();
    assert_eq!(struct_item_1, struct_item_2);
    assert_ne!(*struct_item_1, *struct_item_3);
    assert_eq!(*struct_item_1, struct_item_2);
    assert_ne!(struct_item_1, *struct_item_3);
}
