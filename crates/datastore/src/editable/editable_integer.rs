use crate::definition::IntegerDefinition;
use crate::frozen::IntegerFrozen;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents integer data value in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegerEditable {
    definition: IntegerDefinition,
    value: ShareableString,
}

impl IntegerEditable {
    /// Creates a new `IntegerEditable` instance from a given `IntegerFrozen` value.
    pub fn new(frozen_number: &IntegerFrozen) -> Self {
        Self {
            definition: frozen_number.definition().clone(),
            value: frozen_number.value().clone(),
        }
    }

    /// Converts the current `IntegerEditable` instance into an `IntegerFrozen` instance.
    pub fn freeze(&self) -> IntegerFrozen {
        IntegerFrozen::new_from_editable(self)
    }

    /// Returns the value as a `ShareableString`.
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the number definition.
    pub fn definition(&self) -> &IntegerDefinition {
        &self.definition
    }

    /// Sets the value and updates the hash.
    pub fn set<S: Into<ShareableString>>(&mut self, value: S) {
        self.value = value.into();
    }
}

impl PartialEq<&IntegerEditable> for IntegerEditable {
    fn eq(&self, other: &&IntegerEditable) -> bool {
        self == *other
    }
}

impl PartialEq<IntegerEditable> for &IntegerEditable {
    fn eq(&self, other: &IntegerEditable) -> bool {
        *self == other
    }
}

impl TreePrint for IntegerEditable {
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
