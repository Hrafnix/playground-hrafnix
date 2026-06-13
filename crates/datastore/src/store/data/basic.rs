use crate::definition::BasicDefinition;
use crate::static_store::data::StaticBasic;
use crate::store::{CommonStoreTraitInternal, StoreHashContainer, TreePrint};
use shareable_string::{ShareableString, SharedStringStore};

/// Represents a basic data value in the store (String, Number, etc.).
#[derive(Debug, Clone)]
pub(crate) struct Basic {
    definition: BasicDefinition,
    value: ShareableString,
    current_hash: [u8; 32],
    shared_hash: StoreHashContainer,
}

impl Basic {
    /// Creates a new `Basic` instance with the given definition.
    pub(crate) fn new(definition: BasicDefinition) -> Self {
        let mut s = Self {
            value: definition.default_value().clone(),
            definition,
            current_hash: [0; 32],
            shared_hash: StoreHashContainer::new(),
        };
        s.update_current_hash();
        s.update_shared_hash();
        s
    }

    /// Returns a new `Basic` instance with strings laundered through the provided store.
    pub(crate) fn launder(&self, store: &SharedStringStore) -> Self {
        let mut s = Self {
            definition: self.definition.launder(store),
            value: store.launder(&self.value),
            current_hash: self.current_hash,
            shared_hash: StoreHashContainer::new(),
        };
        s.update_shared_hash();
        s
    }

    /// Returns a reference to the basic definition.
    pub(crate) fn definition(&self) -> &BasicDefinition {
        &self.definition
    }

    /// Returns the current value.
    pub(crate) fn get(&self) -> ShareableString {
        self.value.clone()
    }

    /// Sets a new value and updates the hash.
    pub(crate) fn set<S: Into<ShareableString>>(&mut self, value: S) {
        self.value = value.into();
        self.update_current_hash();
    }

    /// Updates this `Basic` value from a [`StaticBasic`], replacing the stored value and hash.
    pub(crate) fn update_from_static(&mut self, static_basic: &StaticBasic) {
        self.value = static_basic.value().clone();
        self.current_hash = static_basic.hash();
        self.update_shared_hash();
    }
}

impl From<&StaticBasic> for Basic {
    fn from(static_basic: &StaticBasic) -> Self {
        let hash = static_basic.hash();
        let s = Self {
            definition: static_basic.definition().clone(),
            value: static_basic.value().clone(),
            current_hash: hash,
            shared_hash: StoreHashContainer::new(),
        };
        s.shared_hash.set(hash);
        s
    }
}

impl CommonStoreTraitInternal for Basic {
    fn current_shared_hash(&self) -> [u8; 32] {
        self.shared_hash.get()
    }

    fn update_current_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"Basic");

        h.update(&self.value.current_blake3_hash());

        self.current_hash = *h.finalize().as_bytes()
    }
    fn update_shared_hash(&mut self) {
        self.shared_hash.set(self.current_hash);
    }

    fn clear_shared_hash(&mut self) {
        self.shared_hash.clear();
    }

    fn has_changed(&self) -> bool {
        self.current_hash != self.shared_hash.get()
    }

    fn is_valid(&self) -> bool {
        self.shared_hash.get() != [0u8; 32]
    }
}

impl TreePrint for Basic {
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
