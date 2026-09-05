use datastore::prelude::*;

#[test]
fn folder_compile_time_preserves_metadata_and_defaults() {
    let folder = const_folder!("Input folder", true);
    let folder_default = const_folder!("Output folder", false, default = "out");

    assert_eq!(folder.description(), "Input folder");
    assert!(folder.is_input());
    assert_eq!(folder.default_value(), "");
    assert_eq!(folder.into_definition().default_value(), "");
    assert_eq!(folder_default.default_value(), "out");
    assert_eq!(folder_default.into_definition().default_value(), "out");
}
