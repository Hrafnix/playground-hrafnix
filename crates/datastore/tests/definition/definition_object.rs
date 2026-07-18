use datastore::prelude::*;

#[test]
fn test_object_definition_basic() {
    // Why: Test object definition creation and items.
    let mut builder = ObjectDefinition::builder("Test Object");
    builder.insert(
        StoreKey::new("p1".into()).unwrap(),
        ItemDefinition::new("P1", StringDefinition::new("D1")),
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
            ItemDefinition::new("P1", StringDefinition::new("D1")),
        )
        .finish();
    let def_2 = ObjectDefinition::builder("Test Object")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            ItemDefinition::new("P1", StringDefinition::new("D1")),
        )
        .finish();
    let def_3 = ObjectDefinition::builder("Test Object")
        .with(
            StoreKey::new("p1".into()).unwrap(),
            ItemDefinition::new("P1", StringDefinition::new("D2")),
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
            ItemDefinition::new("P1", StringDefinition::new("D1")),
        )
        .with(
            StoreKey::new("p2".into()).unwrap(),
            ItemDefinition::new("P2", FileDefinition::new("D2", "ext", false)),
        )
        .with(
            StoreKey::new("p3".into()).unwrap(),
            ItemDefinition::new("P3", NumberDefinition::new("D3")),
        )
        .with(
            StoreKey::new("p4".into()).unwrap(),
            ItemDefinition::new(
                "P4",
                ChoiceDefinition::new(
                    "D4",
                    vec![
                        ChoiceItemDefinition::new(store_key!("option_1"), "Option 1"),
                        ChoiceItemDefinition::new(store_key!("option_2"), "Option 2"),
                    ],
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
                        (store_key!("col1"), NumberDefinition::new("C1")),
                        (
                            store_key!("col2"),
                            NumberDefinition::new_with_constraint(
                                "C2",
                                NumberConstraint::Min {
                                    value: 1.52,
                                    inclusive: true,
                                },
                            ),
                        ),
                    ],
                ),
            ),
        )
        .with(
            StoreKey::new("p6".into()).unwrap(),
            ItemDefinition::new(
                "P6",
                MapDefinition::new(
                    "D6",
                    StructDefinition::new(
                        "Item",
                        vec![
                            (
                                store_key!("col1"),
                                StructItemDefinition::String(StringDefinition::new("C1")),
                            ),
                            (
                                store_key!("col2"),
                                StructItemDefinition::Number(
                                    NumberDefinition::new_with_constraint(
                                        "C2",
                                        NumberConstraint::Max {
                                            value: 1.0,
                                            inclusive: true,
                                        },
                                    ),
                                ),
                            ),
                        ],
                    ),
                ),
            ),
        )
        .finish();

    assert_eq!(
        format!("{}", def_1),
        "Object Definition (Test)\n    ├── p1 (D1) String - default: \"\"\n    ├── p2 (D2) File - default: \"\" [ext]\n    ├── p3 (D3) Number - default: \"\"\n    ├── p4 (D4) Choice - default: \"\" [option_1 (Option 1), option_2 (Option 2)]\n    ├── p5 (D5) Table\n    │   ├── col1 (C1) Number - default: \"\"\n    │   └── col2 (C2) Number - default: \"\" [Min(1.52, inclusive)]\n    └── p6 (D6) Map\n        └── item_type (Item) Struct\n            ├── col1 (C1) String - default: \"\"\n            └── col2 (C2) Number - default: \"\" [Max(1.0, inclusive)]\n"
    );
}
