use datastore::definition::{BasicDefinition, ItemDefinition, VariableObjectDefinition};
use datastore::frozen::frozen_object_variable::VariableObjectFrozen;
use datastore::key::VariableKey;

#[test]
fn test_variable_object_definition_basic() {
    // Why: Test frozen variable object creation and items.
    let frozen_1 = VariableObjectFrozen::new(
        VariableObjectDefinition::builder("Test Object")
            .with(
                VariableKey::new("v_v1".into()).unwrap(),
                ItemDefinition::new("V1", BasicDefinition::new_string("D1")),
            )
            .finish(),
    );

    assert_eq!(frozen_1.definition().description().as_ref(), "Test Object");
    assert_eq!(frozen_1.definition().count(), 1);
    assert!(frozen_1.definition().contains("v_v1"));
    assert!(frozen_1.definition().contains_str("v_v1"));
    assert_ne!(frozen_1.hash(), [0u8; 32]);
}

#[test]
fn test_variable_object_definition_equality() {
    // Why: Test that two frozen variable objects with the same items are considered equal.
    let frozen_1 = VariableObjectFrozen::new(
        VariableObjectDefinition::builder("Test Object")
            .with(
                VariableKey::new("v_v1".into()).unwrap(),
                ItemDefinition::new("V1", BasicDefinition::new_string("D1")),
            )
            .finish(),
    );
    let frozen_2 = VariableObjectFrozen::new(
        VariableObjectDefinition::builder("Test Object")
            .with(
                VariableKey::new("v_v1".into()).unwrap(),
                ItemDefinition::new("V1", BasicDefinition::new_string("D1")),
            )
            .finish(),
    );
    let frozen_3 = VariableObjectFrozen::new(
        VariableObjectDefinition::builder("Test Object")
            .with(
                VariableKey::new("v_v1".into()).unwrap(),
                ItemDefinition::new("V1", BasicDefinition::new_string("D2")),
            )
            .finish(),
    );

    assert_eq!(frozen_1, frozen_2);
    assert_ne!(frozen_1, frozen_3);
    assert_eq!(&frozen_1, frozen_2);
    assert_ne!(frozen_1, &frozen_3);
}
