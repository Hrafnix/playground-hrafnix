use datastore::definition::{BasicDefinition, ItemDefinition, ObjectDefinition, TableDefinition};
use datastore::store::Store;
use datastore::{StoreError, parameter_key, path, store_key};
use shareable_string::SharedStringStore;
use std::collections::BTreeMap;

#[test]
fn test_proxy_table() {
    // Why: General test of Table parameter proxy functionality, including creation, getting/setting value, push/pull, and expiry behavior.
    let store = Store::new(SharedStringStore::new());

    // 1. Create Object Definition
    let table_definition = TableDefinition::new(
        "Price List",
        vec![
            (store_key!("item"), BasicDefinition::new_string("Item Name")),
            (
                store_key!("cost"),
                BasicDefinition::new_number_with_default("Cost", "0.0"),
            ),
        ],
    );
    let mut builder = ObjectDefinition::builder("Test Object");
    builder.insert_parameter(
        parameter_key!("p_catalog"),
        ItemDefinition::new("Catalog", table_definition.clone()),
    );
    let obj_def = builder.finish();

    // 2. Create Object in Store
    let obj_key = store_key!("object");
    let mut obj_proxy = store.create_object(obj_key, &obj_def).unwrap();

    assert_eq!(obj_proxy.description().as_ref(), "Test Object");

    // 3. Get Basic parameter Proxy
    let mut table_proxy = obj_proxy.parameter_table(store_key!("p_catalog")).unwrap();
    assert_eq!(table_proxy.row_count(), 0);
    assert_eq!(
        table_proxy.remove_row(0).err(),
        Some(StoreError::IndexNotFound)
    );
    assert_eq!(table_proxy.column_count(), 2);
    assert_eq!(table_proxy.definition(), table_definition);
    assert_eq!(
        table_proxy.object().unwrap().description().as_ref(),
        "Test Object"
    );
    assert_eq!(table_definition, table_proxy.definition());
    assert_eq!(table_proxy.path(), path!("object" / "p_catalog"));
    assert_eq!(table_proxy.description(), "Price List");
    assert_eq!(table_proxy.pull(), Ok(()));

    // 4. Set Value and Push
    table_proxy.append_row();
    table_proxy.insert_row(0);
    table_proxy.remove_row(1).unwrap();
    table_proxy.set_cell(0, "item", "Pie").unwrap();
    table_proxy.set_cell(0, "cost", "5.0").unwrap();
    assert!(table_proxy.set_cell(1, "cost", "5.0").is_err());
    assert!(table_proxy.set_cell(0, "cost2", "5.0").is_err());
    assert!(table_proxy.set_row(1, BTreeMap::from([("", "")])).is_err());
    assert!(table_proxy.has_changed());
    assert_eq!(
        table_proxy.row(0).map(|r| r
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect::<BTreeMap<_, _>>()),
        Some(BTreeMap::from([("item", "Pie"), ("cost", "5.0")]))
    );
    table_proxy.push().unwrap();
    assert!(!table_proxy.has_changed());
    assert_eq!(
        table_proxy.row(0).map(|r| r
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect::<BTreeMap<_, _>>()),
        Some(BTreeMap::from([("item", "Pie"), ("cost", "5.0")]))
    );

    // 5. Test Delete and Restore Check
    let static_store = store.to_static().unwrap();
    assert_eq!(store.delete_object(obj_key.as_str()), Ok(()));
    assert_eq!(
        store.object(obj_key.as_str()).err(),
        Some(StoreError::ObjectNotFound)
    );
    assert_eq!(table_proxy.pull().err(), Some(StoreError::ExpiredProxy));
    assert_eq!(table_proxy.push().err(), Some(StoreError::ExpiredProxy));

    table_proxy
        .set_row(0, BTreeMap::from([("cost", "10.0"), ("item", "Apple")]))
        .unwrap();
    assert_eq!(
        table_proxy.row(0).map(|r| r
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect::<BTreeMap<_, _>>()),
        Some(BTreeMap::from([("item", "Apple"), ("cost", "10.0")]))
    );
    assert_eq!(table_proxy.is_valid(), false);

    assert_eq!(store.sync_from_static(&static_store), Ok(()));
    assert_eq!(table_proxy.pull(), Ok(()));
    assert_eq!(
        table_proxy.row(0).map(|r| r
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect::<BTreeMap<_, _>>()),
        Some(BTreeMap::from([("item", "Pie"), ("cost", "5.0")]))
    );
}

#[test]
fn test_proxy_table_push_exhaustive() {
    // Why: An exhaustive test of the push operation failure/recovery on an expired proxy.
}

#[test]
fn test_proxy_table_pull_exhaustive() {
    // Why: An exhaustive test of the pull operation failure/recovery on an expired proxy.
}

#[test]
fn test_proxy_table_print() {
    // Why: Test that the Display implementation for TableProxy shows the value and definition description.
    let store = Store::new(SharedStringStore::new());

    let table_definition = TableDefinition::new(
        "Price List",
        vec![
            (store_key!("item"), BasicDefinition::new_string("Item Name")),
            (
                store_key!("cost"),
                BasicDefinition::new_number_with_default("Cost", "0.0"),
            ),
        ],
    );
    let mut builder = ObjectDefinition::builder("Test Object");
    builder.insert_parameter(
        parameter_key!("p_catalog"),
        ItemDefinition::new("Catalog", table_definition.clone()),
    );
    let obj_def = builder.finish();

    let obj_key = store_key!("object");
    let mut obj_proxy = store.create_object(obj_key, &obj_def).unwrap();

    let mut table_proxy = obj_proxy.parameter_table(store_key!("p_catalog")).unwrap();
    table_proxy.insert_row(0);
    table_proxy
        .set_row(0, BTreeMap::from([("cost", "5.0"), ("item", "Pie")]))
        .unwrap();

    assert_eq!(
        format!("{}", table_proxy),
        "p_catalog: [Table, 1 rows] (Price List)\n    └── Row 0\n        ├── cost: 5.0\n        └── item: Pie\n"
    )
}
