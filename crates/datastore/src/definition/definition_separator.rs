use crate::traits::TreePrint;
use shareable_string::{ShareableString, SharedStringStore};

/// Definition for a separator-based parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct SeparatorDefinition {
    /// Human-readable description of this separator parameter.
    description: ShareableString,
}

impl SeparatorDefinition {
    /// Creates a new separator-based `SeparatorDefinition`.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new<S: Into<ShareableString>>(description: S) -> Self {
        Self {
            description: description.into(),
        }
    }

    /// Returns a new `SeparatorDefinition` with strings laundered through the provided store.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
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
}

impl PartialEq<&SeparatorDefinition> for SeparatorDefinition {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&SeparatorDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<SeparatorDefinition> for &SeparatorDefinition {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &SeparatorDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for SeparatorDefinition {
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
            "{}{}{} ({}) Separator",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
        )
    }
}
