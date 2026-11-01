use crate::background_decoration::writer::BackgroundDecorationDocument;
use crate::common::write_json_file;
use crate::foreground_decoration::writer::ForegroundDecorationDocument;
use crate::metadata::writer::MetadataDocument;
use crate::model::ROOT;
use crate::model::data::ModelFile;
use camino::Utf8Path;
use hrafnix_message::message::Message;
use serde::Serialize;

/// The current JSON representation of the `model` entry used to write a [`ModelFile`]
/// to disk.
///
/// The file version is emitted beside the entry at the top level by [`write_json_file`].
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
struct ModelDocument<'a> {
    /// Descriptive information about the model.
    metadata: MetadataDocument<'a>,
    /// Background decorations, omitted when empty for compatibility with older files.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    background_decorations: Vec<BackgroundDecorationDocument<'a>>,
    /// Foreground decorations, omitted when empty for compatibility with older files.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    foreground_decorations: Vec<ForegroundDecorationDocument<'a>>,
}

impl<'a> From<&'a ModelFile> for ModelDocument<'a> {
    fn from(model: &'a ModelFile) -> Self {
        Self {
            metadata: MetadataDocument::from(model.metadata()),
            background_decorations: model
                .background_decorations()
                .iter()
                .map(BackgroundDecorationDocument::from)
                .collect(),
            foreground_decorations: model
                .foreground_decorations()
                .iter()
                .map(ForegroundDecorationDocument::from)
                .collect(),
        }
    }
}

/// Serializes a model as UTF-8 JSON, atomically creating or replacing the file at `path`.
///
/// The document is written to a temporary file beside the destination and renamed over it,
/// so failures before replacement leave an existing file untouched. Parent directories
/// must already exist. On Unix, the parent directory is synced after replacement.
///
/// # Errors
///
/// Returns a localized error if serialization fails,
/// the complete serialized file exceeds [`MAX_FILE_SIZE`](crate::common::MAX_FILE_SIZE),
/// the file cannot be created or written, or an existing file at `path` is read-only.
/// A directory-sync error explicitly reports that the file has already been replaced,
/// but persistence across a crash could not be confirmed.
///
/// # Examples
///
/// ```no_run
/// use camino::Utf8PathBuf;
/// use hrafnix_files::model::{reader::read_model_file, writer::write_model_file};
///
/// # fn main() -> Result<(), hrafnix_message::message::Message> {
/// let path = Utf8PathBuf::from("model.hxm");
/// let model = read_model_file(&path)?;
/// write_model_file(&path, &model)?;
/// # Ok(())
/// # }
/// ```
pub fn write_model_file(path: &Utf8Path, model: &ModelFile) -> Result<(), Message> {
    write_json_file(path, ROOT, &ModelDocument::from(model))
}
