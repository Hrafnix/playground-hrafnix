use crate::definition::NumberDefinition;
use crate::frozen::NumberFrozen;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents number data value in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberEditable {
    /// Definition metadata for this number value.
    definition: NumberDefinition,
    /// Current value for this number data, stored as a `ShareableString`.
    value: ShareableString,
}

impl NumberEditable {
    /// Creates a new `NumberEditable` instance from a given `NumberFrozen` value.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new(frozen_number: &NumberFrozen) -> Self {
        Self {
            definition: frozen_number.definition().clone(),
            value: frozen_number.value(),
        }
    }

    /// Converts the current `NumberEditable` instance into a `NumberFrozen` instance.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn freeze(&self) -> NumberFrozen {
        NumberFrozen::new_from_editable(self)
    }

    /// Returns the value as a `ShareableString`.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the number definition.
    #[must_use]
    pub const fn definition(&self) -> &NumberDefinition {
        &self.definition
    }

    /// Sets the value and updates the hash.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn set<S: Into<ShareableString>>(&mut self, value: S) {
        self.value = value.into();
    }
}

impl PartialEq<&NumberEditable> for NumberEditable {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&NumberEditable) -> bool {
        self == *other
    }
}

impl PartialEq<NumberEditable> for &NumberEditable {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &NumberEditable) -> bool {
        *self == other
    }
}

impl TreePrint for NumberEditable {
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
            "{}{}{} ({}) Number - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
