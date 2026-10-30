use crate::common::{deserialize_object, read_document};
use crate::component::ROOT;
use crate::component::data::ComponentFile;
use crate::metadata::reader::v1::MetadataDocument;
use camino::Utf8Path;
use hrafnix_message::message::Message;
use serde::Deserialize;

/// The version 1 JSON representation of the `component` entry, separate from the
/// application's component data.
///
/// The file version lives beside the entry at the top level, which [`read_document`]
/// handles. Unknown fields are rejected: anything silently ignored here would be
/// dropped when the file is written back, so unexpected content is an error instead.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ComponentDocument {
    /// Descriptive information about the component as an object with named fields.
    #[serde(deserialize_with = "deserialize_object")]
    metadata: MetadataDocument,
}

impl From<ComponentDocument> for ComponentFile {
    fn from(document: ComponentDocument) -> Self {
        Self {
            metadata: document.metadata.into(),
        }
    }
}

/// Deserializes version 1 component data after the version has been checked.
pub(super) fn read_component(source: &str, path: &Utf8Path) -> Result<ComponentFile, Message> {
    read_document::<ComponentDocument>(source, path, ROOT).map(ComponentFile::from)
}
