/// Descriptive information shared by model and component files.
///
/// Fields are private; use the read-only accessors.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Metadata {
    /// Name shown to users.
    display_name: String,
    /// Free-form description.
    description: String,
    /// Author of the file.
    author: String,
    /// Major part of the minimum Hrafnix version required to open the file.
    minimum_version_major: u16,
    /// Minor part of the minimum Hrafnix version required to open the file.
    minimum_version_minor: u16,
}

impl Metadata {
    /// Creates metadata with descriptive text and a minimum Hrafnix version.
    #[must_use]
    pub const fn new(
        display_name: String,
        description: String,
        author: String,
        minimum_version_major: u16,
        minimum_version_minor: u16,
    ) -> Self {
        Self {
            display_name,
            description,
            author,
            minimum_version_major,
            minimum_version_minor,
        }
    }

    /// Returns the name shown to users.
    #[must_use]
    pub const fn display_name(&self) -> &str {
        self.display_name.as_str()
    }

    /// Returns the free-form description.
    #[must_use]
    pub const fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Returns the author of the file.
    #[must_use]
    pub const fn author(&self) -> &str {
        self.author.as_str()
    }

    /// Returns the major part of the minimum required Hrafnix version.
    #[must_use]
    pub const fn minimum_version_major(&self) -> u16 {
        self.minimum_version_major
    }

    /// Returns the minor part of the minimum required Hrafnix version.
    #[must_use]
    pub const fn minimum_version_minor(&self) -> u16 {
        self.minimum_version_minor
    }
}
