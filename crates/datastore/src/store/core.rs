use crate::StoreError;
use crate::definition::ObjectDefinition;
use crate::key::StoreKey;
use crate::path::StorePath;
use crate::static_store::StaticStore;
use crate::store::data::{Basic, Container, ContainerItem, Object, Table};
use crate::store::traits::{CommonStoreTraitInternal, TreePrint};
use crate::store::{BasicProxy, ContainerProxy, ObjectProxy, TableProxy};
use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use shareable_string::{ShareableString, SharedStringStore};
use std::sync::Arc;

/// The internal implementation of the data store.
#[derive(Debug)]
pub(crate) struct StoreInternal {
    /// The objects contained in the store.
    objects: RwLock<FxHashMap<StoreKey, Object>>,
    /// The string interning store used by the entire data store.
    pub(crate) string_store: SharedStringStore,
    /// The pre-calculated BLAKE3 hash of the store's content.
    blake3_hash: RwLock<[u8; 32]>,
}

impl StoreInternal {
    /// Creates a new `StoreInternal`.
    fn new(string_store: SharedStringStore) -> Self {
        let store = StoreInternal {
            objects: FxHashMap::default().into(),
            string_store,
            blake3_hash: [0u8; 32].into(),
        };
        store.update_blake3_hash_locked();
        store
    }

    /// Updates the BLAKE3 hash while holding the lock.
    fn update_blake3_hash_locked(&self) {
        let objects = self.objects.read();
        self.update_blake3_hash(&objects);
    }

    /// Updates the BLAKE3 hash based on the provided objects.
    fn update_blake3_hash(&self, objects: &FxHashMap<StoreKey, Object>) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"StoreInternal");

        h.update(&(objects.len() as u64).to_le_bytes());

        // Sort keys for deterministic hashing
        let mut keys: Vec<&StoreKey> = objects.keys().collect();
        keys.sort_by(|a, b| a.as_str().cmp(b.as_str()));

        for key in keys {
            h.update(&key.current_blake3_hash());
            h.update(
                &objects
                    .get(key)
                    .expect("key was taken from objects, so it must exist")
                    .current_shared_hash(),
            );
        }

        let digest = h.finalize();
        let mut writer = self.blake3_hash.write();
        *writer = *digest.as_bytes();
    }

    /// Creates a new object in the store.
    pub(crate) fn create_object(
        &self,
        object_key: &StoreKey,
        definition: &ObjectDefinition,
    ) -> Result<(), StoreError> {
        let mut writer = self.objects.write();

        if writer.contains_key(object_key) {
            return Err(StoreError::ObjectKeyAlreadyExists);
        }

        let launder_definition = definition.launder(&self.string_store);
        writer.insert(object_key.clone(), Object::new(&launder_definition));

        self.update_blake3_hash(&writer);

        Ok(())
    }

    /// Deletes an object from the store.
    pub(crate) fn delete_object<K: AsRef<str>>(&self, object_key: K) -> Result<(), StoreError> {
        let mut writer = self.objects.write();

        let mut object = writer
            .remove(object_key.as_ref())
            .ok_or(StoreError::ObjectNotFound)?;

        object.clear_hash_all();

        self.update_blake3_hash(&writer);

        Ok(())
    }

    /// Adds an existing object to the store.
    pub(crate) fn add_object(
        &self,
        object_key: &StoreKey,
        object: &Object,
    ) -> Result<(), StoreError> {
        let laundered_object = object.launder(&self.string_store);
        let mut writer = self.objects.write();

        if writer.contains_key(object_key) {
            return Err(StoreError::ObjectKeyAlreadyExists);
        }

        writer.insert(object_key.launder(&self.string_store), laundered_object);

        self.update_blake3_hash(&writer);

        Ok(())
    }

    /// Returns a copy of the object for the specified object key.
    pub(crate) fn get_object<K: Into<ShareableString>>(
        &self,
        object_key: K,
    ) -> Result<Object, StoreError> {
        let reader = self.objects.read();

        reader
            .get(&object_key.into())
            .cloned()
            .ok_or(StoreError::ObjectNotFound)
    }
}

/// The main data store.
#[derive(Debug, Clone)]
pub struct Store {
    /// The internal store implementation.
    internal: Arc<StoreInternal>,
}

impl Store {
    /// Creates a new, empty `Store`.
    pub fn new(string_store: SharedStringStore) -> Self {
        Self {
            internal: Arc::new(StoreInternal::new(string_store)),
        }
    }

    /// Creates a new object in the store and returns a proxy to it.
    pub fn create_object<K: Into<StoreKey>>(
        &self,
        object_key: K,
        definition: &ObjectDefinition,
    ) -> Result<ObjectProxy, StoreError> {
        let object_key = object_key.into().launder(&self.internal.string_store);
        self.internal.create_object(&object_key, definition)?;
        self.object(object_key)
    }

    /// Returns an `ObjectProxy` for the specified path.
    pub fn object<S: Into<ShareableString>>(&self, key: S) -> Result<ObjectProxy, StoreError> {
        let key = key.into();
        let object = self.internal.get_object(key.clone())?;
        let definition = object.definition().clone();

        let keys = object.keys();
        let object_hash = object.hash_container().clone();
        let last_sync_hash = object.current_shared_hash();

        #[expect(unsafe_code)]
        let store_path = StorePath::new(unsafe { StoreKey::new_unsafe(key) });

        Ok(ObjectProxy::new(
            store_path.clone(),
            self.clone(),
            definition,
            keys,
            object_hash,
            last_sync_hash,
        ))
    }

    /// Internal method to get an item associated with the given key from the specified path.
    pub(crate) fn get_item_at_path<K: AsRef<str>>(
        &self,
        store_path: &StorePath,
        key: K,
    ) -> Result<ContainerItem, StoreError> {
        let segments = store_path.segments();
        if segments.is_empty() {
            let object_key = store_path.object_key();
            let object = self.internal.get_object(object_key)?;
            return object.get_item(key);
        }

        let container = self.get_container_internal(store_path)?;
        container.get_item(key)
    }

    /// Internal method to get an object at the specified path.
    pub(crate) fn get_object_internal<K: Into<ShareableString>>(
        &self,
        object_key: K,
    ) -> Result<Object, StoreError> {
        self.internal.get_object(object_key)
    }

    /// Internal method to get a container at the specified path.
    pub(crate) fn get_container_internal(
        &self,
        store_path: &StorePath,
    ) -> Result<Container, StoreError> {
        let object_key = store_path.object_key();
        let object = self.internal.get_object(object_key)?;

        let mut current_container: Option<Container> = None;

        for segment in store_path.segments() {
            let item = if let Some(container) = &current_container {
                container.get_item(segment)?
            } else {
                object.get_item(segment)?
            };

            match item {
                ContainerItem::Container(c) => current_container = Some(c),
                _ => return Err(StoreError::PropertyNotFound),
            }
        }

        current_container.ok_or(StoreError::PropertyNotFound)
    }

    /// Returns a `ContainerProxy` for the specified path.
    pub fn container(&self, store_path: &StorePath) -> Result<ContainerProxy, StoreError> {
        let container = self.get_container_internal(store_path)?;
        let keys = container.keys();
        let object_hash = container.hash_container().clone();
        let last_sync_hash = container.current_shared_hash();

        Ok(ContainerProxy::new(
            store_path.clone(),
            self.clone(),
            container.definition().clone(),
            keys,
            object_hash,
            last_sync_hash,
        ))
    }

    /// Returns a `TableProxy` for the specified path.
    pub fn table(&self, store_path: &StorePath) -> Result<TableProxy, StoreError> {
        let (parent_path, last_segment) = split_path(store_path)?;
        let item = self.get_item_at_path(&parent_path, &last_segment)?;

        if let ContainerItem::Table(table) = item {
            Ok(TableProxy::new(store_path.clone(), self.clone(), table))
        } else {
            Err(StoreError::PropertyNotFound)
        }
    }

    /// Sets the table data at the specified path.
    pub(crate) fn set_table(&self, store_path: &StorePath, data: &Table) -> Result<(), StoreError> {
        let (parent_path, last_segment) = split_path(store_path)?;
        let segments = parent_path.segments();

        if segments.is_empty() {
            let mut object = self.internal.get_object(parent_path.object_key())?;
            object.set_item(&last_segment, ContainerItem::Table(data.clone()))?;
            let mut writer = self.internal.objects.write();
            writer.insert(parent_path.object_key().clone(), object);
            self.internal.update_blake3_hash(&writer);
            return Ok(());
        }

        let mut container = self.get_container_internal(&parent_path)?;
        container.set_item(&last_segment, ContainerItem::Table(data.clone()))?;

        self.update_container_at_path(&parent_path, container)
    }

    /// Returns a `BasicProxy` for the specified path.
    pub fn basic(&self, store_path: &StorePath) -> Result<BasicProxy, StoreError> {
        let (parent_path, last_segment) = split_path(store_path)?;
        let item = self.get_item_at_path(&parent_path, &last_segment)?;

        if let ContainerItem::Basic(basic) = item {
            Ok(BasicProxy::new(store_path.clone(), self.clone(), basic))
        } else {
            Err(StoreError::PropertyNotFound)
        }
    }

    /// Sets the basic data at the specified path.
    pub(crate) fn set_basic(&self, store_path: &StorePath, data: &Basic) -> Result<(), StoreError> {
        let (parent_path, last_segment) = split_path(store_path)?;
        let segments = parent_path.segments();

        if segments.is_empty() {
            let mut object = self.internal.get_object(parent_path.object_key())?;
            object.set_item(&last_segment, ContainerItem::Basic(data.clone()))?;
            let mut writer = self.internal.objects.write();
            writer.insert(parent_path.object_key().clone(), object);
            self.internal.update_blake3_hash(&writer);
            return Ok(());
        }

        let mut container = self.get_container_internal(&parent_path)?;
        container.set_item(&last_segment, ContainerItem::Basic(data.clone()))?;

        self.update_container_at_path(&parent_path, container)
    }

    /// Updates the container at the specified path and propagates changes up to the object root.
    pub(crate) fn update_container_at_path(
        &self,
        path: &StorePath,
        mut container: Container,
    ) -> Result<(), StoreError> {
        container.update_shared_hash();
        let segments = path.segments();

        if segments.is_empty() {
            // Updating a top-level object - this should be handled by a separate method if needed,
            // but for now let's just return an error as Container cannot be Object anymore.
            return Err(StoreError::ObjectNotFound);
        }

        if segments.len() == 1 {
            let mut object = self.internal.get_object(path.object_key())?;
            let last_segment = segments
                .first()
                .expect("segments.len() == 1, so first() must be Some");
            object.set_item(last_segment, ContainerItem::Container(container))?;

            let mut writer = self.internal.objects.write();
            writer.insert(path.object_key().clone(), object);
            self.internal.update_blake3_hash(&writer);
            return Ok(());
        }

        // We need to update all the way up to the root object
        let (parent_path, last_segment) = split_path(path)?;
        let mut parent_container = self.get_container_internal(&parent_path)?;
        parent_container.set_item(&last_segment, ContainerItem::Container(container))?;

        self.update_container_at_path(&parent_path, parent_container)
    }

    /// Deletes the object with the specified key.
    pub fn delete_object<K: AsRef<str>>(&self, object_key: K) -> Result<(), StoreError> {
        self.internal.delete_object(object_key)
    }

    /// Returns a list of all object keys in the store.
    pub fn object_keys(&self) -> Result<Vec<StoreKey>, StoreError> {
        let reader = self.internal.objects.read();
        Ok(reader.keys().cloned().collect())
    }

    /// Returns the overall BLAKE3 hash of the store.
    pub fn get_blake3_hash(&self) -> [u8; 32] {
        *self.internal.blake3_hash.read()
    }

    /// Launders a `StoreKey` through the store's string store.
    pub fn launder_key(&self, key: StoreKey) -> StoreKey {
        key.launder(&self.internal.string_store)
    }

    /// Launders a [`ShareableString`] through the store's string store, deduplicating it.
    pub fn launder_string(&self, string: ShareableString) -> ShareableString {
        self.internal.string_store.launder(string)
    }

    /// Copies an object from another store.
    pub fn copy_object(
        &self,
        object_key: StoreKey,
        source_store: &Store,
        source_object_key: StoreKey,
    ) -> Result<ObjectProxy, StoreError> {
        let object_key = self.launder_key(object_key);
        let container = source_store.internal.get_object(&source_object_key)?;
        let container = container.launder(&self.internal.string_store);

        self.internal.add_object(&object_key, &container)?;
        self.object(object_key)
    }

    /// Converts this store into a [`StaticStore`] representation.
    pub fn to_static(&self) -> Result<StaticStore, StoreError> {
        StaticStore::try_from(self)
    }

    /// Serializes this store to a JSON string via [`StaticStore`].
    pub fn to_json(&self) -> Result<String, StoreError> {
        let static_store = self.to_static()?;
        serde_json::to_string(&static_store)
            .map_err(|e| StoreError::SerializationError(e.to_string()))
    }

    /// Deserializes a store from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, StoreError> {
        let static_store: StaticStore = serde_json::from_str(json)
            .map_err(|e| StoreError::SerializationError(e.to_string()))?;
        Ok(Self::new_from_static(&static_store))
    }

    /// Creates a new [`Store`] populated from the given [`StaticStore`].
    pub fn new_from_static(static_store: &StaticStore) -> Self {
        let string_store = SharedStringStore::new();
        let mut objects = FxHashMap::default();
        for (key, static_object) in static_store.objects() {
            let key = key.launder(&string_store);
            let object = Object::from(static_object);
            let object = object.launder(&string_store);
            objects.insert(key, object);
        }

        let internal = StoreInternal {
            objects: RwLock::new(objects),
            string_store,
            blake3_hash: RwLock::new(static_store.get_blake3_hash()),
        };

        Store {
            internal: Arc::new(internal),
        }
    }

    /// Synchronizes this store's objects from a [`StaticStore`].
    ///
    /// When `delete_missing` is `true`, objects not present in the static store are removed.
    /// Objects whose definitions match are updated in-place; others are replaced entirely.
    fn update_from_static_internal(
        &self,
        static_store: &StaticStore,
        delete_missing: bool,
    ) -> Result<(), StoreError> {
        let mut objects = self.internal.objects.write();

        if delete_missing {
            for key in objects.keys().cloned().collect::<Vec<_>>() {
                if !static_store.objects().contains_key(&key) {
                    objects.remove(&key);
                }
            }
        }

        for (key, static_object) in static_store.objects() {
            let laundered_key = self.launder_key(key.clone());
            if let Some(object) = objects.get_mut(&laundered_key)
                && object.definition() == static_object.definition()
            {
                object.update_from_static(static_object.items())?;
                continue;
            }

            // If doesn't exist or definition mismatch, replace/add
            let object = Object::from(static_object);
            let object = object.launder(&self.internal.string_store);
            objects.insert(laundered_key, object);
        }

        self.internal.update_blake3_hash(&objects);
        Ok(())
    }

    /// Synchronizes this store from a [`StaticStore`], removing objects not present in it.
    pub fn sync_from_static(&self, static_store: &StaticStore) -> Result<(), StoreError> {
        self.update_from_static_internal(static_store, true)
    }

    /// Merges objects from a [`StaticStore`] into this store, keeping existing objects that are not in the source.
    pub fn merge_from_static(&self, static_store: &StaticStore) -> Result<(), StoreError> {
        self.update_from_static_internal(static_store, false)
    }
}

/// Splits a path into its parent path and the last segment key.
///
/// # Errors
///
/// Returns [`StoreError::InvalidPath`] if the path has no segments (i.e. it points only to an object).
fn split_path(path: &StorePath) -> Result<(StorePath, StoreKey), StoreError> {
    let mut segments = path.segments().clone();
    let last_segment = segments.pop().ok_or(StoreError::InvalidPath)?;
    let mut parent_path = StorePath::new(path.object_key().clone());
    for segment in segments {
        parent_path = parent_path.clone().with_segment(segment);
    }
    Ok((parent_path, last_segment))
}

impl TreePrint for Store {
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

        let objects = self.internal.objects.read();
        let mut keys: Vec<_> = objects.keys().collect();
        keys.sort();

        for (i, key) in keys.iter().enumerate() {
            let is_last = i == keys.len() - 1;
            if let Some(container) = objects.get(*key) {
                container.tree_print(f, key.as_str(), &next_prefix, is_last)?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_display("Store").fmt(f)
    }
}
