use crate::StoreError;
use crate::definition::ObjectDefinition;
use crate::key::StoreKey;
use crate::path::StorePath;
use crate::store::traits::TreePrint;
use crate::store::{BasicProxy, ContainerProxy, Store, StoreHashContainer, TableProxy};
use shareable_string::ShareableString;

/// A proxy for a top-level object in the store.
#[derive(Debug)]
pub struct ObjectProxy {
    path: StorePath,
    store: Store,
    definition: ObjectDefinition,
    keys: Vec<StoreKey>,
    object_hash: StoreHashContainer,
    last_sync_hash: [u8; 32],
}

impl ObjectProxy {
    /// Creates a new `ObjectProxy`.
    pub(crate) fn new(
        path: StorePath,
        store: Store,
        definition: ObjectDefinition,
        keys: Vec<StoreKey>,
        object_hash: StoreHashContainer,
        last_sync_hash: [u8; 32],
    ) -> Self {
        ObjectProxy {
            path,
            store,
            definition,
            keys,
            object_hash,
            last_sync_hash,
        }
    }

    /// Returns the keys of the properties in the object.
    pub fn keys(&self) -> &Vec<StoreKey> {
        &self.keys
    }

    /// Checks if a property with the given key exists in the object.
    pub fn check_key<S: Into<ShareableString>>(&self, key: S) -> Result<bool, StoreError> {
        let key = key.into();
        Ok(self.keys.iter().any(|k| k == &key))
    }

    /// Syncs the proxy with the latest data from the store.
    pub fn sync(&mut self) -> Result<(), StoreError> {
        self.pull()
    }

    /// Returns a `BasicProxy` for the property with the given key.
    pub fn basic<S: Into<ShareableString>>(&mut self, key: S) -> Result<BasicProxy, StoreError> {
        if !self.is_valid() {
            return Err(StoreError::ExpiredProxy);
        }

        let key = key.into();
        self.check_key(key.clone())?;

        #[expect(unsafe_code)]
        let store_key = unsafe { StoreKey::new_unsafe(key) };
        let path = self.path.clone().with_segment(store_key);
        self.store.basic(&path)
    }

    /// Returns a `TableProxy` for the property with the given key.
    pub fn table<S: Into<ShareableString>>(&mut self, key: S) -> Result<TableProxy, StoreError> {
        if !self.is_valid() {
            return Err(StoreError::ExpiredProxy);
        }

        let key = key.into();
        self.check_key(key.clone())?;

        #[expect(unsafe_code)]
        let store_key = unsafe { StoreKey::new_unsafe(key) };
        let path = self.path.clone().with_segment(store_key);
        self.store.table(&path)
    }

    /// Returns a `ContainerProxy` for the property with the given key.
    pub fn container<S: Into<ShareableString>>(
        &mut self,
        key: S,
    ) -> Result<ContainerProxy, StoreError> {
        if !self.is_valid() {
            return Err(StoreError::ExpiredProxy);
        }

        let key = key.into();
        self.check_key(key.clone())?;

        #[expect(unsafe_code)]
        let store_key = unsafe { StoreKey::new_unsafe(key) };
        let path = self.path.clone().with_segment(store_key);
        self.store.container(&path)
    }

    /// Returns all property keys in the object.
    pub fn all_property_keys(&self) -> Result<Vec<StoreKey>, StoreError> {
        Ok(self.keys.clone())
    }

    /// Returns the path to the data this proxy represents.
    pub fn path(&self) -> &StorePath {
        &self.path
    }

    /// Returns a description of the data.
    pub fn description(&self) -> ShareableString {
        self.definition.description()
    }

    /// Checks if the proxy is still valid.
    pub fn is_valid(&self) -> bool {
        self.object_hash.get() != [0u8; 32]
    }

    /// Returns true if the data has changed compared to the store.
    pub fn has_changed(&self) -> bool {
        self.last_sync_hash != self.object_hash.get()
    }

    /// Pulls the latest data from the store.
    pub fn pull(&mut self) -> Result<(), StoreError> {
        if !self.is_valid() {
            let proxy = match self.store.object(self.path.object_key()) {
                Ok(p) => p,
                Err(_) => return Err(StoreError::ExpiredProxy),
            };
            return if proxy.definition == self.definition {
                self.keys = proxy.keys;
                self.object_hash = proxy.object_hash;
                self.last_sync_hash = proxy.last_sync_hash;
                Ok(())
            } else {
                Err(StoreError::ExpiredProxy)
            };
        }

        if !self.has_changed() {
            return Ok(());
        }

        let key = self.path.object_key();
        let proxy = self.store.object(key)?;
        self.keys = proxy.keys;
        self.last_sync_hash = proxy.last_sync_hash;

        Ok(())
    }

    /// Returns an `ObjectProxy` for the object containing this data.
    pub fn object(&self) -> Result<ObjectProxy, StoreError> {
        let key = self.path.object_key();
        self.store.object(key)
    }
}

impl TreePrint for ObjectProxy {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        if let Ok(object) = self.store.get_object_internal(self.path.object_key()) {
            object.tree_print(f, label, prefix, last)
        } else {
            writeln!(
                f,
                "{}{}{}: Error - Object not found",
                prefix,
                Self::branch_char(prefix, last),
                label
            )
        }
    }
}

impl std::fmt::Display for ObjectProxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_display(self.path.object_key().as_str()).fmt(f)
    }
}
