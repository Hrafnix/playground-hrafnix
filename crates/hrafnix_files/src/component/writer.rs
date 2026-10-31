use crate::background_decoration::writer::BackgroundDecorationDocument;
use crate::common::write_json_file;
use crate::component::ROOT;
use crate::component::data::ComponentFile;
use crate::foreground_decoration::writer::ForegroundDecorationDocument;
use crate::metadata::writer::MetadataDocument;
use camino::Utf8Path;
use hrafnix_message::message::Message;
use serde::Serialize;

/// The current JSON representation of the `component` entry used to write a
/// [`ComponentFile`] to disk.
///
/// The file version is emitted beside the entry at the top level by [`write_json_file`].
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
struct ComponentDocument<'a> {
    /// Descriptive information about the component.
    metadata: MetadataDocument<'a>,
    /// Background decorations, omitted when empty for compatibility with older files.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    background_decorations: Vec<BackgroundDecorationDocument<'a>>,
    /// Foreground decorations, omitted when empty for compatibility with older files.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    foreground_decorations: Vec<ForegroundDecorationDocument<'a>>,
}

impl<'a> From<&'a ComponentFile> for ComponentDocument<'a> {
    fn from(component: &'a ComponentFile) -> Self {
        Self {
            metadata: MetadataDocument::from(component.metadata()),
            background_decorations: component
                .background_decorations()
                .iter()
                .map(BackgroundDecorationDocument::from)
                .collect(),
            foreground_decorations: component
                .foreground_decorations()
                .iter()
                .map(ForegroundDecorationDocument::from)
                .collect(),
        }
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
/// Returns a localized error if numeric decoration styles are invalid, serialization fails,
/// the complete serialized file exceeds [`MAX_FILE_SIZE`](crate::common::MAX_FILE_SIZE),
/// the file cannot be created or written, or an existing file at `path` is read-only.
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
