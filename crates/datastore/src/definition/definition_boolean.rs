use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};

/// Definition for a boolean parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BooleanDefinition {
    description: ShareableString,
    true_description: ShareableString,
    false_description: ShareableString,
    default_value: ShareableString,
}

impl BooleanDefinition {
    /// Creates a new `BooleanDefinition` with the specified description.
    pub fn new<S: Into<ShareableString>>(description: S) -> Self {
        Self {
            description: description.into(),
            true_description: ShareableString::new("true"),
            false_description: ShareableString::new("false"),
            default_value: ShareableString::new(""),
        }
    }

    /// Creates a new `BooleanDefinition` with the specified description and default value.
    pub fn new_with_default<S: Into<ShareableString>>(description: S, default_value: bool) -> Self {
        Self {
            description: description.into(),
            true_description: ShareableString::new("true"),
            false_description: ShareableString::new("false"),
            default_value: ShareableString::new(if default_value { "true" } else { "false" }),
        }
    }

    /// Returns a vector of IDs for the choices.
    #[must_use]
    pub fn ids(&self) -> Vec<ShareableString> {
        vec![ShareableString::new("true"), ShareableString::new("false")]
    }

    /// Returns a vector of descriptions for the choices.
    #[must_use]
    pub fn descriptions(&self) -> Vec<ShareableString> {
        vec![
            self.true_description.clone(),
            self.false_description.clone(),
        ]
    }

    /// Returns a new `BooleanDefinition` with strings laundered through the provided store.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            true_description: store.launder(&self.true_description),
            false_description: store.launder(&self.false_description),
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

impl PartialEq<&BooleanDefinition> for BooleanDefinition {
    fn eq(&self, other: &&BooleanDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<BooleanDefinition> for &BooleanDefinition {
    fn eq(&self, other: &BooleanDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for BooleanDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Boolean - default: \"{}\" [{}]",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.default_value,
            [
                self.true_description.clone(),
                self.false_description.clone()
            ]
            .iter()
            .map(|choice| format!("{} ({})", choice.as_str().to_lowercase(), choice))
            .collect::<Vec<_>>()
            .join(", ")
        )
    }
}
