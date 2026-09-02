use crate::definition::IntegerDefinition;
use crate::editable::IntegerEditable;
use crate::traits::TreePrint;
use shareable_string::{ShareableString, SharedStringStore};

/// Represents integer data value in the frozen data.
#[derive(Debug, Clone, PartialEq)]
pub struct IntegerFrozen {
    /// Definition metadata for this integer value.
    definition: IntegerDefinition,
    /// Current value for this integer data, stored as a `ShareableString`.
    value: ShareableString,
    /// Pre-computed BLAKE3 hash of the value for fast diffing.
    hash: [u8; 32],
}

impl IntegerFrozen {
    /// Creates a new `IntegerFrozen` instance.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
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
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
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
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
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

    /// Converts the current `IntegerFrozen` instance into an `IntegerEditable` instance.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn thaw(&self) -> IntegerEditable {
        IntegerEditable::new(self)
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
        h.update(b"Integer");

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
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&IntegerFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<IntegerFrozen> for &IntegerFrozen {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &IntegerFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for IntegerFrozen {
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
            "{}{}{} ({}) Integer - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
