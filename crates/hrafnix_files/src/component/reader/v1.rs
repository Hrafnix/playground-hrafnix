use crate::background_decoration::data::BackgroundDecoration;
use crate::background_decoration::reader::v1::BackgroundDecorationDocument;
use crate::common::{deserialize_object, read_document};
use crate::component::ROOT;
use crate::component::data::ComponentFile;
use crate::foreground_decoration::data::ForegroundDecoration;
use crate::foreground_decoration::reader::v1::ForegroundDecorationDocument;
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
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct ComponentDocument {
    /// Descriptive information about the component as an object with named fields.
    #[serde(deserialize_with = "deserialize_object")]
    metadata: MetadataDocument,
    /// Background decorations in drawing order; absent in older files.
    #[serde(default)]
    background_decorations: Vec<BackgroundDecorationDocument>,
    /// Foreground decorations in drawing order; absent in older files.
    #[serde(default)]
    foreground_decorations: Vec<ForegroundDecorationDocument>,
}

impl From<ComponentDocument> for ComponentFile {
    fn from(document: ComponentDocument) -> Self {
        Self {
            metadata: document.metadata.into(),
            background_decorations: document
                .background_decorations
                .into_iter()
                .map(BackgroundDecoration::from)
                .collect(),
            foreground_decorations: document
                .foreground_decorations
                .into_iter()
                .map(ForegroundDecoration::from)
                .collect(),
        }
    }
}

/// Deserializes version 1 component data after the version has been checked.
pub(super) fn read_component(source: &str, path: &Utf8Path) -> Result<ComponentFile, Message> {
    read_document::<ComponentDocument>(source, path, ROOT).map(ComponentFile::from)
}
