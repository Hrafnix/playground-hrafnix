use crate::definition::BooleanDefinition;
use crate::frozen::BooleanFrozen;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a choice data value in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BooleanEditable {
    definition: BooleanDefinition,
    value: ShareableString,
}

impl BooleanEditable {
    /// Creates a new `BooleanEditable` instance from a given `BooleanFrozen` value.
    #[must_use]
    pub fn new(frozen_choice: &BooleanFrozen) -> Self {
        Self {
            definition: frozen_choice.definition().clone(),
            value: frozen_choice.value().clone(),
        }
    }

    /// Converts the current `BooleanEditable` instance into a `BooleanFrozen` instance.
    #[must_use]
    pub fn freeze(&self) -> BooleanFrozen {
        BooleanFrozen::new_from_editable(self)
    }

    /// Returns the value as a `ShareableString`.
    #[must_use]
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the choice definition.
    #[must_use]
    pub const fn definition(&self) -> &BooleanDefinition {
        &self.definition
    }

    /// Sets the value and updates the hash.
    pub fn set<S: Into<ShareableString>>(&mut self, value: S) {
        self.value = value.into();
    }
}

impl PartialEq<&BooleanEditable> for BooleanEditable {
    fn eq(&self, other: &&BooleanEditable) -> bool {
        self == *other
    }
}

impl PartialEq<BooleanEditable> for &BooleanEditable {
    fn eq(&self, other: &BooleanEditable) -> bool {
        *self == other
    }
}

impl TreePrint for BooleanEditable {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Boolean - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
