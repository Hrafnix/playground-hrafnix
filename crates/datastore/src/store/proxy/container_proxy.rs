use crate::StoreError;
use crate::key::StoreKey;
use crate::path::StorePath;
use crate::store::traits::TreePrint;
use crate::store::{Container, ContainerDefinition, ObjectProxy, Store, StoreHashContainer};
use shareable_string::ShareableString;

/// A proxy for a container in the store.
#[derive(Debug)]
pub struct ContainerProxy {
    path: StorePath,
    store: Store,
    definition: ContainerDefinition,
    keys: Vec<StoreKey>,
    object_hash: StoreHashContainer,
    last_sync_hash: [u8; 32],
}

impl ContainerProxy {
    /// Creates a new `ContainerProxy`.
    pub(crate) fn new(
        path: StorePath,
        store: Store,
        definition: ContainerDefinition,
        keys: Vec<StoreKey>,
        object_hash: StoreHashContainer,
        last_sync_hash: [u8; 32],
    ) -> Self {
        Self {
            path,
            store,
            definition,
            keys,
            object_hash,
            last_sync_hash,
        }
    }

    /// Inserts a new entry into a map container and returns a proxy to it.
    pub fn insert_map_entry<S: Into<StoreKey>>(
        &self,
        key: S,
    ) -> Result<ContainerProxy, StoreError> {
        let key = key.into();
        match &self.definition {
            ContainerDefinition::Map(map_def) => {
                let entry_container = Container::new_struct(map_def.item_type().clone());
                let entry_path = self.path.clone().with_segment(key);
                self.store
                    .update_container_at_path(&entry_path, entry_container)?;
                self.store.container(&entry_path)
            }
            _ => Err(StoreError::PropertyNotFound),
        }
    }

    /// Returns the path to the data this proxy represents.
    pub fn path(&self) -> &StorePath {
        &self.path
    }

    /// Returns a description of the data.
    pub fn description(&self) -> ShareableString {
        match &self.definition {
            ContainerDefinition::Struct(_struct) => _struct.description(),
            ContainerDefinition::Map(map) => map.description(),
        }
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
            let proxy = match self.store.container(&self.path) {
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

        let container = self.store.container(&self.path)?;

        self.keys = container.keys;
        self.last_sync_hash = container.last_sync_hash;

        Ok(())
    }

    /// Returns an `ObjectProxy` for the object containing this data.
    pub fn object(&self) -> Result<ObjectProxy, StoreError> {
        let key = self.path.object_key();
        self.store.object(key)
    }
}

impl TreePrint for ContainerProxy {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        if let Ok(container) = self.store.get_container_internal(&self.path) {
            container.tree_print(f, label, prefix, last)
        } else {
            writeln!(
                f,
                "{}{}{}: Error - Container not found",
                prefix,
                Self::branch_char(prefix, last),
                label
            )
        }
    }
}

impl std::fmt::Display for ContainerProxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = self.path.get_last_key();
        self.tree_display(label.as_ref()).fmt(f)
    }
}
