use datastore::prelude::*;

#[test]
fn test_struct_all_basic_frozen() {
    // Why: Test struct frozen object creation with all basic definitions.
    let struct_frozen = StructFrozen::new(StructDefinition::new(
        "A struct",
        vec![
            (store_key!("field1"), StringDefinition::new("Field 1")),
            (
                store_key!("field2"),
                StringDefinition::new_with_default("Field 2", "Default value"),
            ),
        ],
    ));

    // Check the various parameters of the struct frozen object.
    assert_eq!(struct_frozen.definition().description(), "A struct");
    assert_eq!(struct_frozen.definition().count(), 2);

    let mut keys: Vec<String> = struct_frozen
        .definition()
        .keys()
        .map(|k| k.to_string())
        .collect();
    keys.sort();
    assert_eq!(keys, vec!["field1", "field2"]);

    let item1 = struct_frozen
        .definition()
        .get(&store_key!("field1"))
        .unwrap();
    if let StructItemDefinition::String(def) = item1 {
        assert_eq!(def.description(), "Field 1");
        assert_eq!(def.default_value(), "");
    } else {
        panic!(
            "Expected item1 to be StructItemDefinition::Basic, but got {:?}",
            item1
        );
    }

    let item2 = struct_frozen
        .definition()
        .get(&store_key!("field2"))
        .unwrap();
    if let StructItemDefinition::String(def) = item2 {
        assert_eq!(def.description(), "Field 2");
        assert_eq!(def.default_value(), "Default value");
    } else {
        panic!(
            "Expected item1 to be StructItemDefinition::Table, but got {:?}",
            item1
        );
    }

    let t1 = struct_frozen.get("field1").unwrap().get_string().unwrap();
    let t2 = struct_frozen.get_string("field2").unwrap();
    assert_ne!(t1, t2);
    assert_eq!(t1.value(), "");
    assert_eq!(t2.value(), "Default value");

    assert_eq!(struct_frozen.get("field3"), None);
    assert_eq!(struct_frozen.get_table("field3"), None);
    assert_eq!(struct_frozen.get_table("field1"), None);
    assert_eq!(struct_frozen.get_table("field2"), None);
    assert_ne!(struct_frozen.hash(), [0u8; 32]);
}

#[test]
fn test_struct_all_table_frozen() {
    // Why: Test struct frozen object creation with all table definitions.
    let struct_frozen = StructFrozen::new(StructDefinition::new(
        "A struct",
        vec![
            (
                store_key!("field1"),
                TableDefinition::new("Table field 1", Vec::<(StoreKey, NumberDefinition)>::new()),
            ),
            (
                store_key!("field2"),
                TableDefinition::new("Table field 2", Vec::<(StoreKey, NumberDefinition)>::new()),
            ),
        ],
    ));

    // Check the various parameters of the struct frozen object.
    assert_eq!(struct_frozen.definition().description(), "A struct");
    assert_eq!(struct_frozen.definition().count(), 2);

    let mut keys: Vec<String> = struct_frozen
        .definition()
        .keys()
        .map(|k| k.to_string())
        .collect();
    keys.sort();
    assert_eq!(keys, vec!["field1", "field2"]);

    let item1 = struct_frozen
        .definition()
        .get(&store_key!("field1"))
        .unwrap();
    if let StructItemDefinition::Table(def) = item1 {
        assert_eq!(def.description(), "Table field 1");
        assert_eq!(def.count(), 0);
    } else {
        panic!(
            "Expected item1 to be StructItemDefinition::Basic, but got {:?}",
            item1
        );
    }

    let item2 = struct_frozen
        .definition()
        .get(&store_key!("field2"))
        .unwrap();
    if let StructItemDefinition::Table(def) = item2 {
        assert_eq!(def.description(), "Table field 2");
        assert_eq!(def.count(), 0);
    } else {
        panic!(
            "Expected item1 to be StructItemDefinition::Table, but got {:?}",
            item1
        );
    }

    let t1 = struct_frozen.get("field1").unwrap().get_table().unwrap();
    let t2 = struct_frozen.get_table("field2").unwrap();
    assert_ne!(t1, t2);
    assert_eq!(t1.row_count(), 0);
    assert_eq!(t1.column_count(), 0);
    assert_eq!(t2.row_count(), 0);
    assert_eq!(t2.column_count(), 0);

    assert_eq!(struct_frozen.get("field3"), None);
    assert_eq!(struct_frozen.get_string("field3"), None);
    assert_eq!(struct_frozen.get_string("field1"), None);
    assert_eq!(struct_frozen.get_string("field2"), None);
    assert_ne!(struct_frozen.hash(), [0u8; 32]);
}

#[test]
fn test_struct_mixed_frozen() {
    // Why: Test struct frozen object creation with mixed field types.
    let struct_frozen = StructFrozen::new(StructDefinition::new(
        "A struct",
        vec![
            (
                store_key!("field1"),
                StructItemDefinition::String(StringDefinition::new("Field 1")),
            ),
            (
                store_key!("field2"),
                StructItemDefinition::Table(TableDefinition::new(
                    "Table field",
                    Vec::<(StoreKey, NumberDefinition)>::new(),
                )),
            ),
        ],
    ));

    // Check the various parameters of the struct frozen object.
    assert_eq!(struct_frozen.definition().description(), "A struct");
    assert_eq!(struct_frozen.definition().count(), 2);

    let mut keys: Vec<String> = struct_frozen
        .definition()
        .keys()
        .map(|k| k.to_string())
        .collect();
    keys.sort();
    assert_eq!(keys, vec!["field1", "field2"]);

    let item1 = struct_frozen
        .definition()
        .get(&store_key!("field1"))
        .unwrap();
    if let StructItemDefinition::String(def) = item1 {
        assert_eq!(def.description(), "Field 1");
        assert_eq!(def.default_value(), "");
    } else {
        panic!(
            "Expected item1 to be StructItemDefinition::Basic, but got {:?}",
            item1
        );
    }

    let item2 = struct_frozen
        .definition()
        .get(&store_key!("field2"))
        .unwrap();
    if let StructItemDefinition::Table(def) = item2 {
        assert_eq!(def.description(), "Table field");
        assert_eq!(def.count(), 0);
    } else {
        panic!(
            "Expected item1 to be StructItemDefinition::Table, but got {:?}",
            item1
        );
    }

    let t1 = struct_frozen.get("field1").unwrap().get_string().unwrap();
    let t2 = struct_frozen.get_table("field2").unwrap();
    assert_eq!(t1.value(), "");
    assert_eq!(t2.row_count(), 0);
    assert_eq!(t2.column_count(), 0);

    assert_eq!(struct_frozen.get("field3"), None);
    assert_eq!(struct_frozen.get_string("field3"), None);
    assert_eq!(struct_frozen.get_table("field1"), None);
    assert_eq!(struct_frozen.get_string("field2"), None);
    assert_ne!(struct_frozen.hash(), [0u8; 32]);
}

#[test]
fn test_struct_frozen_equality() {
    // Why: Test that two struct frozen objects with the same items are considered equal.
    let struct_frozen_1 = StructFrozen::new(StructDefinition::new(
        "A struct",
        vec![
            (store_key!("field1"), StringDefinition::new("Field 1")),
            (store_key!("field2"), StringDefinition::new("Field 2")),
        ],
    ));
    let struct_frozen_2 = StructFrozen::new(StructDefinition::new(
        "A struct",
        vec![
            (store_key!("field1"), StringDefinition::new("Field 1")),
            (store_key!("field2"), StringDefinition::new("Field 2")),
        ],
    ));
    let struct_frozen_3 = StructFrozen::new(StructDefinition::new(
        "A struct",
        vec![
            (store_key!("field1"), StringDefinition::new("New Field 1")),
            (store_key!("field2"), StringDefinition::new("New Field 2")),
        ],
    ));

    assert_eq!(struct_frozen_1, struct_frozen_2);
    assert_ne!(struct_frozen_1, struct_frozen_3);
    assert_eq!(&struct_frozen_1, struct_frozen_2);
    assert_ne!(struct_frozen_1, &struct_frozen_3);

    let struct_item_1 = struct_frozen_1.get(&store_key!("field1")).unwrap();
    let struct_item_2 = struct_frozen_2.get(&store_key!("field1")).unwrap();
    let struct_item_3 = struct_frozen_3.get(&store_key!("field1")).unwrap();
    assert_eq!(struct_item_1, struct_item_2);
    assert_ne!(*struct_item_1, *struct_item_3);
    assert_eq!(*struct_item_1, struct_item_2);
    assert_ne!(struct_item_1, *struct_item_3);
}
