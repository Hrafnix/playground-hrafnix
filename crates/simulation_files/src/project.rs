//! Project manifests and filesystem layout support.

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// The Hrafnix project metadata directory.
pub const HRAFNIX_DIRECTORY: &str = ".hrafnix";
/// The Hrafnix project data directory.
pub const DATA_DIRECTORY: &str = "data";
/// The Hrafnix project component directory.
pub const COMPONENTS_DIRECTORY: &str = "components";
/// The Hrafnix project model directory.
pub const MODELS_DIRECTORY: &str = "models";
/// The Hrafnix project manifest filename.
pub const MANIFEST_FILE_NAME: &str = "hrafnix.toml";

/// Metadata stored in a Hrafnix project manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ProjectMetadata {
    /// The project layout format version.
    pub format_version: u16,
}

impl ProjectMetadata {
    /// Creates project metadata.
    #[must_use]
    pub const fn new(format_version: u16) -> Self {
        Self { format_version }
    }
}

/// A Hrafnix project manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectManifest {
    /// Project metadata.
    pub project: ProjectMetadata,
}

impl ProjectManifest {
    /// Creates a project manifest.
    #[must_use]
    pub const fn new(format_version: u16) -> Self {
        Self {
            project: ProjectMetadata::new(format_version),
        }
    }

    /// Serializes this manifest as TOML.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectError::ManifestSerialization`] when the manifest cannot
    /// be represented as TOML.
    pub fn to_toml_string(&self) -> Result<String, ProjectError> {
        toml::to_string(self).map_err(ProjectError::ManifestSerialization)
    }

    /// Parses a manifest from TOML.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectError::ManifestParsing`] when the input is not a valid
    /// Hrafnix project manifest.
    pub fn from_toml_str(input: &str) -> Result<Self, ProjectError> {
        toml::from_str(input).map_err(ProjectError::ManifestParsing)
    }
}

/// An error returned while working with a Hrafnix project.
#[derive(Debug)]
pub enum ProjectError {
    /// The manifest could not be parsed as TOML.
    ManifestParsing(toml::de::Error),
    /// The manifest could not be serialized as TOML.
    ManifestSerialization(toml::ser::Error),
    /// A filesystem operation failed.
    Io {
        /// The failed operation.
        operation: &'static str,
        /// The path involved in the operation.
        path: PathBuf,
        /// The underlying filesystem error.
        source: io::Error,
    },
    /// A required project directory is missing.
    MissingDirectory {
        /// The missing directory's path.
        path: PathBuf,
    },
    /// A required project directory is a non-directory filesystem entry.
    InvalidDirectory {
        /// The invalid directory's path.
        path: PathBuf,
    },
}

impl Display for ProjectError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ManifestParsing(error) => write!(formatter, "invalid project manifest: {error}"),
            Self::ManifestSerialization(error) => {
                write!(formatter, "could not serialize project manifest: {error}")
            }
            Self::Io {
                operation,
                path,
                source,
            } => write!(
                formatter,
                "could not {operation} `{}`: {source}",
                path.display()
            ),
            Self::MissingDirectory { path } => {
                write!(
                    formatter,
                    "project directory `{}` is missing",
                    path.display()
                )
            }
            Self::InvalidDirectory { path } => {
                write!(
                    formatter,
                    "project path `{}` must be a directory",
                    path.display()
                )
            }
        }
    }
}

impl Error for ProjectError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ManifestParsing(error) => Some(error),
            Self::ManifestSerialization(error) => Some(error),
            Self::Io { source, .. } => Some(source),
            Self::MissingDirectory { .. } | Self::InvalidDirectory { .. } => None,
        }
    }
}

/// A validated Hrafnix project and its root path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    /// The project root directory.
    root: PathBuf,
    /// The parsed project manifest.
    manifest: ProjectManifest,
}

impl Project {
    /// Creates a new project at `root`.
    ///
    /// The root, `.hrafnix`, `data`, `components`, and `models` directories are
    /// created. Creation fails if a manifest already exists, so it never
    /// overwrites an existing project.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectError`] when a filesystem operation or manifest
    /// serialization fails.
    pub fn create(
        root: impl Into<PathBuf>,
        manifest: ProjectManifest,
    ) -> Result<Self, ProjectError> {
        let root = root.into();
        let manifest_path = root.join(MANIFEST_FILE_NAME);
        if manifest_path.exists() {
            return Err(io_error(
                "create project manifest",
                manifest_path,
                io::ErrorKind::AlreadyExists,
            ));
        }

        create_directory(&root)?;
        for directory in project_directories(&root) {
            create_directory(&directory)?;
        }

        let contents = manifest.to_toml_string()?;
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&manifest_path)
            .map_err(|source| ProjectError::Io {
                operation: "create project manifest",
                path: manifest_path.clone(),
                source,
            })?;
        file.write_all(contents.as_bytes())
            .map_err(|source| ProjectError::Io {
                operation: "write project manifest",
                path: manifest_path,
                source,
            })?;

        Ok(Self { root, manifest })
    }

    /// Opens and validates a project rooted at `root`.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectError`] when the manifest cannot be read or parsed, or
    /// when the required project directories are absent or invalid.
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, ProjectError> {
        let root = root.into();
        let manifest_path = root.join(MANIFEST_FILE_NAME);
        let contents = fs::read_to_string(&manifest_path).map_err(|source| ProjectError::Io {
            operation: "read project manifest",
            path: manifest_path,
            source,
        })?;
        let manifest = ProjectManifest::from_toml_str(&contents)?;

        for directory in project_directories(&root) {
            validate_directory(directory)?;
        }

        Ok(Self { root, manifest })
    }

    /// Returns the project root.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Returns the parsed project manifest.
    #[must_use]
    pub const fn manifest(&self) -> &ProjectManifest {
        &self.manifest
    }
}

/// Returns the required project directories rooted at `root`.
fn project_directories(root: &Path) -> [PathBuf; 4] {
    [
        root.join(HRAFNIX_DIRECTORY),
        root.join(DATA_DIRECTORY),
        root.join(COMPONENTS_DIRECTORY),
        root.join(MODELS_DIRECTORY),
    ]
}

/// Creates a project directory and its missing parents.
fn create_directory(path: &Path) -> Result<(), ProjectError> {
    fs::create_dir_all(path).map_err(|source| ProjectError::Io {
        operation: "create project directory",
        path: path.to_owned(),
        source,
    })
}

/// Confirms that `path` exists and is a directory.
fn validate_directory(path: PathBuf) -> Result<(), ProjectError> {
    let metadata = match fs::metadata(&path) {
        Ok(metadata) => metadata,
        Err(source) if source.kind() == io::ErrorKind::NotFound => {
            return Err(ProjectError::MissingDirectory { path });
        }
        Err(source) => {
            return Err(ProjectError::Io {
                operation: "inspect project directory",
                path,
                source,
            });
        }
    };

    if metadata.is_dir() {
        Ok(())
    } else {
        Err(ProjectError::InvalidDirectory { path })
    }
}

/// Creates a filesystem error for a failed operation.
fn io_error(operation: &'static str, path: PathBuf, kind: io::ErrorKind) -> ProjectError {
    ProjectError::Io {
        operation,
        path,
        source: io::Error::from(kind),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    static NEXT_TEST_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

    fn test_directory() -> PathBuf {
        let number = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "simulation-files-project-test-{}-{number}",
            std::process::id()
        ))
    }

    #[test]
    fn manifest_round_trips_as_toml() {
        let manifest = ProjectManifest::new(1);
        let Ok(toml) = manifest.to_toml_string() else {
            panic!("manifest must serialize");
        };

        assert_eq!(toml, "[project]\nformat-version = 1\n");
        let parsed = ProjectManifest::from_toml_str(&toml);
        assert!(matches!(parsed, Ok(parsed) if parsed == manifest));
    }

    #[test]
    fn project_creation_creates_and_validates_the_required_layout() {
        let root = test_directory();
        let manifest = ProjectManifest::new(1);

        let created = Project::create(&root, manifest.clone());
        assert!(created.is_ok());
        assert!(root.join(HRAFNIX_DIRECTORY).is_dir());
        assert!(root.join(DATA_DIRECTORY).is_dir());
        assert!(root.join(COMPONENTS_DIRECTORY).is_dir());
        assert!(root.join(MODELS_DIRECTORY).is_dir());
        let opened = Project::open(&root).map(|project| project.manifest().clone());
        assert!(matches!(opened, Ok(opened) if opened == manifest));

        assert!(fs::remove_dir_all(root).is_ok());
    }

    #[test]
    fn project_open_rejects_a_missing_required_directory() {
        let root = test_directory();
        let manifest = ProjectManifest::new(1);
        assert!(Project::create(&root, manifest).is_ok());
        assert!(fs::remove_dir(root.join(DATA_DIRECTORY)).is_ok());

        assert!(matches!(
            Project::open(&root),
            Err(ProjectError::MissingDirectory { path }) if path == root.join(DATA_DIRECTORY)
        ));

        assert!(fs::remove_dir_all(root).is_ok());
    }
}
