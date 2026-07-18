use datastore::prelude::*;

#[test]
fn test_object_variables() {
    let string_store = SharedStringStore::new();
    let store = Store::new(string_store);

    let mut builder = ObjectDefinition::builder("Test Object");
    builder.insert_parameter(
        ParameterKey::new(ShareableString::from("p_prop1")).unwrap(),
        ItemDefinition::new(
            "parameter 1",
            datastore::definition::BasicDefinition::new_string("p1"),
        ),
    );
    builder.insert_variable(
        VariableKey::new(ShareableString::from("v_var1")).unwrap(),
        ItemDefinition::new(
            "Variable 1",
            datastore::definition::BasicDefinition::new_string("v1"),
        ),
    );
    let definition = builder.finish();

    let mut proxy = store
        .create_object(
            StoreKey::new(ShareableString::from("obj1")).unwrap(),
            &definition,
        )
        .unwrap();

    // Test parameter
    assert!(proxy.check_parameter_key("p_prop1").unwrap());
    assert!(!proxy.check_parameter_key("v_var1").unwrap());
    let mut prop1 = proxy.parameter_basic("p_prop1").unwrap();
    assert_eq!(prop1.value().as_str(), ""); // Default value for basic string is empty if not specified
    prop1.set_value("new value");
    assert_eq!(prop1.value().as_str(), "new value");
    prop1.push().unwrap();

    // Test variables
    assert!(proxy.check_variable_key("v_var1").unwrap());
    assert!(!proxy.check_variable_key("p_prop1").unwrap());
    let mut var1 = proxy.variable_basic("v_var1").unwrap();
    assert_eq!(var1.value().as_str(), "");
    var1.set_value("var value");
    assert_eq!(var1.value().as_str(), "var value");
    var1.push().unwrap();

    // Test persistence
    let static_store = store.to_static().unwrap();
    let static_obj = static_store.get("obj1").unwrap();
    assert!(static_obj.get_parameter("p_prop1").is_some());
    assert!(static_obj.get_variable("v_var1").is_some());

    let store2 = Store::new_from_static(&static_store);
    let mut proxy2 = store2.object("obj1").unwrap();
    assert_eq!(
        proxy2.parameter_basic("p_prop1").unwrap().value().as_str(),
        "new value"
    );
    assert_eq!(
        proxy2.variable_basic("v_var1").unwrap().value().as_str(),
        "var value"
    );
}
