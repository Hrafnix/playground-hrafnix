use crate::StoreError;
use crate::definition::{MapDefinition, StructDefinition, StructItemDefinition};
use crate::key::StoreKey;
use crate::static_store::data::{StaticMap, StaticProperty, StaticStruct, StaticStructItem};
use crate::store::{Basic, CommonStoreTraitInternal, StoreHashContainer, Table, TreePrint};
use rustc_hash::FxHashMap;
use shareable_string::SharedStringStore;

/// An item stored within a `Container`.
#[derive(Debug, Clone)]
pub(crate) enum ContainerItem {
    /// A basic data value.
    Basic(Basic),
    /// A table of data.
    Table(Table),
    /// A nested container.
    Container(Container),
}

impl ContainerItem {
    /// Returns `true` if this item's type and definition match the given [`StaticProperty`].
    /// Used to determine whether an in-place update is safe before calling [`update_from_static`].
    pub(crate) fn matches_static(&self, static_property: &StaticProperty) -> bool {
        match (self, static_property) {
            (ContainerItem::Basic(b), StaticProperty::Basic(sb)) => {
                b.definition() == sb.definition()
            }
            (ContainerItem::Table(t), StaticProperty::Table(st)) => {
                t.definition() == st.definition()
            }
            (ContainerItem::Container(c), StaticProperty::Struct(ss)) => {
                matches!(c.definition(), ContainerDefinition::Struct(def) if def == ss.definition())
            }
            (ContainerItem::Container(c), StaticProperty::Map(sm)) => {
                matches!(c.definition(), ContainerDefinition::Map(def) if def == sm.definition())
            }
            _ => false,
        }
    }

    /// Updates this item in-place from the given [`StaticProperty`].
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::SchemaMismatch`] if the item type does not match the static property.
    /// Call [`matches_static`] first to verify compatibility.
    pub(crate) fn update_from_static(
        &mut self,
        static_property: &StaticProperty,
    ) -> Result<(), StoreError> {
        match (self, static_property) {
            (ContainerItem::Basic(b), StaticProperty::Basic(sb)) => {
                b.update_from_static(sb);
                Ok(())
            }
            (ContainerItem::Table(t), StaticProperty::Table(st)) => {
                t.update_from_static(st);
                Ok(())
            }
            (ContainerItem::Container(c), StaticProperty::Struct(ss)) => {
                c.update_from_static_struct(ss.items());
                Ok(())
            }
            (ContainerItem::Container(c), StaticProperty::Map(sm)) => {
                c.update_from_static_map(sm.items());
                Ok(())
            }
            _ => Err(StoreError::SchemaMismatch(
                "Type mismatch in update_from_static - should have been checked by matches_static"
                    .into(),
            )),
        }
    }
}

impl From<&StaticStructItem> for ContainerItem {
    fn from(static_item: &StaticStructItem) -> Self {
        match static_item {
            StaticStructItem::Basic(b) => ContainerItem::Basic(Basic::from(b)),
            StaticStructItem::Table(t) => ContainerItem::Table(Table::from(t)),
        }
    }
}

impl From<&StaticStruct> for ContainerItem {
    fn from(static_struct: &StaticStruct) -> Self {
        ContainerItem::Container(Container::from(static_struct))
    }
}

impl From<&StaticProperty> for ContainerItem {
    fn from(static_property: &StaticProperty) -> Self {
        match static_property {
            StaticProperty::Basic(b) => ContainerItem::Basic(Basic::from(b)),
            StaticProperty::Table(t) => ContainerItem::Table(Table::from(t)),
            StaticProperty::Struct(s) => ContainerItem::Container(Container::from(s)),
            StaticProperty::Map(m) => ContainerItem::Container(Container::from(m)),
        }
    }
}

impl CommonStoreTraitInternal for ContainerItem {
    fn current_shared_hash(&self) -> [u8; 32] {
        match self {
            ContainerItem::Basic(item) => item.current_shared_hash(),
            ContainerItem::Table(item) => item.current_shared_hash(),
            ContainerItem::Container(item) => item.current_shared_hash(),
        }
    }

    fn update_current_hash(&mut self) {
        // ContainerItem delegates hash computation to its inner type; this path is never reached.
        unimplemented!()
    }

    fn update_shared_hash(&mut self) {
        match self {
            ContainerItem::Basic(item) => item.update_shared_hash(),
            ContainerItem::Table(item) => item.update_shared_hash(),
            ContainerItem::Container(item) => item.update_shared_hash(),
        }
    }

    fn clear_shared_hash(&mut self) {
        match self {
            ContainerItem::Basic(item) => item.clear_shared_hash(),
            ContainerItem::Table(item) => item.clear_shared_hash(),
            ContainerItem::Container(item) => item.clear_shared_hash(),
        }
    }

    fn has_changed(&self) -> bool {
        match self {
            ContainerItem::Basic(item) => item.has_changed(),
            ContainerItem::Table(item) => item.has_changed(),
            ContainerItem::Container(item) => item.has_changed(),
        }
    }

    fn is_valid(&self) -> bool {
        // Validity is checked on the containing Object or Container, not on individual items.
        unimplemented!()
    }
}

impl TreePrint for ContainerItem {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        match self {
            ContainerItem::Basic(b) => b.tree_print(f, label, prefix, last),
            ContainerItem::Table(t) => t.tree_print(f, label, prefix, last),
            ContainerItem::Container(c) => c.tree_print(f, label, prefix, last),
        }
    }
}

/// The definition for a `Container`.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ContainerDefinition {
    /// A struct definition.
    Struct(StructDefinition),
    /// A map definition.
    Map(MapDefinition),
}

/// A container that holds multiple `ContainerItem`s.
/// It can represent a struct, a map, or a top-level object.
#[derive(Debug, Clone)]
pub(crate) struct Container {
    definition: ContainerDefinition,
    items: FxHashMap<StoreKey, ContainerItem>,
    shared_hash: StoreHashContainer,
    locked: bool,
}

impl Container {
    /// Returns a new `Container` with strings laundered through the provided store.
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

        let laundered_definition = match &self.definition {
            ContainerDefinition::Struct(s) => ContainerDefinition::Struct(s.launder(store)),
            ContainerDefinition::Map(m) => ContainerDefinition::Map(m.launder(store)),
        };

        let mut laundered = Self {
            definition: laundered_definition,
            items,
            shared_hash: StoreHashContainer::new(),
            locked: self.locked,
        };
        laundered.update_shared_hash();
        laundered
    }

    /// Creates a new `Container` representing a struct.
    pub(crate) fn new_struct(definition: StructDefinition) -> Self {
        let mut items = FxHashMap::default();
        for (key, item_definition) in definition.iter() {
            match item_definition {
                StructItemDefinition::Basic(basic) => {
                    items.insert(key.clone(), ContainerItem::Basic(Basic::new(basic.clone())));
                }
                StructItemDefinition::Table(table) => {
                    items.insert(key.clone(), ContainerItem::Table(Table::new(table.clone())));
                }
            }
        }

        let mut container = Container {
            definition: ContainerDefinition::Struct(definition),
            items,
            shared_hash: StoreHashContainer::default(),
            locked: true,
        };
        container.update_shared_hash();
        container
    }

    /// Creates a new `Container` representing a map.
    pub(crate) fn new_map(definition: MapDefinition) -> Self {
        let mut container = Container {
            definition: ContainerDefinition::Map(definition),
            items: FxHashMap::default(),
            shared_hash: StoreHashContainer::default(),
            locked: false,
        };
        container.update_shared_hash();

        container
    }

    /// Returns the keys of all items in the container.
    pub(crate) fn keys(&self) -> Vec<StoreKey> {
        self.items.keys().cloned().collect()
    }

    /// Returns a reference to the hash container.
    pub(crate) fn hash_container(&self) -> &StoreHashContainer {
        &self.shared_hash
    }

    /// Returns a reference to the container's definition.
    pub(crate) fn definition(&self) -> &ContainerDefinition {
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
        if self.locked && !self.items.contains_key(key) {
            return Err(StoreError::PropertyNotFound);
        }
        self.items.insert(key.clone(), item);
        self.update_shared_hash();
        Ok(())
    }

    /// Clears the hash of this container and all nested items.
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

    /// Updates items in this struct container from the given static items map.
    /// Existing items with matching types are updated in-place; mismatched items are replaced.
    pub(crate) fn update_from_static_struct(
        &mut self,
        items: &std::collections::BTreeMap<StoreKey, StaticStructItem>,
    ) {
        for (key, static_item) in items {
            if let Some(item) = self.items.get_mut(key) {
                match (item, static_item) {
                    (ContainerItem::Basic(b), StaticStructItem::Basic(sb)) => {
                        b.update_from_static(sb);
                    }
                    (ContainerItem::Table(t), StaticStructItem::Table(st)) => {
                        t.update_from_static(st);
                    }
                    _ => {
                        self.items
                            .insert(key.clone(), ContainerItem::from(static_item));
                    }
                }
            } else {
                self.items
                    .insert(key.clone(), ContainerItem::from(static_item));
            }
        }
        self.update_shared_hash();
    }

    /// Updates entries in this map container from the given static structs map.
    /// Existing entries are updated in-place where possible; new entries are inserted.
    pub(crate) fn update_from_static_map(
        &mut self,
        items: &std::collections::BTreeMap<StoreKey, StaticStruct>,
    ) {
        for (key, static_struct) in items {
            if let Some(ContainerItem::Container(c)) = self.items.get_mut(key) {
                c.update_from_static_struct(static_struct.items());
            } else {
                self.items
                    .insert(key.clone(), ContainerItem::from(static_struct));
            }
        }
        self.update_shared_hash();
    }
}

impl From<&StaticStruct> for Container {
    fn from(static_struct: &StaticStruct) -> Self {
        let items = static_struct
            .items()
            .iter()
            .map(|(k, v)| (k.clone(), ContainerItem::from(v)))
            .collect();
        let mut c = Self {
            definition: ContainerDefinition::Struct(static_struct.definition().clone()),
            items,
            shared_hash: StoreHashContainer::new(),
            locked: true,
        };
        c.update_shared_hash();
        c
    }
}

impl From<&StaticMap> for Container {
    fn from(static_map: &StaticMap) -> Self {
        let items = static_map
            .items()
            .iter()
            .map(|(k, v)| (k.clone(), ContainerItem::from(v)))
            .collect();
        let mut c = Self {
            definition: ContainerDefinition::Map(static_map.definition().clone()),
            items,
            shared_hash: StoreHashContainer::new(),
            locked: true,
        };
        c.update_shared_hash();
        c
    }
}

impl CommonStoreTraitInternal for Container {
    fn current_shared_hash(&self) -> [u8; 32] {
        self.shared_hash.get()
    }

    fn update_current_hash(&mut self) {
        // Container hash is computed directly via update_shared_hash; this path is never reached.
        unimplemented!()
    }

    fn update_shared_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        h.update(&[0x01]);

        match &self.definition {
            ContainerDefinition::Struct(_) => {
                h.update(b"Struct");
            }
            ContainerDefinition::Map(_) => {
                h.update(b"Map");
            }
        }

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
        // Change-tracking for containers is handled at the proxy level, not here.
        unimplemented!()
    }

    fn is_valid(&self) -> bool {
        self.shared_hash.get() != [0u8; 32]
    }
}

impl TreePrint for Container {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        let type_str = match &self.definition {
            ContainerDefinition::Struct(_) => "Struct",
            ContainerDefinition::Map(_) => "Map",
        };
        let description = match &self.definition {
            ContainerDefinition::Struct(s) => s.description(),
            ContainerDefinition::Map(m) => m.description(),
        };

        writeln!(
            f,
            "{}{}{}: [{}] ({})",
            prefix,
            Self::branch_char(prefix, last),
            label,
            type_str,
            description
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
