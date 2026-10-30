use crate::metadata::data::Metadata;
use serde::Serialize;

/// The current JSON representation of the `metadata` entry used when writing.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct MetadataDocument<'a> {
    /// Name shown to users.
    display_name: &'a str,
    /// Free-form description.
    description: &'a str,
    /// Author of the file.
    author: &'a str,
    /// Major part of the minimum Hrafnix version.
    minimum_version_major: u16,
    /// Minor part of the minimum Hrafnix version.
    minimum_version_minor: u16,
}

impl<'a> From<&'a Metadata> for MetadataDocument<'a> {
    fn from(metadata: &'a Metadata) -> Self {
        Self {
            display_name: &metadata.display_name,
            description: &metadata.description,
            author: &metadata.author,
            minimum_version_major: metadata.minimum_version_major,
            minimum_version_minor: metadata.minimum_version_minor,
        }
    }
}
