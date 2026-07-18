//! Integration tests for the [`ParameterObjectDefinitionBuilder`] API.

use datastore::prelude::*;

#[test]
fn test_parameter_object_builder_pattern() {
    // Why: Test parameter object definition creation with the builder pattern using the with method.
    let obj_def = ParameterObjectDefinition::builder("Test Object")
        .with(
            ParameterKey::new("p_prop1".into()).unwrap(),
            StringDefinition::new("String prop"),
        )
        .with(
            ParameterKey::new("p_prop2".into()).unwrap(),
            NumberDefinition::new_with_default("Number prop", "0"),
        )
        .finish();

    assert_eq!(obj_def.count(), 2);
}

#[test]
fn test_parameter_object_inheritance() {
    // Why: Test that a parameter object definition can inherit from another.
    let parent_def = ParameterObjectDefinition::builder("Parent")
        .with(
            ParameterKey::new("p_prop1".into()).unwrap(),
            StringDefinition::new_with_default("D1", "V1"),
        )
        .finish();

    let builder = parent_def.inherit("Child");
    assert_eq!(builder.finish().count(), 1);

    let mut builder = parent_def.inherit("Child");
    builder.insert(
        ParameterKey::new("p_prop2".into()).unwrap(),
        StringDefinition::new_with_default("D2", "V2"),
    );

    let child_def = builder.finish();
    assert_eq!(child_def.count(), 2);
    assert!(child_def.contains("p_prop1"));
    assert!(child_def.contains("p_prop2"));

    let mut builder = child_def.inherit("Grandchild");
    builder.remove("p_prop1");
    let grandchild_def = builder.finish();
    assert_eq!(grandchild_def.count(), 1);
    assert!(!grandchild_def.contains("p_prop1"));
}

#[test]
fn test_invalid_parameter_keys() {
    // Why: Test that invalid parameter keys are correctly rejected.
    let res = ParameterKey::new("".into());
    assert!(matches!(res, Err(StoreError::KeyEmpty)));

    let res = ParameterKey::new("Invalid Key!".into());
    assert!(matches!(res, Err(StoreError::KeyInvalidPrefix(_))));
    if let Err(StoreError::KeyInvalidPrefix(s)) = res {
        assert_eq!(s, "Invalid Key!");
    }

    let res = ParameterKey::new("p_Invalid Key!".into());
    assert!(matches!(res, Err(StoreError::KeyInvalidCharacter(_))));
    if let Err(StoreError::KeyInvalidCharacter(s)) = res {
        assert_eq!(s, "p_Invalid Key!");
    }
}

#[test]
fn test_parameter_object_definition_immutability() {
    // Why: Test that the parameter object definition is immutable once created.
    let obj_def = ParameterObjectDefinition::builder("Test Object")
        .with(
            ParameterKey::new("p_prop1".into()).unwrap(),
            StringDefinition::new("String prop"),
        )
        .finish();

    // The point of this test is that obj_def does NOT have .add() or .remove()
    // It is immutable by design.
    assert_eq!(obj_def.count(), 1);
    assert!(obj_def.contains("p_prop1"));
}

#[test]
fn test_parameter_object_definition_builder_new() {
    // Why: Test that a new builder correctly initializes an empty parameter object definition.
    let builder = ParameterObjectDefinitionBuilder::new("Test Description");
    let def = builder.finish();
    assert_eq!(def.description().as_str(), "Test Description");
    assert_eq!(def.count(), 0);
}

#[test]
fn test_parameter_object_definition_builder_insert() {
    // Why: Test that the builder correctly inserts items into the parameter object definition.
    let mut builder = ParameterObjectDefinitionBuilder::new("Test");
    let prop = StringDefinition::new("Desc");
    let key = ParameterKey::new("p_key1".into()).unwrap();

    builder.insert(key.clone(), prop.clone());
    let def = builder.finish();

    assert_eq!(def.count(), 1);
    assert!(def.contains("p_key1"));
}

#[test]
fn test_parameter_object_definition_builder_with_inserted() {
    // Why: Test that the builder correctly adds an item using the fluent interface.
    let prop = StringDefinition::new("Desc");
    let key = ParameterKey::new("p_key1".into()).unwrap();

    let def = ParameterObjectDefinitionBuilder::new("Test")
        .with(key, prop)
        .finish();

    assert_eq!(def.count(), 1);
    assert!(def.contains("p_key1"));
}

#[test]
fn test_parameter_object_definition_builder_remove() {
    // Why: Test that the builder correctly removes items.
    let mut builder = ParameterObjectDefinitionBuilder::new("Test");
    let prop = StringDefinition::new("Desc");
    let key = ParameterKey::new("p_key1".into()).unwrap();

    builder.insert(key, prop);
    assert_eq!(builder.finish().count(), 1);

    let mut builder = ParameterObjectDefinitionBuilder::new("Test");
    builder.insert(
        ParameterKey::new("p_key1".into()).unwrap(),
        StringDefinition::new("Desc"),
    );
    builder.remove("p_key1");
    let def = builder.finish();

    assert_eq!(def.count(), 0);
    assert!(!def.contains("p_key1"));
}

#[test]
fn test_parameter_object_definition_builder_without() {
    // Why: Test that the builder correctly removes an item using the fluent interface.
    let def = ParameterObjectDefinitionBuilder::new("Test")
        .with(
            ParameterKey::new("p_key1".into()).unwrap(),
            StringDefinition::new("Desc"),
        )
        .without("p_key1")
        .finish();

    assert_eq!(def.count(), 0);
}

#[test]
fn test_parameter_object_definition_inherit() {
    // Why: Test that the builder correctly inherits from another parameter object definition.
    let parent_def = ParameterObjectDefinitionBuilder::new("Parent")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    let child_def = ParameterObjectDefinitionBuilder::new("Child")
        .inherit(parent_def)
        .with(
            ParameterKey::new("p_c1".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .finish();

    assert_eq!(child_def.count(), 2);
    assert!(child_def.contains("p_p1"));
    assert!(child_def.contains("p_c1"));
}

#[test]
fn test_parameter_object_definition_inherit_overwrite() {
    // Why: Test that inheriting from another parameter object definition overwrites existing items with the same key.
    let parent_def = ParameterObjectDefinitionBuilder::new("Parent")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    let child_def = ParameterObjectDefinitionBuilder::new("Child")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit(parent_def)
        .finish();

    assert_eq!(child_def.count(), 1);
}

#[test]
fn test_parameter_object_definition_inherit_with_check() {
    // Why: Test that inherit_with_check successfully inherits when there are no key conflicts.
    let parent_def = ParameterObjectDefinitionBuilder::new("Parent")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    let result = ParameterObjectDefinitionBuilder::new("Child")
        .with(
            ParameterKey::new("p_p2".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit_with_check(parent_def);

    assert!(matches!(result, Ok(_)));
}

#[test]
fn test_parameter_object_definition_inherit_with_check_error() {
    // Why: Test that inherit_with_check returns an error when there is a key conflict.
    let parent_def = ParameterObjectDefinitionBuilder::new("Parent")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    let result = ParameterObjectDefinitionBuilder::new("Child")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit_with_check(parent_def);

    assert!(matches!(result, Err(StoreError::KeyConflict(_))));
}

#[test]
fn test_parameter_object_definition_inherit_from_builder() {
    // Why: Test that the builder correctly inherits from another builder.
    let b1 = ParameterObjectDefinitionBuilder::new("B1").with(
        ParameterKey::new("p_p1".into()).unwrap(),
        StringDefinition::new("D1"),
    );

    let b2 = ParameterObjectDefinitionBuilder::new("B2")
        .inherit_from_builder(b1)
        .finish();

    assert_eq!(b2.count(), 1);
    assert!(b2.contains("p_p1"));
}

#[test]
fn test_parameter_object_definition_inherit_from_builder_with_check() {
    // Why: Test that inherit_from_builder_with_check successfully inherits when there are no key conflicts.
    let b1 = ParameterObjectDefinitionBuilder::new("B1").with(
        ParameterKey::new("p_p1".into()).unwrap(),
        StringDefinition::new("D1"),
    );

    let result = ParameterObjectDefinitionBuilder::new("B2")
        .with(
            ParameterKey::new("p_p2".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit_from_builder_with_check(b1);

    assert!(matches!(result, Ok(_)));
}

#[test]
fn test_parameter_object_definition_inherit_from_builder_with_check_error() {
    // Why: Test that inherit_from_builder_with_check returns an error when there is a key conflict.
    let b1 = ParameterObjectDefinitionBuilder::new("B1").with(
        ParameterKey::new("p_p1".into()).unwrap(),
        StringDefinition::new("D1"),
    );

    let result = ParameterObjectDefinitionBuilder::new("B2")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit_from_builder_with_check(b1);

    assert!(matches!(result, Err(StoreError::KeyConflict(_))));
}

#[test]
fn test_parameter_object_definition_getters() {
    // Why: Test that parameter object definition getters correctly return the expected values and iterators.
    let def = ParameterObjectDefinitionBuilder::new("Test")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    assert_eq!(def.description().as_str(), "Test");
    assert_eq!(def.description_ref().as_str(), "Test");
    assert_eq!(def.count(), 1);
    assert!(def.contains("p_p1"));
    assert!(def.contains_str("p_p1"));
    assert!(def.get("p_p1").is_some());
    assert!(def.get_str("p_p1").is_some());
    assert!(def.get("p2").is_none());

    let keys: Vec<_> = def.keys().collect();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].as_str(), "p_p1");

    let iter_items: Vec<_> = def.iter().collect();
    assert_eq!(iter_items.len(), 1);
    assert_eq!(iter_items[0].0.as_str(), "p_p1");
}

#[test]
fn test_parameter_object_definition_launder() {
    // Why: Test that laundering a parameter object definition correctly transfers strings to a new store.
    let store = SharedStringStore::new();
    let def = ParameterObjectDefinitionBuilder::new("Test")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .with(
            ParameterKey::new("p_p2".into()).unwrap(),
            FileDefinition::new("D2", "ext", false),
        )
        .with(
            ParameterKey::new("p_p3".into()).unwrap(),
            NumberDefinition::new("D3"),
        )
        .with(
            ParameterKey::new("p_p4".into()).unwrap(),
            ChoiceDefinition::new(
                "D4",
                vec![
                    ChoiceItemDefinition::new(store_key!("option_1"), "Option 1"),
                    ChoiceItemDefinition::new(store_key!("option_2"), "Option 2"),
                ],
            ),
        )
        .with(
            ParameterKey::new("p_p5".into()).unwrap(),
            TableDefinition::new(
                "D5",
                vec![
                    (store_key!("col1"), NumberDefinition::new("C1")),
                    (store_key!("col2"), NumberDefinition::new("C2")),
                ],
            ),
        )
        .with(
            ParameterKey::new("p_p6".into()).unwrap(),
            MapDefinition::new(
                "D6",
                StructDefinition::new(
                    "Item",
                    vec![
                        (
                            store_key!("col1"),
                            StructItemDefinition::String(StringDefinition::new("C1")),
                        ),
                        (
                            store_key!("col2"),
                            StructItemDefinition::Number(NumberDefinition::new("C2")),
                        ),
                    ],
                ),
            ),
        )
        .finish();

    let laundered = def.launder(&store);
    assert_eq!(laundered.description().as_str(), "Test");
    assert_eq!(laundered.count(), 6);
    assert!(laundered.contains("p_p1"));
    assert!(laundered.contains("p_p2"));
    assert!(laundered.contains("p_p3"));
    assert!(laundered.contains("p_p4"));
    assert!(laundered.contains("p_p5"));
    assert!(laundered.contains("p_p6"));

    assert!(store.contains("p_p1"));
    assert!(store.contains("p_p2"));
    assert!(store.contains("p_p3"));
    assert!(store.contains("p_p4"));
    assert!(store.contains("p_p5"));
    assert!(store.contains("p_p6"));
    assert!(store.contains("D1"));
    assert!(store.contains("D2"));
    assert!(store.contains("D3"));
    assert!(store.contains("D4"));
    assert!(store.contains("D5"));
    assert!(store.contains("D6"));
    assert!(store.contains("ext"));
    assert!(store.contains("option_1"));
    assert!(store.contains("option_2"));
    assert!(store.contains("Option 1"));
    assert!(store.contains("Option 2"));
    assert!(store.contains("col1"));
    assert!(store.contains("col2"));
    assert!(store.contains("C1"));
    assert!(store.contains("C2"));
    assert!(store.contains("Item"));
}
