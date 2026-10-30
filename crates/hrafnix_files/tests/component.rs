//! Integration tests for component file conversion, reading, writing, and diagnostics.

// Panicking helpers are idiomatic in tests.
#![allow(clippy::unwrap_used)]

mod common;

use camino::Utf8PathBuf;
use common::{METADATA, TestDirectory};
use hrafnix_files::component::{
    data::ComponentFile, reader::read_component_file, writer::write_component_file,
};
use hrafnix_shareable_string::{ShareableString, SharedStringStore};
use hrafnix_translation::generate_translation_map;
use std::fs;

/// Returns the component stored in `tests/data/minimal.hxc`.
fn minimal_component() -> ComponentFile {
    ComponentFile {
        metadata: common::metadata("component"),
    }
}

/// Returns the LF-normalized fixture text for [`minimal_component`].
fn minimal_text() -> String {
    fs::read_to_string(Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/minimal.hxc"))
        .unwrap()
        .replace("\r\n", "\n")
}

#[test]
fn root_must_be_an_object() {
    common::assert_object_root("component", |path| read_component_file(path).map(|_| ()));
}

#[test]
fn validates_the_entire_json_document() {
    common::assert_json_trailing_content("component", |path| read_component_file(path).map(|_| ()));
}

#[test]
fn validates_utf8_before_parsing() {
    common::assert_utf8_handling("component", |path| read_component_file(path).map(|_| ()));
}

#[cfg(unix)]
#[test]
fn opening_fifo_does_not_block() {
    common::assert_fifo_rejected(|path| read_component_file(path).map(|_| ()));
}

#[cfg(unix)]
#[test]
fn saving_below_fifo_does_not_block() {
    common::assert_fifo_parent_rejected(|path| write_component_file(path, &minimal_component()));
}

#[cfg(unix)]
#[test]
fn directory_access_failure_does_not_replace_existing_file() {
    common::assert_save_error_preserves_file(|path| {
        write_component_file(path, &minimal_component())
    });
}

#[test]
fn read_only_component_file_is_not_replaced() {
    common::assert_read_only_file_preserved(|path| {
        write_component_file(path, &minimal_component())
    });
}

#[test]
fn oversized_component_file_is_rejected() {
    common::assert_oversized_file_rejected(|path| read_component_file(path).map(|_| ()));
}

#[test]
fn unknown_content_is_rejected_instead_of_being_dropped() {
    let directory = TestDirectory::new();
    let path = directory.path.join("unknown.hxc");
    fs::write(
        &path,
        format!("{{\"file-version\": 1, \"component\": {{\"metadata\": {METADATA}, \"unknown\": {{}}}}}}"),
    )
    .unwrap();

    let error = read_component_file(&path).unwrap_err();

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_json_data"
    );
}

#[test]
fn component_round_trips_through_the_writer_and_disk() {
    let directory = TestDirectory::new();
    let source = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/minimal.hxc");
    let destination = directory.path.join("round-trip.hxc");
    let component = read_component_file(&source).unwrap();
    write_component_file(&destination, &component).unwrap();

    assert_eq!(fs::read_to_string(&destination).unwrap(), minimal_text());
    assert_eq!(read_component_file(&destination).unwrap(), component);
}

#[test]
fn writing_replaces_an_existing_component_file() {
    let directory = TestDirectory::new();
    let path = directory.path.join("existing.hxc");
    fs::write(
        &path,
        "old content that must be completely replaced".repeat(10),
    )
    .unwrap();
    let component = minimal_component();

    write_component_file(&path, &component).unwrap();

    assert_eq!(fs::read_to_string(&path).unwrap(), minimal_text());
    assert_eq!(read_component_file(&path).unwrap(), minimal_component());
    assert_eq!(directory.entries(), ["existing.hxc"]);
}

#[cfg(unix)]
#[test]
fn writing_long_filenames_succeeds() {
    common::assert_long_filenames(
        "hxc",
        |path| write_component_file(path, &minimal_component()),
        |path| read_component_file(path).map(|_| ()),
    );
}

#[test]
fn writing_to_a_directory_fails_and_leaves_its_contents_and_no_temporary_file() {
    let directory = TestDirectory::new();
    fs::write(directory.path.join("keep.txt"), "kept").unwrap();
    let component = minimal_component();

    let error = write_component_file(&directory.path, &component).unwrap_err();

    assert_eq!(
        error.category(),
        hrafnix_message::message::MessageCategory::File
    );
    // Renaming a file over a directory yields EISDIR on Unix; Windows reports access denied.
    #[cfg(unix)]
    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_not_a_file"
    );
    assert!(directory.path.is_dir());
    assert_eq!(directory.entries(), ["keep.txt"]);
    assert_eq!(directory.sibling_temporary_files(), Vec::<String>::new());
}

#[test]
fn writing_to_a_missing_parent_reports_localized_io_error() {
    let directory = TestDirectory::new();
    let path = directory.path.join("missing").join("component.hxc");
    let component = minimal_component();
    let error = write_component_file(&path, &component).unwrap_err();
    let map = generate_translation_map(&SharedStringStore::new());

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_not_found"
    );
    assert_eq!(
        error
            .translated_message(&map, "en")
            .as_ref()
            .map(ShareableString::as_str),
        Some(format!("File or directory not found: '{path}'").as_str())
    );
    assert!(error.extra_detail().is_some());
    assert!(!path.exists());
}

#[test]
fn writing_to_an_invalid_path_reports_localized_io_error() {
    let path = Utf8PathBuf::from("invalid\0.hxc");
    let component = minimal_component();
    let error = write_component_file(&path, &component).unwrap_err();

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_invalid_input"
    );
    assert!(error.extra_detail().is_some());
}

#[test]
fn opening_missing_component_reports_localized_io_error() {
    let path =
        Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/missing component.hxc");
    let error = read_component_file(&path).unwrap_err();
    let map = generate_translation_map(&SharedStringStore::new());

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_not_found"
    );
    assert_eq!(
        error
            .translated_message(&map, "en")
            .as_ref()
            .map(ShareableString::as_str),
        Some(format!("File or directory not found: '{path}'").as_str())
    );
    assert_eq!(
        error
            .translated_message(&map, "de")
            .as_ref()
            .map(ShareableString::as_str),
        Some(format!("Datei oder Verzeichnis nicht gefunden: '{path}'").as_str())
    );
    assert!(error.extra_detail().is_some());
}

#[test]
fn opening_directory_reports_not_a_file() {
    let path = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    let error = read_component_file(&path).unwrap_err();
    let map = generate_translation_map(&SharedStringStore::new());

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_not_a_file"
    );
    assert_eq!(
        error
            .translated_message(&map, "en")
            .as_ref()
            .map(ShareableString::as_str),
        Some(format!("Path is not a file: '{path}'").as_str())
    );
}

#[test]
fn invalid_path_reports_localized_io_error_without_os_text() {
    let path = Utf8PathBuf::from("invalid\0.hxc");
    let error = read_component_file(&path).unwrap_err();
    let map = generate_translation_map(&SharedStringStore::new());

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_invalid_input"
    );
    assert_eq!(
        error
            .translated_message(&map, "en")
            .as_ref()
            .map(ShareableString::as_str),
        Some(format!("Invalid path or file operation argument: '{path}'").as_str())
    );
    assert!(error.extra_detail().is_some());
}

#[test]
fn opening_existing_component_succeeds() {
    let path = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/minimal.hxc");

    assert_eq!(read_component_file(&path).unwrap(), minimal_component());
}
