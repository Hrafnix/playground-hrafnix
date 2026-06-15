use crate::StoreError;
use crate::definition::ObjectDefinition;
use crate::key::{ParameterKey, StoreKey, VariableKey};
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
    parameter_keys: Vec<ParameterKey>,
    variable_keys: Vec<VariableKey>,
    object_hash: StoreHashContainer,
    last_sync_hash: [u8; 32],
}

impl ObjectProxy {
    /// Creates a new `ObjectProxy`.
    pub(crate) fn new(
        path: StorePath,
        store: Store,
        definition: ObjectDefinition,
        parameter_keys: Vec<ParameterKey>,
        variable_keys: Vec<VariableKey>,
        object_hash: StoreHashContainer,
        last_sync_hash: [u8; 32],
    ) -> Self {
        ObjectProxy {
            path,
            store,
            definition,
            parameter_keys,
            variable_keys,
            object_hash,
            last_sync_hash,
        }
    }

    /// Returns the keys of the parameter in the object.
    pub fn parameter_keys(&self) -> &Vec<ParameterKey> {
        &self.parameter_keys
    }

    /// Checks if a parameter with the given key exists in the object.
    pub fn check_parameter_key<S: Into<ShareableString>>(
        &self,
        key: S,
    ) -> Result<bool, StoreError> {
        let key = key.into();
        Ok(self.parameter_keys.iter().any(|k| k == &key))
    }

    /// Returns the keys of the variables in the object.
    pub fn variable_keys(&self) -> &Vec<VariableKey> {
        &self.variable_keys
    }

    /// Checks if a variable with the given key exists in the object.
    pub fn check_variable_key<S: Into<ShareableString>>(&self, key: S) -> Result<bool, StoreError> {
        let key = key.into();
        Ok(self.variable_keys.iter().any(|k| k == &key))
    }

    /// Syncs the proxy with the latest data from the store.
    pub fn sync(&mut self) -> Result<(), StoreError> {
        self.pull()
    }

    /// Returns a `BasicProxy` for the parameter with the given key.
    pub fn parameter_basic<S: Into<ShareableString>>(
        &mut self,
        key: S,
    ) -> Result<BasicProxy, StoreError> {
        if !self.is_valid() {
            return Err(StoreError::ExpiredProxy);
        }

        let key = key.into();
        if !self.check_parameter_key(key.clone())? {
            return Err(StoreError::ParameterNotFound);
        }

        #[expect(unsafe_code)]
        let store_key = unsafe { StoreKey::new_unsafe(key) };
        let path = self.path.clone().with_segment(store_key);
        self.store.basic(&path)
    }

    /// Returns a `TableProxy` for the parameter with the given key.
    pub fn parameter_table<S: Into<ShareableString>>(
        &mut self,
        key: S,
    ) -> Result<TableProxy, StoreError> {
        if !self.is_valid() {
            return Err(StoreError::ExpiredProxy);
        }

        let key = key.into();
        if !self.check_parameter_key(key.clone())? {
            return Err(StoreError::ParameterNotFound);
        }

        #[expect(unsafe_code)]
        let store_key = unsafe { StoreKey::new_unsafe(key) };
        let path = self.path.clone().with_segment(store_key);
        self.store.table(&path)
    }

    /// Returns a `ContainerProxy` for the parameter with the given key.
    pub fn parameter_container<S: Into<ShareableString>>(
        &mut self,
        key: S,
    ) -> Result<ContainerProxy, StoreError> {
        if !self.is_valid() {
            return Err(StoreError::ExpiredProxy);
        }

        let key = key.into();
        if !self.check_parameter_key(key.clone())? {
            return Err(StoreError::ParameterNotFound);
        }

        #[expect(unsafe_code)]
        let store_key = unsafe { StoreKey::new_unsafe(key) };
        let path = self.path.clone().with_segment(store_key);
        self.store.container(&path)
    }

    /// Returns a `BasicProxy` for the variable with the given key.
    pub fn variable_basic<S: Into<ShareableString>>(
        &mut self,
        key: S,
    ) -> Result<BasicProxy, StoreError> {
        if !self.is_valid() {
            return Err(StoreError::ExpiredProxy);
        }

        let key = key.into();
        if !self.check_variable_key(key.clone())? {
            return Err(StoreError::VariableNotFound);
        }

        #[expect(unsafe_code)]
        let store_key = unsafe { StoreKey::new_unsafe(key) };
        let path = self.path.clone().with_segment(store_key);
        self.store.basic(&path)
    }

    /// Returns a `TableProxy` for the variable with the given key.
    pub fn variable_table<S: Into<ShareableString>>(
        &mut self,
        key: S,
    ) -> Result<TableProxy, StoreError> {
        if !self.is_valid() {
            return Err(StoreError::ExpiredProxy);
        }

        let key = key.into();
        if !self.check_variable_key(key.clone())? {
            return Err(StoreError::VariableNotFound);
        }

        #[expect(unsafe_code)]
        let store_key = unsafe { StoreKey::new_unsafe(key) };
        let path = self.path.clone().with_segment(store_key);
        self.store.table(&path)
    }

    /// Returns a `ContainerProxy` for the variable with the given key.
    pub fn variable_container<S: Into<ShareableString>>(
        &mut self,
        key: S,
    ) -> Result<ContainerProxy, StoreError> {
        if !self.is_valid() {
            return Err(StoreError::ExpiredProxy);
        }

        let key = key.into();
        if !self.check_variable_key(key.clone())? {
            return Err(StoreError::VariableNotFound);
        }

        #[expect(unsafe_code)]
        let store_key = unsafe { StoreKey::new_unsafe(key) };
        let path = self.path.clone().with_segment(store_key);
        self.store.container(&path)
    }

    /// Returns all parameter keys in the object.
    pub fn all_parameter_keys(&self) -> Result<Vec<ParameterKey>, StoreError> {
        Ok(self.parameter_keys.clone())
    }

    /// Returns all variable keys in the object.
    pub fn all_variable_keys(&self) -> Result<Vec<VariableKey>, StoreError> {
        Ok(self.variable_keys.clone())
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
                self.parameter_keys = proxy.parameter_keys;
                self.variable_keys = proxy.variable_keys;
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
        self.parameter_keys = proxy.parameter_keys;
        self.variable_keys = proxy.variable_keys;
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
