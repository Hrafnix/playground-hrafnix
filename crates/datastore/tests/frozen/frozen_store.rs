use datastore::prelude::*;

fn make_parameter(
    description: &str,
    items: Vec<(&str, StringDefinition)>,
) -> ParameterObjectFrozen {
    let mut builder = ParameterObjectDefinition::builder(description);
    for (k, v) in items {
        builder.insert(ParameterKey::new(k.into()).unwrap(), v);
    }
    ParameterObjectFrozen::new(builder.finish())
}

fn make_variable(description: &str, items: Vec<(&str, StringDefinition)>) -> VariableObjectFrozen {
    let mut builder = VariableObjectDefinition::builder(description);
    for (k, v) in items {
        builder.insert(VariableKey::new(k.into()).unwrap(), v);
    }
    VariableObjectFrozen::new(builder.finish())
}

fn make_global(description: &str, items: Vec<(&str, StringDefinition)>) -> GlobalObjectFrozen {
    let mut builder = GlobalObjectDefinition::builder(description);
    for (k, v) in items {
        builder.insert(GlobalKey::new(k.into()).unwrap(), v);
    }
    GlobalObjectFrozen::new(builder.finish())
}

#[test]
fn test_frozen_store_new() {
    // Why: Verify that a FrozenStore can be constructed and accessors return correct objects.
    let param = make_parameter("Params", vec![("p_a", StringDefinition::new("A"))]);
    let var = make_variable("Vars", vec![("v_b", StringDefinition::new("B"))]);
    let global = make_global("Globals", vec![("g_c", StringDefinition::new("C"))]);

    let store = FrozenStore::new(param.clone(), var.clone(), global.clone());

    assert_eq!(store.parameter(), &param);
    assert_eq!(store.variable(), &var);
    assert_eq!(store.global(), &global);
}

#[test]
fn test_frozen_store_equality() {
    // Why: Two stores built from identical objects compare equal.
    let a = FrozenStore::new(
        make_parameter("P", vec![("p_x", StringDefinition::new("X"))]),
        make_variable("V", vec![]),
        make_global("G", vec![]),
    );
    let b = FrozenStore::new(
        make_parameter("P", vec![("p_x", StringDefinition::new("X"))]),
        make_variable("V", vec![]),
        make_global("G", vec![]),
    );
    assert_eq!(a, b);
    assert_eq!(&a, b);
    assert_eq!(a, &b);
}

#[test]
fn test_frozen_store_merge_disjoint_keys() {
    // Why: Merging two stores with disjoint keys should include all items from both.
    let store_a = FrozenStore::new(
        make_parameter("Params", vec![("p_a", StringDefinition::new("A"))]),
        make_variable("Vars", vec![("v_a", StringDefinition::new("A"))]),
        make_global("Globals", vec![("g_a", StringDefinition::new("A"))]),
    );
    let store_b = FrozenStore::new(
        make_parameter("Params", vec![("p_b", StringDefinition::new("B"))]),
        make_variable("Vars", vec![("v_b", StringDefinition::new("B"))]),
        make_global("Globals", vec![("g_b", StringDefinition::new("B"))]),
    );

    let merged = store_a.merge(&store_b);

    assert_eq!(merged.parameter().definition().count(), 2);
    assert!(merged.parameter().definition().contains_str("p_a"));
    assert!(merged.parameter().definition().contains_str("p_b"));

    assert_eq!(merged.variable().definition().count(), 2);
    assert!(merged.variable().definition().contains_str("v_a"));
    assert!(merged.variable().definition().contains_str("v_b"));

    assert_eq!(merged.global().definition().count(), 2);
    assert!(merged.global().definition().contains_str("g_a"));
    assert!(merged.global().definition().contains_str("g_b"));
}

#[test]
fn test_frozen_store_merge_self_takes_precedence() {
    // Why: When both stores share a key, self's item value is preserved.
    let store_a = FrozenStore::new(
        make_parameter(
            "Params",
            vec![("p_x", StringDefinition::new_with_default("X in A", "value_a"))],
        ),
        make_variable(
            "Vars",
            vec![("v_x", StringDefinition::new_with_default("X in A", "value_a"))],
        ),
        make_global(
            "Globals",
            vec![("g_x", StringDefinition::new_with_default("X in A", "value_a"))],
        ),
    );
    let store_b = FrozenStore::new(
        make_parameter(
            "Params",
            vec![("p_x", StringDefinition::new_with_default("X in B", "value_b"))],
        ),
        make_variable(
            "Vars",
            vec![("v_x", StringDefinition::new_with_default("X in B", "value_b"))],
        ),
        make_global(
            "Globals",
            vec![("g_x", StringDefinition::new_with_default("X in B", "value_b"))],
        ),
    );

    let merged = store_a.merge(&store_b);

    assert_eq!(merged.parameter().definition().count(), 1);

    let p_item = merged.parameter().get("p_x").unwrap();
    if let ItemFrozen::String(s) = p_item {
        assert_eq!(s.value(), "value_a");
    } else {
        panic!("expected a String item");
    }

    let v_item = merged.variable().get("v_x").unwrap();
    if let ItemFrozen::String(s) = v_item {
        assert_eq!(s.value(), "value_a");
    } else {
        panic!("expected a String item");
    }

    let g_item = merged.global().get("g_x").unwrap();
    if let ItemFrozen::String(s) = g_item {
        assert_eq!(s.value(), "value_a");
    } else {
        panic!("expected a String item");
    }
}

#[test]
fn test_frozen_store_merge_keeps_maps_unchanged() {
    // Why: Map items in self are not deep-merged; they are kept as atomic items from self.
    let map_def_a = MapDefinition::new(
        "Map A",
        vec![(store_key!("field"), StringDefinition::new("Field"))],
    );
    let map_def_b = MapDefinition::new(
        "Map B",
        vec![(store_key!("other_field"), StringDefinition::new("Other"))],
    );

    let mut builder_a = ParameterObjectDefinition::builder("Params");
    builder_a.insert(
        ParameterKey::new("p_map".into()).unwrap(),
        ItemDefinitionType::Map(map_def_a),
    );
    let mut builder_b = ParameterObjectDefinition::builder("Params");
    builder_b.insert(
        ParameterKey::new("p_map".into()).unwrap(),
        ItemDefinitionType::Map(map_def_b),
    );

    let store_a = FrozenStore::new(
        ParameterObjectFrozen::new(builder_a.finish()),
        make_variable("Vars", vec![]),
        make_global("Globals", vec![]),
    );
    let store_b = FrozenStore::new(
        ParameterObjectFrozen::new(builder_b.finish()),
        make_variable("Vars", vec![]),
        make_global("Globals", vec![]),
    );

    let merged = store_a.merge(&store_b);

    let map_item = merged.parameter().get("p_map").unwrap();
    if let ItemFrozen::Map(m) = map_item {
        // Self's map is preserved.
        assert_eq!(m.definition().description().as_ref(), "Map A");
    } else {
        panic!("expected a Map item");
    }
}

#[test]
fn test_parameter_merge_from() {
    // Why: Directly verify ParameterObjectFrozen::merge_from.
    let a = make_parameter(
        "P",
        vec![("p_a", StringDefinition::new_with_default("A", "val_a"))],
    );
    let b = make_parameter(
        "P",
        vec![
            ("p_a", StringDefinition::new_with_default("A override", "override")),
            ("p_b", StringDefinition::new("B")),
        ],
    );

    let merged = a.merge_from(&b);

    assert_eq!(merged.definition().count(), 2);
    // p_a from self is kept unchanged
    if let ItemFrozen::String(s) = merged.get("p_a").unwrap() {
        assert_eq!(s.value(), "val_a");
    } else {
        panic!("expected String");
    }
    // p_b from other is added
    assert!(merged.definition().contains_str("p_b"));
}

#[test]
fn test_variable_merge_from() {
    // Why: Directly verify VariableObjectFrozen::merge_from.
    let a = make_variable("V", vec![("v_a", StringDefinition::new("A"))]);
    let b = make_variable("V", vec![("v_b", StringDefinition::new("B"))]);

    let merged = a.merge_from(&b);

    assert_eq!(merged.definition().count(), 2);
    assert!(merged.definition().contains_str("v_a"));
    assert!(merged.definition().contains_str("v_b"));
}

#[test]
fn test_global_merge_from() {
    // Why: Directly verify GlobalObjectFrozen::merge_from.
    let a = make_global("G", vec![("g_a", StringDefinition::new("A"))]);
    let b = make_global("G", vec![("g_b", StringDefinition::new("B"))]);

    let merged = a.merge_from(&b);

    assert_eq!(merged.definition().count(), 2);
    assert!(merged.definition().contains_str("g_a"));
    assert!(merged.definition().contains_str("g_b"));
}
