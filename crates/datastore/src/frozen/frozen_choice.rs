use crate::definition::ChoiceDefinition;
use crate::editable::ChoiceEditable;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a choice data value in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChoiceFrozen {
    /// Definition metadata for this choice value.
    definition: ChoiceDefinition,
    /// Currently valued choice data, stored as a `ShareableString`.
    value: ShareableString,
    /// Pre-computed BLAKE3 hash of the value for fast diffing.
    hash: [u8; 32],
}

impl ChoiceFrozen {
    /// Creates a new `ChoiceFrozen` instance.
    #[must_use]
    pub fn new(definition: ChoiceDefinition) -> Self {
        let value = definition.default_value();

        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `ChoiceFrozen` instance with a specified value.
    #[must_use]
    pub fn new_with_value(definition: ChoiceDefinition, value: ShareableString) -> Self {
        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `ChoiceFrozen` instance from a given `ChoiceEditable` value.
    #[must_use]
    pub fn new_from_editable(basic: &ChoiceEditable) -> Self {
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

    /// Converts the current `ChoiceFrozen` instance into a `ChoiceEditable` instance.
    #[must_use]
    pub fn thaw(&self) -> ChoiceEditable {
        ChoiceEditable::new(self)
    }

    /// Recomputes and stores the BLAKE3 hash of the current value.
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"Choice");

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
    pub const fn definition(&self) -> &ChoiceDefinition {
        &self.definition
    }

    /// Returns the pre-calculated BLAKE3 hash of the value.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }
}

impl PartialEq<&ChoiceFrozen> for ChoiceFrozen {
    fn eq(&self, other: &&ChoiceFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<ChoiceFrozen> for &ChoiceFrozen {
    fn eq(&self, other: &ChoiceFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for ChoiceFrozen {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Choice - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
