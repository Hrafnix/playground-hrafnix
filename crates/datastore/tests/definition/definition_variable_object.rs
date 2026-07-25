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
            VariableKey::new("v_v1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .with(
            VariableKey::new("v_v2".into()).unwrap(),
            BooleanDefinition::new("D2"),
        )
        .with(
            VariableKey::new("v_v3".into()).unwrap(),
            FileDefinition::new("D3", "ext", false),
        )
        .with(
            VariableKey::new("v_v4".into()).unwrap(),
            NumberDefinition::new("D4"),
        )
        .with(
            VariableKey::new("v_v5".into()).unwrap(),
            ChoiceDefinition::new(
                "D5",
                vec![
                    ChoiceItemDefinition::new(store_key!("option_1"), "Option 1"),
                    ChoiceItemDefinition::new(store_key!("option_2"), "Option 2"),
                ],
            ),
        )
        .with(
            VariableKey::new("v_v6".into()).unwrap(),
            TableDefinition::new(
                "D6",
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
            VariableKey::new("v_v7".into()).unwrap(),
            MapDefinition::new(
                "D7",
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
                ],
            ),
        )
        .finish();

    assert_eq!(
        format!("{}", def_1),
        "Variable Object Definition (Test)\n    ├── v_v1 (D1) String - default: \"\"\n    ├── v_v2 (D2) Boolean - default: \"\" [true (true), false (false)]\n    ├── v_v3 (D3) File - default: \"\" [ext]\n    ├── v_v4 (D4) Number - default: \"\"\n    ├── v_v5 (D5) Choice - default: \"\" [option_1 (Option 1), option_2 (Option 2)]\n    ├── v_v6 (D6) Table\n    │   ├── col1 (C1) Number - default: \"\"\n    │   └── col2 (C2) Number - default: \"\" [Min(1.52, inclusive)]\n    └── v_v7 (D7) Map\n        ├── col1 (C1) String - default: \"\"\n        └── col2 (C2) Number - default: \"\" [Max(1.0, inclusive)]\n"
    );
}
