use datastore::prelude::*;

#[test]
fn test_variable_object_definition_basic() {
    // Why: Test variable object definition creation and items.
    let mut builder = VariableObjectDefinition::builder("Test Object");
    builder.insert(
        VariableKey::new("v_v1".into()).unwrap(),
        StringDefinition::new("D1"),
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
            StringDefinition::new("D1"),
        )
        .finish();
    let def_2 = VariableObjectDefinition::builder("Test Object")
        .with(
            VariableKey::new("v_v1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();
    let def_3 = VariableObjectDefinition::builder("Test Object")
        .with(
            VariableKey::new("v_v1".into()).unwrap(),
            StringDefinition::new("D2"),
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
            StringDefinition::new("D1"),
        )
        .with(
            VariableKey::new("v_p2".into()).unwrap(),
            BooleanDefinition::new("D2"),
        )
        .with(
            VariableKey::new("v_p3".into()).unwrap(),
            FileDefinition::new("D3", "ext", false),
        )
        .with(
            VariableKey::new("v_p4".into()).unwrap(),
            IntegerDefinition::new("D4"),
        )
        .with(
            VariableKey::new("v_p5".into()).unwrap(),
            NumberDefinition::new("D5"),
        )
        .with(
            VariableKey::new("v_p6".into()).unwrap(),
            ChoiceDefinition::new(
                "D6",
                vec![
                    ChoiceItemDefinition::new(store_key!("option_1"), "Option 1"),
                    ChoiceItemDefinition::new(store_key!("option_2"), "Option 2"),
                ],
            ),
        )
        .with(
            VariableKey::new("v_p7".into()).unwrap(),
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
            VariableKey::new("v_p8".into()).unwrap(),
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
        format!("{}", def_1),
        "Variable Object Definition (Test)\n    ├── v_p1 (D1) String - default: \"\"\n    ├── v_p2 (D2) Boolean - default: \"\" [true (true), false (false)]\n    ├── v_p3 (D3) File - default: \"\" [ext]\n    ├── v_p4 (D4) Integer - default: \"\"\n    ├── v_p5 (D5) Number - default: \"\"\n    ├── v_p6 (D6) Choice - default: \"\" [option_1 (Option 1), option_2 (Option 2)]\n    ├── v_p7 (D7) Table\n    │   ├── col1 (C1) Number - default: \"\"\n    │   └── col2 (C2) Number - default: \"\" [Min(1.52, inclusive)]\n    └── v_p8 (D8) Map\n        ├── col1 (C1) String - default: \"\"\n        ├── col2 (C2) Number - default: \"\" [Max(1.0, inclusive)]\n        └── col3 (C3) Table\n            ├── col3_1 (C3_1) Number - default: \"\"\n            └── col3_2 (C3_2) Number - default: \"\" [Range(0.0, 10.0, inclusive, exclusive)]\n"
    );
}
