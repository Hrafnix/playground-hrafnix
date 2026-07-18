use datastore::definition::{
    BasicDefinition, ChoiceDefinition, FileDefinition, ItemDefinition, MapDefinition,
    ParameterObjectDefinition, ParameterObjectDefinitionBuilder, StructDefinition, TableDefinition,
};
use datastore::key::ParameterKey;
use datastore::store_key;

#[test]
fn test_parameter_object_definition_basic() {
    // Why: Test parameter object definition creation and items.
    let mut builder = ParameterObjectDefinition::builder("Test Object");
    builder.insert(
        ParameterKey::new("p_p1".into()).unwrap(),
        ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
    );
    let obj_def = builder.finish();

    assert_eq!(obj_def.description().as_ref(), "Test Object");
    assert_eq!(obj_def.count(), 1);
    assert!(obj_def.contains("p_p1"));
    assert!(obj_def.contains_str("p_p1"));
}

#[test]
fn test_parameter_object_definition_equality() {
    // Why: Test that two parameter object definitions with the same items are considered equal.
    let def_1 = ParameterObjectDefinition::builder("Test Object")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .finish();
    let def_2 = ParameterObjectDefinition::builder("Test Object")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .finish();
    let def_3 = ParameterObjectDefinition::builder("Test Object")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D2")),
        )
        .finish();

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(&def_1, def_2);
    assert_ne!(def_1, &def_3);
}

#[test]
fn test_parameter_object_definition_print() {
    // Why: Test parameter object definition print.
    let def_1 = ParameterObjectDefinitionBuilder::new("Test")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .with(
            ParameterKey::new("p_p2".into()).unwrap(),
            ItemDefinition::new(
                "P2",
                BasicDefinition::new_file("D2", FileDefinition::new("ext", false)),
            ),
        )
        .with(
            ParameterKey::new("p_p3".into()).unwrap(),
            ItemDefinition::new("P3", BasicDefinition::new_number("D3")),
        )
        .with(
            ParameterKey::new("p_p4".into()).unwrap(),
            ItemDefinition::new(
                "P4",
                BasicDefinition::new_choice(
                    "D4",
                    ChoiceDefinition::new(vec!["Option 1".into(), "Option 2".into()]),
                ),
            ),
        )
        .with(
            ParameterKey::new("p_p5".into()).unwrap(),
            ItemDefinition::new(
                "P5",
                TableDefinition::new(
                    "D5",
                    vec![
                        (store_key!("col1"), BasicDefinition::new_string("C1")),
                        (store_key!("col2"), BasicDefinition::new_number("C2")),
                    ],
                ),
            ),
        )
        .with(
            ParameterKey::new("p_p6".into()).unwrap(),
            ItemDefinition::new(
                "P6",
                MapDefinition::new(
                    "P6",
                    StructDefinition::new(
                        "Item",
                        vec![
                            (store_key!("col1"), BasicDefinition::new_string("C1")),
                            (store_key!("col2"), BasicDefinition::new_number("C2")),
                        ],
                    ),
                ),
            ),
        )
        .finish();

    assert_eq!(
        format!("{}", def_1),
        "Parameter Object Definition (Test)\n    ├── p_p1 (D1) String - default: \"\" \n    ├── p_p2 (D2) File - default: \"\" [ext]\n    ├── p_p3 (D3) Number - default: \"\" \n    ├── p_p4 (D4) Choice - default: \"\" [Option 1, Option 2]\n    ├── p_p5 (D5) Table\n    │   ├── col1 (C1) String - default: \"\" \n    │   └── col2 (C2) Number - default: \"\" \n    └── p_p6 (P6) Map\n        └── item_type (Item) Struct\n            ├── col1 (C1) String - default: \"\" \n            └── col2 (C2) Number - default: \"\" \n"
    );
}
