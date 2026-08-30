use crate::definition::BooleanDefinition;
use crate::frozen::BooleanFrozen;
use crate::traits::TreePrint;
use shareable_string::ShareableString;

/// Represents a boolean data value in the editable data.
#[derive(Debug, Clone, PartialEq)]
pub struct BooleanEditable {
    /// Definition metadata for this boolean value.
    definition: BooleanDefinition,
    /// Current value for this boolean data, stored as a `ShareableString`.
    value: ShareableString,
}

impl BooleanEditable {
    /// Creates a new `BooleanEditable` instance from a given `BooleanFrozen` value.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new(frozen_boolean: &BooleanFrozen) -> Self {
        Self {
            definition: frozen_boolean.definition().clone(),
            value: frozen_boolean.value(),
        }
    }

    /// Converts the current `BooleanEditable` instance into a `BooleanFrozen` instance.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn freeze(&self) -> BooleanFrozen {
        BooleanFrozen::new_from_editable(self)
    }

    /// Returns the value as a `ShareableString`.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the boolean definition.
    #[must_use]
    pub const fn definition(&self) -> &BooleanDefinition {
        &self.definition
    }

    /// Sets the value and updates the hash.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn set<S: Into<ShareableString>>(&mut self, value: S) {
        self.value = value.into();
    }
}

impl PartialEq<&BooleanEditable> for BooleanEditable {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&BooleanEditable) -> bool {
        self == *other
    }
}

impl PartialEq<BooleanEditable> for &BooleanEditable {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &BooleanEditable) -> bool {
        *self == other
    }
}

impl TreePrint for BooleanEditable {
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
            "{}{}{} ({}) Boolean - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
