use crate::definition::IntegerDefinition;
use crate::editable::IntegerEditable;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents integer data value in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegerFrozen {
    definition: IntegerDefinition,
    value: ShareableString,
    hash: [u8; 32],
}

impl IntegerFrozen {
    /// Creates a new `IntegerFrozen` instance.
    #[must_use]
    pub fn new(definition: IntegerDefinition) -> Self {
        let value = definition.default_value();

        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `IntegerFrozen` instance with a specified value.
    #[must_use]
    pub fn new_with_value(definition: IntegerDefinition, value: ShareableString) -> Self {
        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `IntegerFrozen` instance from a given `IntegerEditable` value.
    #[must_use]
    pub fn new_from_editable(basic: &IntegerEditable) -> Self {
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

    /// Converts the current `IntegerFrozen` instance into a `IntegerEditable` instance.
    #[must_use]
    pub fn thaw(&self) -> IntegerEditable {
        IntegerEditable::new(self)
    }

    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"Integer");

        h.update(&self.value.current_blake3_hash());

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the value as a `ShareableString`.
    #[must_use]
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the integer definition.
    #[must_use]
    pub const fn definition(&self) -> &IntegerDefinition {
        &self.definition
    }

    /// Returns the pre-calculated BLAKE3 hash of the value.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }
}

impl PartialEq<&IntegerFrozen> for IntegerFrozen {
    fn eq(&self, other: &&IntegerFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<IntegerFrozen> for &IntegerFrozen {
    fn eq(&self, other: &IntegerFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for IntegerFrozen {
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
