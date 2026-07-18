use datastore::definition::{
    BasicDefinition, ChoiceDefinition, FileDefinition, ItemDefinition, MapDefinition,
    ParameterObjectDefinition, ParameterObjectDefinitionBuilder, StructDefinition,
    StructItemDefinition, TableDefinition,
};
use datastore::frozen::frozen_object_parameter::ParameterObjectFrozen;
use datastore::key::ParameterKey;
use datastore::store_key;

#[test]
fn test_parameter_object_definition_basic() {
    // Why: Test frozen parameter object creation and items.
    let frozen_1 = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                ParameterKey::new("p_p1".into()).unwrap(),
                ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
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
                ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
            )
            .finish(),
    );
    let frozen_2 = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                ParameterKey::new("p_p1".into()).unwrap(),
                ItemDefinition::new("P1", BasicDefinition::new_string("D1")),
            )
            .finish(),
    );
    let frozen_3 = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                ParameterKey::new("p_p1".into()).unwrap(),
                ItemDefinition::new("P1", BasicDefinition::new_string("D2")),
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
                ParameterKey::new("p_p7".into()).unwrap(),
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
            .finish(),
    );

    assert_eq!(
        format!("{}", frozen_1),
        "Frozen Parameter Object (Test)\n    ├── p_p1 (D1) String - \"\"\n    ├── p_p2 (D2) File - \"\"\n    ├── p_p3 (D3) Number - \"\"\n    ├── p_p4 (D4) Choice - \"\"\n    ├── p_p5 (D5) Table 0 rows\n    ├── p_p6 (D6) Struct\n    │   ├── f1 (F1) String - \"\"\n    │   └── f2 (T1) Table 0 rows\n    └── p_p7 (D7) Map\n"
    );
}
