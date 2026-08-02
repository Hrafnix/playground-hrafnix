use datastore::prelude::*;

#[test]
fn test_variable_object_builder_pattern() {
    // Why: Test variable object definition creation with the builder pattern using the with method.
    let obj_def = VariableObjectDefinition::builder("Test Object")
        .with(
            VariableKey::new("v_prop1".into()).unwrap(),
            StringDefinition::new("String prop"),
        )
        .with(
            VariableKey::new("v_prop2".into()).unwrap(),
            NumberDefinition::new_with_default("Number prop", "0"),
        )
        .finish();

    assert_eq!(obj_def.count(), 2);
}

#[test]
fn test_variable_object_inheritance() {
    // Why: Test that a variable object definition can inherit from another.
    let parent_def = VariableObjectDefinition::builder("Parent")
        .with(
            VariableKey::new("v_prop1".into()).unwrap(),
            StringDefinition::new_with_default("D1", "V1"),
        )
        .finish();

    let builder = parent_def.inherit("Child");
    assert_eq!(builder.finish().count(), 1);

    let mut builder = parent_def.inherit("Child");
    builder.insert(
        VariableKey::new("v_prop2".into()).unwrap(),
        StringDefinition::new_with_default("D2", "V2"),
    );

    let child_def = builder.finish();
    assert_eq!(child_def.count(), 2);
    assert!(child_def.contains("v_prop1"));
    assert!(child_def.contains("v_prop2"));

    let mut builder = child_def.inherit("Grandchild");
    builder.remove("v_prop1");
    let grandchild_def = builder.finish();
    assert_eq!(grandchild_def.count(), 1);
    assert!(!grandchild_def.contains("v_prop1"));
}

#[test]
fn test_variable_object_definition_immutability() {
    // Why: Test that the variable object definition is immutable once created.
    let obj_def = VariableObjectDefinition::builder("Test Object")
        .with(
            VariableKey::new("v_prop1".into()).unwrap(),
            StringDefinition::new("String prop"),
        )
        .finish();

    // The point of this test is that obj_def does NOT have .add() or .remove()
    // It is immutable by design.
    assert_eq!(obj_def.count(), 1);
    assert!(obj_def.contains("v_prop1"));
}

#[test]
fn test_variable_object_definition_builder_new() {
    // Why: Test that a new builder correctly initializes an empty variable object definition.
    let builder = VariableObjectDefinitionBuilder::new("Test Description");
    let def = builder.finish();
    assert_eq!(def.description().as_str(), "Test Description");
    assert_eq!(def.count(), 0);
}

#[test]
fn test_variable_object_definition_builder_insert() {
    // Why: Test that the builder correctly inserts items into the variable object definition.
    let mut builder = VariableObjectDefinitionBuilder::new("Test");
    let prop = StringDefinition::new("Desc");
    let key = VariableKey::new("v_key1".into()).unwrap();

    builder.insert(key.clone(), prop.clone());
    let def = builder.finish();

    assert_eq!(def.count(), 1);
    assert!(def.contains("v_key1"));
}

#[test]
fn test_variable_object_definition_builder_with_inserted() {
    // Why: Test that the builder correctly adds an item using the fluent interface.
    let prop = StringDefinition::new("Desc");
    let key = VariableKey::new("v_key1".into()).unwrap();

    let def = VariableObjectDefinitionBuilder::new("Test")
        .with(key, prop)
        .finish();

    assert_eq!(def.count(), 1);
    assert!(def.contains("v_key1"));
}

#[test]
fn test_variable_object_definition_builder_remove() {
    // Why: Test that the builder correctly removes items.
    let mut builder = VariableObjectDefinitionBuilder::new("Test");
    let prop = StringDefinition::new("Desc");
    let key = VariableKey::new("v_key1".into()).unwrap();

    builder.insert(key, prop);
    assert_eq!(builder.finish().count(), 1);

    let mut builder = VariableObjectDefinitionBuilder::new("Test");
    builder.insert(
        VariableKey::new("v_key1".into()).unwrap(),
        StringDefinition::new("Desc"),
    );
    builder.remove("v_key1");
    let def = builder.finish();

    assert_eq!(def.count(), 0);
    assert!(!def.contains("v_key1"));
}

#[test]
fn test_variable_object_definition_builder_without() {
    // Why: Test that the builder correctly removes an item using the fluent interface.
    let def = VariableObjectDefinitionBuilder::new("Test")
        .with(
            VariableKey::new("v_key1".into()).unwrap(),
            StringDefinition::new("Desc"),
        )
        .without("v_key1")
        .finish();

    assert_eq!(def.count(), 0);
}

#[test]
fn test_variable_object_definition_inherit() {
    // Why: Test that the builder correctly inherits from another variable object definition.
    let parent_def = VariableObjectDefinitionBuilder::new("Parent")
        .with(
            VariableKey::new("v_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    let child_def = VariableObjectDefinitionBuilder::new("Child")
        .inherit(&parent_def)
        .with(
            VariableKey::new("v_c1".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .finish();

    assert_eq!(child_def.count(), 2);
    assert!(child_def.contains("v_p1"));
    assert!(child_def.contains("v_c1"));

    let keys: Vec<_> = child_def
        .keys()
        .map(datastore::key::VariableKey::as_str)
        .collect();
    assert_eq!(keys[0], "v_p1");
    assert_eq!(keys[1], "v_c1");
}

#[test]
fn test_variable_object_definition_inherit_overwrite() {
    // Why: Test that inheriting from another variable object definition overwrites existing items with the same key.
    let parent_def = VariableObjectDefinitionBuilder::new("Parent")
        .with(
            VariableKey::new("v_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    let child_def = VariableObjectDefinitionBuilder::new("Child")
        .with(
            VariableKey::new("v_p1".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit(&parent_def)
        .finish();

    assert_eq!(child_def.count(), 1);
}

#[test]
fn test_variable_object_definition_inherit_with_check() {
    // Why: Test that inherit_with_check successfully inherits when there are no key conflicts.
    let parent_def = VariableObjectDefinitionBuilder::new("Parent")
        .with(
            VariableKey::new("v_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    let result = VariableObjectDefinitionBuilder::new("Child")
        .with(
            VariableKey::new("v_p2".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit_with_check(&parent_def);

    assert!(result.is_ok());

    let child_def_builder = result.unwrap();
    let child_def = child_def_builder.finish();

    let keys: Vec<_> = child_def
        .keys()
        .map(datastore::key::VariableKey::as_str)
        .collect();
    assert_eq!(keys[0], "v_p2");
    assert_eq!(keys[1], "v_p1");
}

#[test]
fn test_variable_object_definition_inherit_with_check_error() {
    // Why: Test that inherit_with_check returns an error when there is a key conflict.
    let parent_def = VariableObjectDefinitionBuilder::new("Parent")
        .with(
            VariableKey::new("v_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    let result = VariableObjectDefinitionBuilder::new("Child")
        .with(
            VariableKey::new("v_p1".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit_with_check(&parent_def);

    assert!(matches!(result, Err(StoreError::KeyConflict(_))));
}

#[test]
fn test_variable_object_definition_inherit_from_builder() {
    // Why: Test that the builder correctly inherits from another builder.
    let b1 = VariableObjectDefinitionBuilder::new("B1").with(
        VariableKey::new("v_v1".into()).unwrap(),
        StringDefinition::new("D1"),
    );

    let b2 = VariableObjectDefinitionBuilder::new("B2")
        .inherit_from_builder(b1)
        .finish();

    assert_eq!(b2.count(), 1);
    assert!(b2.contains("v_v1"));
}

#[test]
fn test_variable_object_definition_inherit_from_builder_with_check() {
    // Why: Test that inherit_from_builder_with_check successfully inherits when there are no key conflicts.
    let b1 = VariableObjectDefinitionBuilder::new("B1").with(
        VariableKey::new("v_p1".into()).unwrap(),
        StringDefinition::new("D1"),
    );

    let result = VariableObjectDefinitionBuilder::new("B2")
        .with(
            VariableKey::new("v_p2".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit_from_builder_with_check(b1);

    assert!(result.is_ok());
}

#[test]
fn test_variable_object_definition_inherit_from_builder_with_check_error() {
    // Why: Test that inherit_from_builder_with_check returns an error when there is a key conflict.
    let b1 = VariableObjectDefinitionBuilder::new("B1").with(
        VariableKey::new("v_p1".into()).unwrap(),
        StringDefinition::new("D1"),
    );

    let result = VariableObjectDefinitionBuilder::new("B2")
        .with(
            VariableKey::new("v_p1".into()).unwrap(),
            StringDefinition::new("D2"),
        )
        .inherit_from_builder_with_check(b1);

    assert!(matches!(result, Err(StoreError::KeyConflict(_))));
}

#[test]
fn test_variable_object_definition_getters() {
    // Why: Test that variable object definition getters correctly return the expected values and iterators.
    let def = VariableObjectDefinitionBuilder::new("Test")
        .with(
            VariableKey::new("v_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();

    assert_eq!(def.description().as_str(), "Test");
    assert_eq!(def.description_ref().as_str(), "Test");
    assert_eq!(def.count(), 1);
    assert!(def.contains("v_p1"));
    assert!(def.contains_str("v_p1"));
    assert!(def.get("v_p1").is_some());
    assert!(def.get_str("v_p1").is_some());
    assert!(def.get("v2").is_none());

    let keys: Vec<_> = def.keys().collect();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].as_str(), "v_p1");

    let iter_items: Vec<_> = def.iter().collect();
    assert_eq!(iter_items.len(), 1);
    assert_eq!(iter_items[0].0.as_str(), "v_p1");
}

#[test]
fn test_variable_object_definition_launder() {
    // Why: Test that laundering a variable object definition correctly transfers strings to a new store.
    let store = SharedStringStore::new();
    let def = VariableObjectDefinitionBuilder::new("Test")
        .with(
            VariableKey::new("v_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .with(
            VariableKey::new("v_p2".into()).unwrap(),
            BooleanDefinition::new("D2"),
        )
        .with(
            VariableKey::new("v_p3".into()).unwrap(),
            FileDefinition::new("D3", "ext", false),
        )
        .with(
            VariableKey::new("v_p4".into()).unwrap(),
            IntegerDefinition::new("D4"),
        )
        .with(
            VariableKey::new("v_p5".into()).unwrap(),
            NumberDefinition::new("D5"),
        )
        .with(
            VariableKey::new("v_p6".into()).unwrap(),
            ChoiceDefinition::new(
                "D6",
                vec![
                    ChoiceItemDefinition::new(store_key!("option_1"), "Option 1"),
                    ChoiceItemDefinition::new(store_key!("option_2"), "Option 2"),
                ],
            ),
        )
        .with(
            VariableKey::new("v_p7".into()).unwrap(),
            TableDefinition::new(
                "D7",
                vec![
                    (store_key!("col1"), NumberDefinition::new("C1")),
                    (store_key!("col2"), NumberDefinition::new("C2")),
                ],
            ),
        )
        .with(
            VariableKey::new("v_p8".into()).unwrap(),
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
    assert!(laundered.contains("v_p1"));
    assert!(laundered.contains("v_p2"));
    assert!(laundered.contains("v_p3"));
    assert!(laundered.contains("v_p4"));
    assert!(laundered.contains("v_p5"));
    assert!(laundered.contains("v_p6"));
    assert!(laundered.contains("v_p7"));
    assert!(laundered.contains("v_p8"));

    assert!(store.contains("v_p1"));
    assert!(store.contains("v_p2"));
    assert!(store.contains("v_p3"));
    assert!(store.contains("v_p4"));
    assert!(store.contains("v_p5"));
    assert!(store.contains("v_p6"));
    assert!(store.contains("v_p7"));
    assert!(store.contains("v_p8"));
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
