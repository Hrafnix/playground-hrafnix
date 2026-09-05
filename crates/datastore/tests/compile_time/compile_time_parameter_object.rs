use datastore::prelude::*;

#[test]
fn parameter_object_compile_time_converts_both_macro_forms() {
    const ITEMS: &[(ConstParameterKey, ItemCompileTime)] = &[(
        parameter_key!("p_name"),
        item_compile_time!(string = string_compile_time!("Name")),
    )];
    const FROM_SLICE: ParameterObjectCompileTime =
        parameter_object_compile_time!("Parameters", ITEMS);
    const FROM_LITERALS: ParameterObjectCompileTime = parameter_object_compile_time!(
        "Parameters",
        [(
            "p_enabled",
            item_compile_time!(boolean = boolean_compile_time!("Enabled")),
        )],
    );

    assert_eq!(
        FROM_SLICE
            .keys()
            .map(|key| key.to_string())
            .collect::<Vec<_>>(),
        ["p_name"]
    );
    assert_eq!(
        FROM_LITERALS
            .into_definition()
            .keys()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["p_enabled"]
    );
}

#[test]
#[should_panic(expected = "ParameterObjectCompileTime item keys must be unique")]
fn parameter_object_compile_time_rejects_duplicate_keys() {
    const DUPLICATES: &[(ConstParameterKey, ItemCompileTime)] = &[
        (
            parameter_key!("p_duplicate"),
            item_compile_time!(string = string_compile_time!("First")),
        ),
        (
            parameter_key!("p_duplicate"),
            item_compile_time!(string = string_compile_time!("Second")),
        ),
    ];
    #[allow(clippy::disallowed_methods)]
    let _ = ParameterObjectCompileTime::__new(std::hint::black_box("Duplicates"), DUPLICATES);
}
