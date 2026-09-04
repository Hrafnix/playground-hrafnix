use datastore::prelude::*;

#[test]
fn string_compile_time_converts_with_and_without_default() {
    let string = string_compile_time!("Name");
    let default = string_compile_time!("Name default", default = "Untitled");

    assert_eq!(string.description(), "Name");
    assert_eq!(string.default_value(), "");
    assert_eq!(default.default_value(), "Untitled");
    assert_eq!(default.into_definition().default_value(), "Untitled");
}
