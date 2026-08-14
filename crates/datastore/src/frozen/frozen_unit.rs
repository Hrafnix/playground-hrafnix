use crate::definition::UnitDefinition;
use crate::editable::UnitEditable;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a unit data value in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnitFrozen {
    /// Definition metadata for this unit value.
    definition: UnitDefinition,
    /// Currently valued unit data, stored as a `ShareableString`.
    value: ShareableString,
    /// Pre-computed BLAKE3 hash of the value for fast diffing.
    hash: [u8; 32],
}

impl UnitFrozen {
    /// Creates a new `UnitFrozen` instance.
    #[must_use]
    #[hotpath::measure]
    pub fn new(definition: UnitDefinition) -> Self {
        let value = definition.default_value();

        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `UnitFrozen` instance with a specified value.
    #[must_use]
    #[hotpath::measure]
    pub fn new_with_value(definition: UnitDefinition, value: ShareableString) -> Self {
        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `UnitFrozen` instance from a given `UnitEditable` value.
    #[must_use]
    #[hotpath::measure]
    pub fn new_from_editable(basic: &UnitEditable) -> Self {
        let definition = basic.definition().clone();
        let value = basic.value();
        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Converts the current `UnitFrozen` instance into a `UnitEditable` instance.
    #[must_use]
    #[hotpath::measure]
    pub fn thaw(&self) -> UnitEditable {
        UnitEditable::new(self)
    }

    /// Recomputes and stores the BLAKE3 hash of the current value.
    #[hotpath::measure]
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"Unit");

        h.update(&self.value.current_blake3_hash());

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the value as a `ShareableString`.
    #[must_use]
    #[hotpath::measure]
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the unit definition.
    #[must_use]
    pub const fn definition(&self) -> &UnitDefinition {
        &self.definition
    }

    /// Returns the pre-calculated BLAKE3 hash of the value.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }
}

impl PartialEq<&UnitFrozen> for UnitFrozen {
    #[hotpath::measure]
    fn eq(&self, other: &&UnitFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<UnitFrozen> for &UnitFrozen {
    #[hotpath::measure]
    fn eq(&self, other: &UnitFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for UnitFrozen {
    #[hotpath::measure]
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
