use crate::definition::BasicDefinition;
use crate::store::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a basic data value in the static store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticBasic {
    definition: BasicDefinition,
    value: ShareableString,
    hash: [u8; 32],
}

impl StaticBasic {
    /// Creates a new `StaticBasic` instance.
    pub fn new(definition: BasicDefinition, value: ShareableString) -> Self {
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
        h.update(b"Basic");

        h.update(&self.value.current_blake3_hash());

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the value as a `ShareableString`.
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the basic definition.
    pub fn definition(&self) -> &BasicDefinition {
        &self.definition
    }

    /// Returns the pre-calculated BLAKE3 hash of the value.
    pub fn hash(&self) -> [u8; 32] {
        self.hash
    }
}

impl TreePrint for StaticBasic {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{}: {} ({})",
            prefix,
            Self::branch_char(prefix, last),
            label,
            self.value,
            self.definition.description()
        )
    }
}
