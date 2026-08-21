use crate::definition::FolderDefinition;
use crate::editable::FolderEditable;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a folder data value in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FolderFrozen {
    /// Definition metadata for this folder value.
    definition: FolderDefinition,
    /// Current value for this folder data, stored as a `ShareableString`.
    value: ShareableString,
    /// Pre-computed BLAKE3 hash of the value for fast diffing.
    hash: [u8; 32],
}

impl FolderFrozen {
    /// Creates a new `FolderFrozen` instance.
    #[must_use]
    #[hotpath::measure]
    pub fn new(definition: FolderDefinition) -> Self {
        let value = definition.default_value();

        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `FolderFrozen` instance with a specified value.
    #[must_use]
    #[hotpath::measure]
    pub fn new_with_value(definition: FolderDefinition, value: ShareableString) -> Self {
        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `FolderFrozen` instance from a given `FolderEditable` value.
    #[must_use]
    #[hotpath::measure]
    pub fn new_from_editable(basic: &FolderEditable) -> Self {
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

    /// Converts the current `FolderFrozen` instance into a `FolderEditable` instance.
    #[must_use]
    #[hotpath::measure]
    pub fn thaw(&self) -> FolderEditable {
        FolderEditable::new(self)
    }

    /// Recomputes and stores the BLAKE3 hash of the current value.
    #[hotpath::measure]
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"Folder");

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

    /// Returns a reference to the folder definition.
    #[must_use]
    pub const fn definition(&self) -> &FolderDefinition {
        &self.definition
    }

    /// Returns the pre-calculated BLAKE3 hash of the value.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }
}

impl PartialEq<&FolderFrozen> for FolderFrozen {
    #[hotpath::measure]
    fn eq(&self, other: &&FolderFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<FolderFrozen> for &FolderFrozen {
    #[hotpath::measure]
    fn eq(&self, other: &FolderFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for FolderFrozen {
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
            "{}{}{} ({}) Folder - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
