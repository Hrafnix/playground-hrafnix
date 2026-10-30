use crate::common::{deserialize_object, read_document};
use crate::metadata::reader::v1::MetadataDocument;
use crate::model::ROOT;
use crate::model::data::ModelFile;
use camino::Utf8Path;
use hrafnix_message::message::Message;
use serde::Deserialize;

/// The version 1 JSON representation of the `model` entry, separate from the
/// application's model data.
///
/// The file version lives beside the entry at the top level, which [`read_document`]
/// handles. Unknown fields are rejected: anything silently ignored here would be
/// dropped when the file is written back, so unexpected content is an error instead.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelDocument {
    /// Descriptive information about the model as an object with named fields.
    #[serde(deserialize_with = "deserialize_object")]
    metadata: MetadataDocument,
}

impl From<ModelDocument> for ModelFile {
    fn from(document: ModelDocument) -> Self {
        Self {
            metadata: document.metadata.into(),
        }
    }
}

/// Deserializes version 1 model data after the version has been checked.
pub(super) fn read_model(source: &str, path: &Utf8Path) -> Result<ModelFile, Message> {
    read_document::<ModelDocument>(source, path, ROOT).map(ModelFile::from)
}
