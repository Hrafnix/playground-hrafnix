use datastore::prelude::*;

#[test]
fn file_and_folder_compile_time_preserve_metadata_and_defaults() {
    let file = const_file!("Input", "*.csv", true);
    let file_default = const_file!("Output", "*.json", false, default = "out.json");
    let folder = const_folder!("Input folder", true);
    let folder_default = const_folder!("Output folder", false, default = "out");

    assert!(file.is_input());
    assert_eq!(file.extension_filter(), "*.csv");
    assert_eq!(file_default.default_value(), "out.json");
    let file_definition = file_default.into_definition();
    assert_eq!(file_definition.extension_filter(), "*.json");
    assert!(!file_definition.is_input());

    assert!(folder.is_input());
    assert_eq!(folder_default.default_value(), "out");
    assert_eq!(folder_default.into_definition().default_value(), "out");
}
