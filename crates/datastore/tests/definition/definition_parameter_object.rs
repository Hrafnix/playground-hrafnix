use datastore::prelude::*;

#[test]
fn test_parameter_object_definition_basic() {
    // Why: Test parameter object definition creation and items.
    let mut builder = ParameterObjectDefinition::builder("Test Object");
    builder.insert(
        ParameterKey::new("p_p1".into()).unwrap(),
        StringDefinition::new("D1"),
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
            StringDefinition::new("D1"),
        )
        .finish();
    let def_2 = ParameterObjectDefinition::builder("Test Object")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();
    let def_3 = ParameterObjectDefinition::builder("Test Object")
        .with(
            ParameterKey::new("p_p1".into()).unwrap(),
            StringDefinition::new("D2"),
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
            StringDefinition::new("D1"),
        )
        .with(
            ParameterKey::new("p_p2".into()).unwrap(),
            BooleanDefinition::new("D2"),
        )
        .with(
            ParameterKey::new("p_p3".into()).unwrap(),
            FileDefinition::new("D3", "ext", false),
        )
        .with(
            ParameterKey::new("p_p4".into()).unwrap(),
            IntegerDefinition::new("D4"),
        )
        .with(
            ParameterKey::new("p_p5".into()).unwrap(),
            NumberDefinition::new("D5"),
        )
        .with(
            ParameterKey::new("p_p6".into()).unwrap(),
            ChoiceDefinition::new(
                "D6",
                vec![
                    ChoiceItemDefinition::new(store_key!("option_1"), "Option 1"),
                    ChoiceItemDefinition::new(store_key!("option_2"), "Option 2"),
                ],
            ),
        )
        .with(
            ParameterKey::new("p_p7".into()).unwrap(),
            TableDefinition::new(
                "D7",
                vec![
                    (store_key!("col1"), NumberDefinition::new("C1")),
                    (
                        store_key!("col2"),
                        NumberDefinition::new_with_constraint(
                            "C2",
                            NumberConstraint::Min {
                                min: 1.52,
                                inclusive: true,
                            },
                        ),
                    ),
                ],
            ),
        )
        .with(
            ParameterKey::new("p_p8".into()).unwrap(),
            MapDefinition::new(
                "D8",
                vec![
                    (
                        store_key!("col1"),
                        MapItemDefinition::String(StringDefinition::new("C1")),
                    ),
                    (
                        store_key!("col2"),
                        MapItemDefinition::Number(NumberDefinition::new_with_constraint(
                            "C2",
                            NumberConstraint::Max {
                                max: 1.0,
                                inclusive: true,
                            },
                        )),
                    ),
                    (
                        store_key!("col3"),
                        MapItemDefinition::Table(TableDefinition::new(
                            "C3",
                            vec![
                                (store_key!("col3_1"), NumberDefinition::new("C3_1")),
                                (
                                    store_key!("col3_2"),
                                    NumberDefinition::new_with_constraint(
                                        "C3_2",
                                        NumberConstraint::Range {
                                            min: 0.0,
                                            max: 10.0,
                                            min_inclusive: true,
                                            max_inclusive: false,
                                        },
                                    ),
                                ),
                            ],
                        )),
                    ),
                ],
            ),
        )
        .finish();

    assert_eq!(
        format!("{def_1}"),
        "Parameter Object Definition (Test)\n    ├── p_p1 (D1) String - default: \"\"\n    ├── p_p2 (D2) Boolean - default: \"\" [true (true), false (false)]\n    ├── p_p3 (D3) File - default: \"\" [ext]\n    ├── p_p4 (D4) Integer - default: \"\"\n    ├── p_p5 (D5) Number - default: \"\"\n    ├── p_p6 (D6) Choice - default: \"\" [option_1 (Option 1), option_2 (Option 2)]\n    ├── p_p7 (D7) Table\n    │   ├── col1 (C1) Number - default: \"\"\n    │   └── col2 (C2) Number - default: \"\" [Min(1.52, inclusive)]\n    └── p_p8 (D8) Map\n        ├── col1 (C1) String - default: \"\"\n        ├── col2 (C2) Number - default: \"\" [Max(1.0, inclusive)]\n        └── col3 (C3) Table\n            ├── col3_1 (C3_1) Number - default: \"\"\n            └── col3_2 (C3_2) Number - default: \"\" [Range(0.0, 10.0, inclusive, exclusive)]\n"
    );
}
