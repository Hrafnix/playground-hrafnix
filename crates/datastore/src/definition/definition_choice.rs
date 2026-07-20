use crate::key::StoreKey;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};

/// Definition for a single choice item in a choice-based parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChoiceItemDefinition {
    id: StoreKey,
    description: ShareableString,
}

impl ChoiceItemDefinition {
    /// Creates a new `ChoiceItemDefinition` with the specified value and description.
    pub fn new<K: Into<StoreKey>, S: Into<ShareableString>>(id: K, description: S) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
        }
    }

    /// Returns the ID of the choice item.
    pub fn id(&self) -> StoreKey {
        self.id.clone()
    }

    /// Returns the description of the choice item.
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the description.
    pub fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns a new `ChoiceItemDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            id: self.id.launder(store),
            description: store.launder(&self.description),
        }
    }
}

/// Definition for a choice-based parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChoiceDefinition {
    description: ShareableString,
    choices: Vec<ChoiceItemDefinition>,
    default_value: ShareableString,
}

impl ChoiceDefinition {
    /// Creates a new `ChoiceDefinition` with the specified choices.
    pub fn new<S: Into<ShareableString>>(
        description: S,
        choices: Vec<ChoiceItemDefinition>,
    ) -> Self {
        Self {
            description: description.into(),
            choices,
            default_value: Default::default(),
        }
    }

    /// Creates a new `ChoiceDefinition` with the specified choices and default value.
    pub fn new_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        choices: Vec<ChoiceItemDefinition>,
        default_value: S2,
    ) -> Self {
        Self {
            description: description.into(),
            choices,
            default_value: default_value.into(),
        }
    }

    /// Returns a reference to the list of choices.
    pub fn choices(&self) -> &[ChoiceItemDefinition] {
        &self.choices
    }

    /// Returns true if the given value is a valid choice.
    pub fn contains<S: Into<ShareableString>>(&self, value: S) -> bool {
        let value = value.into();
        self.choices.iter().any(|choice| choice.id() == value)
    }

    /// Returns a vector of IDs for the choices.
    pub fn ids(&self) -> Vec<StoreKey> {
        self.choices.iter().map(|choice| choice.id()).collect()
    }

    /// Returns a vector of descriptions for the choices.
    pub fn descriptions(&self) -> Vec<ShareableString> {
        self.choices
            .iter()
            .map(|choice| choice.description())
            .collect()
    }

    /// Returns a new `ChoiceDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            choices: self
                .choices
                .iter()
                .map(|choice| choice.launder(store))
                .collect(),
            default_value: store.launder(&self.default_value),
        }
    }

    /// Returns the description of the parameter.
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the description.
    pub fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns the default value of the parameter.
    pub fn default_value(&self) -> ShareableString {
        self.default_value.clone()
    }

    /// Returns a reference to the default value.
    pub fn default_value_ref(&self) -> &ShareableString {
        &self.default_value
    }
}

impl PartialEq<&ChoiceDefinition> for ChoiceDefinition {
    fn eq(&self, other: &&ChoiceDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<ChoiceDefinition> for &ChoiceDefinition {
    fn eq(&self, other: &ChoiceDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for ChoiceDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Choice - default: \"{}\" [{}]",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.default_value,
            self.choices
                .iter()
                .map(|choice| format!("{} ({})", choice.id(), choice.description()))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
