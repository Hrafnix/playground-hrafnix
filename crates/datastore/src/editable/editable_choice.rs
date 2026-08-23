use crate::definition::ChoiceDefinition;
use crate::frozen::ChoiceFrozen;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a choice data value in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChoiceEditable {
    /// Definition metadata for this choice value.
    definition: ChoiceDefinition,
    /// Current value for this choice data, stored as a `ShareableString`.
    value: ShareableString,
}

impl ChoiceEditable {
    /// Creates a new `ChoiceEditable` instance from a given `ChoiceFrozen` value.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new(frozen_choice: &ChoiceFrozen) -> Self {
        Self {
            definition: frozen_choice.definition().clone(),
            value: frozen_choice.value(),
        }
    }

    /// Converts the current `ChoiceEditable` instance into a `ChoiceFrozen` instance.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn freeze(&self) -> ChoiceFrozen {
        ChoiceFrozen::new_from_editable(self)
    }

    /// Returns the value as a `ShareableString`.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the choice definition.
    #[must_use]
    pub const fn definition(&self) -> &ChoiceDefinition {
        &self.definition
    }

    /// Sets the value and updates the hash.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn set<S: Into<ShareableString>>(&mut self, value: S) {
        self.value = value.into();
    }
}

impl PartialEq<&ChoiceEditable> for ChoiceEditable {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&ChoiceEditable) -> bool {
        self == *other
    }
}

impl PartialEq<ChoiceEditable> for &ChoiceEditable {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ChoiceEditable) -> bool {
        *self == other
    }
}

impl TreePrint for ChoiceEditable {
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
            "{}{}{} ({}) Choice - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
