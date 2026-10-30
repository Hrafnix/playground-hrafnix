use crate::metadata::data::Metadata;
use serde::Deserialize;

/// The version 1 JSON representation of the `metadata` entry, separate from the
/// application's metadata.
///
/// Every field is required and unknown fields are rejected so nothing is silently
/// dropped when the file is written back.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub(crate) struct MetadataDocument {
    /// Name shown to users.
    display_name: String,
    /// Free-form description.
    description: String,
    /// Author of the file.
    author: String,
    /// Major part of the minimum Hrafnix version.
    minimum_version_major: u16,
    /// Minor part of the minimum Hrafnix version.
    minimum_version_minor: u16,
}

impl From<MetadataDocument> for Metadata {
    fn from(document: MetadataDocument) -> Self {
        Self {
            display_name: document.display_name,
            description: document.description,
            author: document.author,
            minimum_version_major: document.minimum_version_major,
            minimum_version_minor: document.minimum_version_minor,
        }
    }
}
