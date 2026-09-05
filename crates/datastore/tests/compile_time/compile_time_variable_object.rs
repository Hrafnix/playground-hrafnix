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

#[test]
#[should_panic(expected = "VariableObjectCompileTime item keys must be unique")]
fn variable_object_compile_time_rejects_duplicate_keys() {
    const DUPLICATES: &[(ConstVariableKey, ItemCompileTime)] = &[
        (
            variable_key!("v_duplicate"),
            const_item!(string = const_string!("First")),
        ),
        (
            variable_key!("v_duplicate"),
            const_item!(string = const_string!("Second")),
        ),
    ];
    #[allow(clippy::disallowed_methods)]
    let _ = VariableObjectCompileTime::__new(std::hint::black_box("Duplicates"), DUPLICATES);
}
