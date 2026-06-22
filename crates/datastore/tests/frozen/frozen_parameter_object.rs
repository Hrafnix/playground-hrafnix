use datastore::definition::{BasicDefinition, ItemDefinition, ParameterObjectDefinition};
use datastore::frozen::frozen_object_parameter::ParameterObjectFrozen;
use datastore::key::ParameterKey;

#[test]
fn test_parameter_object_definition_basic() {
    // Why: Test frozen parameter object creation and items.
    let frozen_1 = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                ParameterKey::new("p_p1".into()).unwrap(),
                ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
            )
            .finish(),
    );

    assert_eq!(frozen_1.definition().description().as_ref(), "Test Object");
    assert_eq!(frozen_1.definition().count(), 1);
    assert!(frozen_1.definition().contains("p_p1"));
    assert!(frozen_1.definition().contains_str("p_p1"));
    assert_ne!(frozen_1.hash(), [0u8; 32]);
}

#[test]
fn test_parameter_object_definition_equality() {
    // Why: Test that two frozen parameter object definitions with the same items are considered equal.
    let frozen_1 = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                ParameterKey::new("p_p1".into()).unwrap(),
                ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
            )
            .finish(),
    );
    let frozen_2 = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                ParameterKey::new("p_p1".into()).unwrap(),
                ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
            )
            .finish(),
    );
    let frozen_3 = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                ParameterKey::new("p_p1".into()).unwrap(),
                ItemDefinition::new("P1", BasicDefinition::new_string("D2")),
            )
            .finish(),
    );

    assert_eq!(frozen_1, frozen_2);
    assert_ne!(frozen_1, frozen_3);
    assert_eq!(&frozen_1, frozen_2);
    assert_ne!(frozen_1, &frozen_3);
}
