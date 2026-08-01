use crate::definition::FileDefinition;
use crate::editable::FileEditable;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a file data value in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileFrozen {
    definition: FileDefinition,
    value: ShareableString,
    hash: [u8; 32],
}

impl FileFrozen {
    /// Creates a new `FileFrozen` instance.
    #[must_use]
    pub fn new(definition: FileDefinition) -> Self {
        let value = definition.default_value();

        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `FileFrozen` instance with a specified value.
    #[must_use]
    pub fn new_with_value(definition: FileDefinition, value: ShareableString) -> Self {
        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `FileFrozen` instance from a given `FileEditable` value.
    #[must_use]
    pub fn new_from_editable(basic: &FileEditable) -> Self {
        let definition = basic.definition().clone();
        let value = basic.value().clone();
        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Converts the current `FileFrozen` instance into a `FileEditable` instance.
    #[must_use]
    pub fn thaw(&self) -> FileEditable {
        FileEditable::new(self)
    }

    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"File");

        h.update(&self.value.current_blake3_hash());

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the value as a `ShareableString`.
    #[must_use]
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the file definition.
    #[must_use]
    pub fn definition(&self) -> &FileDefinition {
        &self.definition
    }

    /// Returns the pre-calculated BLAKE3 hash of the value.
    #[must_use]
    pub fn hash(&self) -> [u8; 32] {
        self.hash
    }
}

impl PartialEq<&FileFrozen> for FileFrozen {
    fn eq(&self, other: &&FileFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<FileFrozen> for &FileFrozen {
    fn eq(&self, other: &FileFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for FileFrozen {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) File - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
