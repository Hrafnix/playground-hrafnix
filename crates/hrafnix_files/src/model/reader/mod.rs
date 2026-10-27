use crate::common::{file_error, read_file_to_string, read_file_version};
use crate::model::ROOT;
use crate::model::data::ModelFile;
use camino::Utf8Path;
use hrafnix_message::message::Message;

/// Readers for version 1 of the model file format.
mod v1;

/// Reads a Hrafnix model file from the given path and returns a `ModelFile` struct.
///
/// File-type checks are not atomic with opening. Concurrent replacement with a
/// special file may still block the open.
///
/// # Errors
///
/// Returns a localized error if the path does not exist, is not a file, cannot be read,
/// is larger than [`MAX_FILE_SIZE`](crate::common::MAX_FILE_SIZE),
/// is not valid UTF-8, or contains invalid JSON, invalid model data, or a missing, invalid,
/// or unsupported file version.
pub fn read_model_file(path: &Utf8Path) -> Result<ModelFile, Message> {
    let source = read_file_to_string(path)?;
    read_model(&source, path)
}

/// Detects the file version, then deserializes the same text with the version-specific
/// reader.
fn read_model(source: &str, path: &Utf8Path) -> Result<ModelFile, Message> {
    let version = read_file_version(source, path, ROOT)?;

    match version {
        1 => v1::read_model(source, path),
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
    use hrafnix_shareable_string::ShareableString;

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
            "{\"file-version\": 1, \"model\": {}}",
            "{\"file-version\":1,\"model\":{}}",
            "{\"model\": {}, \"file-version\": 1}",
            "{\n  \"file-version\": 1,\n  \"model\": {}\n}\n",
            " \t\r\n{ \"file-version\" : 1 , \"model\" : { } } \t\r\n",
            "\u{feff}{\"file-version\": 1, \"model\": {}}",
            "{\"file-version\": 1, \"m\\u006fdel\": {}}",
        ] {
            assert_eq!(
                read_model(json, Utf8Path::new("model.hxm")).unwrap(),
                ModelFile {},
                "{json}"
            );
        }
    }

    #[test]
    fn header_errors_use_existing_diagnostics() {
        for (json, key, parameter) in [
            ("", "files_json_unexpected_eof", None),
            ("not JSON", "files_json_syntax", None),
            ("[]", "files_expected_root", Some(("root", "model"))),
            ("\"model\"", "files_expected_root", Some(("root", "model"))),
            (
                "{\"file-version\": 1, \"component\": {}}",
                "files_expected_root",
                Some(("root", "model")),
            ),
            (
                "{\"file-version\": 1, \"wrapper\": {\"model\": {}}}",
                "files_expected_root",
                Some(("root", "model")),
            ),
            (
                "{\"file-version\": 1}",
                "files_expected_root",
                Some(("root", "model")),
            ),
            ("{}", "files_missing_file_version", None),
            ("{\"model\": {}}", "files_missing_file_version", None),
            (
                "{\"FILE-VERSION\": 1, \"model\": {}}",
                "files_missing_file_version",
                None,
            ),
            (
                "{\"model\": {}, \"version\": 1}",
                "files_missing_file_version",
                None,
            ),
            (
                "{\"model\": {\"file-version\": 1}}",
                "files_missing_file_version",
                None,
            ),
            (
                "{\"file-version\": 1, \"file-version\": 1, \"model\": {}}",
                "files_duplicate_file_version",
                None,
            ),
            (
                "{\"file-version\": 1, \"model\": {}, \"file-version\": 1}",
                "files_duplicate_file_version",
                None,
            ),
            (
                "{\"file-version\": 1, \"model\": {}, \"model\": {}}",
                "files_duplicate_root",
                Some(("root", "model")),
            ),
            (
                "{\"file-version\": 1, \"model\": {}, \"other\": \"value\"}",
                "files_unknown_top_level_key",
                Some(("key", "other")),
            ),
            (
                "{\"other\": \"value\", \"file-version\": 1, \"model\": {}}",
                "files_unknown_top_level_key",
                Some(("key", "other")),
            ),
            (
                "{\"file-version\": \"1\", \"model\": {}}",
                "files_invalid_file_version",
                Some(("version", "\"1\"")),
            ),
            (
                "{\"file-version\": \"\", \"model\": {}}",
                "files_invalid_file_version",
                Some(("version", "\"\"")),
            ),
            (
                "{\"file-version\": 1.0, \"model\": {}}",
                "files_invalid_file_version",
                Some(("version", "1.0")),
            ),
            (
                "{\"file-version\": -1, \"model\": {}}",
                "files_invalid_file_version",
                Some(("version", "-1")),
            ),
            (
                "{\"file-version\": null, \"model\": {}}",
                "files_invalid_file_version",
                Some(("version", "null")),
            ),
            (
                "{\"file-version\": 4294967296, \"model\": {}}",
                "files_invalid_file_version",
                Some(("version", "4294967296")),
            ),
            (
                "{\"file-version\": 0, \"model\": {}}",
                "files_unsupported_file_version",
                Some(("version", "0")),
            ),
            (
                "{\"file-version\": 2, \"model\": {}}",
                "files_unsupported_file_version",
                Some(("version", "2")),
            ),
        ] {
            let error = read_model(json, Utf8Path::new("model.hxm")).unwrap_err();
            assert_eq!(error.translate_data().message_key().as_str(), key, "{json}");
            assert_eq!(param(&error, "path"), Some("model.hxm"), "{json}");
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
                "{\"file-version\": 1, \"model\": {",
                "files_json_unexpected_eof",
            ),
            (
                "{\"file-version\": 1, \"model\": {}",
                "files_json_unexpected_eof",
            ),
            ("{\"file-version\": 1, \"model\": {}}}", "files_json_syntax"),
            ("{\"file-version\": 1, \"model\": {},}", "files_json_syntax"),
            ("{\"file-version\": 01, \"model\": {}}", "files_json_syntax"),
            ("{file-version: 1, model: {}}", "files_json_syntax"),
            (
                "{\"file-version\": 1, \"model\": {}} {}",
                "files_json_syntax",
            ),
            (
                "{\"file-version\": 1, \"model\": {}} // comment",
                "files_json_syntax",
            ),
            (
                "\u{feff}\u{feff}{\"file-version\": 1, \"model\": {}}",
                "files_json_syntax",
            ),
        ] {
            let error = read_model(json, Utf8Path::new("model.hxm")).unwrap_err();
            assert_eq!(error.translate_data().message_key().as_str(), key, "{json}");
            assert_eq!(param(&error, "path"), Some("model.hxm"), "{json}");
            assert!(param(&error, "line").is_some(), "{json}");
            assert!(param(&error, "column").is_some(), "{json}");
            assert!(error.extra_detail().is_some(), "{json}");
        }
    }

    #[test]
    fn unsupported_versions_are_rejected_before_version_specific_deserialization() {
        for json in [
            "{\"file-version\": 2, \"model\": {\"extension\": {}}}",
            "{\"file-version\": 2, \"model\": [], \"extension\": {}}",
            "{\"file-version\": 2}",
            "{\"file-version\": 2, \"component\": {}}",
        ] {
            let error = read_model(json, Utf8Path::new("model.hxm")).unwrap_err();

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
        let json = "{\"file-version\": 2, \"model\": {\"broken";
        let error = read_model(json, Utf8Path::new("model.hxm")).unwrap_err();

        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_json_unexpected_eof"
        );
    }

    #[test]
    fn unknown_version_one_contents_are_rejected() {
        for json in [
            "{\"file-version\": 1, \"model\": {\"extension\": {\"value\": \"ignored\"}}}",
            "{\"file-version\": 1, \"model\": {\"extra\": \"attribute\"}}",
            "{\"file-version\": 1, \"model\": {\"file-version\": 1}}",
            "{\"file-version\": 1, \"model\": \"text\"}",
            "{\"file-version\": 1, \"model\": null}",
        ] {
            let error = read_model(json, Utf8Path::new("model.hxm")).unwrap_err();

            assert_eq!(
                error.translate_data().message_key().as_str(),
                "files_json_data",
                "{json}"
            );
            assert_eq!(param(&error, "path"), Some("model.hxm"), "{json}");
            assert!(error.extra_detail().is_some(), "{json}");
        }
    }
}
