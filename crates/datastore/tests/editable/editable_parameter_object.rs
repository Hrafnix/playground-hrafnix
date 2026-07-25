use datastore::prelude::*;

#[test]
fn test_editable_parameter_object_roundtrip() {
    // Why: Editable parameter objects should thaw from frozen, allow item edits, and freeze back
    // to an equivalent frozen object reflecting those edits.
    let frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                ParameterKey::new("p_p1".into()).unwrap(),
                StringDefinition::new("D1"),
            )
            .finish(),
    );

    let mut editable = frozen.thaw();
    assert_eq!(editable.definition().description().as_ref(), "Test Object");

    let item = editable.get("p_p1").expect("p_p1 item");
    assert_eq!(item.get_string().unwrap().value(), "");

    if let ItemEditable::String(string_editable) = editable.get_mut("p_p1").expect("p_p1 item") {
        string_editable.set("edited");
    }

    let frozen_2 = editable.freeze();
    assert_ne!(frozen_2.hash(), frozen.hash());
    assert_eq!(
        frozen_2.get("p_p1").unwrap().get_string().unwrap().value(),
        "edited"
    );
}

#[test]
fn test_editable_parameter_object_equality() {
    // Why: Two editable parameter objects thawed from the same frozen object should be equal.
    let frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                ParameterKey::new("p_p1".into()).unwrap(),
                StringDefinition::new("D1"),
            )
            .finish(),
    );

    let editable_1 = frozen.thaw();
    let editable_2 = frozen.thaw();
    assert_eq!(editable_1, editable_2);
    assert_eq!(&editable_1, editable_2);
}
