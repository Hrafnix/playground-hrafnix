use crate::definition::ChoiceDefinition;
use crate::frozen::ChoiceFrozen;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a choice data value in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChoiceEditable {
    definition: ChoiceDefinition,
    value: ShareableString,
}

impl ChoiceEditable {
    /// Creates a new `ChoiceEditable` instance from a given `ChoiceFrozen` value.
    #[must_use]
    pub fn new(frozen_choice: &ChoiceFrozen) -> Self {
        Self {
            definition: frozen_choice.definition().clone(),
            value: frozen_choice.value().clone(),
        }
    }

    /// Converts the current `ChoiceEditable` instance into a `ChoiceFrozen` instance.
    #[must_use]
    pub fn freeze(&self) -> ChoiceFrozen {
        ChoiceFrozen::new_from_editable(self)
    }

    /// Returns the value as a `ShareableString`.
    #[must_use]
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the choice definition.
    #[must_use]
    pub const fn definition(&self) -> &ChoiceDefinition {
        &self.definition
    }

    /// Sets the value and updates the hash.
    pub fn set<S: Into<ShareableString>>(&mut self, value: S) {
        self.value = value.into();
    }
}

impl PartialEq<&ChoiceEditable> for ChoiceEditable {
    fn eq(&self, other: &&ChoiceEditable) -> bool {
        self == *other
    }
}

impl PartialEq<ChoiceEditable> for &ChoiceEditable {
    fn eq(&self, other: &ChoiceEditable) -> bool {
        *self == other
    }
}

impl TreePrint for ChoiceEditable {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Choice - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
