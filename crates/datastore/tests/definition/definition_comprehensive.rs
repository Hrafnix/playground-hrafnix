//! Comprehensive integration tests that exercise every definition type together.
//!
//! These tests complement the focused checks in `definition_checks` by building
//! realistic composite definitions and verifying the interaction between
//! [`StringDefinition`], [`ObjectDefinition`],
//! [`MapDefinition`], [`TableDefinition`], [`ChoiceDefinition`], and
//! [`FileDefinition`].
use datastore::prelude::*;

#[test]
fn test_basic_definition_comprehensive() {
    // Why: Test that various types of basic definitions can be created and hold correct values.
    let def = StringDefinition::new_with_default("Desc", "Default");
    assert_eq!(def.description_ref().as_ref(), "Desc");
    assert_eq!(def.default_value_ref().as_ref(), "Default");

    let num_def = NumberDefinition::new_with_default("Num", "10");
    assert_eq!(num_def.default_value().as_ref(), "10");

    let file_def = FileDefinition::new_with_default("File", "*.txt", false, "file.txt");
    assert_eq!(file_def.default_value().as_ref(), "file.txt");

    let choice_def = ChoiceDefinition::new_with_default(
        "Choice",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
        "a",
    );
    assert_eq!(choice_def.choices().len(), 2);
    assert_eq!(choice_def.default_value().as_ref(), "a");
}

#[test]
fn test_table_definition_comprehensive() {
    // Why: Test that a table definition correctly stores column definitions and supports basic operations.
    let table_def = TableDefinition::new(
        "Table Desc",
        vec![
            (store_key!("col1"), NumberDefinition::new("C1")),
            (store_key!("col2"), NumberDefinition::new("C2")),
        ],
    );

    assert_eq!(table_def.description_ref().as_ref(), "Table Desc");
    assert_eq!(table_def.count(), 2);
    assert!(table_def.contains_key_str("col1"));
    assert!(table_def.get_str("col2").is_some());
    assert!(table_def.get_str("nonexistent").is_none());

    let keys: Vec<String> = table_def.keys().map(|k| k.as_ref().to_string()).collect();
    assert!(keys.contains(&"col1".to_string()));
    assert!(keys.contains(&"col2".to_string()));

    let iter_count = table_def.iter().count();
    assert_eq!(iter_count, 2);
}

#[test]
fn test_map_definition_comprehensive() {
    // Why: Test that a map definition correctly stores its entry schema fields.
    let map_def = MapDefinition::new(
        "Map Desc",
        vec![
            (
                store_key!("f1"),
                MapItemDefinition::String(StringDefinition::new("F1")),
            ),
            (
                store_key!("f2"),
                MapItemDefinition::Table(TableDefinition::new(
                    "T1",
                    Vec::<(StoreKey, NumberDefinition)>::new(),
                )),
            ),
        ],
    );

    assert_eq!(map_def.description_ref().as_ref(), "Map Desc");
    assert_eq!(map_def.count(), 2);
    assert!(map_def.contains_key_str("f1"));
    assert!(map_def.get_str("f2").is_some());

    let keys: Vec<String> = map_def.keys().map(|k| k.as_str().to_string()).collect();
    assert!(keys.contains(&"f1".to_string()));
    assert!(keys.contains(&"f2".to_string()));

    let iter_count = map_def.iter().count();
    assert_eq!(iter_count, 2);
}

#[test]
fn test_parameter_definition_comprehensive() {
    // Why: Test that an item definition correctly wraps a basic definition.
    let basic_def = StringDefinition::new("Basic");
    assert_eq!(basic_def.description_ref().as_ref(), "Basic");
}

#[test]
fn test_object_definition_comprehensive() {
    // Why: Test that an object definition correctly handles items added via the builder.
    let obj_def = ObjectDefinition::builder("Obj Desc")
        .with(store_key!("p_p1"), StringDefinition::new("D1"))
        .with(store_key!("p_p2"), NumberDefinition::new("D2"))
        .finish();

    assert_eq!(obj_def.description_ref().as_ref(), "Obj Desc");
    assert_eq!(obj_def.count(), 2);
    assert!(obj_def.contains_str("p_p1"));
    assert!(obj_def.get_str("p_p2").is_some());

    let keys: Vec<String> = obj_def.keys().map(|k| k.as_ref().to_string()).collect();
    assert!(keys.contains(&"p_p1".to_string()));
    assert!(keys.contains(&"p_p2".to_string()));

    let iter_count = obj_def.iter().count();
    assert_eq!(iter_count, 2);
}

#[test]
fn test_launder_comprehensive() {
    // Why: Test that all definition types correctly support the launder operation for string store migration.
    let store = SharedStringStore::new();

    // Test BasicDefinition launder
    let basic_def = StringDefinition::new("Basic");
    let laundered_basic = basic_def.launder(&store);
    assert_eq!(laundered_basic.description(), basic_def.description());

    // Test TableDefinition launder
    let table_def = TableDefinition::new(
        "Table",
        vec![(store_key!("col"), NumberDefinition::new("C"))],
    );
    let laundered_table = table_def.launder(&store);
    assert_eq!(laundered_table.description(), table_def.description());
    assert!(laundered_table.contains_key("col"));

    // Test MapDefinition launder
    let map_def = MapDefinition::new(
        "Map",
        vec![(store_key!("field"), NumberDefinition::new("F"))],
    );
    let laundered_map = map_def.launder(&store);
    assert_eq!(laundered_map.description(), map_def.description());
    assert!(laundered_map.contains_key("field"));

    // Test ObjectDefinition launder
    let obj_def = ObjectDefinition::builder("Obj")
        .with(store_key!("p_prop"), basic_def)
        .finish();
    let laundered_obj = obj_def.launder(&store);
    assert_eq!(laundered_obj.description(), obj_def.description());
    assert!(laundered_obj.contains("p_prop"));
}
