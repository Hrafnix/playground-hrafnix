use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};

/// Definition for a string-based parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringDefinition {
    description: ShareableString,
    default_value: ShareableString,
}

impl StringDefinition {
    /// Creates a new string-based `StringDefinition`.
    pub fn new<S: Into<ShareableString>>(description: S) -> Self {
        Self {
            description: description.into(),
            default_value: ShareableString::default(),
        }
    }

    /// Creates a new string-based `StringDefinition` with a default value.
    pub fn new_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        default_value: S2,
    ) -> Self {
        Self {
            description: description.into(),
            default_value: default_value.into(),
        }
    }

    /// Returns a new `StringDefinition` with strings laundered through the provided store.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
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

impl PartialEq<&StringDefinition> for StringDefinition {
    fn eq(&self, other: &&StringDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<StringDefinition> for &StringDefinition {
    fn eq(&self, other: &StringDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for StringDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) String - default: \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.default_value,
        )
    }
}
