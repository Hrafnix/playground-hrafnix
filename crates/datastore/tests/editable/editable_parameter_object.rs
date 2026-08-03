use datastore::prelude::*;

#[test]
fn test_editable_parameter_object_round_trip() {
    // Why: Editable parameter objects should thaw from frozen, allow item edits, and freeze back
    // to an equivalent frozen object reflecting those edits.
    let frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                ParameterKey::new("p_p1".into()).unwrap(),
                StringDefinition::new("D1"),
            )
            .finish(),
    );

    let mut editable = frozen.thaw();
    assert_eq!(editable.definition().description().as_ref(), "Test Object");

    let item = editable.get("p_p1").expect("p_p1 item");
    assert_eq!(item.get_string().unwrap().value(), "");

    if let ItemEditable::String(string_editable) = editable.get_mut("p_p1").expect("p_p1 item") {
        string_editable.set("edited");
    }

    let frozen_2 = editable.freeze();
    assert_ne!(frozen_2.hash(), frozen.hash());
    assert_eq!(
        frozen_2.get("p_p1").unwrap().get_string().unwrap().value(),
        "edited"
    );
}

#[test]
fn test_editable_parameter_object_equality() {
    // Why: Two editable parameter objects thawed from the same frozen object should be equal.
    let frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                ParameterKey::new("p_p1".into()).unwrap(),
                StringDefinition::new("D1"),
            )
            .finish(),
    );

    let editable_1 = frozen.thaw();
    let editable_2 = frozen.thaw();
    assert_eq!(editable_1, editable_2);
    assert_eq!(&editable_1, editable_2);
}

#[test]
fn test_editable_parameter_object_print() {
    // Why: Test parameter object editable print.
    let frozen_1 = ParameterObjectFrozen::new(
        ParameterObjectDefinitionBuilder::new("Test")
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
                ParameterKey::new("p_p4_v1".into()).unwrap(),
                IntegerDefinition::new("D4"),
            )
            .with(
                ParameterKey::new("p_p4_v2".into()).unwrap(),
                IntegerDefinition::new_with_constraint("D4", IntegerConstraint::min(0, true)),
            )
            .with(
                ParameterKey::new("p_p4_v3".into()).unwrap(),
                IntegerDefinition::new_with_constraint("D4", IntegerConstraint::min(20, false)),
            )
            .with(
                ParameterKey::new("p_p4_v4".into()).unwrap(),
                IntegerDefinition::new_with_constraint("D4", IntegerConstraint::max(10, true)),
            )
            .with(
                ParameterKey::new("p_p4_v5".into()).unwrap(),
                IntegerDefinition::new_with_constraint(
                    "D4",
                    IntegerConstraint::range(0, 10, true, true),
                ),
            )
            .with(
                ParameterKey::new("p_p4_v6".into()).unwrap(),
                IntegerDefinition::new_with_constraint(
                    "D4",
                    IntegerConstraint::range(32, 80, false, true),
                ),
            )
            .with(
                ParameterKey::new("p_p4_v7".into()).unwrap(),
                IntegerDefinition::new_with_constraint(
                    "D4",
                    IntegerConstraint::range(10, 150, false, true),
                ),
            )
            .with(
                ParameterKey::new("p_p4_v8".into()).unwrap(),
                IntegerDefinition::new_with_constraint(
                    "D4",
                    IntegerConstraint::range(40, 100, false, false),
                ),
            )
            .with(
                ParameterKey::new("p_p5_v1".into()).unwrap(),
                NumberDefinition::new("D5"),
            )
            .with(
                ParameterKey::new("p_p5_v2".into()).unwrap(),
                NumberDefinition::new_with_constraint("D5", NumberConstraint::min(1.0, true)),
            )
            .with(
                ParameterKey::new("p_p5_v3".into()).unwrap(),
                NumberDefinition::new_with_constraint("D5", NumberConstraint::max(21.0, false)),
            )
            .with(
                ParameterKey::new("p_p5_v4".into()).unwrap(),
                NumberDefinition::new_with_constraint("D5", NumberConstraint::max(11.0, true)),
            )
            .with(
                ParameterKey::new("p_p5_v5".into()).unwrap(),
                NumberDefinition::new_with_constraint("D5", NumberConstraint::max(100.0, false)),
            )
            .with(
                ParameterKey::new("p_p5_v6".into()).unwrap(),
                NumberDefinition::new_with_constraint(
                    "D5",
                    NumberConstraint::range(2.0, 12.0, true, false),
                ),
            )
            .with(
                ParameterKey::new("p_p5_v7".into()).unwrap(),
                NumberDefinition::new_with_constraint(
                    "D5",
                    NumberConstraint::range(3.0, 99.0, false, false),
                ),
            )
            .with(
                ParameterKey::new("p_p5_v8".into()).unwrap(),
                NumberDefinition::new_with_constraint(
                    "D5",
                    NumberConstraint::range(5.0, 70.0, false, true),
                ),
            )
            .with(
                ParameterKey::new("p_p5_v9".into()).unwrap(),
                NumberDefinition::new_with_constraint(
                    "D5",
                    NumberConstraint::range(6.0, 1200.0, true, true),
                ),
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
                                NumberConstraint::min(1.52, true),
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
            .finish(),
    );
    let mut editable_1 = frozen_1.thaw();

    assert_eq!(
        format!("{editable_1}"),
        "Parameter Object Editable (Test)\n    ├── p_p1 (D1) String - \"\"\n    ├── p_p2 (D2) Boolean - \"\"\n    ├── p_p3 (D3) File - \"\"\n    ├── p_p4_v1 (D4) Integer - \"\"\n    ├── p_p4_v2 (D4) Integer - \"\"\n    ├── p_p4_v3 (D4) Integer - \"\"\n    ├── p_p4_v4 (D4) Integer - \"\"\n    ├── p_p4_v5 (D4) Integer - \"\"\n    ├── p_p4_v6 (D4) Integer - \"\"\n    ├── p_p4_v7 (D4) Integer - \"\"\n    ├── p_p4_v8 (D4) Integer - \"\"\n    ├── p_p5_v1 (D5) Number - \"\"\n    ├── p_p5_v2 (D5) Number - \"\"\n    ├── p_p5_v3 (D5) Number - \"\"\n    ├── p_p5_v4 (D5) Number - \"\"\n    ├── p_p5_v5 (D5) Number - \"\"\n    ├── p_p5_v6 (D5) Number - \"\"\n    ├── p_p5_v7 (D5) Number - \"\"\n    ├── p_p5_v8 (D5) Number - \"\"\n    ├── p_p5_v9 (D5) Number - \"\"\n    ├── p_p6 (D6) Choice - \"\"\n    ├── p_p7 (D7) Table 0 rows\n    │   ├── data\n    │   └── Parameter \"\"\n    └── p_p8 (D8) Map\n"
    );

    editable_1
        .get_mut("p_p1")
        .unwrap()
        .get_mut_string()
        .unwrap()
        .set("edited");
    editable_1
        .get_mut("p_p2")
        .unwrap()
        .get_mut_boolean()
        .unwrap()
        .set("true");
    editable_1
        .get_mut("p_p3")
        .unwrap()
        .get_mut_file()
        .unwrap()
        .set("test.ext");
    editable_1
        .get_mut("p_p4_v1")
        .unwrap()
        .get_mut_integer()
        .unwrap()
        .set("1");
    editable_1
        .get_mut("p_p4_v2")
        .unwrap()
        .get_mut_integer()
        .unwrap()
        .set("2");
    editable_1
        .get_mut("p_p4_v3")
        .unwrap()
        .get_mut_integer()
        .unwrap()
        .set("3");
    editable_1
        .get_mut("p_p4_v4")
        .unwrap()
        .get_mut_integer()
        .unwrap()
        .set("4");
    editable_1
        .get_mut("p_p4_v5")
        .unwrap()
        .get_mut_integer()
        .unwrap()
        .set("5");
    editable_1
        .get_mut("p_p4_v6")
        .unwrap()
        .get_mut_integer()
        .unwrap()
        .set("6");
    editable_1
        .get_mut("p_p4_v7")
        .unwrap()
        .get_mut_integer()
        .unwrap()
        .set("7");
    editable_1
        .get_mut("p_p4_v8")
        .unwrap()
        .get_mut_integer()
        .unwrap()
        .set("8");
    editable_1
        .get_mut("p_p5_v1")
        .unwrap()
        .get_mut_number()
        .unwrap()
        .set("1.0");
    editable_1
        .get_mut("p_p5_v2")
        .unwrap()
        .get_mut_number()
        .unwrap()
        .set("2.0");
    editable_1
        .get_mut("p_p5_v3")
        .unwrap()
        .get_mut_number()
        .unwrap()
        .set("3.0");
    editable_1
        .get_mut("p_p5_v4")
        .unwrap()
        .get_mut_number()
        .unwrap()
        .set("4.0");
    editable_1
        .get_mut("p_p5_v5")
        .unwrap()
        .get_mut_number()
        .unwrap()
        .set("5.0");
    editable_1
        .get_mut("p_p5_v6")
        .unwrap()
        .get_mut_number()
        .unwrap()
        .set("6.0");
    editable_1
        .get_mut("p_p5_v7")
        .unwrap()
        .get_mut_number()
        .unwrap()
        .set("7.0");
    editable_1
        .get_mut("p_p5_v8")
        .unwrap()
        .get_mut_number()
        .unwrap()
        .set("8.0");
    editable_1
        .get_mut("p_p5_v9")
        .unwrap()
        .get_mut_number()
        .unwrap()
        .set("9.0");
    editable_1
        .get_mut("p_p6")
        .unwrap()
        .get_mut_choice()
        .unwrap()
        .set("test");

    let table = editable_1.get_mut("p_p7").unwrap().get_mut_table().unwrap();
    table.add_row(1);
    table.set_cell(0, "col1", "100.0").unwrap();
    table.set_cell(0, "col2", "200.0").unwrap();

    let map = editable_1.get_mut("p_p8").unwrap().get_mut_map().unwrap();
    map.create(store_key!("key1"));

    let item = map.get_mut("key1").unwrap();
    item.get_mut("col1")
        .unwrap()
        .get_mut_string()
        .unwrap()
        .set("test 1");
    item.get_mut("col2")
        .unwrap()
        .get_mut_number()
        .unwrap()
        .set("55.0");
    let map_table = item.get_mut("col3").unwrap().get_mut_table().unwrap();
    map_table.add_row(1);
    map_table.set_cell(0, "col3_1", "150.0").unwrap();
    map_table.set_cell(0, "col3_2", "250.0").unwrap();

    assert_eq!(
        format!("{editable_1}"),
        "Parameter Object Editable (Test)\n    ├── p_p1 (D1) String - \"edited\"\n    ├── p_p2 (D2) Boolean - \"true\"\n    ├── p_p3 (D3) File - \"test.ext\"\n    ├── p_p4_v1 (D4) Integer - \"1\"\n    ├── p_p4_v2 (D4) Integer - \"2\"\n    ├── p_p4_v3 (D4) Integer - \"3\"\n    ├── p_p4_v4 (D4) Integer - \"4\"\n    ├── p_p4_v5 (D4) Integer - \"5\"\n    ├── p_p4_v6 (D4) Integer - \"6\"\n    ├── p_p4_v7 (D4) Integer - \"7\"\n    ├── p_p4_v8 (D4) Integer - \"8\"\n    ├── p_p5_v1 (D5) Number - \"1.0\"\n    ├── p_p5_v2 (D5) Number - \"2.0\"\n    ├── p_p5_v3 (D5) Number - \"3.0\"\n    ├── p_p5_v4 (D5) Number - \"4.0\"\n    ├── p_p5_v5 (D5) Number - \"5.0\"\n    ├── p_p5_v6 (D5) Number - \"6.0\"\n    ├── p_p5_v7 (D5) Number - \"7.0\"\n    ├── p_p5_v8 (D5) Number - \"8.0\"\n    ├── p_p5_v9 (D5) Number - \"9.0\"\n    ├── p_p6 (D6) Choice - \"test\"\n    ├── p_p7 (D7) Table 1 rows\n    │   ├── data\n    │   │   └── Row 0\n    │   │       ├── col1 \"100.0\"\n    │   │       └── col2 \"200.0\"\n    │   └── Parameter \"\"\n    └── p_p8 (D8) Map\n        └── key1\n            ├── col1 (C1) String - \"test 1\"\n            ├── col2 (C2) Number - \"55.0\"\n            └── col3 (C3) Table 1 rows\n                ├── data\n                │   └── Row 0\n                │       ├── col3_1 \"150.0\"\n                │       └── col3_2 \"250.0\"\n                └── Parameter \"\"\n"
    );
}
