use crate::definition::NumberDefinition;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents number data value in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberFrozen {
    definition: NumberDefinition,
    value: ShareableString,
    hash: [u8; 32],
}

impl NumberFrozen {
    /// Creates a new `NumberFrozen` instance.
    pub fn new(definition: NumberDefinition) -> Self {
        let value = definition.default_value();

        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `NumberFrozen` instance with a specified value.
    pub fn new_with_value(definition: NumberDefinition, value: ShareableString) -> Self {
        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"Number");

        h.update(&self.value.current_blake3_hash());

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the value as a `ShareableString`.
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the number definition.
    pub fn definition(&self) -> &NumberDefinition {
        &self.definition
    }

    /// Returns the pre-calculated BLAKE3 hash of the value.
    pub fn hash(&self) -> [u8; 32] {
        self.hash
    }
}

impl PartialEq<&NumberFrozen> for NumberFrozen {
    fn eq(&self, other: &&NumberFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<NumberFrozen> for &NumberFrozen {
    fn eq(&self, other: &NumberFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for NumberFrozen {
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
