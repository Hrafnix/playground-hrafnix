use datastore::prelude::*;

#[test]
fn test_object_builder_pattern() {
    // Why: Test object creation with the builder pattern using with_inserted parameter.
    let obj_def = GlobalObjectDefinition::builder("Test Object")
        .with(
            GlobalKey::new("g_prop1".into()).unwrap(),
            StringDefinition::new("String prop"),
        )
        .with(
            GlobalKey::new("g_prop2".into()).unwrap(),
            NumberDefinition::new_with_default("Number prop", "0"),
        )
        .finish();

    // Check that the object definition has the expected number of parameters.
    assert_eq!(obj_def.count(), 2);
}

#[test]
fn test_object_inheritance() {
    // Why: Test that an object definition can inherit from another.
    let parent_def = GlobalObjectDefinition::builder("Parent")
        .with(
            GlobalKey::new("g_prop1".into()).unwrap(),
            StringDefinition::new_with_default("D1", "V1"),
        )
        .finish();

    let builder = parent_def.inherit("Child");
    assert_eq!(builder.finish().count(), 1);

    let mut builder = parent_def.inherit("Child");
    builder.insert(
        GlobalKey::new("g_prop2".into()).unwrap(),
        StringDefinition::new_with_default("D2", "V2"),
    );

    let child_def = builder.finish();
    assert_eq!(child_def.count(), 2);
    assert!(child_def.contains("g_prop1"));
    assert!(child_def.contains("g_prop2"));

    let mut builder = child_def.inherit("Grandchild");
    builder.remove("g_prop1");
    let grandchild_def = builder.finish();
    assert_eq!(grandchild_def.count(), 1);
    assert!(!grandchild_def.contains("g_prop1"));
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
    let obj_def = GlobalObjectDefinition::builder("Test Object")
        .with(
            GlobalKey::new("g_prop1".into()).unwrap(),
            StringDefinition::new("String prop"),
        )
        .finish();

    // The point of this test is that obj_def does NOT have .add() or .remove()
    // It is immutable by design.
    assert_eq!(obj_def.count(), 1);
    assert!(obj_def.contains("g_prop1"));
}

#[test]
fn test_object_definition_builder_new() {
    // Why: Test that a new builder correctly initializes an empty object definition.
    let builder = GlobalObjectDefinitionBuilder::new("Test Description");
    let def = builder.finish();
    assert_eq!(def.description().as_str(), "Test Description");
    assert_eq!(def.count(), 0);
}

#[test]
fn test_object_definition_builder_insert() {
    // Why: Test that the builder correctly inserts items into the object definition.
    let mut builder = GlobalObjectDefinitionBuilder::new("Test");
    let prop = StringDefinition::new("Desc");
    let key = GlobalKey::new("g_key1".into()).unwrap();

    builder.insert(key.clone(), prop.clone());
    let def = builder.finish();

    assert_eq!(def.count(), 1);
    assert!(def.contains("g_key1"));
}

#[test]
fn test_object_definition_builder_with_inserted() {
    // Why: Test that the builder correctly adds an item using the fluent interface.
    let prop = StringDefinition::new("Desc");
    let key = GlobalKey::new("g_key1".into()).unwrap();

    let def = GlobalObjectDefinitionBuilder::new("Test")
        .with(key, prop)
        .finish();

    assert_eq!(def.count(), 1);
    assert!(def.contains("g_key1"));
}

#[test]
fn test_object_definition_builder_remove() {
    // Why: Test that the builder correctly removes items.
    let mut builder = GlobalObjectDefinitionBuilder::new("Test");
    let prop = StringDefinition::new("Desc");
    let key = GlobalKey::new("g_key1".into()).unwrap();

    builder.insert(key, prop);
    assert_eq!(builder.finish().count(), 1);

    let mut builder = GlobalObjectDefinitionBuilder::new("Test");
    builder.insert(
        GlobalKey::new("g_key1".into()).unwrap(),
        StringDefinition::new("Desc"),
    );
    builder.remove("g_key1");
    let def = builder.finish();

    assert_eq!(def.count(), 0);
    assert!(!def.contains("g_key1"));
}

#[test]
fn test_object_definition_builder_without() {
    // Why: Test that the builder correctly removes an item using the fluent interface.
    let def = GlobalObjectDefinitionBuilder::new("Test")
        .with(
            GlobalKey::new("g_key1".into()).unwrap(),
            StringDefinition::new("Desc"),
        )
        .without("g_key1")
        .finish();

    assert_eq!(def.count(), 0);
}

#[test]
fn test_object_definition_inherit() {
    // Why: Test that the builder correctly inherits from another object definition.
    let parent_def = GlobalObjectDefinitionBuilder::new("Parent")
        .with(
            GlobalKey::new("g_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    let child_def = GlobalObjectDefinitionBuilder::new("Child")
        .inherit(&parent_def)
        .with(
            GlobalKey::new("g_c1".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .finish();

    assert_eq!(child_def.count(), 2);
    assert!(child_def.contains("g_p1"));
    assert!(child_def.contains("g_c1"));

    let keys: Vec<_> = child_def.keys().map(|key| key.as_str()).collect();
    assert_eq!(keys[0], "g_p1");
    assert_eq!(keys[1], "g_c1");
}

#[test]
fn test_object_definition_inherit_overwrite() {
    // Why: Test that inheriting from another object definition overwrites existing items with the same key.
    let parent_def = GlobalObjectDefinitionBuilder::new("Parent")
        .with(
            GlobalKey::new("g_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    let child_def = GlobalObjectDefinitionBuilder::new("Child")
        .with(
            GlobalKey::new("g_p1".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit(&parent_def)
        .finish();

    assert_eq!(child_def.count(), 1);
}

#[test]
fn test_object_definition_inherit_with_check() {
    // Why: Test that try_inherit successfully inherits when there are no key conflicts.
    let parent_def = GlobalObjectDefinitionBuilder::new("Parent")
        .with(
            GlobalKey::new("g_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    let result = GlobalObjectDefinitionBuilder::new("Child")
        .with(
            GlobalKey::new("g_p2".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit_with_check(&parent_def);

    assert!(matches!(result, Ok(_)));

    let child_def_builder = result.unwrap();
    let child_def = child_def_builder.finish();

    let keys: Vec<_> = child_def.keys().map(|key| key.as_str()).collect();
    assert_eq!(keys[0], "g_p2");
    assert_eq!(keys[1], "g_p1");
}

#[test]
fn test_object_definition_inherit_with_check_error() {
    // Why: Test that try_inherit returns an error when there is a key conflict.
    let parent_def = GlobalObjectDefinitionBuilder::new("Parent")
        .with(
            GlobalKey::new("g_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    let result = GlobalObjectDefinitionBuilder::new("Child")
        .with(
            GlobalKey::new("g_p1".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit_with_check(&parent_def);

    assert!(matches!(result, Err(StoreError::KeyConflict(_))));
}

#[test]
fn test_object_definition_inherit_from_builder() {
    // Why: Test that the builder correctly inherits from another builder.
    let b1 = GlobalObjectDefinitionBuilder::new("B1").with(
        GlobalKey::new("g_p1".into()).unwrap(),
        StringDefinition::new("D1"),
    );

    let b2 = GlobalObjectDefinitionBuilder::new("B2")
        .inherit_from_builder(b1)
        .finish();

    assert_eq!(b2.count(), 1);
    assert!(b2.contains("g_p1"));
}

#[test]
fn test_object_definition_inherit_from_builder_with_check() {
    // Why: Test that try_inherit_from_builder successfully inherits when there are no key conflicts.
    let b1 = GlobalObjectDefinitionBuilder::new("B1").with(
        GlobalKey::new("g_p1".into()).unwrap(),
        StringDefinition::new("D1"),
    );

    let result = GlobalObjectDefinitionBuilder::new("B2")
        .with(
            GlobalKey::new("g_p2".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit_from_builder_with_check(b1);

    assert!(matches!(result, Ok(_)));
}

#[test]
fn test_object_definition_inherit_from_builder_with_check_error() {
    // Why: Test that try_inherit_from_builder returns an error when there is a key conflict.
    let b1 = GlobalObjectDefinitionBuilder::new("B1").with(
        GlobalKey::new("g_p1".into()).unwrap(),
        StringDefinition::new("D1"),
    );

    let result = GlobalObjectDefinitionBuilder::new("B2")
        .with(
            GlobalKey::new("g_p1".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit_from_builder_with_check(b1);

    assert!(matches!(result, Err(StoreError::KeyConflict(_))));
}

#[test]
fn test_object_definition_getters() {
    // Why: Test that object definition getters correctly return the expected values and iterators.
    let def = GlobalObjectDefinitionBuilder::new("Test")
        .with(
            GlobalKey::new("g_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    assert_eq!(def.description().as_str(), "Test");
    assert_eq!(def.description_ref().as_str(), "Test");
    assert_eq!(def.count(), 1);
    assert!(def.contains("g_p1"));
    assert!(def.contains_str("g_p1"));
    assert!(def.get("g_p1").is_some());
    assert!(def.get_str("g_p1").is_some());
    assert!(def.get("g_p2").is_none());

    let keys: Vec<_> = def.keys().collect();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].as_str(), "g_p1");

    let iter_items: Vec<_> = def.iter().collect();
    assert_eq!(iter_items.len(), 1);
    assert_eq!(iter_items[0].0.as_str(), "g_p1");
}

#[test]
fn test_object_definition_launder() {
    // Why: Test that laundering an object definition correctly transfers strings to a new store.
    let store = SharedStringStore::new();
    let def = GlobalObjectDefinitionBuilder::new("Test")
        .with(
            GlobalKey::new("g_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .with(
            GlobalKey::new("g_p2".into()).unwrap(),
            BooleanDefinition::new("D2"),
        )
        .with(
            GlobalKey::new("g_p3".into()).unwrap(),
            FileDefinition::new("D3", "ext", false),
        )
        .with(
            GlobalKey::new("g_p4".into()).unwrap(),
            IntegerDefinition::new("D4"),
        )
        .with(
            GlobalKey::new("g_p5".into()).unwrap(),
            NumberDefinition::new("D5"),
        )
        .with(
            GlobalKey::new("g_p6".into()).unwrap(),
            ChoiceDefinition::new(
                "D6",
                vec![
                    ChoiceItemDefinition::new(store_key!("option_1"), "Option 1"),
                    ChoiceItemDefinition::new(store_key!("option_2"), "Option 2"),
                ],
            ),
        )
        .with(
            GlobalKey::new("g_p7".into()).unwrap(),
            TableDefinition::new(
                "D7",
                vec![
                    (store_key!("col1"), NumberDefinition::new("C1")),
                    (store_key!("col2"), NumberDefinition::new("C2")),
                ],
            ),
        )
        .with(
            GlobalKey::new("g_p8".into()).unwrap(),
            MapDefinition::new(
                "D8",
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
    assert_eq!(laundered.count(), 8);
    assert!(laundered.contains("g_p1"));
    assert!(laundered.contains("g_p2"));
    assert!(laundered.contains("g_p3"));
    assert!(laundered.contains("g_p4"));
    assert!(laundered.contains("g_p5"));
    assert!(laundered.contains("g_p6"));
    assert!(laundered.contains("g_p7"));
    assert!(laundered.contains("g_p8"));

    assert!(store.contains("g_p1"));
    assert!(store.contains("g_p2"));
    assert!(store.contains("g_p3"));
    assert!(store.contains("g_p4"));
    assert!(store.contains("g_p5"));
    assert!(store.contains("g_p6"));
    assert!(store.contains("g_p7"));
    assert!(store.contains("g_p8"));
    assert!(store.contains("D1"));
    assert!(store.contains("D2"));
    assert!(store.contains("D3"));
    assert!(store.contains("D4"));
    assert!(store.contains("D5"));
    assert!(store.contains("D6"));
    assert!(store.contains("D7"));
    assert!(store.contains("D8"));
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
