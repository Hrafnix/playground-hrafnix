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
            VariableKey::new("v_p4_v1".into()).unwrap(),
            IntegerDefinition::new("D4"),
        )
        .with(
            VariableKey::new("v_p4_v2".into()).unwrap(),
            IntegerDefinition::new_with_constraint("D4", IntegerConstraint::min(0, true)),
        )
        .with(
            VariableKey::new("v_p4_v3".into()).unwrap(),
            IntegerDefinition::new_with_constraint("D4", IntegerConstraint::min(20, false)),
        )
        .with(
            VariableKey::new("v_p4_v4".into()).unwrap(),
            IntegerDefinition::new_with_constraint("D4", IntegerConstraint::max(10, true)),
        )
        .with(
            VariableKey::new("v_p4_v5".into()).unwrap(),
            IntegerDefinition::new_with_constraint(
                "D4",
                IntegerConstraint::range(0, 10, true, true),
            ),
        )
        .with(
            VariableKey::new("v_p4_v6".into()).unwrap(),
            IntegerDefinition::new_with_constraint(
                "D4",
                IntegerConstraint::range(32, 80, false, true),
            ),
        )
        .with(
            VariableKey::new("v_p4_v7".into()).unwrap(),
            IntegerDefinition::new_with_constraint(
                "D4",
                IntegerConstraint::range(10, 150, false, true),
            ),
        )
        .with(
            VariableKey::new("v_p4_v8".into()).unwrap(),
            IntegerDefinition::new_with_constraint(
                "D4",
                IntegerConstraint::range(40, 100, false, false),
            ),
        )
        .with(
            VariableKey::new("v_p5_v1".into()).unwrap(),
            NumberDefinition::new("D5"),
        )
        .with(
            VariableKey::new("v_p5_v2".into()).unwrap(),
            NumberDefinition::new_with_constraint("D5", NumberConstraint::min(1.0, true)),
        )
        .with(
            VariableKey::new("v_p5_v3".into()).unwrap(),
            NumberDefinition::new_with_constraint("D5", NumberConstraint::max(21.0, false)),
        )
        .with(
            VariableKey::new("v_p5_v4".into()).unwrap(),
            NumberDefinition::new_with_constraint("D5", NumberConstraint::max(11.0, true)),
        )
        .with(
            VariableKey::new("v_p5_v5".into()).unwrap(),
            NumberDefinition::new_with_constraint("D5", NumberConstraint::max(100.0, false)),
        )
        .with(
            VariableKey::new("v_p5_v6".into()).unwrap(),
            NumberDefinition::new_with_constraint(
                "D5",
                NumberConstraint::range(2.0, 12.0, true, false),
            ),
        )
        .with(
            VariableKey::new("v_p5_v7".into()).unwrap(),
            NumberDefinition::new_with_constraint(
                "D5",
                NumberConstraint::range(3.0, 99.0, false, false),
            ),
        )
        .with(
            VariableKey::new("v_p5_v8".into()).unwrap(),
            NumberDefinition::new_with_constraint(
                "D5",
                NumberConstraint::range(5.0, 70.0, false, true),
            ),
        )
        .with(
            VariableKey::new("v_p5_v9".into()).unwrap(),
            NumberDefinition::new_with_constraint(
                "D5",
                NumberConstraint::range(6.0, 1200.0, true, true),
            ),
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
                            NumberConstraint::min(1.52, true),
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
                            NumberConstraint::max(1.0, true),
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
                                        NumberConstraint::range(0.0, 10.0, true, false),
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
        "Variable Object Definition (Test)\n    ├── v_p1 (D1) String - default: \"\"\n    ├── v_p2 (D2) Boolean - default: \"\" [true (True), false (False)]\n    ├── v_p3 (D3) File - default: \"\" [ext]\n    ├── v_p4_v1 (D4) Integer - default: \"\"\n    ├── v_p4_v2 (D4) Integer - default: \"\" [Min(0, inclusive)]\n    ├── v_p4_v3 (D4) Integer - default: \"\" [Min(20, exclusive)]\n    ├── v_p4_v4 (D4) Integer - default: \"\" [Max(10, inclusive)]\n    ├── v_p4_v5 (D4) Integer - default: \"\" [Range(0, 10, inclusive, inclusive)]\n    ├── v_p4_v6 (D4) Integer - default: \"\" [Range(32, 80, exclusive, inclusive)]\n    ├── v_p4_v7 (D4) Integer - default: \"\" [Range(10, 150, exclusive, inclusive)]\n    ├── v_p4_v8 (D4) Integer - default: \"\" [Range(40, 100, exclusive, exclusive)]\n    ├── v_p5_v1 (D5) Number - default: \"\"\n    ├── v_p5_v2 (D5) Number - default: \"\" [Min(1.0, inclusive)]\n    ├── v_p5_v3 (D5) Number - default: \"\" [Max(21.0, exclusive)]\n    ├── v_p5_v4 (D5) Number - default: \"\" [Max(11.0, inclusive)]\n    ├── v_p5_v5 (D5) Number - default: \"\" [Max(100.0, exclusive)]\n    ├── v_p5_v6 (D5) Number - default: \"\" [Range(2.0, 12.0, inclusive, exclusive)]\n    ├── v_p5_v7 (D5) Number - default: \"\" [Range(3.0, 99.0, exclusive, exclusive)]\n    ├── v_p5_v8 (D5) Number - default: \"\" [Range(5.0, 70.0, exclusive, inclusive)]\n    ├── v_p5_v9 (D5) Number - default: \"\" [Range(6.0, 1200.0, inclusive, inclusive)]\n    ├── v_p6 (D6) Choice - default: \"\" [option_1 (Option 1), option_2 (Option 2)]\n    ├── v_p7 (D7) Table\n    │   ├── col1 (C1) Number - default: \"\"\n    │   └── col2 (C2) Number - default: \"\" [Min(1.52, inclusive)]\n    └── v_p8 (D8) Map\n        ├── col1 (C1) String - default: \"\"\n        ├── col2 (C2) Number - default: \"\" [Max(1.0, inclusive)]\n        └── col3 (C3) Table\n            ├── col3_1 (C3_1) Number - default: \"\"\n            └── col3_2 (C3_2) Number - default: \"\" [Range(0.0, 10.0, inclusive, exclusive)]\n"
    );
}
