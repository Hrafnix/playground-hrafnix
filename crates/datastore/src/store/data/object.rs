use crate::StoreError;
use crate::definition::{ItemDefinitionType, ObjectDefinition};
use crate::key::{ParameterKey, StoreKey, VariableKey};
use crate::static_store::data::StaticObject;
use crate::store::{
    Basic, CommonStoreTraitInternal, Container, ContainerItem, StoreHashContainer, Table, TreePrint,
};
use rustc_hash::FxHashMap;
use shareable_string::SharedStringStore;
use std::collections::hash_map::Entry;

/// A top-level object in the store.
#[derive(Debug, Clone)]
pub(crate) struct Object {
    definition: ObjectDefinition,
    parameters: FxHashMap<ParameterKey, ContainerItem>,
    variables: FxHashMap<VariableKey, ContainerItem>,
    shared_hash: StoreHashContainer,
}

impl Object {
    /// Creates a new `Object` from a definition.
    pub(crate) fn new(definition: &ObjectDefinition) -> Self {
        let mut parameters = FxHashMap::default();
        for (key, item_definition) in definition.parameter_iter() {
            match item_definition.item_type() {
                ItemDefinitionType::Basic(basic) => {
                    parameters.insert(key.clone(), ContainerItem::Basic(Basic::new(basic.clone())));
                }
                ItemDefinitionType::Struct(_struct) => {
                    parameters.insert(
                        key.clone(),
                        ContainerItem::Container(Container::new_struct(_struct.clone())),
                    );
                }
                ItemDefinitionType::Table(table) => {
                    parameters.insert(key.clone(), ContainerItem::Table(Table::new(table.clone())));
                }
                ItemDefinitionType::Map(map) => {
                    parameters.insert(
                        key.clone(),
                        ContainerItem::Container(Container::new_map(map.clone())),
                    );
                }
            }
        }

        let mut variables = FxHashMap::default();
        for (key, item_definition) in definition.variable_iter() {
            match item_definition.item_type() {
                ItemDefinitionType::Basic(basic) => {
                    variables.insert(key.clone(), ContainerItem::Basic(Basic::new(basic.clone())));
                }
                ItemDefinitionType::Struct(_struct) => {
                    variables.insert(
                        key.clone(),
                        ContainerItem::Container(Container::new_struct(_struct.clone())),
                    );
                }
                ItemDefinitionType::Table(table) => {
                    variables.insert(key.clone(), ContainerItem::Table(Table::new(table.clone())));
                }
                ItemDefinitionType::Map(map) => {
                    variables.insert(
                        key.clone(),
                        ContainerItem::Container(Container::new_map(map.clone())),
                    );
                }
            }
        }

        let mut object = Object {
            definition: definition.clone(),
            parameters,
            variables,
            shared_hash: StoreHashContainer::default(),
        };
        object.update_shared_hash();

        object
    }

    /// Returns a new `Object` with strings laundered through the provided store.
    pub(crate) fn launder(&self, store: &SharedStringStore) -> Self {
        let mut parameter = FxHashMap::default();
        for (key, item) in &self.parameters {
            let laundered_item = match item {
                ContainerItem::Basic(b) => ContainerItem::Basic(b.launder(store)),
                ContainerItem::Table(t) => ContainerItem::Table(t.launder(store)),
                ContainerItem::Container(c) => ContainerItem::Container(c.launder(store)),
            };
            parameter.insert(key.launder(store), laundered_item);
        }

        let mut variables = FxHashMap::default();
        for (key, item) in &self.variables {
            let laundered_item = match item {
                ContainerItem::Basic(b) => ContainerItem::Basic(b.launder(store)),
                ContainerItem::Table(t) => ContainerItem::Table(t.launder(store)),
                ContainerItem::Container(c) => ContainerItem::Container(c.launder(store)),
            };
            variables.insert(key.launder(store), laundered_item);
        }

        let laundered_definition = self.definition.launder(store);

        let mut laundered = Self {
            definition: laundered_definition,
            parameters: parameter,
            variables,
            shared_hash: StoreHashContainer::default(),
        };
        laundered.update_shared_hash();
        laundered
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
        self.parameters
            .get(key.as_ref())
            .or_else(|| self.variables.get(key.as_ref()))
            .cloned()
            .ok_or_else(|| {
                if key.as_ref().starts_with("v_") {
                    StoreError::VariableNotFound
                } else {
                    StoreError::ParameterNotFound
                }
            })
    }

    /// Sets the item for the given key and updates the hash.
    pub(crate) fn set_item(
        &mut self,
        key: &StoreKey,
        item: ContainerItem,
    ) -> Result<(), StoreError> {
        if let Ok(pk) = ParameterKey::new(key.as_shareable_string().clone()) {
            if let Entry::Occupied(mut entry) = self.parameters.entry(pk) {
                entry.insert(item);
                self.update_shared_hash();
                return Ok(());
            }
        }

        if let Ok(vk) = VariableKey::new(key.as_shareable_string().clone()) {
            if let Entry::Occupied(mut entry) = self.variables.entry(vk) {
                entry.insert(item);
                self.update_shared_hash();
                return Ok(());
            }
        }

        Err(if key.starts_with("v_") {
            StoreError::VariableNotFound
        } else {
            StoreError::ParameterNotFound
        })
    }

    /// Updates items in this object from the given static parameter map.
    /// Items with matching types are updated in-place; type-mismatched items are replaced.
    pub(crate) fn update_from_static(
        &mut self,
        parameter: &std::collections::BTreeMap<
            ParameterKey,
            crate::static_store::data::ItemParameter,
        >,
        variables: &std::collections::BTreeMap<
            VariableKey,
            crate::static_store::data::ItemParameter,
        >,
    ) -> Result<(), StoreError> {
        for (key, static_parameter) in parameter {
            if let Some(item) = self.parameters.get_mut(key)
                && item.matches_static(static_parameter)
            {
                item.update_from_static(static_parameter)?;
                continue;
            }

            // If doesn't exist or type mismatch, replace it.
            self.parameters
                .insert(key.clone(), ContainerItem::from(static_parameter));
        }

        for (key, static_variable) in variables {
            if let Some(item) = self.variables.get_mut(key)
                && item.matches_static(static_variable)
            {
                item.update_from_static(static_variable)?;
                continue;
            }

            // If doesn't exist or type mismatch, replace it.
            self.variables
                .insert(key.clone(), ContainerItem::from(static_variable));
        }

        self.update_shared_hash();
        Ok(())
    }

    /// Clears the hash of this object and all nested items.
    pub(crate) fn clear_hash_all(&mut self) {
        self.clear_shared_hash();
        for item in self.parameters.values_mut() {
            match item {
                ContainerItem::Basic(item) => item.clear_shared_hash(),
                ContainerItem::Table(item) => item.clear_shared_hash(),
                ContainerItem::Container(item) => item.clear_hash_all(),
            }
        }
        for item in self.variables.values_mut() {
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
        let parameters = static_object
            .parameters()
            .iter()
            .map(|(k, v)| (k.clone(), ContainerItem::from(v)))
            .collect();
        let variables = static_object
            .variables()
            .iter()
            .map(|(k, v)| (k.clone(), ContainerItem::from(v)))
            .collect();
        let o = Self {
            definition: static_object.definition().clone(),
            parameters,
            variables,
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

        h.update(&(self.parameters.len() as u64).to_le_bytes());

        // Sort keys for deterministic hashing
        let mut p_keys: Vec<_> = self.parameters.keys().cloned().collect();
        p_keys.sort();

        for key in p_keys {
            h.update(&key.current_blake3_hash());
            if let Some(value) = self.parameters.get_mut(&key) {
                value.update_shared_hash();
                h.update(&value.current_shared_hash());
            }
        }

        h.update(&(self.variables.len() as u64).to_le_bytes());

        // Sort keys for deterministic hashing
        let mut v_keys: Vec<_> = self.variables.keys().cloned().collect();
        v_keys.sort();

        for key in v_keys {
            h.update(&key.current_blake3_hash());
            if let Some(value) = self.variables.get_mut(&key) {
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
        let mut keys: Vec<_> = self.parameters.keys().collect();
        keys.sort();

        for (i, key) in keys.iter().enumerate() {
            let item_last = self.variables.is_empty() && i == keys.len() - 1;
            if let Some(item) = self.parameters.get(*key) {
                item.tree_print(f, key.as_str(), &next_prefix, item_last)?;
            }
        }

        let mut keys: Vec<_> = self.variables.keys().collect();
        keys.sort();

        for (i, key) in keys.iter().enumerate() {
            let item_last = i == keys.len() - 1;
            if let Some(item) = self.variables.get(*key) {
                item.tree_print(f, key.as_str(), &next_prefix, item_last)?;
            }
        }

        Ok(())
    }
}
