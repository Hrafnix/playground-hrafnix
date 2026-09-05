use datastore::definition::{FolderDefinition, SeparatorDefinition, TabDefinition};
use datastore::prelude::*;
use units::{UnitFamilyId, UnitId};

#[test]
fn test_object_frozen_basic() {
    // Why: Test frozen object creation and items.
    let frozen_1 = GlobalObjectFrozen::new(
        GlobalObjectDefinition::builder("Test Object")
            .with(
                GlobalKey::new("g_p1".into()).unwrap(),
                StringDefinition::new("D1"),
            )
            .finish(),
    );

    assert_eq!(frozen_1.definition().description().as_ref(), "Test Object");
    assert_eq!(frozen_1.definition().count(), 1);
    assert!(frozen_1.definition().contains("g_p1"));
    assert!(frozen_1.definition().contains_str("g_p1"));
    assert_ne!(frozen_1.hash(), [0u8; 32]);
}

#[test]
fn test_object_frozen_ordered_iter_and_print() {
    let frozen = GlobalObjectFrozen::new(
        GlobalObjectDefinition::builder("Ordered")
            .with(
                GlobalKey::new("g_z".into()).unwrap(),
                StringDefinition::new("First"),
            )
            .with(
                GlobalKey::new("g_a".into()).unwrap(),
                StringDefinition::new("Second"),
            )
            .finish(),
    );

    assert_eq!(
        frozen
            .ordered_iter()
            .map(|(key, _)| key.as_str())
            .collect::<Vec<_>>(),
        ["g_z", "g_a"]
    );
    assert_eq!(
        format!("{frozen}"),
        concat!(
            "Global Object Frozen (Ordered)\n",
            "    ├── g_z (First) String - \"\"\n",
            "    └── g_a (Second) String - \"\"\n",
        )
    );
}

#[test]
fn test_object_frozen_equality() {
    // Why: Test that two frozen objects with the same items are considered equal.
    let frozen_1 = GlobalObjectFrozen::new(
        GlobalObjectDefinition::builder("Test Object")
            .with(
                GlobalKey::new("g_p1".into()).unwrap(),
                StringDefinition::new("D1"),
            )
            .finish(),
    );
    let frozen_2 = GlobalObjectFrozen::new(
        GlobalObjectDefinition::builder("Test Object")
            .with(
                GlobalKey::new("g_p1".into()).unwrap(),
                StringDefinition::new("D1"),
            )
            .finish(),
    );
    let frozen_3 = GlobalObjectFrozen::new(
        GlobalObjectDefinition::builder("Test Object")
            .with(
                GlobalKey::new("g_p1".into()).unwrap(),
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
fn test_editable_global_object_print() {
    // Why: Test global object editable print.
    let frozen_1 = GlobalObjectFrozen::new(
        GlobalObjectDefinitionBuilder::new("Test")
            .with(
                GlobalKey::new("g_p1".into()).unwrap(),
                StringDefinition::new("D1"),
            )
            .with(
                GlobalKey::new("g_p2".into()).unwrap(),
                BooleanDefinition::new("D2"),
            )
            .with(
                GlobalKey::new("g_p3".into()).unwrap(),
                FileDefinition::new("D3", "ext", false),
            )
            .with(
                GlobalKey::new("g_p4_v1".into()).unwrap(),
                IntegerDefinition::new("D4"),
            )
            .with(
                GlobalKey::new("g_p4_v2".into()).unwrap(),
                IntegerDefinition::new_with_constraint("D4", IntegerConstraint::min(0, true)),
            )
            .with(
                GlobalKey::new("g_p4_v3".into()).unwrap(),
                IntegerDefinition::new_with_constraint("D4", IntegerConstraint::min(20, false)),
            )
            .with(
                GlobalKey::new("g_p4_v4".into()).unwrap(),
                IntegerDefinition::new_with_constraint("D4", IntegerConstraint::max(10, true)),
            )
            .with(
                GlobalKey::new("g_p4_v5".into()).unwrap(),
                IntegerDefinition::new_with_constraint(
                    "D4",
                    IntegerConstraint::range(0, 10, true, true),
                ),
            )
            .with(
                GlobalKey::new("g_p4_v6".into()).unwrap(),
                IntegerDefinition::new_with_constraint(
                    "D4",
                    IntegerConstraint::range(32, 80, false, true),
                ),
            )
            .with(
                GlobalKey::new("g_p4_v7".into()).unwrap(),
                IntegerDefinition::new_with_constraint(
                    "D4",
                    IntegerConstraint::range(10, 150, false, true),
                ),
            )
            .with(
                GlobalKey::new("g_p4_v8".into()).unwrap(),
                IntegerDefinition::new_with_constraint(
                    "D4",
                    IntegerConstraint::range(40, 100, false, false),
                ),
            )
            .with(
                GlobalKey::new("g_p5_v1".into()).unwrap(),
                NumberDefinition::new("D5"),
            )
            .with(
                GlobalKey::new("g_p5_v2".into()).unwrap(),
                NumberDefinition::new_with_constraint("D5", NumberConstraint::min(1.0, true)),
            )
            .with(
                GlobalKey::new("g_p5_v3".into()).unwrap(),
                NumberDefinition::new_with_constraint("D5", NumberConstraint::max(21.0, false)),
            )
            .with(
                GlobalKey::new("g_p5_v4".into()).unwrap(),
                NumberDefinition::new_with_constraint("D5", NumberConstraint::max(11.0, true)),
            )
            .with(
                GlobalKey::new("g_p5_v5".into()).unwrap(),
                NumberDefinition::new_with_constraint("D5", NumberConstraint::max(100.0, false)),
            )
            .with(
                GlobalKey::new("g_p5_v6".into()).unwrap(),
                NumberDefinition::new_with_constraint(
                    "D5",
                    NumberConstraint::range(2.0, 12.0, true, false),
                ),
            )
            .with(
                GlobalKey::new("g_p5_v7".into()).unwrap(),
                NumberDefinition::new_with_constraint(
                    "D5",
                    NumberConstraint::range(3.0, 99.0, false, false),
                ),
            )
            .with(
                GlobalKey::new("g_p5_v8".into()).unwrap(),
                NumberDefinition::new_with_constraint(
                    "D5",
                    NumberConstraint::range(5.0, 70.0, false, true),
                ),
            )
            .with(
                GlobalKey::new("g_p5_v9".into()).unwrap(),
                NumberDefinition::new_with_constraint(
                    "D5",
                    NumberConstraint::range(6.0, 1200.0, true, true),
                ),
            )
            .with(
                GlobalKey::new("g_p6".into()).unwrap(),
                ChoiceDefinition::new(
                    "D6",
                    vec![
                        ChoiceItemDefinition::new(store_key!("option_1"), "Option 1"),
                        ChoiceItemDefinition::new(store_key!("option_2"), "Option 2"),
                    ],
                ),
            )
            .with(
                GlobalKey::new("g_p7".into()).unwrap(),
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
                GlobalKey::new("g_p8".into()).unwrap(),
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
            .with(
                GlobalKey::new("g_p9".into()).unwrap(),
                FolderDefinition::new("D9", true),
            )
            .with(
                GlobalKey::new("g_p10".into()).unwrap(),
                NumberWithUnitsDefinition::new("D10", UnitId::Length_Meter),
            )
            .with(
                GlobalKey::new("g_p11".into()).unwrap(),
                SeparatorDefinition::new("D11"),
            )
            .with(
                GlobalKey::new("g_p12".into()).unwrap(),
                TabDefinition::new("D12"),
            )
            .with(
                GlobalKey::new("g_p13".into()).unwrap(),
                UnitDefinition::new("D13", UnitFamilyId::Length),
            )
            .finish(),
    );

    assert_eq!(
        format!("{frozen_1}"),
        concat!(
            "Global Object Frozen (Test)\n",
            "    ├── g_p1 (D1) String - \"\"\n",
            "    ├── g_p2 (D2) Boolean - \"\"\n",
            "    ├── g_p3 (D3) File - \"\"\n",
            "    ├── g_p4_v1 (D4) Integer - \"\"\n",
            "    ├── g_p4_v2 (D4) Integer - \"\"\n",
            "    ├── g_p4_v3 (D4) Integer - \"\"\n",
            "    ├── g_p4_v4 (D4) Integer - \"\"\n",
            "    ├── g_p4_v5 (D4) Integer - \"\"\n",
            "    ├── g_p4_v6 (D4) Integer - \"\"\n",
            "    ├── g_p4_v7 (D4) Integer - \"\"\n",
            "    ├── g_p4_v8 (D4) Integer - \"\"\n",
            "    ├── g_p5_v1 (D5) Number - \"\"\n",
            "    ├── g_p5_v2 (D5) Number - \"\"\n",
            "    ├── g_p5_v3 (D5) Number - \"\"\n",
            "    ├── g_p5_v4 (D5) Number - \"\"\n",
            "    ├── g_p5_v5 (D5) Number - \"\"\n",
            "    ├── g_p5_v6 (D5) Number - \"\"\n",
            "    ├── g_p5_v7 (D5) Number - \"\"\n",
            "    ├── g_p5_v8 (D5) Number - \"\"\n",
            "    ├── g_p5_v9 (D5) Number - \"\"\n",
            "    ├── g_p6 (D6) Choice - \"\"\n",
            "    ├── g_p7 (D7) Table 0 rows\n",
            "    │   ├── data\n",
            "    │   └── Parameter \"\"\n",
            "    ├── g_p8 (D8) Map\n",
            "    ├── g_p9 (D9) Folder - \"\"\n",
            "    ├── g_p10 (D10) Number - \"\"\n",
            "    ├── g_p11 (D11) Separator\n",
            "    ├── g_p12 (D12) Tab\n",
            "    └── g_p13 (D13) Unit - \"\"\n",
        )
    );

    let mut editable_1 = frozen_1.thaw();

    editable_set_value(&mut editable_1, "g_p1", "edited").unwrap();
    editable_set_value(&mut editable_1, "g_p2", "true").unwrap();
    editable_set_value(&mut editable_1, "g_p3", "test.ext").unwrap();
    editable_set_value(&mut editable_1, "g_p4_v1", "1").unwrap();
    editable_set_value(&mut editable_1, "g_p4_v2", "2").unwrap();
    editable_set_value(&mut editable_1, "g_p4_v3", "3").unwrap();
    editable_set_value(&mut editable_1, "g_p4_v4", "4").unwrap();
    editable_set_value(&mut editable_1, "g_p4_v5", "5").unwrap();
    editable_set_value(&mut editable_1, "g_p4_v6", "6").unwrap();
    editable_set_value(&mut editable_1, "g_p4_v7", "7").unwrap();
    editable_set_value(&mut editable_1, "g_p4_v8", "8").unwrap();
    editable_set_value(&mut editable_1, "g_p5_v1", "1.0").unwrap();
    editable_set_value(&mut editable_1, "g_p5_v2", "2.0").unwrap();
    editable_set_value(&mut editable_1, "g_p5_v3", "3.0").unwrap();
    editable_set_value(&mut editable_1, "g_p5_v4", "4.0").unwrap();
    editable_set_value(&mut editable_1, "g_p5_v5", "5.0").unwrap();
    editable_set_value(&mut editable_1, "g_p5_v6", "6.0").unwrap();
    editable_set_value(&mut editable_1, "g_p5_v7", "7.0").unwrap();
    editable_set_value(&mut editable_1, "g_p5_v8", "8.0").unwrap();
    editable_set_value(&mut editable_1, "g_p5_v9", "9.0").unwrap();
    editable_set_value(&mut editable_1, "g_p6", "test").unwrap();

    let table = editable_1.get_mut("g_p7").unwrap().get_mut_table().unwrap();
    table.add_row(1);
    table.set_cell(0, "col1", "100.0").unwrap();
    table.set_cell(0, "col2", "200.0").unwrap();

    let map = editable_1.get_mut("g_p8").unwrap().get_mut_map().unwrap();
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

    editable_set_value(&mut editable_1, "g_p9", "output/files").unwrap();
    editable_set_value(&mut editable_1, "g_p10", "10.0").unwrap();
    editable_set_value(
        &mut editable_1,
        "g_p13",
        UnitId::Length_Centimeter.string_id().as_str(),
    )
    .unwrap();

    let frozen_1 = editable_1.freeze();

    assert_eq!(
        format!("{frozen_1}"),
        concat!(
            "Global Object Frozen (Test)\n",
            "    ├── g_p1 (D1) String - \"edited\"\n",
            "    ├── g_p2 (D2) Boolean - \"true\"\n",
            "    ├── g_p3 (D3) File - \"test.ext\"\n",
            "    ├── g_p4_v1 (D4) Integer - \"1\"\n",
            "    ├── g_p4_v2 (D4) Integer - \"2\"\n",
            "    ├── g_p4_v3 (D4) Integer - \"3\"\n",
            "    ├── g_p4_v4 (D4) Integer - \"4\"\n",
            "    ├── g_p4_v5 (D4) Integer - \"5\"\n",
            "    ├── g_p4_v6 (D4) Integer - \"6\"\n",
            "    ├── g_p4_v7 (D4) Integer - \"7\"\n",
            "    ├── g_p4_v8 (D4) Integer - \"8\"\n",
            "    ├── g_p5_v1 (D5) Number - \"1.0\"\n",
            "    ├── g_p5_v2 (D5) Number - \"2.0\"\n",
            "    ├── g_p5_v3 (D5) Number - \"3.0\"\n",
            "    ├── g_p5_v4 (D5) Number - \"4.0\"\n",
            "    ├── g_p5_v5 (D5) Number - \"5.0\"\n",
            "    ├── g_p5_v6 (D5) Number - \"6.0\"\n",
            "    ├── g_p5_v7 (D5) Number - \"7.0\"\n",
            "    ├── g_p5_v8 (D5) Number - \"8.0\"\n",
            "    ├── g_p5_v9 (D5) Number - \"9.0\"\n",
            "    ├── g_p6 (D6) Choice - \"test\"\n",
            "    ├── g_p7 (D7) Table 1 rows\n",
            "    │   ├── data\n",
            "    │   │   └── Row 0\n",
            "    │   │       ├── col1 \"100.0\"\n",
            "    │   │       └── col2 \"200.0\"\n",
            "    │   └── Parameter \"\"\n",
            "    ├── g_p8 (D8) Map\n",
            "    │   └── key1\n",
            "    │       ├── col1 (C1) String - \"test 1\"\n",
            "    │       ├── col2 (C2) Number - \"55.0\"\n",
            "    │       └── col3 (C3) Table 1 rows\n",
            "    │           ├── data\n",
            "    │           │   └── Row 0\n",
            "    │           │       ├── col3_1 \"150.0\"\n",
            "    │           │       └── col3_2 \"250.0\"\n",
            "    │           └── Parameter \"\"\n",
            "    ├── g_p9 (D9) Folder - \"output/files\"\n",
            "    ├── g_p10 (D10) Number - \"10.0\"\n",
            "    ├── g_p11 (D11) Separator\n",
            "    ├── g_p12 (D12) Tab\n",
            "    └── g_p13 (D13) Unit - \"u_length_centimeter\"\n",
        )
    );
}
