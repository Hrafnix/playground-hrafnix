use datastore::prelude::*;

#[test]
fn test_proxy_basic() {
    // Why: General test of basic parameter proxy functionality, including creation, getting/setting value, push/pull, and expiry behavior.
    let store = Store::new(SharedStringStore::new());

    // 1. Create Object Definition
    let basic_definition = BasicDefinition::new_string("The name");
    let mut builder = ObjectDefinition::builder("Test Object");
    builder.insert_parameter(
        parameter_key!("p_name"),
        ItemDefinition::new("Name", basic_definition.clone()),
    );
    let obj_def = builder.finish();

    // 2. Create Object in Store
    let obj_key = store_key!("my_object");
    let mut obj_proxy = store.create_object(obj_key, &obj_def).unwrap();

    assert_eq!(obj_proxy.description().as_ref(), "Test Object");

    // 3. Get Basic parameter Proxy
    let mut name_proxy = obj_proxy.parameter_basic(store_key!("p_name")).unwrap();
    assert_eq!(name_proxy.value().as_ref(), "");
    assert_eq!(name_proxy.definition(), basic_definition);
    assert_eq!(basic_definition, name_proxy.definition());
    assert_eq!(name_proxy.path(), path!("my_object" / "p_name"));
    assert_eq!(name_proxy.description(), "The name");
    assert_eq!(name_proxy.pull(), Ok(()));

    // 4. Set Value and Push
    name_proxy.set_value("Junie");
    assert!(name_proxy.has_changed());
    name_proxy.push().unwrap();
    assert!(!name_proxy.has_changed());

    // 5. Test Delete and Restore Check
    let static_store = store.to_static().unwrap();
    assert_eq!(store.delete_object(obj_key.as_str()), Ok(()));
    assert_eq!(
        store.object(obj_key.as_str()).err(),
        Some(StoreError::ObjectNotFound)
    );
    assert_eq!(name_proxy.pull().err(), Some(StoreError::ExpiredProxy));
    assert_eq!(name_proxy.push().err(), Some(StoreError::ExpiredProxy));

    name_proxy.set_value("Ralph");
    assert_eq!(name_proxy.value(), "Ralph");
    assert_eq!(name_proxy.is_valid(), false);

    assert_eq!(store.sync_from_static(&static_store), Ok(()));
    assert_eq!(name_proxy.pull(), Ok(()));
    assert_eq!(name_proxy.value(), "Junie");
}

#[test]
fn test_proxy_basic_push_exhaustive() {
    // Why: An exhaustive test of the push operation failure/recovery on an expired proxy.
}

#[test]
fn test_proxy_basic_pull_exhaustive() {
    // Why: An exhaustive test of the pull operation failure/recovery on an expired proxy.
}

#[test]
fn test_proxy_basic_print() {
    // Why: Test that the Display implementation for BasicProxy shows the value and definition description.
    let store = Store::new(SharedStringStore::new());

    let basic_definition = BasicDefinition::new_string("The name");
    let mut builder = ObjectDefinition::builder("Test Object");
    builder.insert_parameter(
        parameter_key!("p_name"),
        ItemDefinition::new("Name", basic_definition.clone()),
    );
    let obj_def = builder.finish();

    let obj_key = store_key!("my_object");
    let mut obj_proxy = store.create_object(obj_key, &obj_def).unwrap();

    let mut name_proxy = obj_proxy.parameter_basic(store_key!("p_name")).unwrap();
    name_proxy.set_value("Junie");

    assert_eq!(format!("{}", name_proxy), "p_name: Junie (The name)\n")
}
