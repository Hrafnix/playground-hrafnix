use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};

/// Definition for a boolean parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BooleanDefinition {
    /// Human-readable description of this boolean parameter.
    description: ShareableString,
    /// Label displayed when the value is `true`.
    true_description: ShareableString,
    /// Label displayed when the value is `false`.
    false_description: ShareableString,
    /// Default value for this boolean parameter.
    default_value: ShareableString,
}

impl BooleanDefinition {
    /// Creates a new `BooleanDefinition` with the specified description.
    #[hotpath::measure]
    pub fn new<S: Into<ShareableString>>(description: S) -> Self {
        Self {
            description: description.into(),
            true_description: ShareableString::new("True"),
            false_description: ShareableString::new("False"),
            default_value: ShareableString::new(""),
        }
    }

    /// Creates a new `BooleanDefinition` with the specified description and default value.
    #[hotpath::measure]
    pub fn new_with_default<S: Into<ShareableString>>(description: S, default_value: bool) -> Self {
        Self {
            description: description.into(),
            true_description: ShareableString::new("True"),
            false_description: ShareableString::new("False"),
            default_value: ShareableString::new(if default_value { "true" } else { "false" }),
        }
    }

    /// Returns a vector of IDs for the choices.
    #[must_use]
    #[hotpath::measure]
    pub fn ids(&self) -> Vec<ShareableString> {
        vec![ShareableString::new("true"), ShareableString::new("false")]
    }

    /// Returns a vector of descriptions for the choices.
    #[must_use]
    #[hotpath::measure]
    pub fn descriptions(&self) -> Vec<ShareableString> {
        vec![
            self.true_description.clone(),
            self.false_description.clone(),
        ]
    }

    /// Returns a new `BooleanDefinition` with strings laundered through the provided store.
    #[must_use]
    #[hotpath::measure]
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

impl PartialEq<&BooleanDefinition> for BooleanDefinition {
    #[hotpath::measure]
    fn eq(&self, other: &&BooleanDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<BooleanDefinition> for &BooleanDefinition {
    #[hotpath::measure]
    fn eq(&self, other: &BooleanDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for BooleanDefinition {
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
