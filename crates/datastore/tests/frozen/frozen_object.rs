use datastore::prelude::*;

#[test]
fn test_object_frozen_basic() {
    // Why: Test frozen object creation and items.
    let frozen_1 = ObjectFrozen::new(
        ObjectDefinition::builder("Test Object")
            .with(
                StoreKey::new("p1".into()).unwrap(),
                ItemDefinition::new("P1", StringDefinition::new("D1")),
            )
            .finish(),
    );

    assert_eq!(frozen_1.definition().description().as_ref(), "Test Object");
    assert_eq!(frozen_1.definition().count(), 1);
    assert!(frozen_1.definition().contains("p1"));
    assert!(frozen_1.definition().contains_str("p1"));
    assert_ne!(frozen_1.hash(), [0u8; 32]);
}

#[test]
fn test_object_frozen_equality() {
    // Why: Test that two frozen objects with the same items are considered equal.
    let frozen_1 = ObjectFrozen::new(
        ObjectDefinition::builder("Test Object")
            .with(
                StoreKey::new("p1".into()).unwrap(),
                ItemDefinition::new("P1", StringDefinition::new("D1")),
            )
            .finish(),
    );
    let frozen_2 = ObjectFrozen::new(
        ObjectDefinition::builder("Test Object")
            .with(
                StoreKey::new("p1".into()).unwrap(),
                ItemDefinition::new("P1", StringDefinition::new("D1")),
            )
            .finish(),
    );
    let frozen_3 = ObjectFrozen::new(
        ObjectDefinition::builder("Test Object")
            .with(
                StoreKey::new("p1".into()).unwrap(),
                ItemDefinition::new("P1", StringDefinition::new("D2")),
            )
            .finish(),
    );

    assert_eq!(frozen_1, frozen_2);
    assert_ne!(frozen_1, frozen_3);
    assert_eq!(&frozen_1, frozen_2);
    assert_ne!(frozen_1, &frozen_3);
}

#[test]
fn test_object_frozen_print_empty() {
    // Why: Test frozen object print based on an ObjectDefinition.
    let frozen_1 = ObjectFrozen::new(
        ObjectDefinitionBuilder::new("Test")
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
                            (store_key!("col2"), NumberDefinition::new("C2")),
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
                                    StructItemDefinition::Number(NumberDefinition::new("C2")),
                                ),
                            ],
                        ),
                    ),
                ),
            )
            .finish(),
    );

    assert_eq!(
        format!("{}", frozen_1),
        "Frozen Object (Test)\n    ├── p1 (D1) String - \"\"\n    ├── p2 (D2) File - \"\"\n    ├── p3 (D3) Number - \"\"\n    ├── p4 (D4) Choice - \"\"\n    ├── p5 (D5) Table 0 rows\n    └── p6 (D6) Map\n"
    );
}
