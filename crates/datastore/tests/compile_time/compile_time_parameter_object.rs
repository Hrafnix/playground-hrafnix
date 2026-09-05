use datastore::prelude::*;

#[test]
fn parameter_object_compile_time_converts_both_macro_forms() {
    const ITEMS: &[(ConstParameterKey, ItemCompileTime)] = &[(
        parameter_key!("p_name"),
        const_item!(string = const_string!("Name")),
    )];
    const FROM_SLICE: ParameterObjectCompileTime = const_parameter_object!("Parameters", ITEMS);
    const FROM_LITERALS: ParameterObjectCompileTime = const_parameter_object!(
        "Parameters",
        [(
            "p_enabled",
            const_item!(boolean = const_boolean!("Enabled")),
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
            const_item!(string = const_string!("First")),
        ),
        (
            parameter_key!("p_duplicate"),
            const_item!(string = const_string!("Second")),
        ),
    ];
    #[allow(clippy::disallowed_methods)]
    let _ = ParameterObjectCompileTime::__new(std::hint::black_box("Duplicates"), DUPLICATES);
}
