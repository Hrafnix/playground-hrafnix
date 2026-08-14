use crate::definition::SeparatorDefinition;
use crate::editable::SeparatorEditable;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};

/// Represents a separator structural element in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeparatorFrozen {
    /// Definition metadata for this separator element.
    definition: SeparatorDefinition,
    /// Pre-computed BLAKE3 hash for fast diffing.
    hash: [u8; 32],
}

impl SeparatorFrozen {
    /// Creates a new `SeparatorFrozen` instance.
    #[must_use]
    pub fn new(definition: SeparatorDefinition) -> Self {
        let mut s = Self {
            definition,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `SeparatorFrozen` instance from a given `SeparatorEditable` value.
    #[must_use]
    pub fn new_from_editable(separator: &SeparatorEditable) -> Self {
        let definition = separator.definition().clone();
        let mut s = Self {
            definition,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Converts the current `SeparatorFrozen` instance into a `SeparatorEditable` instance.
    #[must_use]
    pub fn thaw(&self) -> SeparatorEditable {
        SeparatorEditable::new(self)
    }

    /// Recomputes and stores the BLAKE3 hash.
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"Separator");

        h.update(&self.definition.description_ref().current_blake3_hash());

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns a reference to the separator definition.
    #[must_use]
    pub const fn definition(&self) -> &SeparatorDefinition {
        &self.definition
    }

    /// Returns the pre-calculated BLAKE3 hash.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }
}

impl PartialEq<&SeparatorFrozen> for SeparatorFrozen {
    fn eq(&self, other: &&SeparatorFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<SeparatorFrozen> for &SeparatorFrozen {
    fn eq(&self, other: &SeparatorFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for SeparatorFrozen {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Separator",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
        )
    }
}
