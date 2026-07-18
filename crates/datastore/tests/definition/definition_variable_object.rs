use datastore::definition::{BasicDefinition, ItemDefinition, VariableObjectDefinition};
use datastore::key::VariableKey;

#[test]
fn test_variable_object_definition_basic() {
    // Why: Test variable object definition creation and items.
    let mut builder = VariableObjectDefinition::builder("Test Object");
    builder.insert(
        VariableKey::new("v_v1".into()).unwrap(),
        ItemDefinition::new("V1", BasicDefinition::new_string("D1")),
    );
    let obj_def = builder.finish();

    assert_eq!(obj_def.description().as_ref(), "Test Object");
    assert_eq!(obj_def.count(), 1);
    assert!(obj_def.contains("v_v1"));
    assert!(obj_def.contains_str("v_v1"));
}

#[test]
fn test_variable_object_definition_equality() {
    // Why: Test that two variable object definitions with the same items are considered equal.
    let def_1 = VariableObjectDefinition::builder("Test Object")
        .with(
            VariableKey::new("v_v1".into()).unwrap(),
            ItemDefinition::new("V1", BasicDefinition::new_string("D1")),
        )
        .finish();
    let def_2 = VariableObjectDefinition::builder("Test Object")
        .with(
            VariableKey::new("v_v1".into()).unwrap(),
            ItemDefinition::new("V1", BasicDefinition::new_string("D1")),
        )
        .finish();
    let def_3 = VariableObjectDefinition::builder("Test Object")
        .with(
            VariableKey::new("v_v1".into()).unwrap(),
            ItemDefinition::new("V1", BasicDefinition::new_string("D2")),
        )
        .finish();

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(&def_1, def_2);
    assert_ne!(def_1, &def_3);
}
