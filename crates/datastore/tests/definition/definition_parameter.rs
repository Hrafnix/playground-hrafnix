use datastore::prelude::*;

#[test]
fn test_parameter_definition() {
    // Why: Test basic parameter definition creation and parameter.
    let basic_prop = ItemDefinition::new("Basic Prop", BasicDefinition::new_string("String"));

    // Check the various parameter of the parameter definition.
    assert_eq!(basic_prop.description().as_ref(), "Basic Prop");
    assert!(matches!(
        basic_prop.item_type(),
        datastore::definition::ItemDefinitionType::Basic(_)
    ));
    assert_eq!(basic_prop.is_gui_visible(), true);
}

#[test]
fn test_struct_parameter_definition() {
    // Why: Test struct parameter definition creation and parameter.
    let struct_prop = ItemDefinition::new(
        "Struct Prop",
        StructDefinition::new("Struct", Vec::<(StoreKey, StructItemDefinition)>::new()),
    );

    // Check the various parameter of the parameter definition.
    assert!(matches!(
        struct_prop.item_type(),
        datastore::definition::ItemDefinitionType::Struct(_)
    ));
    assert_eq!(struct_prop.is_gui_visible(), true);
}

#[test]
fn test_table_parameter_definition() {
    // Why: Test table parameter definition creation and parameter.
    let table_prop = ItemDefinition::new(
        "Table Prop",
        TableDefinition::new("Table", Vec::<(StoreKey, BasicDefinition)>::new()),
    );

    // Check the various parameter of the parameter definition.
    assert!(matches!(
        table_prop.item_type(),
        datastore::definition::ItemDefinitionType::Table(_)
    ));
    assert_eq!(table_prop.is_gui_visible(), true);
}

#[test]
fn test_map_parameter_definition() {
    // Why: Test map parameter definition creation and parameter.
    let map_prop = ItemDefinition::new(
        "Map Prop",
        MapDefinition::new(
            "Map",
            StructDefinition::new("Item", Vec::<(StoreKey, StructItemDefinition)>::new()),
        ),
    );

    // Check the various parameter of the parameter definition.
    assert!(matches!(
        map_prop.item_type(),
        datastore::definition::ItemDefinitionType::Map(_)
    ));
    assert_eq!(map_prop.is_gui_visible(), true);
}

#[test]
fn test_parameter_gui_visibility() {
    // Why: Test basic parameter definition creation and parameter with gui invisibility.
    let basic_prop =
        ItemDefinition::new_gui_invisible("Basic Prop", BasicDefinition::new_string("String"));

    // Check the various parameter of the parameter definition.
    assert_eq!(basic_prop.description().as_ref(), "Basic Prop");
    assert!(matches!(
        basic_prop.item_type(),
        datastore::definition::ItemDefinitionType::Basic(_)
    ));
    assert_eq!(basic_prop.is_gui_visible(), false);
}

#[test]
fn test_parameter_definition_type_equality() {
    // Why: Test that two parameter definition items with the same parameter are considered equal and ref equal.
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
    // Why: Test that two parameter definitions with the same parameter are considered equal and ref equal.
    let def_1 = ItemDefinition::new("Basic Prop", BasicDefinition::new_string("String"));
    let def_2 = ItemDefinition::new("Basic Prop", BasicDefinition::new_string("String"));
    let def_3 = ItemDefinition::new("Basic Prop", BasicDefinition::new_string("New String"));

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(&def_1, def_2);
    assert_ne!(def_1, &def_3);
}
