use datastore::prelude::*;

#[test]
fn test_parameter_object_definition_basic() {
    // Why: Test frozen parameter object creation and items.
    let frozen_1 = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                ParameterKey::new("p_p1".into()).unwrap(),
                StringDefinition::new("D1"),
            )
            .finish(),
    );

    assert_eq!(frozen_1.definition().description().as_ref(), "Test Object");
    assert_eq!(frozen_1.definition().count(), 1);
    assert!(frozen_1.definition().contains("p_p1"));
    assert!(frozen_1.definition().contains_str("p_p1"));
    assert_ne!(frozen_1.hash(), [0u8; 32]);
}

#[test]
fn test_parameter_object_definition_equality() {
    // Why: Test that two frozen parameter object definitions with the same items are considered equal.
    let frozen_1 = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                ParameterKey::new("p_p1".into()).unwrap(),
                StringDefinition::new("D1"),
            )
            .finish(),
    );
    let frozen_2 = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                ParameterKey::new("p_p1".into()).unwrap(),
                StringDefinition::new("D1"),
            )
            .finish(),
    );
    let frozen_3 = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                ParameterKey::new("p_p1".into()).unwrap(),
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
fn test_parameter_object_frozen_print_empty() {
    // Why: Test frozen parameter object print based on a ParameterObjectDefinition.
    let frozen_1 = ParameterObjectFrozen::new(
        ParameterObjectDefinitionBuilder::new("Test")
            .with(
                ParameterKey::new("p_p1".into()).unwrap(),
                StringDefinition::new("D1"),
            )
            .with(
                ParameterKey::new("p_p2".into()).unwrap(),
                FileDefinition::new("D2", "ext", false),
            )
            .with(
                ParameterKey::new("p_p3".into()).unwrap(),
                NumberDefinition::new("D3"),
            )
            .with(
                ParameterKey::new("p_p4".into()).unwrap(),
                ChoiceDefinition::new(
                    "D4",
                    vec![
                        ChoiceItemDefinition::new(store_key!("option_1"), "Option 1"),
                        ChoiceItemDefinition::new(store_key!("option_2"), "Option 2"),
                    ],
                ),
            )
            .with(
                ParameterKey::new("p_p5".into()).unwrap(),
                TableDefinition::new(
                    "D5",
                    vec![
                        (store_key!("col1"), NumberDefinition::new("C1")),
                        (store_key!("col2"), NumberDefinition::new("C2")),
                    ],
                ),
            )
            .with(
                ParameterKey::new("p_p6".into()).unwrap(),
                MapDefinition::new(
                    "D6",
                    vec![
                        (
                            store_key!("col1"),
                            MapItemDefinition::String(StringDefinition::new("C1")),
                        ),
                        (
                            store_key!("col2"),
                            MapItemDefinition::Number(NumberDefinition::new("C2")),
                        ),
                    ],
                ),
            )
            .finish(),
    );

    assert_eq!(
        format!("{}", frozen_1),
        "Frozen Parameter Object (Test)\n    ├── p_p1 (D1) String - \"\"\n    ├── p_p2 (D2) File - \"\"\n    ├── p_p3 (D3) Number - \"\"\n    ├── p_p4 (D4) Choice - \"\"\n    ├── p_p5 (D5) Table 0 rows\n    └── p_p6 (D6) Map\n"
    );
}
