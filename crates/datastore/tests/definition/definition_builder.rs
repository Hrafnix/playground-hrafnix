//! Integration tests for the [`ObjectDefinitionBuilder`] API.
//!
//! Covers construction via the builder pattern, parameter inheritance, conflict
//! detection, removal of parameter, and the various `with_*` / `without`
//! convenience methods.
use datastore::definition::{
    BasicDefinition, ItemDefinition, ObjectDefinition, ObjectDefinitionBuilder,
};
use datastore::key::{ParameterKey, StoreKey};
use datastore::{StoreError, store_key};
use shareable_string::SharedStringStore;

#[test]
fn test_object_builder_pattern() {
    // Why: Test object creation with the builder pattern using with_inserted parameter.
    let obj_def = ObjectDefinition::builder("Test Object")
        .with_parameter_inserted(
            ParameterKey::new("p_prop1".into()).unwrap(),
            ItemDefinition::new("parameter 1", BasicDefinition::new_string("String prop")),
        )
        .with_parameter_inserted(
            ParameterKey::new("p_prop2".into()).unwrap(),
            ItemDefinition::new(
                "parameter 2",
                BasicDefinition::new_number_with_default("Number prop", "0"),
            ),
        )
        .finish();

    // Check that the object definition has the expected number of parameter.
    assert_eq!(obj_def.parameter_count(), 2);
}

#[test]
fn test_object_inheritance() {
    let parent_def = ObjectDefinition::builder("Parent")
        .with_parameter_inserted(
            ParameterKey::new("p_prop1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string_with_default("D1", "V1")),
        )
        .finish();

    let builder = parent_def.new_inherit("Child");
    assert_eq!(builder.finish().parameter_count(), 1);

    let mut builder = parent_def.new_inherit("Child");
    builder.insert_parameter(
        ParameterKey::new("p_prop2".into()).unwrap(),
        ItemDefinition::new("P2", BasicDefinition::new_string_with_default("D2", "V2")),
    );

    let child_def = builder.finish();
    assert_eq!(child_def.parameter_count(), 2);
    assert!(child_def.parameter_contains_key(store_key!("p_prop1")));
    assert!(child_def.parameter_contains_key(store_key!("p_prop2")));

    let mut builder = child_def.new_inherit("Grandchild");
    builder.remove_parameter(store_key!("p_prop1"));
    let grandchild_def = builder.finish();
    assert_eq!(grandchild_def.parameter_count(), 1);
    assert!(!grandchild_def.parameter_contains_key(store_key!("p_prop1")));
}

#[test]
fn test_invalid_keys() {
    let res = StoreKey::new("".into());
    assert!(matches!(res, Err(StoreError::KeyEmpty)));

    let res = StoreKey::new("Invalid Key!".into());
    assert!(matches!(res, Err(StoreError::KeyInvalidCharacter(_))));
    if let Err(StoreError::KeyInvalidCharacter(s)) = res {
        assert_eq!(s, "Invalid Key!");
    }
}

#[test]
fn test_object_definition_immutability() {
    let obj_def = ObjectDefinition::builder("Test Object")
        .with_parameter_inserted(
            ParameterKey::new("p_prop1".into()).unwrap(),
            ItemDefinition::new("parameter 1", BasicDefinition::new_string("String prop")),
        )
        .finish();

    // The point of this test is that obj_def does NOT have .add() or .remove()
    // It is immutable by design.
    assert_eq!(obj_def.parameter_count(), 1);
    assert!(obj_def.parameter_contains_key(store_key!("p_prop1")));
}

#[test]
fn test_object_definition_builder_new() {
    let builder = ObjectDefinitionBuilder::new("Test Description");
    let def = builder.finish();
    assert_eq!(def.description().as_str(), "Test Description");
    assert_eq!(def.parameter_count(), 0);
}

#[test]
fn test_object_definition_builder_insert() {
    let mut builder = ObjectDefinitionBuilder::new("Test");
    let prop = ItemDefinition::new("Prop", BasicDefinition::new_string("Desc"));
    let key = ParameterKey::new("p_key1".into()).unwrap();

    builder.insert_parameter(key.clone(), prop.clone());
    let def = builder.finish();

    assert_eq!(def.parameter_count(), 1);
    assert!(def.parameter_contains_key("p_key1"));
    assert_eq!(
        def.parameter_get("p_key1").unwrap().description().as_str(),
        "Prop"
    );
}

#[test]
fn test_object_definition_builder_with_inserted() {
    let prop = ItemDefinition::new("Prop", BasicDefinition::new_string("Desc"));
    let key = ParameterKey::new("p_key1".into()).unwrap();

    let def = ObjectDefinitionBuilder::new("Test")
        .with_parameter_inserted(key, prop)
        .finish();

    assert_eq!(def.parameter_count(), 1);
    assert!(def.parameter_contains_key("p_key1"));
}

#[test]
fn test_object_definition_builder_remove() {
    let mut builder = ObjectDefinitionBuilder::new("Test");
    let prop = ItemDefinition::new("Prop", BasicDefinition::new_string("Desc"));
    let key = ParameterKey::new("p_key1".into()).unwrap();

    builder.insert_parameter(key, prop);
    assert_eq!(builder.finish().parameter_count(), 1);

    let mut builder = ObjectDefinitionBuilder::new("Test");
    builder.insert_parameter(
        ParameterKey::new("p_key1".into()).unwrap(),
        ItemDefinition::new("Prop", BasicDefinition::new_string("Desc")),
    );
    builder.remove_parameter(store_key!("p_key1"));
    let def = builder.finish();

    assert_eq!(def.parameter_count(), 0);
    assert!(!def.parameter_contains_key(store_key!("p_key1")));
}

#[test]
fn test_object_definition_builder_without() {
    let def = ObjectDefinitionBuilder::new("Test")
        .with_parameter_inserted(
            ParameterKey::new("p_key1".into()).unwrap(),
            ItemDefinition::new("Prop", BasicDefinition::new_string("Desc")),
        )
        .without_parameter(store_key!("p_key1"))
        .finish();

    assert_eq!(def.parameter_count(), 0);
}

#[test]
fn test_object_definition_inherit() {
    let parent_def = ObjectDefinitionBuilder::new("Parent")
        .with_parameter_inserted(
            ParameterKey::new("p_p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .finish();

    let child_def = ObjectDefinitionBuilder::new("Child")
        .with_inherited(parent_def)
        .with_parameter_inserted(
            ParameterKey::new("p_c1".into()).unwrap(),
            ItemDefinition::new("C1", BasicDefinition::new_string("D2")),
        )
        .finish();

    assert_eq!(child_def.parameter_count(), 2);
    assert!(child_def.parameter_contains_key(store_key!("p_p1")));
    assert!(child_def.parameter_contains_key(store_key!("p_c1")));
}

#[test]
fn test_object_definition_inherit_overwrite() {
    let parent_def = ObjectDefinitionBuilder::new("Parent")
        .with_parameter_inserted(
            ParameterKey::new("p_p1".into()).unwrap(),
            ItemDefinition::new("ParentProp", BasicDefinition::new_string("D1")),
        )
        .finish();

    let child_def = ObjectDefinitionBuilder::new("Child")
        .with_parameter_inserted(
            ParameterKey::new("p_p1".into()).unwrap(),
            ItemDefinition::new("ChildProp", BasicDefinition::new_string("D2")),
        )
        .with_inherited(parent_def)
        .finish();

    assert_eq!(child_def.parameter_count(), 1);
    assert_eq!(
        child_def
            .parameter_get(store_key!("p_p1"))
            .unwrap()
            .description()
            .as_str(),
        "ParentProp"
    );
}

#[test]
fn test_object_definition_inherit_with_check() {
    let parent_def = ObjectDefinitionBuilder::new("Parent")
        .with_parameter_inserted(
            ParameterKey::new("p_p1".into()).unwrap(),
            ItemDefinition::new("ParentProp", BasicDefinition::new_string("D1")),
        )
        .finish();

    let result = ObjectDefinitionBuilder::new("Child")
        .with_parameter_inserted(
            ParameterKey::new("p_p2".into()).unwrap(),
            ItemDefinition::new("ChildProp", BasicDefinition::new_string("D2")),
        )
        .with_inherited_checked(parent_def);

    assert!(matches!(result, Ok(_)));
}

#[test]
fn test_object_definition_inherit_with_check_error() {
    let parent_def = ObjectDefinitionBuilder::new("Parent")
        .with_parameter_inserted(
            ParameterKey::new("p_p1".into()).unwrap(),
            ItemDefinition::new("ParentProp", BasicDefinition::new_string("D1")),
        )
        .finish();

    let result = ObjectDefinitionBuilder::new("Child")
        .with_parameter_inserted(
            ParameterKey::new("p_p1".into()).unwrap(),
            ItemDefinition::new("ChildProp", BasicDefinition::new_string("D2")),
        )
        .with_inherited_checked(parent_def);

    assert!(matches!(result, Err(StoreError::ParameterConflict(_))));
}

#[test]
fn test_object_definition_inherit_from_builder() {
    let b1 = ObjectDefinitionBuilder::new("B1").with_parameter_inserted(
        ParameterKey::new("p_p1".into()).unwrap(),
        ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
    );

    let b2 = ObjectDefinitionBuilder::new("B2")
        .with_inherited_from_builder(b1)
        .finish();

    assert_eq!(b2.parameter_count(), 1);
    assert!(b2.parameter_contains_key("p_p1"));
}

#[test]
fn test_object_definition_inherit_from_builder_with_check() {
    let b1 = ObjectDefinitionBuilder::new("B1").with_parameter_inserted(
        ParameterKey::new("p_p1".into()).unwrap(),
        ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
    );

    let result = ObjectDefinitionBuilder::new("B2")
        .with_parameter_inserted(
            ParameterKey::new("p_p2".into()).unwrap(),
            ItemDefinition::new("P2", BasicDefinition::new_string("D2")),
        )
        .with_inherited_from_builder_checked(b1);

    assert!(matches!(result, Ok(_)));
}

#[test]
fn test_object_definition_inherit_from_builder_with_check_error() {
    let b1 = ObjectDefinitionBuilder::new("B1").with_parameter_inserted(
        ParameterKey::new("p_p1".into()).unwrap(),
        ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
    );

    let result = ObjectDefinitionBuilder::new("B2")
        .with_parameter_inserted(
            ParameterKey::new("p_p1".into()).unwrap(),
            ItemDefinition::new("P2", BasicDefinition::new_string("D2")),
        )
        .with_inherited_from_builder_checked(b1);

    assert!(matches!(result, Err(StoreError::ParameterConflict(_))));
}

#[test]
fn test_object_definition_getters() {
    let def = ObjectDefinitionBuilder::new("Test")
        .with_parameter_inserted(
            ParameterKey::new("p_p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .finish();

    assert_eq!(def.description().as_str(), "Test");
    assert_eq!(def.description_ref().as_str(), "Test");
    assert_eq!(def.parameter_count(), 1);
    assert!(def.parameter_contains_key(store_key!("p_p1")));
    assert!(def.parameter_contains_key_str("p_p1"));
    assert!(def.parameter_get(store_key!("p_p1")).is_some());
    assert!(def.parameter_get_str("p_p1").is_some());
    assert!(def.parameter_get(store_key!("p_p2")).is_none());

    let keys: Vec<_> = def.parameter_keys().collect();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].as_str(), "p_p1");

    let iter_items: Vec<_> = def.parameter_iter().collect();
    assert_eq!(iter_items.len(), 1);
    assert_eq!(iter_items[0].0.as_str(), "p_p1");
}

#[test]
fn test_object_definition_launder() {
    let store = SharedStringStore::new();
    let def = ObjectDefinitionBuilder::new("Test")
        .with_parameter_inserted(
            ParameterKey::new("p_p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .finish();

    let laundered = def.launder(&store);
    assert_eq!(laundered.description().as_str(), "Test");
    assert!(laundered.parameter_contains_key(store_key!("p_p1")));
}
