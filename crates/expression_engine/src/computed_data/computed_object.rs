use crate::computed_data::ComputedItem;
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents computed Global data for an object, mapping field names
/// to their corresponding computed data items.
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalObjectComputedData {
    data: BTreeMap<ShareableString, ComputedItem>,
}

impl GlobalObjectComputedData {
    pub(crate) const fn new(data: BTreeMap<ShareableString, ComputedItem>) -> Self {
        Self { data }
    }

    /// Returns a reference to the data of the global object computed data.
    #[must_use]
    pub const fn data(&self) -> &BTreeMap<ShareableString, ComputedItem> {
        &self.data
    }

    /// Returns a reference to the computed item associated with the given key, if it exists.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ComputedItem> {
        self.data.get(&key.into())
    }

    pub(crate) fn extend(&mut self, other: GlobalObjectComputedData) {
        for (key, item) in other.data {
            if self.data.contains_key(&key) {
                continue;
            }

            self.data.insert(key, item);
        }
    }

    /// Returns an iterator over the key-value pairs in the global object computed data.
    pub fn iter(&self) -> impl Iterator<Item = (&ShareableString, &ComputedItem)> {
        self.data.iter()
    }
}

/// Represents computed Parameter data for an object, mapping field names
/// to their corresponding computed data items.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterObjectComputedData {
    data: BTreeMap<ShareableString, ComputedItem>,
}

impl ParameterObjectComputedData {
    pub(crate) const fn new(data: BTreeMap<ShareableString, ComputedItem>) -> Self {
        Self { data }
    }

    /// Returns a reference to the data of the parameter object computed data.
    #[must_use]
    pub const fn data(&self) -> &BTreeMap<ShareableString, ComputedItem> {
        &self.data
    }

    /// Returns a reference to the computed item associated with the given key, if it exists.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ComputedItem> {
        self.data.get(&key.into())
    }

    /// Returns an iterator over the key-value pairs in the parameter object computed data.
    pub fn iter(&self) -> impl Iterator<Item = (&ShareableString, &ComputedItem)> {
        self.data.iter()
    }
}

/// Represents computed Variable data for an object, mapping field names
/// to their corresponding computed data items.
#[derive(Debug, Clone, PartialEq)]
pub struct VariableObjectComputedData {
    data: BTreeMap<ShareableString, ComputedItem>,
}

impl VariableObjectComputedData {
    pub(crate) const fn new(data: BTreeMap<ShareableString, ComputedItem>) -> Self {
        Self { data }
    }

    /// Returns a reference to the data of the variable object computed data.
    #[must_use]
    pub const fn data(&self) -> &BTreeMap<ShareableString, ComputedItem> {
        &self.data
    }

    /// Returns a reference to the computed item associated with the given key, if it exists.
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ComputedItem> {
        self.data.get(&key.into())
    }

    /// Returns an iterator over the key-value pairs in the variable object computed data.
    pub fn iter(&self) -> impl Iterator<Item = (&ShareableString, &ComputedItem)> {
        self.data.iter()
    }
}
