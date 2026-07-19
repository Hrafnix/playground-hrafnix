use datastore::prelude::*;

#[test]
fn test_object_definition_basic() {
    // Why: Test object definition creation and items.
    let mut builder = ObjectDefinition::builder("Test Object");
    builder.insert(
        GlobalKey::new("g_p1".into()).unwrap(),
        StringDefinition::new("D1"),
    );
    let obj_def = builder.finish();

    assert_eq!(obj_def.description().as_ref(), "Test Object");
    assert_eq!(obj_def.count(), 1);
    assert!(obj_def.contains("g_p1"));
    assert!(obj_def.contains_str("g_p1"));
}

#[test]
fn test_object_definition_equality() {
    // Why: Test that two object definitions with the same items are considered equal.
    let def_1 = ObjectDefinition::builder("Test Object")
        .with(
            GlobalKey::new("g_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();
    let def_2 = ObjectDefinition::builder("Test Object")
        .with(
            GlobalKey::new("g_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .finish();
    let def_3 = ObjectDefinition::builder("Test Object")
        .with(
            GlobalKey::new("g_p1".into()).unwrap(),
            StringDefinition::new("D2"),
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
            GlobalKey::new("g_p1".into()).unwrap(),
            StringDefinition::new("D1"),
        )
        .with(
            GlobalKey::new("g_p2".into()).unwrap(),
            FileDefinition::new("D2", "ext", false),
        )
        .with(
            GlobalKey::new("g_p3".into()).unwrap(),
            NumberDefinition::new("D3"),
        )
        .with(
            GlobalKey::new("g_p4".into()).unwrap(),
            ChoiceDefinition::new(
                "D4",
                vec![
                    ChoiceItemDefinition::new(store_key!("option_1"), "Option 1"),
                    ChoiceItemDefinition::new(store_key!("option_2"), "Option 2"),
                ],
            ),
        )
        .with(
            GlobalKey::new("g_p5".into()).unwrap(),
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
        )
        .with(
            GlobalKey::new("g_p6".into()).unwrap(),
            MapDefinition::new(
                "D6",
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
                                value: 1.0,
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
        "Object Definition (Test)\n    ├── g_p1 (D1) String - default: \"\"\n    ├── g_p2 (D2) File - default: \"\" [ext]\n    ├── g_p3 (D3) Number - default: \"\"\n    ├── g_p4 (D4) Choice - default: \"\" [option_1 (Option 1), option_2 (Option 2)]\n    ├── g_p5 (D5) Table\n    │   ├── col1 (C1) Number - default: \"\"\n    │   └── col2 (C2) Number - default: \"\" [Min(1.52, inclusive)]\n    └── g_p6 (D6) Map\n        ├── col1 (C1) String - default: \"\"\n        └── col2 (C2) Number - default: \"\" [Max(1.0, inclusive)]\n"
    );
}
