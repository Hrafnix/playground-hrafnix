use crate::definition::StringDefinition;
use crate::frozen::StringFrozen;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a string data value in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringEditable {
    definition: StringDefinition,
    value: ShareableString,
}

impl StringEditable {
    /// Creates a new `StringEditable` instance from a given `StringFrozen` value.
    #[must_use]
    pub fn new(frozen_string: &StringFrozen) -> Self {
        Self {
            definition: frozen_string.definition().clone(),
            value: frozen_string.value().clone(),
        }
    }

    /// Converts the current `StringEditable` instance into a `StringFrozen` instance.
    #[must_use]
    pub fn freeze(&self) -> StringFrozen {
        StringFrozen::new_from_editable(self)
    }

    /// Creates a new `StringEditable` instance with a specified value.
    #[must_use]
    pub fn new_with_value(definition: StringDefinition, value: ShareableString) -> Self {
        Self { definition, value }
    }

    /// Returns the value as a `ShareableString`.
    #[must_use]
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the string definition.
    #[must_use]
    pub fn definition(&self) -> &StringDefinition {
        &self.definition
    }

    /// Sets the value and updates the hash.
    pub fn set<S: Into<ShareableString>>(&mut self, value: S) {
        self.value = value.into();
    }
}

impl PartialEq<&StringEditable> for StringEditable {
    fn eq(&self, other: &&StringEditable) -> bool {
        self == *other
    }
}

impl PartialEq<StringEditable> for &StringEditable {
    fn eq(&self, other: &StringEditable) -> bool {
        *self == other
    }
}

impl TreePrint for StringEditable {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) String - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
