use datastore::prelude::*;

#[test]
fn file_compile_time_preserves_metadata_and_defaults() {
    let file = const_file!("Input", "*.csv", true);
    let file_default = const_file!("Output", "*.json", false, default = "out.json");

    assert_eq!(file.description(), "Input");
    assert!(file.is_input());
    assert_eq!(file.extension_filter(), "*.csv");
    assert_eq!(file.default_value(), "");
    assert_eq!(file.into_definition().default_value(), "");
    assert_eq!(file_default.default_value(), "out.json");
    let file_definition = file_default.into_definition();
    assert_eq!(file_definition.extension_filter(), "*.json");
    assert!(!file_definition.is_input());
}
