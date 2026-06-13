use crate::StoreError;
use crate::definition::{ObjectDefinition, PropertyDefinitionType};
use crate::key::StoreKey;
use crate::static_store::data::StaticObject;
use crate::store::{
    Basic, CommonStoreTraitInternal, Container, ContainerItem, StoreHashContainer, Table, TreePrint,
};
use rustc_hash::FxHashMap;
use shareable_string::SharedStringStore;

/// A top-level object in the store.
#[derive(Debug, Clone)]
pub(crate) struct Object {
    definition: ObjectDefinition,
    items: FxHashMap<StoreKey, ContainerItem>,
    shared_hash: StoreHashContainer,
}

impl Object {
    /// Creates a new `Object` from a definition.
    pub(crate) fn new(definition: &ObjectDefinition) -> Self {
        let mut items = FxHashMap::default();
        for (key, item_definition) in definition.iter() {
            match item_definition.item_type() {
                PropertyDefinitionType::Basic(basic) => {
                    items.insert(key.clone(), ContainerItem::Basic(Basic::new(basic.clone())));
                }
                PropertyDefinitionType::Struct(_struct) => {
                    items.insert(
                        key.clone(),
                        ContainerItem::Container(Container::new_struct(_struct.clone())),
                    );
                }
                PropertyDefinitionType::Table(table) => {
                    items.insert(key.clone(), ContainerItem::Table(Table::new(table.clone())));
                }
                PropertyDefinitionType::Map(map) => {
                    items.insert(
                        key.clone(),
                        ContainerItem::Container(Container::new_map(map.clone())),
                    );
                }
            }
        }
        let mut object = Object {
            definition: definition.clone(),
            items,
            shared_hash: StoreHashContainer::default(),
        };
        object.update_shared_hash();

        object
    }

    /// Returns a new `Object` with strings laundered through the provided store.
    pub(crate) fn launder(&self, store: &SharedStringStore) -> Self {
        let mut items = FxHashMap::default();
        for (key, item) in &self.items {
            let laundered_item = match item {
                ContainerItem::Basic(b) => ContainerItem::Basic(b.launder(store)),
                ContainerItem::Table(t) => ContainerItem::Table(t.launder(store)),
                ContainerItem::Container(c) => ContainerItem::Container(c.launder(store)),
            };
            items.insert(key.launder(store), laundered_item);
        }

        let laundered_definition = self.definition.launder(store);

        let mut laundered = Self {
            definition: laundered_definition,
            items,
            shared_hash: StoreHashContainer::default(),
        };
        laundered.update_shared_hash();
        laundered
    }

    /// Returns the keys of all items in the object.
    pub(crate) fn keys(&self) -> Vec<StoreKey> {
        self.items.keys().cloned().collect()
    }

    /// Returns a reference to the hash container.
    pub(crate) fn hash_container(&self) -> &StoreHashContainer {
        &self.shared_hash
    }

    /// Returns a reference to the object's definition.
    pub(crate) fn definition(&self) -> &ObjectDefinition {
        &self.definition
    }

    /// Returns the item associated with the given key.
    pub(crate) fn get_item<K: AsRef<str>>(&self, key: K) -> Result<ContainerItem, StoreError> {
        self.items
            .get(key.as_ref())
            .cloned()
            .ok_or(StoreError::PropertyNotFound)
    }

    /// Sets the item for the given key and updates the hash.
    pub(crate) fn set_item(
        &mut self,
        key: &StoreKey,
        item: ContainerItem,
    ) -> Result<(), StoreError> {
        if !self.items.contains_key(key) {
            return Err(StoreError::PropertyNotFound);
        }
        self.items.insert(key.clone(), item);
        self.update_shared_hash();
        Ok(())
    }

    /// Updates items in this object from the given static properties map.
    /// Items with matching types are updated in-place; type-mismatched items are replaced.
    pub(crate) fn update_from_static(
        &mut self,
        items: &std::collections::BTreeMap<StoreKey, crate::static_store::data::StaticProperty>,
    ) -> Result<(), StoreError> {
        for (key, static_property) in items {
            if let Some(item) = self.items.get_mut(key)
                && item.matches_static(static_property)
            {
                item.update_from_static(static_property)?;
                continue;
            }

            // If doesn't exist or type mismatch, replace it.
            self.items
                .insert(key.clone(), ContainerItem::from(static_property));
        }
        self.update_shared_hash();
        Ok(())
    }

    /// Clears the hash of this object and all nested items.
    pub(crate) fn clear_hash_all(&mut self) {
        self.clear_shared_hash();
        for item in self.items.values_mut() {
            match item {
                ContainerItem::Basic(item) => item.clear_shared_hash(),
                ContainerItem::Table(item) => item.clear_shared_hash(),
                ContainerItem::Container(item) => item.clear_hash_all(),
            }
        }
    }
}

impl From<&StaticObject> for Object {
    fn from(static_object: &StaticObject) -> Self {
        let items = static_object
            .items()
            .iter()
            .map(|(k, v)| (k.clone(), ContainerItem::from(v)))
            .collect();
        let o = Self {
            definition: static_object.definition().clone(),
            items,
            shared_hash: StoreHashContainer::new(),
        };
        o.shared_hash.set(static_object.hash());
        o
    }
}

impl CommonStoreTraitInternal for Object {
    fn current_shared_hash(&self) -> [u8; 32] {
        self.shared_hash.get()
    }

    fn update_current_hash(&mut self) {
        // Object hash is computed directly via update_shared_hash; this path is never reached.
        unimplemented!()
    }

    fn update_shared_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        h.update(&[0x01]);
        h.update(b"Object");

        h.update(&(self.items.len() as u64).to_le_bytes());

        // Sort keys for deterministic hashing
        let mut keys: Vec<StoreKey> = self.items.keys().cloned().collect();
        keys.sort_by(|a, b| a.as_str().cmp(b.as_str()));

        for key in keys {
            h.update(&key.current_blake3_hash());
            if let Some(value) = self.items.get_mut(&key) {
                value.update_shared_hash();
                h.update(&value.current_shared_hash());
            }
        }

        let digest = h.finalize();
        self.shared_hash.set(*digest.as_bytes());
    }

    fn clear_shared_hash(&mut self) {
        self.shared_hash.clear();
    }

    fn has_changed(&self) -> bool {
        // Change-tracking for objects is handled at the proxy level, not here.
        unimplemented!()
    }

    fn is_valid(&self) -> bool {
        self.shared_hash.get() != [0u8; 32]
    }
}

impl TreePrint for Object {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{}: [Object] ({})",
            prefix,
            Self::branch_char(prefix, last),
            label,
            self.definition.description()
        )?;

        let next_prefix = Self::next_prefix(prefix, last);
        let mut keys: Vec<_> = self.items.keys().collect();
        keys.sort();

        for (i, key) in keys.iter().enumerate() {
            let item_last = i == keys.len() - 1;
            if let Some(item) = self.items.get(*key) {
                item.tree_print(f, key.as_str(), &next_prefix, item_last)?;
            }
        }
        Ok(())
    }
}
