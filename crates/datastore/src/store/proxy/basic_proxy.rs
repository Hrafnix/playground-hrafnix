use crate::StoreError;
use crate::definition::basic_definition::BasicDefinition;
use crate::path::StorePath;
use crate::store::traits::TreePrint;
use crate::store::{Basic, CommonStoreTraitInternal, ObjectProxy, Store};
use shareable_string::ShareableString;

/// A proxy for a basic data value in the store.
#[derive(Debug)]
pub struct BasicProxy {
    path: StorePath,
    store: Store,
    data: Basic,
}

impl BasicProxy {
    /// Creates a new `BasicProxy`.
    pub(crate) fn new(path: StorePath, store: Store, data: Basic) -> Self {
        Self { path, store, data }
    }

    /// Returns a reference to the basic definition.
    pub fn definition(&self) -> &BasicDefinition {
        self.data.definition()
    }

    /// Returns the current value from the proxy.
    pub fn value(&self) -> ShareableString {
        self.data.get()
    }

    /// Sets a new value in the proxy.
    pub fn set_value<S: Into<ShareableString>>(&mut self, value: S) {
        self.data.set(value.into());
    }

    /// Returns the path to the data this proxy represents.
    pub fn path(&self) -> &StorePath {
        &self.path
    }

    /// Returns a description of the data.
    pub fn description(&self) -> ShareableString {
        self.definition().description()
    }

    /// Checks if the proxy is still valid.
    pub fn is_valid(&self) -> bool {
        self.data.is_valid()
    }

    /// Returns true if the data has changed compared to the store.
    pub fn has_changed(&self) -> bool {
        self.data.has_changed()
    }

    /// Pulls the latest data from the store.
    pub fn pull(&mut self) -> Result<(), StoreError> {
        if !self.is_valid() {
            let proxy = match self.store.basic(&self.path) {
                Ok(p) => p,
                Err(_) => {
                    return Err(StoreError::ExpiredProxy);
                }
            };
            return if proxy.definition() == self.definition() {
                self.data = proxy.data;
                Ok(())
            } else {
                Err(StoreError::ExpiredProxy)
            };
        }

        if !self.has_changed() {
            return Ok(());
        }

        let proxy = self.store.basic(&self.path)?;

        self.data = proxy.data;
        Ok(())
    }

    /// Pushes the local changes to the store.
    pub fn push(&mut self) -> Result<(), StoreError> {
        if !self.is_valid() {
            let proxy = match self.store.basic(&self.path) {
                Ok(p) => p,
                Err(_) => return Err(StoreError::ExpiredProxy),
            };
            if proxy.definition() == self.definition() {
                self.data = proxy.data;
            } else {
                return Err(StoreError::ExpiredProxy);
            }
        }

        self.store.set_basic(&self.path, &self.data)?;
        Ok(())
    }

    /// Returns an `ObjectProxy` for the object containing this data.
    pub fn object(&self) -> Result<ObjectProxy, StoreError> {
        let key = self.path.object_key();
        self.store.object(key)
    }
}

impl TreePrint for BasicProxy {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        self.data.tree_print(f, label, prefix, last)
    }
}

impl std::fmt::Display for BasicProxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = self.path.get_last_key();
        self.tree_display(label.as_ref()).fmt(f)
    }
}
