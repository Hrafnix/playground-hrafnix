use crate::definition::BooleanDefinition;
use crate::editable::BooleanEditable;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a choice data value in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BooleanFrozen {
    definition: BooleanDefinition,
    value: ShareableString,
    hash: [u8; 32],
}

impl BooleanFrozen {
    /// Creates a new `BooleanFrozen` instance.
    #[must_use]
    pub fn new(definition: BooleanDefinition) -> Self {
        let value = definition.default_value();

        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `BooleanFrozen` instance with a specified value.
    #[must_use]
    pub fn new_with_value(definition: BooleanDefinition, value: ShareableString) -> Self {
        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `BooleanFrozen` instance from a given `BooleanEditable` value.
    #[must_use]
    pub fn new_from_editable(basic: &BooleanEditable) -> Self {
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

    /// Converts the current `BooleanFrozen` instance into a `BooleanEditable` instance.
    #[must_use]
    pub fn thaw(&self) -> BooleanEditable {
        BooleanEditable::new(self)
    }

    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"Boolean");

        h.update(&self.value.current_blake3_hash());

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
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

    /// Returns the pre-calculated BLAKE3 hash of the value.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }
}

impl PartialEq<&BooleanFrozen> for BooleanFrozen {
    fn eq(&self, other: &&BooleanFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<BooleanFrozen> for &BooleanFrozen {
    fn eq(&self, other: &BooleanFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for BooleanFrozen {
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
