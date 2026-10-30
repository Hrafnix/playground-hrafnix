use crate::common::read_document;
use crate::component::ROOT;
use crate::component::data::ComponentFile;
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
struct ComponentDocument {}

impl From<ComponentDocument> for ComponentFile {
    fn from(_document: ComponentDocument) -> Self {
        Self {}
    }
}

/// Deserializes version 1 component data after the version has been checked.
pub(super) fn read_component(source: &str, path: &Utf8Path) -> Result<ComponentFile, Message> {
    read_document::<ComponentDocument>(source, path, ROOT).map(ComponentFile::from)
}
