use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};

/// Definition for a file-based parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileDefinition {
    description: ShareableString,
    extension_filter: ShareableString,
    bundle_on_archive: bool,
    default_value: ShareableString,
}

impl FileDefinition {
    /// Creates a new file-based `FileDefinition`.
    pub fn new<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        extension_filter: S2,
        bundle_on_archive: bool,
    ) -> Self {
        Self {
            description: description.into(),
            extension_filter: extension_filter.into(),
            bundle_on_archive,
            default_value: ShareableString::default(),
        }
    }

    /// Creates a new file-based `FileDefinition` with a default value.
    pub fn new_with_default<
        S1: Into<ShareableString>,
        S2: Into<ShareableString>,
        S3: Into<ShareableString>,
    >(
        description: S1,
        extension_filter: S2,
        bundle_on_archive: bool,
        default_value: S3,
    ) -> Self {
        Self {
            description: description.into(),
            extension_filter: extension_filter.into(),
            bundle_on_archive,
            default_value: default_value.into(),
        }
    }

    /// Returns the extension filter.
    #[must_use]
    pub fn extension_filter(&self) -> ShareableString {
        self.extension_filter.clone()
    }

    /// Returns a reference to the extension filter.
    #[must_use]
    pub fn extension_filter_ref(&self) -> &ShareableString {
        &self.extension_filter
    }

    /// Returns whether the file should be bundled on archive.
    #[must_use]
    pub fn bundle_on_archive(&self) -> bool {
        self.bundle_on_archive
    }

    /// Returns a new `FileDefinition` with strings laundered through the provided store.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            extension_filter: store.launder(&self.extension_filter),
            bundle_on_archive: self.bundle_on_archive,
            default_value: store.launder(&self.default_value),
        }
    }

    /// Returns the description of the parameter.
    #[must_use]
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the description.
    #[must_use]
    pub fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns the default value of the parameter.
    #[must_use]
    pub fn default_value(&self) -> ShareableString {
        self.default_value.clone()
    }

    /// Returns a reference to the default value.
    #[must_use]
    pub fn default_value_ref(&self) -> &ShareableString {
        &self.default_value
    }
}

impl PartialEq<&FileDefinition> for FileDefinition {
    fn eq(&self, other: &&FileDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<FileDefinition> for &FileDefinition {
    fn eq(&self, other: &FileDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for FileDefinition {
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
