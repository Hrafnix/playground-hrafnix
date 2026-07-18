use datastore::prelude::*;

#[test]
fn test_object_definition_basic() {
    // Why: Test object definition creation and parameter.
    let mut builder = ObjectDefinition::builder("Test Object");
    builder.insert_parameter(
        ParameterKey::new("p_prop1".into()).unwrap(),
        ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
    );
    let obj_def = builder.finish();

    assert_eq!(obj_def.description().as_ref(), "Test Object");
    assert_eq!(obj_def.parameter_count(), 1);
    assert!(obj_def.parameter_contains_key(store_key!("p_prop1")));
    assert!(obj_def.parameter_contains_key_str("p_prop1"));
}

#[test]
fn test_object_definition_equality() {
    // Why: Test that two object definitions with the same parameter are considered equal and ref equal.
    let def_1 = ObjectDefinition::builder("Test Object")
        .with_parameter_inserted(
            ParameterKey::new("p_prop1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .finish();
    let def_2 = ObjectDefinition::builder("Test Object")
        .with_parameter_inserted(
            ParameterKey::new("p_prop1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .finish();
    let def_3 = ObjectDefinition::builder("Test Object")
        .with_parameter_inserted(
            ParameterKey::new("p_prop1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D2")),
        )
        .finish();

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(&def_1, def_2);
    assert_ne!(def_1, &def_3);
}
