use datastore::prelude::*;

#[test]
fn variable_object_compile_time_converts_both_macro_forms() {
    const ITEMS: &[(ConstVariableKey, ItemCompileTime)] = &[(
        variable_key!("v_result"),
        const_item!(number = const_number!("Result")),
    )];
    const FROM_SLICE: VariableObjectCompileTime = const_variable_object!("Variables", ITEMS);
    const FROM_LITERALS: VariableObjectCompileTime = const_variable_object!(
        "Variables",
        [(
            "v_enabled",
            const_item!(boolean = const_boolean!("Enabled")),
        )],
    );

    assert_eq!(FROM_SLICE.description(), "Variables");
    assert_eq!(FROM_SLICE.items(), ITEMS);
    assert_eq!(FROM_SLICE.count(), 1);
    assert!(FROM_SLICE.contains("v_result"));
    assert!(!FROM_SLICE.contains("v_missing"));
    assert!(matches!(
        FROM_SLICE.get("v_result"),
        Some(ItemCompileTime::Number(_))
    ));
    assert_eq!(FROM_SLICE.get("v_missing"), None);
    assert_eq!(FROM_SLICE.iter().count(), 1);
    assert_eq!(
        FROM_SLICE
            .keys()
            .map(|key| key.to_string())
            .collect::<Vec<_>>(),
        ["v_result"]
    );
    assert_eq!(
        FROM_LITERALS
            .into_definition()
            .keys()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["v_enabled"]
    );
}
