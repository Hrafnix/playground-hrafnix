use datastore::prelude::*;

#[test]
fn test_object_definition_basic() {
    // Why: Test object definition creation and items.
    let mut builder = ObjectDefinition::builder("Test Object");
    builder.insert(
        StoreKey::new("p1".into()).unwrap(),
        ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
    );
    let obj_def = builder.finish();

    assert_eq!(obj_def.description().as_ref(), "Test Object");
    assert_eq!(obj_def.count(), 1);
    assert!(obj_def.contains("p1"));
    assert!(obj_def.contains_str("p1"));
}

#[test]
fn test_object_definition_equality() {
    // Why: Test that two object definitions with the same items are considered equal.
    let def_1 = ObjectDefinition::builder("Test Object")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .finish();
    let def_2 = ObjectDefinition::builder("Test Object")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .finish();
    let def_3 = ObjectDefinition::builder("Test Object")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D2")),
        )
        .finish();

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(&def_1, def_2);
    assert_ne!(def_1, &def_3);
}

#[test]
fn test_object_definition_print() {
    // Why: Test object definition print.
    let def_1 = ObjectDefinitionBuilder::new("Test")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
        )
        .with(
            StoreKey::new("p2".into()).unwrap(),
            ItemDefinition::new(
                "P2",
                BasicDefinition::new_file("D2", FileDefinition::new("ext", false)),
            ),
        )
        .with(
            StoreKey::new("p3".into()).unwrap(),
            ItemDefinition::new("P3", BasicDefinition::new_number("D3")),
        )
        .with(
            StoreKey::new("p4".into()).unwrap(),
            ItemDefinition::new(
                "P4",
                BasicDefinition::new_choice(
                    "D4",
                    ChoiceDefinition::new(vec!["Option 1".into(), "Option 2".into()]),
                ),
            ),
        )
        .with(
            StoreKey::new("p5".into()).unwrap(),
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
            StoreKey::new("p6".into()).unwrap(),
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
            StoreKey::new("p7".into()).unwrap(),
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
        "Object Definition (Test)\n    ├── p1 (D1) String - default: \"\" \n    ├── p2 (D2) File - default: \"\" [ext]\n    ├── p3 (D3) Number - default: \"\" \n    ├── p4 (D4) Choice - default: \"\" [Option 1, Option 2]\n    ├── p5 (D5) Table\n    │   ├── col1 (C1) String - default: \"\" \n    │   └── col2 (C2) Number - default: \"\" \n    ├── p6 (D6) Struct\n    │   ├── f1 (F1) String - default: \"\" \n    │   └── f2 (T1) Table\n    │       ├── col1 (C1) String - default: \"\" \n    │       └── col2 (C2) Number - default: \"\" \n    └── p7 (D7) Map\n        └── item_type (Item) Struct\n            ├── col1 (C1) String - default: \"\" \n            └── col2 (C2) Number - default: \"\" \n"
    );
}
