use crate::traits::TreePrint;
use shareable_string::{ShareableString, SharedStringStore};

/// Definition for a file-based parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct FileDefinition {
    /// Human-readable description of this file parameter.
    description: ShareableString,
    /// File-extension filter (e.g., `"*.csv"`) used in open-file dialogs.
    extension_filter: ShareableString,
    /// Whether this file should be included when archiving the project and whether file dialogs should be used to select the file.
    is_input: bool,
    /// Default value for this file parameter.
    default_value: ShareableString,
}

impl FileDefinition {
    /// Creates a new file-based `FileDefinition`.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        extension_filter: S2,
        is_input: bool,
    ) -> Self {
        Self {
            description: description.into(),
            extension_filter: extension_filter.into(),
            is_input,
            default_value: ShareableString::default(),
        }
    }

    /// Creates a new file-based `FileDefinition` with a default value.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new_with_default<
        S1: Into<ShareableString>,
        S2: Into<ShareableString>,
        S3: Into<ShareableString>,
    >(
        description: S1,
        extension_filter: S2,
        is_input: bool,
        default_value: S3,
    ) -> Self {
        Self {
            description: description.into(),
            extension_filter: extension_filter.into(),
            is_input,
            default_value: default_value.into(),
        }
    }

    /// Returns the extension filter.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn extension_filter(&self) -> ShareableString {
        self.extension_filter.clone()
    }

    /// Returns a reference to the extension filter.
    #[must_use]
    pub const fn extension_filter_ref(&self) -> &ShareableString {
        &self.extension_filter
    }

    /// Returns whether the file should be bundled when archived and whether file dialogs should be used to select the file.
    #[must_use]
    pub const fn is_input(&self) -> bool {
        self.is_input
    }

    /// Returns a new `FileDefinition` with strings laundered through the provided store.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            extension_filter: store.launder(&self.extension_filter),
            is_input: self.is_input,
            default_value: store.launder(&self.default_value),
        }
    }

    /// Returns the description of the parameter.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the description.
    #[must_use]
    pub const fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns the default value of the parameter.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn default_value(&self) -> ShareableString {
        self.default_value.clone()
    }

    /// Returns a reference to the default value.
    #[must_use]
    pub const fn default_value_ref(&self) -> &ShareableString {
        &self.default_value
    }
}

impl PartialEq<&FileDefinition> for FileDefinition {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&FileDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<FileDefinition> for &FileDefinition {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &FileDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for FileDefinition {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) File - default: \"{}\" [{}]",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.default_value,
            self.extension_filter
        )
    }
}
