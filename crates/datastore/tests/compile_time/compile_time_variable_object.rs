use datastore::prelude::*;

#[test]
fn variable_object_compile_time_converts_both_macro_forms() {
    const ITEMS: &[(ConstVariableKey, ItemCompileTime)] = &[(
        variable_key!("v_result"),
        item_compile_time!(number = number_compile_time!("Result")),
    )];
    const FROM_SLICE: VariableObjectCompileTime = variable_object_compile_time!("Variables", ITEMS);
    const FROM_LITERALS: VariableObjectCompileTime = variable_object_compile_time!(
        "Variables",
        [(
            "v_enabled",
            item_compile_time!(boolean = boolean_compile_time!("Enabled")),
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
            item_compile_time!(string = string_compile_time!("First")),
        ),
        (
            variable_key!("v_duplicate"),
            item_compile_time!(string = string_compile_time!("Second")),
        ),
    ];
    let _ = VariableObjectCompileTime::__new(std::hint::black_box("Duplicates"), DUPLICATES);
}
