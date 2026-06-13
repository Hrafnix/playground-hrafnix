use datastore::definition::{
    BasicDefinition, MapDefinition, PropertyDefinition, StructDefinition, StructItemDefinition,
    TableDefinition,
};
use datastore::key::StoreKey;

#[test]
fn test_property_definition() {
    // Why: Test basic property definition creation and properties.
    let basic_prop = PropertyDefinition::new("Basic Prop", BasicDefinition::new_string("String"));

    // Check the various properties of the property definition.
    assert_eq!(basic_prop.description().as_ref(), "Basic Prop");
    assert!(matches!(
        basic_prop.item_type(),
        datastore::definition::PropertyDefinitionType::Basic(_)
    ));
    assert_eq!(basic_prop.is_gui_visible(), true);
}

#[test]
fn test_struct_property_definition() {
    // Why: Test struct property definition creation and properties.
    let struct_prop = PropertyDefinition::new(
        "Struct Prop",
        StructDefinition::new("Struct", Vec::<(StoreKey, StructItemDefinition)>::new()),
    );

    // Check the various properties of the property definition.
    assert!(matches!(
        struct_prop.item_type(),
        datastore::definition::PropertyDefinitionType::Struct(_)
    ));
    assert_eq!(struct_prop.is_gui_visible(), true);
}

#[test]
fn test_table_property_definition() {
    // Why: Test table property definition creation and properties.
    let table_prop = PropertyDefinition::new(
        "Table Prop",
        TableDefinition::new("Table", Vec::<(StoreKey, BasicDefinition)>::new()),
    );

    // Check the various properties of the property definition.
    assert!(matches!(
        table_prop.item_type(),
        datastore::definition::PropertyDefinitionType::Table(_)
    ));
    assert_eq!(table_prop.is_gui_visible(), true);
}

#[test]
fn test_map_property_definition() {
    // Why: Test map property definition creation and properties.
    let map_prop = PropertyDefinition::new(
        "Map Prop",
        MapDefinition::new(
            "Map",
            StructDefinition::new("Item", Vec::<(StoreKey, StructItemDefinition)>::new()),
        ),
    );

    // Check the various properties of the property definition.
    assert!(matches!(
        map_prop.item_type(),
        datastore::definition::PropertyDefinitionType::Map(_)
    ));
    assert_eq!(map_prop.is_gui_visible(), true);
}

#[test]
fn test_property_gui_visibility() {
    // Why: Test basic property definition creation and properties with gui invisibility.
    let basic_prop =
        PropertyDefinition::new_gui_invisible("Basic Prop", BasicDefinition::new_string("String"));

    // Check the various properties of the property definition.
    assert_eq!(basic_prop.description().as_ref(), "Basic Prop");
    assert!(matches!(
        basic_prop.item_type(),
        datastore::definition::PropertyDefinitionType::Basic(_)
    ));
    assert_eq!(basic_prop.is_gui_visible(), false);
}

#[test]
fn test_property_definition_type_equality() {
    // Why: Test that two property definition items with the same properties are considered equal and ref equal.
    let def_1 = PropertyDefinition::new("Basic Prop", BasicDefinition::new_string("String"));
    let def_2 = PropertyDefinition::new("Basic Prop", BasicDefinition::new_string("String"));
    let def_3 = PropertyDefinition::new("Basic Prop", BasicDefinition::new_string("New String"));

    assert_eq!(*def_1.item_type(), *def_2.item_type());
    assert_ne!(*def_1.item_type(), *def_3.item_type());
    assert_eq!(*def_1.item_type(), def_2.item_type());
    assert_ne!(def_1.item_type(), *def_3.item_type());
}

#[test]
fn test_property_definition_equality() {
    // Why: Test that two property definitions with the same properties are considered equal and ref equal.
    let def_1 = PropertyDefinition::new("Basic Prop", BasicDefinition::new_string("String"));
    let def_2 = PropertyDefinition::new("Basic Prop", BasicDefinition::new_string("String"));
    let def_3 = PropertyDefinition::new("Basic Prop", BasicDefinition::new_string("New String"));

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(&def_1, def_2);
    assert_ne!(def_1, &def_3);
}
