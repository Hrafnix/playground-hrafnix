use crate::definition::UnitDefinition;
use crate::frozen::UnitFrozen;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a unit data value in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnitEditable {
    /// Definition metadata for this unit value.
    definition: UnitDefinition,
    /// Current value for this unit data, stored as a `ShareableString`.
    value: ShareableString,
}

impl UnitEditable {
    /// Creates a new `UnitEditable` instance from a given `UnitFrozen` value.
    #[must_use]
    pub fn new(frozen_unit: &UnitFrozen) -> Self {
        Self {
            definition: frozen_unit.definition().clone(),
            value: frozen_unit.value(),
        }
    }

    /// Converts the current `UnitEditable` instance into a `UnitFrozen` instance.
    #[must_use]
    pub fn freeze(&self) -> UnitFrozen {
        UnitFrozen::new_from_editable(self)
    }

    /// Returns the value as a `ShareableString`.
    #[must_use]
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the unit definition.
    #[must_use]
    pub const fn definition(&self) -> &UnitDefinition {
        &self.definition
    }

    /// Sets the value and updates the hash.
    pub fn set<S: Into<ShareableString>>(&mut self, value: S) {
        self.value = value.into();
    }
}

impl PartialEq<&UnitEditable> for UnitEditable {
    fn eq(&self, other: &&UnitEditable) -> bool {
        self == *other
    }
}

impl PartialEq<UnitEditable> for &UnitEditable {
    fn eq(&self, other: &UnitEditable) -> bool {
        *self == other
    }
}

impl TreePrint for UnitEditable {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Unit - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
