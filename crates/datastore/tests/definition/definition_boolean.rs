use datastore::prelude::*;

#[test]
fn test_definition_boolean() {
    // Why: Test boolean definition creation and definition.
    let def_1 = BooleanDefinition::new("A Boolean parameter");

    // Check the various data items of the boolean definition.
    assert_eq!(def_1.description(), "A Boolean parameter");
    assert_eq!(def_1.description_ref(), "A Boolean parameter");
    assert_eq!(def_1.default_value(), "");
    assert_eq!(def_1.default_value_ref(), "");
    assert_eq!(def_1.ids(), vec!["true", "false"]);
    assert_eq!(def_1.descriptions(), vec!["True", "False"]);
}

#[test]
fn test_definition_boolean_with_default() {
    // Why: Test boolean definition creation with a default value.
    let def = BooleanDefinition::new_with_default("A Default Boolean parameter", true);

    // Check the various data items of the boolean definition.
    assert_eq!(def.description(), "A Default Boolean parameter");
    assert_eq!(def.description_ref(), "A Default Boolean parameter");
    assert_eq!(def.default_value(), "true");
    assert_eq!(def.default_value_ref(), "true");
    assert_eq!(def.ids(), vec!["true", "false"]);
    assert_eq!(def.descriptions(), vec!["True", "False"]);
}

#[test]
fn test_definition_boolean_equality() {
    // Why: Test boolean definition equality.
    let def_1 = BooleanDefinition::new_with_default("A Default Boolean parameter", true);
    let def_2 = BooleanDefinition::new_with_default("A Default Boolean parameter", true);
    let def_3 = BooleanDefinition::new_with_default("A Default Boolean parameter", false);

    // Check equality of the three boolean definitions.
    assert_eq!(def_1, def_2);
    assert_eq!(def_1, &def_2);
    assert_eq!(&def_1, def_2);
    assert_eq!(&def_1, &def_2);

    assert_ne!(def_1, def_3);
    assert_ne!(&def_1, def_3);
    assert_ne!(def_1, &def_3);
    assert_ne!(&def_1, &def_3);
}
