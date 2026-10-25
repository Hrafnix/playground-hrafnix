use hrafnix_datastore::prelude::*;
use hrafnix_keys::store_key;
use hrafnix_units::UnitId;

#[test]
fn test_merge_compatible_ignores_descriptions_and_defaults() {
    // Why: Cosmetic differences should not block a merge.
    let a: ItemDefinitionType = NumberDefinition::new_with_default("A", "1").into();
    let b: ItemDefinitionType = NumberDefinition::new_with_default("B", "2").into();
    assert_ne!(a, b);
    assert!(a.is_merge_compatible(&b));

    let a: ItemDefinitionType = StringDefinition::new_with_default("A", "x").into();
    let b: ItemDefinitionType = StringDefinition::new("B").into();
    assert!(a.is_merge_compatible(&b));
}

#[test]
fn test_merge_compatible_rejects_different_variants() {
    // Why: Values of different item types are never interchangeable.
    let a: ItemDefinitionType = NumberDefinition::new("A").into();
    let b: ItemDefinitionType = StringDefinition::new("A").into();
    assert!(!a.is_merge_compatible(&b));
}

#[test]
fn test_merge_compatible_choice_ids() {
    // Why: Choice IDs determine valid values; labels and order do not.
    let a = ChoiceDefinition::new(
        "A",
        vec![
            ChoiceItemDefinition::new(store_key!("a"), "A"),
            ChoiceItemDefinition::new(store_key!("b"), "B"),
        ],
    );
    let b = ChoiceDefinition::new_with_default(
        "Other",
        vec![
            ChoiceItemDefinition::new(store_key!("b"), "Bee"),
            ChoiceItemDefinition::new(store_key!("a"), "Ay"),
        ],
        "a",
    );
    let c = ChoiceDefinition::new("A", vec![ChoiceItemDefinition::new(store_key!("a"), "A")]);
    assert!(a.is_merge_compatible(&b));
    assert!(!a.is_merge_compatible(&c));
}

#[test]
fn test_merge_compatible_number_with_units_family() {
    // Why: Preferred units are display-only; the unit family must still match.
    let a = NumberWithUnitsDefinition::new("A", UnitId::Length_Meter);
    let b = NumberWithUnitsDefinition::new("B", UnitId::Length_Foot);
    let c = NumberWithUnitsDefinition::new("A", UnitId::Area_SquareMeter);
    assert!(a.is_merge_compatible(&b));
    assert!(!a.is_merge_compatible(&c));
}

#[test]
fn test_merge_compatible_table_columns() {
    // Why: Table columns are positional, so keys and order must match.
    let a = TableDefinition::new(
        "A",
        vec![
            (store_key!("x"), NumberDefinition::new("X")),
            (store_key!("y"), NumberDefinition::new("Y")),
        ],
    );
    let b = TableDefinition::new(
        "B",
        vec![
            (store_key!("x"), NumberDefinition::new("X2")),
            (store_key!("y"), NumberDefinition::new("Y2")),
        ],
    );
    let c = TableDefinition::new(
        "A",
        vec![
            (store_key!("y"), NumberDefinition::new("Y")),
            (store_key!("x"), NumberDefinition::new("X")),
        ],
    );
    assert!(a.is_merge_compatible(&b));
    assert!(!a.is_merge_compatible(&c));
}

#[test]
fn test_merge_compatible_table_column_requirements() {
    // Why: Merged rows keep `self`'s definition without re-validation, so rows accepted
    // under a looser requirement must not land under a stricter one.
    let none = TableDefinition::new("A", vec![(store_key!("x"), NumberDefinition::new("X"))]);
    let increasing = TableDefinition::new_with_column_requirements(
        "B",
        vec![(
            store_key!("x"),
            NumberDefinition::new("X2"),
            TableColumnRequirement::Increasing,
        )],
    );
    let also_increasing = TableDefinition::new_with_column_requirements(
        "C",
        vec![(
            store_key!("x"),
            NumberDefinition::new("X3"),
            TableColumnRequirement::Increasing,
        )],
    );
    let decreasing = TableDefinition::new_with_column_requirements(
        "D",
        vec![(
            store_key!("x"),
            NumberDefinition::new("X"),
            TableColumnRequirement::Decreasing,
        )],
    );
    assert!(increasing.is_merge_compatible(&also_increasing));
    assert!(!none.is_merge_compatible(&increasing));
    assert!(!increasing.is_merge_compatible(&none));
    assert!(!increasing.is_merge_compatible(&decreasing));

    let none = TableWithUnitsDefinition::new(
        "A",
        vec![(
            store_key!("x"),
            NumberWithUnitsDefinition::new("X", UnitId::Length_Meter),
        )],
    );
    let increasing = TableWithUnitsDefinition::new_with_column_requirements(
        "B",
        vec![(
            store_key!("x"),
            NumberWithUnitsDefinition::new("X", UnitId::Length_Foot),
            TableColumnRequirement::Increasing,
        )],
    );
    let also_increasing = TableWithUnitsDefinition::new_with_column_requirements(
        "C",
        vec![(
            store_key!("x"),
            NumberWithUnitsDefinition::new("X", UnitId::Length_Meter),
            TableColumnRequirement::Increasing,
        )],
    );
    assert!(increasing.is_merge_compatible(&also_increasing));
    assert!(!none.is_merge_compatible(&increasing));
    assert!(!increasing.is_merge_compatible(&none));
}

#[test]
fn test_merge_compatible_map_items() {
    let a = MapDefinition::new(
        "A",
        vec![(
            store_key!("n"),
            MapItemDefinition::Number(NumberDefinition::new("N")),
        )],
    );
    let b = MapDefinition::new(
        "B",
        vec![(
            store_key!("n"),
            MapItemDefinition::Number(NumberDefinition::new_with_default("N2", "3")),
        )],
    );
    let c = MapDefinition::new(
        "A",
        vec![(
            store_key!("n"),
            MapItemDefinition::String(StringDefinition::new("N")),
        )],
    );
    assert!(a.is_merge_compatible(&b));
    assert!(!a.is_merge_compatible(&c));
}
