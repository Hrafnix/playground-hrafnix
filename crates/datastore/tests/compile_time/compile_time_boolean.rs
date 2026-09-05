use datastore::prelude::*;

#[test]
fn boolean_compile_time_converts_with_and_without_default() {
    let boolean = const_boolean!("Boolean");
    let default = const_boolean!("Boolean default", default = false);
    let true_default = const_boolean!("Boolean true", default = true);

    assert_eq!(BooleanCompileTime::ids(), ["true", "false"]);
    assert_eq!(boolean.description(), "Boolean");
    assert_eq!(boolean.descriptions(), ["True", "False"]);
    assert_eq!(boolean.default_value(), "");
    assert_eq!(boolean.into_definition().default_value(), "");
    assert_eq!(default.default_value(), "false");
    assert_eq!(default.into_definition().default_value(), "false");
    assert_eq!(true_default.into_definition().default_value(), "true");
}
