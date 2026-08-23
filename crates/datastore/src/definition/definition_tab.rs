use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};

/// Definition for a tab-based parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TabDefinition {
    /// Human-readable description of this tab parameter.
    description: ShareableString,
}

impl TabDefinition {
    /// Creates a new tab-based `TabDefinition`.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new<S: Into<ShareableString>>(description: S) -> Self {
        Self {
            description: description.into(),
        }
    }

    /// Returns a new `TabDefinition` with strings laundered through the provided store.
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

impl PartialEq<&TabDefinition> for TabDefinition {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&TabDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<TabDefinition> for &TabDefinition {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &TabDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for TabDefinition {
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
            "{}{}{} ({}) Tab",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
        )
    }
}
