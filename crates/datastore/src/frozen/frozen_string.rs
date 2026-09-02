use crate::definition::StringDefinition;
use crate::editable::StringEditable;
use crate::traits::TreePrint;
use shareable_string::{ShareableString, SharedStringStore};

/// Represents a string data value in the frozen data.
#[derive(Debug, Clone, PartialEq)]
pub struct StringFrozen {
    /// Definition metadata for this string value.
    definition: StringDefinition,
    /// Current value for this string data, stored as a `ShareableString`.
    value: ShareableString,
    /// Pre-computed BLAKE3 hash of the value for fast diffing.
    hash: [u8; 32],
}

impl StringFrozen {
    /// Creates a new `StringFrozen` instance.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new(definition: StringDefinition) -> Self {
        let value = definition.default_value();

        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `StringFrozen` instance with a specified value.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new_with_value(definition: StringDefinition, value: ShareableString) -> Self {
        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `StringFrozen` instance from a given `StringEditable` value.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new_from_editable(basic: &StringEditable) -> Self {
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

    /// Converts the current `StringFrozen` instance into a `StringEditable` instance.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn thaw(&self) -> StringEditable {
        StringEditable::new(self)
    }

    /// Returns a copy whose strings are interned in `store`.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self::new_with_value(self.definition.launder(store), store.launder(&self.value))
    }

    /// Recomputes and stores the BLAKE3 hash of the current value.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"String");

        h.update(&self.value.current_blake3_hash());

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the value as a `ShareableString`.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the string definition.
    #[must_use]
    pub const fn definition(&self) -> &StringDefinition {
        &self.definition
    }

    /// Returns the pre-calculated BLAKE3 hash of the value.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }
}

impl PartialEq<&StringFrozen> for StringFrozen {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&StringFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<StringFrozen> for &StringFrozen {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &StringFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for StringFrozen {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) String - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
