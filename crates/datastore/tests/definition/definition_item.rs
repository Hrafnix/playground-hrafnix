use datastore::prelude::*;

#[test]
fn test_item_definition() {
    // Why: Test basic item definition creation and definition.
    let basic_prop = ItemDefinition::new("Basic Prop", BasicDefinition::new_string("String"));

    // Check the various data items of the item definition.
    assert_eq!(basic_prop.description().as_ref(), "Basic Prop");
    assert!(matches!(
        basic_prop.item_type(),
        datastore::definition::ItemDefinitionType::Basic(_)
    ));
}

#[test]
fn test_table_item_definition() {
    // Why: Test table item definition creation and definition.
    let table_prop = ItemDefinition::new(
        "Table Prop",
        TableDefinition::new("Table", Vec::<(StoreKey, BasicDefinition)>::new()),
    );

    // Check the various data items of the item definition.
    assert!(matches!(
        table_prop.item_type(),
        datastore::definition::ItemDefinitionType::Table(_)
    ));
}

#[test]
fn test_map_item_definition() {
    // Why: Test map item definition creation and definition.
    let map_prop = ItemDefinition::new(
        "Map Prop",
        MapDefinition::new(
            "Map",
            StructDefinition::new("Item", Vec::<(StoreKey, StructItemDefinition)>::new()),
        ),
    );

    // Check the various data items of the item definition.
    assert!(matches!(
        map_prop.item_type(),
        datastore::definition::ItemDefinitionType::Map(_)
    ));
}

#[test]
fn test_item_definition_type_equality() {
    // Why: Test that two item definition items with the same data are considered equal.
    let def_1 = ItemDefinition::new("Basic Prop", BasicDefinition::new_string("String"));
    let def_2 = ItemDefinition::new("Basic Prop", BasicDefinition::new_string("String"));
    let def_3 = ItemDefinition::new("Basic Prop", BasicDefinition::new_string("New String"));

    assert_eq!(*def_1.item_type(), *def_2.item_type());
    assert_ne!(*def_1.item_type(), *def_3.item_type());
    assert_eq!(*def_1.item_type(), def_2.item_type());
    assert_ne!(def_1.item_type(), *def_3.item_type());
}

#[test]
fn test_parameter_definition_equality() {
    // Why: Test that two item definitions with the same data are considered equal.
    let def_1 = ItemDefinition::new("Basic Prop", BasicDefinition::new_string("String"));
    let def_2 = ItemDefinition::new("Basic Prop", BasicDefinition::new_string("String"));
    let def_3 = ItemDefinition::new("Basic Prop", BasicDefinition::new_string("New String"));

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(&def_1, def_2);
    assert_ne!(def_1, &def_3);
}
