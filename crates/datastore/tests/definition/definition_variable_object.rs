use datastore::definition::{
    BasicDefinition, ChoiceDefinition, FileDefinition, ItemDefinition, MapDefinition,
    StructDefinition, StructItemDefinition, TableDefinition, VariableObjectDefinition,
    VariableObjectDefinitionBuilder,
};
use datastore::key::VariableKey;
use datastore::store_key;

#[test]
fn test_variable_object_definition_basic() {
    // Why: Test variable object definition creation and items.
    let mut builder = VariableObjectDefinition::builder("Test Object");
    builder.insert(
        VariableKey::new("v_v1".into()).unwrap(),
        ItemDefinition::new("V1", BasicDefinition::new_string("D1")),
    );
    let obj_def = builder.finish();

    assert_eq!(obj_def.description().as_ref(), "Test Object");
    assert_eq!(obj_def.count(), 1);
    assert!(obj_def.contains("v_v1"));
    assert!(obj_def.contains_str("v_v1"));
}

#[test]
fn test_variable_object_definition_equality() {
    // Why: Test that two variable object definitions with the same items are considered equal.
    let def_1 = VariableObjectDefinition::builder("Test Object")
        .with(
            VariableKey::new("v_v1".into()).unwrap(),
            ItemDefinition::new("V1", BasicDefinition::new_string("D1")),
        )
        .finish();
    let def_2 = VariableObjectDefinition::builder("Test Object")
        .with(
            VariableKey::new("v_v1".into()).unwrap(),
            ItemDefinition::new("V1", BasicDefinition::new_string("D1")),
        )
        .finish();
    let def_3 = VariableObjectDefinition::builder("Test Object")
        .with(
            VariableKey::new("v_v1".into()).unwrap(),
            ItemDefinition::new("V1", BasicDefinition::new_string("D2")),
        )
        .finish();

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(&def_1, def_2);
    assert_ne!(def_1, &def_3);
}

#[test]
fn test_variable_object_definition_print() {
    // Why: Test variable object definition print.
    let def_1 = VariableObjectDefinitionBuilder::new("Test")
        .with(
            VariableKey::new("v_p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .with(
            VariableKey::new("v_p2".into()).unwrap(),
            ItemDefinition::new(
                "P2",
                BasicDefinition::new_file("D2", FileDefinition::new("ext", false)),
            ),
        )
        .with(
            VariableKey::new("v_p3".into()).unwrap(),
            ItemDefinition::new("P3", BasicDefinition::new_number("D3")),
        )
        .with(
            VariableKey::new("v_p4".into()).unwrap(),
            ItemDefinition::new(
                "P4",
                BasicDefinition::new_choice(
                    "D4",
                    ChoiceDefinition::new(vec!["Option 1".into(), "Option 2".into()]),
                ),
            ),
        )
        .with(
            VariableKey::new("v_p5".into()).unwrap(),
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
            VariableKey::new("v_p6".into()).unwrap(),
            ItemDefinition::new(
                "P6",
                StructDefinition::new(
                    "D6",
                    vec![
                        (
                            store_key!("f1"),
                            StructItemDefinition::Basic(BasicDefinition::new_string("F1")),
                        ),
                        (
                            store_key!("f2"),
                            StructItemDefinition::Table(TableDefinition::new(
                                "T1",
                                vec![
                                    (store_key!("col1"), BasicDefinition::new_string("C1")),
                                    (store_key!("col2"), BasicDefinition::new_number("C2")),
                                ],
                            )),
                        ),
                    ],
                ),
            ),
        )
        .with(
            VariableKey::new("v_p7".into()).unwrap(),
            ItemDefinition::new(
                "P7",
                MapDefinition::new(
                    "D7",
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
        "Variable Object Definition (Test)\n    ├── v_p1 (D1) String - default: \"\" \n    ├── v_p2 (D2) File - default: \"\" [ext]\n    ├── v_p3 (D3) Number - default: \"\" \n    ├── v_p4 (D4) Choice - default: \"\" [Option 1, Option 2]\n    ├── v_p5 (D5) Table\n    │   ├── col1 (C1) String - default: \"\" \n    │   └── col2 (C2) Number - default: \"\" \n    ├── v_p6 (D6) Struct\n    │   ├── f1 (F1) String - default: \"\" \n    │   └── f2 (T1) Table\n    │       ├── col1 (C1) String - default: \"\" \n    │       └── col2 (C2) Number - default: \"\" \n    └── v_p7 (D7) Map\n        └── item_type (Item) Struct\n            ├── col1 (C1) String - default: \"\" \n            └── col2 (C2) Number - default: \"\" \n"
    );
}
