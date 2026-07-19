use datastore::prelude::*;

#[test]
fn test_object_builder_pattern() {
    // Why: Test object creation with the builder pattern using with_inserted parameter.
    let obj_def = ObjectDefinition::builder("Test Object")
        .with(
            StoreKey::new("prop1".into()).unwrap(),
            StringDefinition::new("String prop"),
        )
        .with(
            StoreKey::new("prop2".into()).unwrap(),
            NumberDefinition::new_with_default("Number prop", "0"),
        )
        .finish();

    // Check that the object definition has the expected number of parameters.
    assert_eq!(obj_def.count(), 2);
}

#[test]
fn test_object_inheritance() {
    // Why: Test that an object definition can inherit from another.
    let parent_def = ObjectDefinition::builder("Parent")
        .with(
            StoreKey::new("prop1".into()).unwrap(),
            StringDefinition::new_with_default("D1", "V1"),
        )
        .finish();

    let builder = parent_def.inherit("Child");
    assert_eq!(builder.finish().count(), 1);

    let mut builder = parent_def.inherit("Child");
    builder.insert(
        StoreKey::new("prop2".into()).unwrap(),
        StringDefinition::new_with_default("D2", "V2"),
    );

    let child_def = builder.finish();
    assert_eq!(child_def.count(), 2);
    assert!(child_def.contains("prop1"));
    assert!(child_def.contains("prop2"));

    let mut builder = child_def.inherit("Grandchild");
    builder.remove("prop1");
    let grandchild_def = builder.finish();
    assert_eq!(grandchild_def.count(), 1);
    assert!(!grandchild_def.contains("prop1"));
}

#[test]
fn test_invalid_keys() {
    // Why: Test that invalid keys are correctly rejected.
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
    // Why: Test that the object definition is immutable once created.
    let obj_def = ObjectDefinition::builder("Test Object")
        .with(
            StoreKey::new("prop1".into()).unwrap(),
            StringDefinition::new("String prop"),
        )
        .finish();

    // The point of this test is that obj_def does NOT have .add() or .remove()
    // It is immutable by design.
    assert_eq!(obj_def.count(), 1);
    assert!(obj_def.contains("prop1"));
}

#[test]
fn test_object_definition_builder_new() {
    // Why: Test that a new builder correctly initializes an empty object definition.
    let builder = ObjectDefinitionBuilder::new("Test Description");
    let def = builder.finish();
    assert_eq!(def.description().as_str(), "Test Description");
    assert_eq!(def.count(), 0);
}

#[test]
fn test_object_definition_builder_insert() {
    // Why: Test that the builder correctly inserts items into the object definition.
    let mut builder = ObjectDefinitionBuilder::new("Test");
    let prop = StringDefinition::new("Desc");
    let key = StoreKey::new("key1".into()).unwrap();

    builder.insert(key.clone(), prop.clone());
    let def = builder.finish();

    assert_eq!(def.count(), 1);
    assert!(def.contains("key1"));
}

#[test]
fn test_object_definition_builder_with_inserted() {
    // Why: Test that the builder correctly adds an item using the fluent interface.
    let prop = StringDefinition::new("Desc");
    let key = StoreKey::new("key1".into()).unwrap();

    let def = ObjectDefinitionBuilder::new("Test")
        .with(key, prop)
        .finish();

    assert_eq!(def.count(), 1);
    assert!(def.contains("key1"));
}

#[test]
fn test_object_definition_builder_remove() {
    // Why: Test that the builder correctly removes items.
    let mut builder = ObjectDefinitionBuilder::new("Test");
    let prop = StringDefinition::new("Desc");
    let key = StoreKey::new("key1".into()).unwrap();

    builder.insert(key, prop);
    assert_eq!(builder.finish().count(), 1);

    let mut builder = ObjectDefinitionBuilder::new("Test");
    builder.insert(
        StoreKey::new("key1".into()).unwrap(),
        StringDefinition::new("Desc"),
    );
    builder.remove("key1");
    let def = builder.finish();

    assert_eq!(def.count(), 0);
    assert!(!def.contains("key1"));
}

#[test]
fn test_object_definition_builder_without() {
    // Why: Test that the builder correctly removes an item using the fluent interface.
    let def = ObjectDefinitionBuilder::new("Test")
        .with(
            StoreKey::new("key1".into()).unwrap(),
            StringDefinition::new("Desc"),
        )
        .without("key1")
        .finish();

    assert_eq!(def.count(), 0);
}

#[test]
fn test_object_definition_inherit() {
    // Why: Test that the builder correctly inherits from another object definition.
    let parent_def = ObjectDefinitionBuilder::new("Parent")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    let child_def = ObjectDefinitionBuilder::new("Child")
        .inherit(parent_def)
        .with(
            StoreKey::new("c1".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .finish();

    assert_eq!(child_def.count(), 2);
    assert!(child_def.contains("p1"));
    assert!(child_def.contains("c1"));
}

#[test]
fn test_object_definition_inherit_overwrite() {
    // Why: Test that inheriting from another object definition overwrites existing items with the same key.
    let parent_def = ObjectDefinitionBuilder::new("Parent")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    let child_def = ObjectDefinitionBuilder::new("Child")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit(parent_def)
        .finish();

    assert_eq!(child_def.count(), 1);
}

#[test]
fn test_object_definition_inherit_with_check() {
    // Why: Test that try_inherit successfully inherits when there are no key conflicts.
    let parent_def = ObjectDefinitionBuilder::new("Parent")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    let result = ObjectDefinitionBuilder::new("Child")
        .with(
            StoreKey::new("p2".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit_with_check(parent_def);

    assert!(matches!(result, Ok(_)));
}

#[test]
fn test_object_definition_inherit_with_check_error() {
    // Why: Test that try_inherit returns an error when there is a key conflict.
    let parent_def = ObjectDefinitionBuilder::new("Parent")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    let result = ObjectDefinitionBuilder::new("Child")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit_with_check(parent_def);

    assert!(matches!(result, Err(StoreError::KeyConflict(_))));
}

#[test]
fn test_object_definition_inherit_from_builder() {
    // Why: Test that the builder correctly inherits from another builder.
    let b1 = ObjectDefinitionBuilder::new("B1").with(
        StoreKey::new("p1".into()).unwrap(),
        StringDefinition::new("D1"),
    );

    let b2 = ObjectDefinitionBuilder::new("B2")
        .inherit_from_builder(b1)
        .finish();

    assert_eq!(b2.count(), 1);
    assert!(b2.contains("p1"));
}

#[test]
fn test_object_definition_inherit_from_builder_with_check() {
    // Why: Test that try_inherit_from_builder successfully inherits when there are no key conflicts.
    let b1 = ObjectDefinitionBuilder::new("B1").with(
        StoreKey::new("p1".into()).unwrap(),
        StringDefinition::new("D1"),
    );

    let result = ObjectDefinitionBuilder::new("B2")
        .with(
            StoreKey::new("p2".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit_from_builder_with_check(b1);

    assert!(matches!(result, Ok(_)));
}

#[test]
fn test_object_definition_inherit_from_builder_with_check_error() {
    // Why: Test that try_inherit_from_builder returns an error when there is a key conflict.
    let b1 = ObjectDefinitionBuilder::new("B1").with(
        StoreKey::new("p1".into()).unwrap(),
        StringDefinition::new("D1"),
    );

    let result = ObjectDefinitionBuilder::new("B2")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit_from_builder_with_check(b1);

    assert!(matches!(result, Err(StoreError::KeyConflict(_))));
}

#[test]
fn test_object_definition_getters() {
    // Why: Test that object definition getters correctly return the expected values and iterators.
    let def = ObjectDefinitionBuilder::new("Test")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    assert_eq!(def.description().as_str(), "Test");
    assert_eq!(def.description_ref().as_str(), "Test");
    assert_eq!(def.count(), 1);
    assert!(def.contains("p1"));
    assert!(def.contains_str("p1"));
    assert!(def.get("p1").is_some());
    assert!(def.get_str("p1").is_some());
    assert!(def.get("p2").is_none());

    let keys: Vec<_> = def.keys().collect();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].as_str(), "p1");

    let iter_items: Vec<_> = def.iter().collect();
    assert_eq!(iter_items.len(), 1);
    assert_eq!(iter_items[0].0.as_str(), "p1");
}

#[test]
fn test_object_definition_launder() {
    // Why: Test that laundering an object definition correctly transfers strings to a new store.
    let store = SharedStringStore::new();
    let def = ObjectDefinitionBuilder::new("Test")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .with(
            StoreKey::new("p2".into()).unwrap(),
            FileDefinition::new("D2", "ext", false),
        )
        .with(
            StoreKey::new("p3".into()).unwrap(),
            NumberDefinition::new("D3"),
        )
        .with(
            StoreKey::new("p4".into()).unwrap(),
            ChoiceDefinition::new(
                "D4",
                vec![
                    ChoiceItemDefinition::new(store_key!("option_1"), "Option 1"),
                    ChoiceItemDefinition::new(store_key!("option_2"), "Option 2"),
                ],
            ),
        )
        .with(
            StoreKey::new("p5".into()).unwrap(),
            TableDefinition::new(
                "D5",
                vec![
                    (store_key!("col1"), NumberDefinition::new("C1")),
                    (store_key!("col2"), NumberDefinition::new("C2")),
                ],
            ),
        )
        .with(
            StoreKey::new("p6".into()).unwrap(),
            MapDefinition::new(
                "D6",
                vec![
                    (
                        store_key!("col1"),
                        MapItemDefinition::String(StringDefinition::new("C1")),
                    ),
                    (
                        store_key!("col2"),
                        MapItemDefinition::Number(NumberDefinition::new("C2")),
                    ),
                ],
            ),
        )
        .finish();

    let laundered = def.launder(&store);
    assert_eq!(laundered.description().as_str(), "Test");
    assert_eq!(laundered.count(), 6);
    assert!(laundered.contains("p1"));
    assert!(laundered.contains("p2"));
    assert!(laundered.contains("p3"));
    assert!(laundered.contains("p4"));
    assert!(laundered.contains("p5"));
    assert!(laundered.contains("p6"));

    assert!(store.contains("p1"));
    assert!(store.contains("p2"));
    assert!(store.contains("p3"));
    assert!(store.contains("p4"));
    assert!(store.contains("p5"));
    assert!(store.contains("p6"));
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
}
