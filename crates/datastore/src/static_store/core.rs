use crate::StoreError;
use crate::key::StoreKey;
use crate::static_store::data::StaticObject;
use crate::store::{Store, TreePrint};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Represents a read-only, persistent store of objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticStore {
    /// The objects contained in the store.
    objects: BTreeMap<StoreKey, StaticObject>,
    /// The pre-calculated BLAKE3 hash of the store's content.
    hash: [u8; 32],
}

impl StaticStore {
    /// Creates a new `StaticStore` with the given objects.
    pub fn new(objects: BTreeMap<StoreKey, StaticObject>) -> Self {
        let objects = objects.into_iter().collect();
        let mut s = Self {
            objects,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"StoreInternal");

        h.update(&(self.objects.len() as u64).to_le_bytes());

        for (key, obj) in &self.objects {
            h.update(&key.current_blake3_hash());
            h.update(&obj.hash());
        }

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the pre-calculated BLAKE3 hash of the store's content.
    pub fn get_blake3_hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns a reference to the internal map of objects. Used for store conversions.
    pub(crate) fn objects(&self) -> &BTreeMap<StoreKey, StaticObject> {
        &self.objects
    }

    /// Returns a reference to the static object with the specified key, if it exists.
    pub fn get<S: AsRef<str>>(&self, key: S) -> Option<&StaticObject> {
        self.objects.get(key.as_ref())
    }

    /// Returns an iterator over the key-object pairs in the store.
    pub fn iter(&self) -> impl Iterator<Item = (&StoreKey, &StaticObject)> {
        self.objects.iter()
    }
}

impl TryFrom<&Store> for StaticStore {
    type Error = StoreError;

    fn try_from(store: &Store) -> Result<Self, Self::Error> {
        let mut objects = BTreeMap::new();
        if let Ok(keys) = store.object_keys() {
            for key in keys {
                if let Ok(object) = store.get_object_internal(&key) {
                    objects.insert(key, StaticObject::try_from(&object)?);
                }
            }
        }
        Ok(Self::new(objects))
    }
}

impl TreePrint for StaticStore {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(f, "{}{}{}", prefix, Self::branch_char(prefix, last), label)?;
        let mut next_prefix = Self::next_prefix(prefix, last);
        next_prefix.pop();
        next_prefix.pop();
        let keys: Vec<_> = self.objects.keys().collect();
        for (i, key) in keys.iter().enumerate() {
            let is_last = i == keys.len() - 1;
            self.objects[*key].tree_print(f, key.as_str(), &next_prefix, is_last)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for StaticStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_display("Static Store").fmt(f)
    }
}
