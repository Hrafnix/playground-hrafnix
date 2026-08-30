use crate::definition::IntegerDefinition;
use crate::frozen::IntegerFrozen;
use crate::traits::TreePrint;
use shareable_string::ShareableString;

/// Represents integer data value in the editable data.
#[derive(Debug, Clone, PartialEq)]
pub struct IntegerEditable {
    /// Definition metadata for this integer value.
    definition: IntegerDefinition,
    /// Current value for this integer data, stored as a `ShareableString`.
    value: ShareableString,
}

impl IntegerEditable {
    /// Creates a new `IntegerEditable` instance from a given `IntegerFrozen` value.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new(frozen_number: &IntegerFrozen) -> Self {
        Self {
            definition: frozen_number.definition().clone(),
            value: frozen_number.value(),
        }
    }

    /// Converts the current `IntegerEditable` instance into an `IntegerFrozen` instance.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn freeze(&self) -> IntegerFrozen {
        IntegerFrozen::new_from_editable(self)
    }

    /// Returns the value as a `ShareableString`.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the number definition.
    #[must_use]
    pub const fn definition(&self) -> &IntegerDefinition {
        &self.definition
    }

    /// Sets the value and updates the hash.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn set<S: Into<ShareableString>>(&mut self, value: S) {
        self.value = value.into();
    }
}

impl PartialEq<&IntegerEditable> for IntegerEditable {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&IntegerEditable) -> bool {
        self == *other
    }
}

impl PartialEq<IntegerEditable> for &IntegerEditable {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &IntegerEditable) -> bool {
        *self == other
    }
}

impl TreePrint for IntegerEditable {
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
            "{}{}{} ({}) Integer - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
