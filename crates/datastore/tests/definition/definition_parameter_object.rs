use datastore::definition::{BasicDefinition, ItemDefinition, ParameterObjectDefinition};
use datastore::key::ParameterKey;

#[test]
fn test_parameter_object_definition_basic() {
    // Why: Test parameter object definition creation and items.
    let mut builder = ParameterObjectDefinition::builder("Test Object");
    builder.insert(
        ParameterKey::new("p_p1".into()).unwrap(),
        ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
    );
    let obj_def = builder.finish();

    assert_eq!(obj_def.description().as_ref(), "Test Object");
    assert_eq!(obj_def.count(), 1);
    assert!(obj_def.contains("p_p1"));
    assert!(obj_def.contains_str("p_p1"));
}

#[test]
fn test_parameter_object_definition_equality() {
    // Why: Test that two parameter object definitions with the same items are considered equal.
    let def_1 = ParameterObjectDefinition::builder("Test Object")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .finish();
    let def_2 = ParameterObjectDefinition::builder("Test Object")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .finish();
    let def_3 = ParameterObjectDefinition::builder("Test Object")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D2")),
        )
        .finish();

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(&def_1, def_2);
    assert_ne!(def_1, &def_3);
}
