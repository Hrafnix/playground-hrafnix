/// Descriptive information shared by model and component files.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Metadata {
    /// Name shown to users.
    pub display_name: String,
    /// Free-form description.
    pub description: String,
    /// Author of the file.
    pub author: String,
    /// Major part of the minimum Hrafnix version required to open the file.
    pub minimum_version_major: u16,
    /// Minor part of the minimum Hrafnix version required to open the file.
    pub minimum_version_minor: u16,
}
