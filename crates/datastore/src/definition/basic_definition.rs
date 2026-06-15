use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::sync::Arc;

/// Definition for a file-based parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileDefinition {
    extension_filter: ShareableString,
}

impl FileDefinition {
    /// Creates a new `FileDefinition` with the specified extension filter.
    pub fn new<S: Into<ShareableString>>(extension_filter: S) -> Self {
        Self {
            extension_filter: extension_filter.into(),
        }
    }

    /// Returns the extension filter.
    pub fn extension_filter(&self) -> ShareableString {
        self.extension_filter.clone()
    }

    /// Returns a reference to the extension filter.
    pub fn extension_filter_ref(&self) -> &ShareableString {
        &self.extension_filter
    }

    /// Returns a new `FileDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            extension_filter: store.launder(&self.extension_filter),
        }
    }
}

/// Definition for a choice-based parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChoiceDefinition {
    choices: Vec<ShareableString>,
}

impl ChoiceDefinition {
    /// Creates a new `ChoiceDefinition` with the specified choices.
    pub fn new(choices: Vec<ShareableString>) -> Self {
        Self { choices }
    }

    /// Returns a reference to the list of choices.
    pub fn choices(&self) -> &Vec<ShareableString> {
        &self.choices
    }

    /// Returns a new `ChoiceDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            choices: self
                .choices
                .iter()
                .map(|choice| store.launder(choice))
                .collect(),
        }
    }
}

/// The type of basic definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum BasicDefinitionType {
    /// A string value.
    #[default]
    String,
    /// A numeric value.
    Number,
    /// A file path.
    File(FileDefinition),
    /// A value chosen from a predefined list.
    Choice(ChoiceDefinition),
}

impl BasicDefinitionType {
    /// Returns a new `BasicDefinitionType` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        match self {
            Self::String => Self::String,
            Self::Number => Self::Number,
            Self::File(def) => Self::File(def.launder(store)),
            Self::Choice(def) => Self::Choice(def.launder(store)),
        }
    }
}

/// Definition for a basic parameter (String, Number, File, or Choice).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BasicDefinition {
    description: ShareableString,
    item_type: Arc<BasicDefinitionType>,
    default_value: ShareableString,
}

impl BasicDefinition {
    /// Creates a new `BasicDefinition`.
    fn new<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        item_type: BasicDefinitionType,
        default_value: Option<S2>,
    ) -> Self {
        Self {
            description: description.into(),
            item_type: Arc::new(item_type),
            default_value: default_value
                .map(|v| v.into())
                .unwrap_or_else(|| ShareableString::new("")),
        }
    }

    /// Creates a new string-based `BasicDefinition`.
    pub fn new_string<S: Into<ShareableString>>(description: S) -> Self {
        Self::new(
            description,
            BasicDefinitionType::String,
            Option::<ShareableString>::None,
        )
    }

    /// Creates a new string-based `BasicDefinition` with a default value.
    pub fn new_string_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        default_value: S2,
    ) -> Self {
        Self::new(
            description,
            BasicDefinitionType::String,
            Some(default_value),
        )
    }

    /// Creates a new number-based `BasicDefinition`.
    pub fn new_number<S: Into<ShareableString>>(description: S) -> Self {
        Self::new(
            description,
            BasicDefinitionType::Number,
            Option::<ShareableString>::None,
        )
    }

    /// Creates a new number-based `BasicDefinition` with a default value.
    pub fn new_number_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        default_value: S2,
    ) -> Self {
        Self::new(
            description,
            BasicDefinitionType::Number,
            Some(default_value),
        )
    }

    /// Creates a new file-based `BasicDefinition`.
    pub fn new_file<S: Into<ShareableString>>(description: S, definition: FileDefinition) -> Self {
        Self::new(
            description,
            BasicDefinitionType::File(definition),
            Option::<ShareableString>::None,
        )
    }

    /// Creates a new file-based `BasicDefinition` with a default value.
    pub fn new_file_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        definition: FileDefinition,
        default_value: S2,
    ) -> Self {
        Self::new(
            description,
            BasicDefinitionType::File(definition),
            Some(default_value),
        )
    }

    /// Creates a new choice-based `BasicDefinition`.
    pub fn new_choice<S: Into<ShareableString>>(
        description: S,
        definition: ChoiceDefinition,
    ) -> Self {
        Self::new(
            description,
            BasicDefinitionType::Choice(definition),
            Option::<ShareableString>::None,
        )
    }

    /// Creates a new choice-based `BasicDefinition` with a default value.
    pub fn new_choice_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        definition: ChoiceDefinition,
        default_value: S2,
    ) -> Self {
        Self::new(
            description,
            BasicDefinitionType::Choice(definition),
            Some(default_value),
        )
    }

    /// Returns the description of the parameter.
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the type definition.
    pub fn type_definition(&self) -> &BasicDefinitionType {
        self.item_type.as_ref()
    }

    /// Returns the default value of the parameter.
    pub fn default_value(&self) -> ShareableString {
        self.default_value.clone()
    }

    /// Returns a reference to the description.
    pub fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns a reference to the default value.
    pub fn default_value_ref(&self) -> &ShareableString {
        &self.default_value
    }

    /// Returns a new `BasicDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            item_type: Arc::new(self.item_type.launder(store)),
            default_value: store.launder(&self.default_value),
        }
    }
}

impl PartialEq<&BasicDefinition> for BasicDefinition {
    fn eq(&self, other: &&BasicDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<BasicDefinition> for &BasicDefinition {
    fn eq(&self, other: &BasicDefinition) -> bool {
        *self == other
    }
}
