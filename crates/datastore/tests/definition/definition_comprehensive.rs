//! Comprehensive integration tests that exercise every definition type together.
//!
//! These tests complement the focused checks in `definition_checks` by building
//! realistic composite definitions and verifying the interaction between
//! [`BasicDefinition`], [`ObjectDefinition`], [`StructDefinition`],
//! [`MapDefinition`], [`TableDefinition`], [`ChoiceDefinition`], and
//! [`FileDefinition`].
use datastore::definition::{
    BasicDefinition, BasicDefinitionType, ChoiceDefinition, FileDefinition, ItemDefinition,
    ItemDefinitionType, MapDefinition, ObjectDefinition, StructDefinition, StructItemDefinition,
    TableDefinition,
};
use datastore::key::StoreKey;
use datastore::{parameter_key, store_key};
use shareable_string::SharedStringStore;

#[test]
fn test_basic_definition_comprehensive() {
    let def = BasicDefinition::new_string_with_default("Desc", "Default");
    assert_eq!(def.description_ref().as_ref(), "Desc");
    assert_eq!(def.default_value_ref().as_ref(), "Default");

    let num_def = BasicDefinition::new_number_with_default("Num", "10");
    assert!(matches!(
        num_def.type_definition(),
        BasicDefinitionType::Number
    ));
    assert_eq!(num_def.default_value().as_ref(), "10");

    let file_info = FileDefinition::new("*.txt");
    assert_eq!(file_info.extension_filter_ref().as_ref(), "*.txt");
    let file_def = BasicDefinition::new_file_with_default("File", file_info, "file.txt");
    assert_eq!(file_def.default_value().as_ref(), "file.txt");

    let choice_info = ChoiceDefinition::new(vec!["A".into(), "B".into()]);
    assert_eq!(choice_info.choices().len(), 2);
    let choice_def = BasicDefinition::new_choice_with_default("Choice", choice_info, "A");
    assert_eq!(choice_def.default_value().as_ref(), "A");
}

#[test]
fn test_table_definition_comprehensive() {
    let table_def = TableDefinition::new(
        "Table Desc",
        vec![
            (store_key!("col1"), BasicDefinition::new_string("C1")),
            (store_key!("col2"), BasicDefinition::new_number("C2")),
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
fn test_struct_definition_comprehensive() {
    let struct_def = StructDefinition::new(
        "Struct Desc",
        vec![
            (
                store_key!("f1"),
                StructItemDefinition::Basic(BasicDefinition::new_string("F1")),
            ),
            (
                store_key!("f2"),
                StructItemDefinition::Table(TableDefinition::new(
                    "T1",
                    Vec::<(StoreKey, BasicDefinition)>::new(),
                )),
            ),
        ],
    );

    assert_eq!(struct_def.description_ref().as_ref(), "Struct Desc");
    assert_eq!(struct_def.count(), 2);
    assert!(struct_def.contains_key_str("f1"));
    assert!(struct_def.get_str("f2").is_some());

    let keys: Vec<String> = struct_def.keys().map(|k| k.as_str().to_string()).collect();
    assert!(keys.contains(&"f1".to_string()));
    assert!(keys.contains(&"f2".to_string()));

    let iter_count = struct_def.iter().count();
    assert_eq!(iter_count, 2);
}

#[test]
fn test_map_definition_comprehensive() {
    let struct_def = StructDefinition::new("Item", Vec::<(StoreKey, StructItemDefinition)>::new());
    let map_def = MapDefinition::new("Map Desc", struct_def);
    assert_eq!(map_def.description_ref().as_ref(), "Map Desc");
}

#[test]
fn test_parameter_definition_comprehensive() {
    let basic_def = BasicDefinition::new_string("Basic");
    let prop_def = ItemDefinition::new("Prop Desc", basic_def);
    assert_eq!(prop_def.description_ref().as_ref(), "Prop Desc");
    assert!(matches!(prop_def.item_type(), ItemDefinitionType::Basic(_)));
}

#[test]
fn test_object_definition_comprehensive() {
    let obj_def = ObjectDefinition::builder("Obj Desc")
        .with_parameter_inserted(
            parameter_key!("p_p1"),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .with_parameter_inserted(
            parameter_key!("p_p2"),
            ItemDefinition::new("P2", BasicDefinition::new_number("D2")),
        )
        .finish();

    assert_eq!(obj_def.description_ref().as_ref(), "Obj Desc");
    assert_eq!(obj_def.parameter_count(), 2);
    assert!(obj_def.parameter_contains_key_str("p_p1"));
    assert!(obj_def.parameter_get_str("p_p2").is_some());

    let keys: Vec<String> = obj_def
        .parameter_keys()
        .map(|k| k.as_ref().to_string())
        .collect();
    assert!(keys.contains(&"p_p1".to_string()));
    assert!(keys.contains(&"p_p2".to_string()));

    let iter_count = obj_def.parameter_iter().count();
    assert_eq!(iter_count, 2);
}

#[test]
fn test_launder_comprehensive() {
    let store = SharedStringStore::new();

    // Test BasicDefinition launder
    let basic_def = BasicDefinition::new_string("Basic");
    let laundered_basic = basic_def.launder(&store);
    assert_eq!(laundered_basic.description(), basic_def.description());

    // Test TableDefinition launder
    let table_def = TableDefinition::new(
        "Table",
        vec![(store_key!("col"), BasicDefinition::new_string("C"))],
    );
    let laundered_table = table_def.launder(&store);
    assert_eq!(laundered_table.description(), table_def.description());
    assert!(laundered_table.contains_key("col"));

    // Test StructDefinition launder
    let struct_def = StructDefinition::new(
        "Struct",
        vec![(store_key!("field"), BasicDefinition::new_string("F"))],
    );
    let laundered_struct = struct_def.launder(&store);
    assert_eq!(laundered_struct.description(), struct_def.description());
    assert!(laundered_struct.contains_key("field"));

    // Test MapDefinition launder
    let map_def = MapDefinition::new("Map", struct_def.clone());
    let laundered_map = map_def.launder(&store);
    assert_eq!(laundered_map.description(), map_def.description());

    // Test ItemDefinition launder
    let prop_def = ItemDefinition::new("Prop", basic_def);
    let laundered_prop = prop_def.launder(&store);
    assert_eq!(laundered_prop.description(), prop_def.description());

    // Test ObjectDefinition launder
    let obj_def = ObjectDefinition::builder("Obj")
        .with_parameter_inserted(parameter_key!("p_prop"), prop_def)
        .finish();
    let laundered_obj = obj_def.launder(&store);
    assert_eq!(laundered_obj.description(), obj_def.description());
    assert!(laundered_obj.parameter_contains_key("p_prop"));
}
