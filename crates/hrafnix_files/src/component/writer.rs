use crate::common::write_json_file;
use crate::component::ROOT;
use crate::component::data::ComponentFile;
use camino::Utf8Path;
use hrafnix_message::message::Message;
use serde::Serialize;

/// The current JSON representation of the `component` entry used to write a
/// [`ComponentFile`] to disk.
///
/// The file version is emitted beside the entry at the top level by [`write_json_file`].
#[derive(Debug, Serialize)]
struct ComponentDocument {}

impl From<&ComponentFile> for ComponentDocument {
    fn from(_component: &ComponentFile) -> Self {
        Self {}
    }
}

/// Serializes a component as UTF-8 JSON, atomically creating or replacing the file at `path`.
///
/// The document is written to a temporary file beside the destination and renamed over it,
/// so failures before replacement leave an existing file untouched. Parent directories
/// must already exist. On Unix, the parent directory is synced after replacement.
///
/// # Errors
///
/// Returns a localized error if serialization fails, the file cannot be created or written,
/// or an existing file at `path` is read-only.
/// A directory-sync error explicitly reports that the file has already been replaced,
/// but persistence across a crash could not be confirmed.
///
/// # Examples
///
/// ```no_run
/// use camino::Utf8PathBuf;
/// use hrafnix_files::component::{reader::read_component_file, writer::write_component_file};
///
/// # fn main() -> Result<(), hrafnix_message::message::Message> {
/// let path = Utf8PathBuf::from("component.hxc");
/// let component = read_component_file(&path)?;
/// write_component_file(&path, &component)?;
/// # Ok(())
/// # }
/// ```
pub fn write_component_file(path: &Utf8Path, component: &ComponentFile) -> Result<(), Message> {
    write_json_file(path, ROOT, &ComponentDocument::from(component))
}
