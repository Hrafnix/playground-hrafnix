use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};

/// Definition for a folder-based parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FolderDefinition {
    /// Human-readable description of this folder parameter.
    description: ShareableString,
    /// Whether this folder should be included when archiving the project and whether file dialogs should be used to select the folder.
    is_input: bool,
    /// Default value for this folder parameter.
    default_value: ShareableString,
}

impl FolderDefinition {
    /// Creates a new folder-based `FolderDefinition`.
    #[hotpath::measure]
    pub fn new<S: Into<ShareableString>>(description: S, is_input: bool) -> Self {
        Self {
            description: description.into(),
            is_input,
            default_value: ShareableString::default(),
        }
    }

    /// Creates a new folder-based `FolderDefinition` with a default value.
    #[hotpath::measure]
    pub fn new_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        is_input: bool,
        default_value: S2,
    ) -> Self {
        Self {
            description: description.into(),
            is_input,
            default_value: default_value.into(),
        }
    }

    /// Returns whether the folder should be included when archiving the project and whether file dialogs should be used to select the folder.
    #[must_use]
    pub const fn is_input(&self) -> bool {
        self.is_input
    }

    /// Returns a new `FolderDefinition` with strings laundered through the provided store.
    #[must_use]
    #[hotpath::measure]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            is_input: self.is_input,
            default_value: store.launder(&self.default_value),
        }
    }

    /// Returns the description of the parameter.
    #[must_use]
    #[hotpath::measure]
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
    #[hotpath::measure]
    pub fn default_value(&self) -> ShareableString {
        self.default_value.clone()
    }

    /// Returns a reference to the default value.
    #[must_use]
    pub const fn default_value_ref(&self) -> &ShareableString {
        &self.default_value
    }
}

impl PartialEq<&FolderDefinition> for FolderDefinition {
    #[hotpath::measure]
    fn eq(&self, other: &&FolderDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<FolderDefinition> for &FolderDefinition {
    #[hotpath::measure]
    fn eq(&self, other: &FolderDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for FolderDefinition {
    #[hotpath::measure]
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Folder - default: \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.default_value
        )
    }
}
