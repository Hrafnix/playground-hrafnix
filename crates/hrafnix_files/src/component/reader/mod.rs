use crate::common::{file_error, read_file_to_string, read_file_version};
use crate::component::ROOT;
use crate::component::data::ComponentFile;
use camino::Utf8Path;
use hrafnix_message::message::Message;

/// Readers for version 1 of the component file format.
mod v1;

/// Reads a Hrafnix component file from the given path and returns a `ComponentFile` struct.
///
/// File-type checks are not atomic with opening. Concurrent replacement with a
/// special file may still block the open.
///
/// # Errors
///
/// Returns a localized error if the path does not exist, is not a file, cannot be read,
/// is larger than [`MAX_FILE_SIZE`](crate::common::MAX_FILE_SIZE),
/// is not valid UTF-8, or contains invalid JSON, invalid component data, or a missing, invalid,
/// or unsupported file version.
pub fn read_component_file(path: &Utf8Path) -> Result<ComponentFile, Message> {
    let source = read_file_to_string(path)?;
    read_component(&source, path)
}

/// Detects the file version, then deserializes the same text with the version-specific
/// reader.
fn read_component(source: &str, path: &Utf8Path) -> Result<ComponentFile, Message> {
    let version = read_file_version(source, path, ROOT)?;

    match version {
        1 => v1::read_component(source, path),
        _ => Err(file_error(
            path,
            "files_unsupported_file_version",
            [("version", version.to_string())],
        )),
    }
}

#[cfg(test)]
mod tests {
    // Panicking helpers are idiomatic in tests.
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::metadata::data::Metadata;
    use hrafnix_shareable_string::ShareableString;

    /// Valid version 1 contents of the root entry.
    const CONTENTS: &str = "{\"metadata\": {\"display-name\": \"Name\", \"description\": \"Description\", \"author\": \"Author\", \"minimum-version-major\": 1, \"minimum-version-minor\": 2}}";

    /// Returns the component described by [`CONTENTS`].
    fn expected() -> ComponentFile {
        ComponentFile::new(
            Metadata::new(
                "Name".to_owned(),
                "Description".to_owned(),
                "Author".to_owned(),
                1,
                2,
            ),
            Vec::new(),
            Vec::new(),
        )
    }

    /// Returns the named parameter of `error` as text.
    fn param<'a>(error: &'a Message, name: &str) -> Option<&'a str> {
        error
            .translate_data()
            .message_params()
            .get(name)
            .map(ShareableString::as_str)
    }

    #[test]
    fn version_one_documents_succeed() {
        for json in [
            format!("{{\"file-version\": 1, \"component\": {CONTENTS}}}"),
            format!("{{\"file-version\":1,\"component\":{CONTENTS}}}"),
            format!("{{\"component\": {CONTENTS}, \"file-version\": 1}}"),
            format!("{{\n  \"file-version\": 1,\n  \"component\": {CONTENTS}\n}}\n"),
            format!(" \t\r\n{{ \"file-version\" : 1 , \"component\" : {CONTENTS} }} \t\r\n"),
            format!("\u{feff}{{\"file-version\": 1, \"component\": {CONTENTS}}}"),
            format!("{{\"file-version\": 1, \"comp\\u006fnent\": {CONTENTS}}}"),
        ] {
            assert_eq!(
                read_component(&json, Utf8Path::new("component.hxc")).unwrap(),
                expected(),
                "{json}"
            );
        }
    }

    #[test]
    fn header_errors_use_existing_diagnostics() {
        for (json, key, parameter) in [
            ("", "files_json_unexpected_eof", None),
            ("not JSON", "files_json_syntax", None),
            ("[]", "files_expected_root", Some(("root", "component"))),
            (
                "\"component\"",
                "files_expected_root",
                Some(("root", "component")),
            ),
            (
                "{\"file-version\": 1, \"model\": {}}",
                "files_expected_root",
                Some(("root", "component")),
            ),
            (
                "{\"file-version\": 1, \"wrapper\": {\"component\": {}}}",
                "files_expected_root",
                Some(("root", "component")),
            ),
            (
                "{\"file-version\": 1}",
                "files_expected_root",
                Some(("root", "component")),
            ),
            ("{}", "files_missing_file_version", None),
            ("{\"component\": {}}", "files_missing_file_version", None),
            (
                "{\"FILE-VERSION\": 1, \"component\": {}}",
                "files_missing_file_version",
                None,
            ),
            (
                "{\"component\": {}, \"version\": 1}",
                "files_missing_file_version",
                None,
            ),
            (
                "{\"component\": {\"file-version\": 1}}",
                "files_missing_file_version",
                None,
            ),
            (
                "{\"file-version\": 1, \"file-version\": 1, \"component\": {}}",
                "files_duplicate_file_version",
                None,
            ),
            (
                "{\"file-version\": 1, \"component\": {}, \"file-version\": 1}",
                "files_duplicate_file_version",
                None,
            ),
            (
                "{\"file-version\": \"1\", \"component\": {}}",
                "files_invalid_file_version",
                Some(("version", "\"1\"")),
            ),
            (
                "{\"file-version\": \"\", \"component\": {}}",
                "files_invalid_file_version",
                Some(("version", "\"\"")),
            ),
            (
                "{\"file-version\": 1.0, \"component\": {}}",
                "files_invalid_file_version",
                Some(("version", "1.0")),
            ),
            (
                "{\"file-version\": -1, \"component\": {}}",
                "files_invalid_file_version",
                Some(("version", "-1")),
            ),
            (
                "{\"file-version\": null, \"component\": {}}",
                "files_invalid_file_version",
                Some(("version", "null")),
            ),
            (
                "{\"file-version\": 4294967296, \"component\": {}}",
                "files_invalid_file_version",
                Some(("version", "4294967296")),
            ),
            (
                "{\"file-version\": 0, \"component\": {}}",
                "files_unsupported_file_version",
                Some(("version", "0")),
            ),
            (
                "{\"file-version\": 2, \"component\": {}}",
                "files_unsupported_file_version",
                Some(("version", "2")),
            ),
        ] {
            let error = read_component(json, Utf8Path::new("component.hxc")).unwrap_err();
            assert_eq!(error.translate_data().message_key().as_str(), key, "{json}");
            assert_eq!(param(&error, "path"), Some("component.hxc"), "{json}");
            if let Some((name, value)) = parameter {
                assert_eq!(param(&error, name), Some(value), "{json}");
            }
        }
    }

    #[test]
    fn malformed_documents_report_json_errors() {
        for (json, key) in [
            ("{\"file-version\": 1", "files_json_unexpected_eof"),
            (
                "{\"file-version\": 1, \"component\": {",
                "files_json_unexpected_eof",
            ),
            (
                "{\"file-version\": 1, \"component\": {}",
                "files_json_unexpected_eof",
            ),
            (
                "{\"file-version\": 1, \"component\": {}}}",
                "files_json_syntax",
            ),
            (
                "{\"file-version\": 1, \"component\": {},}",
                "files_json_syntax",
            ),
            (
                "{\"file-version\": 01, \"component\": {}}",
                "files_json_syntax",
            ),
            ("{file-version: 1, component: {}}", "files_json_syntax"),
            (
                "{\"file-version\": 1, \"component\": {}} {}",
                "files_json_syntax",
            ),
            (
                "{\"file-version\": 1, \"component\": {}} // comment",
                "files_json_syntax",
            ),
            (
                "\u{feff}\u{feff}{\"file-version\": 1, \"component\": {}}",
                "files_json_syntax",
            ),
        ] {
            let error = read_component(json, Utf8Path::new("component.hxc")).unwrap_err();
            assert_eq!(error.translate_data().message_key().as_str(), key, "{json}");
            assert_eq!(param(&error, "path"), Some("component.hxc"), "{json}");
            assert!(param(&error, "line").is_some(), "{json}");
            assert!(param(&error, "column").is_some(), "{json}");
            assert!(error.extra_detail().is_some(), "{json}");
        }
    }

    #[test]
    fn unsupported_versions_are_rejected_before_version_specific_deserialization() {
        for json in [
            "{\"file-version\": 2, \"component\": {\"extension\": {}}}",
            "{\"file-version\": 2, \"component\": [], \"extension\": {}}",
            "{\"file-version\": 2}",
            "{\"file-version\": 2, \"model\": {}}",
        ] {
            let error = read_component(json, Utf8Path::new("component.hxc")).unwrap_err();

            assert_eq!(
                error.translate_data().message_key().as_str(),
                "files_unsupported_file_version",
                "{json}"
            );
            assert_eq!(param(&error, "version"), Some("2"), "{json}");
        }
    }

    #[test]
    fn malformed_contents_are_reported_before_unsupported_versions() {
        let json = "{\"file-version\": 2, \"component\": {\"broken";
        let error = read_component(json, Utf8Path::new("component.hxc")).unwrap_err();

        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_json_unexpected_eof"
        );
    }

    #[test]
    fn unknown_version_one_contents_are_rejected() {
        /// Valid metadata with every field present.
        const METADATA: &str = "{\"display-name\": \"\", \"description\": \"\", \"author\": \"\", \"minimum-version-major\": 0, \"minimum-version-minor\": 0}";

        for json in [
            format!(
                "{{\"file-version\": 1, \"component\": {{\"metadata\": {METADATA}, \"extension\": {{\"value\": \"ignored\"}}}}}}"
            ),
            format!(
                "{{\"file-version\": 1, \"component\": {{\"metadata\": {METADATA}, \"extra\": \"attribute\"}}}}"
            ),
            format!(
                "{{\"file-version\": 1, \"component\": {{\"metadata\": {METADATA}, \"file-version\": 1}}}}"
            ),
            format!(
                "{{\"file-version\": 1, \"component\": {{\"metadata\": {METADATA}, \"metadata\": {METADATA}}}}}"
            ),
            "{\"file-version\": 1, \"component\": \"text\"}".to_owned(),
            "{\"file-version\": 1, \"component\": null}".to_owned(),
            "{\"file-version\": 1, \"component\": {}}".to_owned(),
            "{\"file-version\": 1, \"component\": {\"metadata\": null}}".to_owned(),
            "{\"file-version\": 1, \"component\": {\"metadata\": [\"Name\", \"Description\", \"Author\", 1, 2]}}".to_owned(),
            "{\"file-version\": 1, \"component\": {\"metadata\": {}}}".to_owned(),
            "{\"file-version\": 1, \"component\": {\"metadata\": {\"display-name\": \"\", \"description\": \"\", \"author\": \"\", \"minimum-version-major\": 0, \"minimum-version-minor\": 0, \"extra\": \"attribute\"}}}".to_owned(),
            "{\"file-version\": 1, \"component\": {\"metadata\": {\"display-name\": \"\", \"description\": \"\", \"author\": \"\", \"minimum-version-major\": 0}}}".to_owned(),
            "{\"file-version\": 1, \"component\": {\"metadata\": {\"display-name\": 1, \"description\": \"\", \"author\": \"\", \"minimum-version-major\": 0, \"minimum-version-minor\": 0}}}".to_owned(),
            "{\"file-version\": 1, \"component\": {\"metadata\": {\"display-name\": \"\", \"description\": \"\", \"author\": \"\", \"minimum-version-major\": 65536, \"minimum-version-minor\": 0}}}".to_owned(),
            "{\"file-version\": 1, \"component\": {\"metadata\": {\"display-name\": \"\", \"description\": \"\", \"author\": \"\", \"minimum-version-major\": 0, \"minimum-version-minor\": -1}}}".to_owned(),
            "{\"file-version\": 1, \"component\": {\"metadata\": {\"display-name\": \"\", \"description\": \"\", \"author\": \"\", \"minimum-version-major\": 1.5, \"minimum-version-minor\": 0}}}".to_owned(),
            "{\"file-version\": 1, \"component\": {\"metadata\": {\"displayName\": \"\", \"description\": \"\", \"author\": \"\", \"minimum-version-major\": 0, \"minimum-version-minor\": 0}}}".to_owned(),
        ] {
            let error = read_component(&json, Utf8Path::new("component.hxc")).unwrap_err();

            assert_eq!(
                error.translate_data().message_key().as_str(),
                "files_json_data",
                "{json}"
            );
            assert_eq!(param(&error, "path"), Some("component.hxc"), "{json}");
            assert!(param(&error, "line").is_some(), "{json}");
            assert!(param(&error, "column").is_some(), "{json}");
            assert!(error.extra_detail().is_some(), "{json}");
        }
    }

    #[test]
    fn top_level_errors_after_valid_contents_are_reported() {
        for (json, key, parameter) in [
            (
                format!(
                    "{{\"file-version\": 1, \"component\": {CONTENTS}, \"component\": {CONTENTS}}}"
                ),
                "files_duplicate_root",
                ("root", "component"),
            ),
            (
                format!("{{\"file-version\": 1, \"component\": {CONTENTS}, \"other\": \"value\"}}"),
                "files_unknown_top_level_key",
                ("key", "other"),
            ),
            (
                format!("{{\"other\": \"value\", \"file-version\": 1, \"component\": {CONTENTS}}}"),
                "files_unknown_top_level_key",
                ("key", "other"),
            ),
        ] {
            let error = read_component(&json, Utf8Path::new("component.hxc")).unwrap_err();

            assert_eq!(error.translate_data().message_key().as_str(), key, "{json}");
            assert_eq!(param(&error, "path"), Some("component.hxc"), "{json}");
            assert_eq!(param(&error, parameter.0), Some(parameter.1), "{json}");
        }
    }
}
