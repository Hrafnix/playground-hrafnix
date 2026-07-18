use datastore::prelude::*;

#[test]
fn test_variable_object_definition_basic() {
    // Why: Test frozen variable object creation and items.
    let frozen_1 = VariableObjectFrozen::new(
        VariableObjectDefinition::builder("Test Object")
            .with(
                VariableKey::new("v_v1".into()).unwrap(),
                StringDefinition::new("D1"),
            )
            .finish(),
    );

    assert_eq!(frozen_1.definition().description().as_ref(), "Test Object");
    assert_eq!(frozen_1.definition().count(), 1);
    assert!(frozen_1.definition().contains("v_v1"));
    assert!(frozen_1.definition().contains_str("v_v1"));
    assert_ne!(frozen_1.hash(), [0u8; 32]);
}

#[test]
fn test_variable_object_definition_equality() {
    // Why: Test that two frozen variable objects with the same items are considered equal.
    let frozen_1 = VariableObjectFrozen::new(
        VariableObjectDefinition::builder("Test Object")
            .with(
                VariableKey::new("v_v1".into()).unwrap(),
                StringDefinition::new("D1"),
            )
            .finish(),
    );
    let frozen_2 = VariableObjectFrozen::new(
        VariableObjectDefinition::builder("Test Object")
            .with(
                VariableKey::new("v_v1".into()).unwrap(),
                StringDefinition::new("D1"),
            )
            .finish(),
    );
    let frozen_3 = VariableObjectFrozen::new(
        VariableObjectDefinition::builder("Test Object")
            .with(
                VariableKey::new("v_v1".into()).unwrap(),
                StringDefinition::new("D2"),
            )
            .finish(),
    );

    assert_eq!(frozen_1, frozen_2);
    assert_ne!(frozen_1, frozen_3);
    assert_eq!(&frozen_1, frozen_2);
    assert_ne!(frozen_1, &frozen_3);
}

#[test]
fn test_variable_object_frozen_print_empty() {
    // Why: Test frozen variable object print based on a VariableObjectDefinition.
    let frozen_1 = VariableObjectFrozen::new(
        VariableObjectDefinitionBuilder::new("Test")
            .with(
                VariableKey::new("v_p1".into()).unwrap(),
                StringDefinition::new("D1"),
            )
            .with(
                VariableKey::new("v_p2".into()).unwrap(),
                FileDefinition::new("D2", "ext", false),
            )
            .with(
                VariableKey::new("v_p3".into()).unwrap(),
                NumberDefinition::new("D3"),
            )
            .with(
                VariableKey::new("v_p4".into()).unwrap(),
                ChoiceDefinition::new(
                    "D4",
                    vec![
                        ChoiceItemDefinition::new(store_key!("option_1"), "Option 1"),
                        ChoiceItemDefinition::new(store_key!("option_2"), "Option 2"),
                    ],
                ),
            )
            .with(
                VariableKey::new("v_p5".into()).unwrap(),
                TableDefinition::new(
                    "D5",
                    vec![
                        (store_key!("col1"), NumberDefinition::new("C1")),
                        (store_key!("col2"), NumberDefinition::new("C2")),
                    ],
                ),
            )
            .with(
                VariableKey::new("v_p6".into()).unwrap(),
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
            )
            .finish(),
    );

    assert_eq!(
        format!("{}", frozen_1),
        "Frozen Variable Object (Test)\n    ├── v_p1 (D1) String - \"\"\n    ├── v_p2 (D2) File - \"\"\n    ├── v_p3 (D3) Number - \"\"\n    ├── v_p4 (D4) Choice - \"\"\n    ├── v_p5 (D5) Table 0 rows\n    └── v_p6 (D6) Map\n"
    );
}
