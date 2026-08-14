use crate::definition::TabDefinition;
use crate::editable::TabEditable;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};

/// Represents a tab structural element in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TabFrozen {
    /// Definition metadata for this tab element.
    definition: TabDefinition,
    /// Pre-computed BLAKE3 hash for fast diffing.
    hash: [u8; 32],
}

impl TabFrozen {
    /// Creates a new `TabFrozen` instance.
    #[must_use]
    pub fn new(definition: TabDefinition) -> Self {
        let mut s = Self {
            definition,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `TabFrozen` instance from a given `TabEditable` value.
    #[must_use]
    pub fn new_from_editable(tab: &TabEditable) -> Self {
        let definition = tab.definition().clone();
        let mut s = Self {
            definition,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Converts the current `TabFrozen` instance into a `TabEditable` instance.
    #[must_use]
    pub fn thaw(&self) -> TabEditable {
        TabEditable::new(self)
    }

    /// Recomputes and stores the BLAKE3 hash.
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"Tab");

        h.update(&self.definition.description_ref().current_blake3_hash());

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns a reference to the tab definition.
    #[must_use]
    pub const fn definition(&self) -> &TabDefinition {
        &self.definition
    }

    /// Returns the pre-calculated BLAKE3 hash.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }
}

impl PartialEq<&TabFrozen> for TabFrozen {
    fn eq(&self, other: &&TabFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<TabFrozen> for &TabFrozen {
    fn eq(&self, other: &TabFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for TabFrozen {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Tab",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
        )
    }
}
