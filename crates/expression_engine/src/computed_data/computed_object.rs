use crate::computed_data::ComputedItem;
use shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Represents computed Global data for an object, mapping field names
/// to their corresponding computed data items.
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalObjectComputedData {
    /// The map from a field name to its evaluated [`ComputedItem`].
    data: BTreeMap<ShareableString, ComputedItem>,
}

impl GlobalObjectComputedData {
    /// Creates a new `GlobalObjectComputedData` wrapping the given `data` map.
    pub(crate) const fn new(data: BTreeMap<ShareableString, ComputedItem>) -> Self {
        Self { data }
    }

    /// Returns a reference to the data of the global object computed data.
    #[must_use]
    pub const fn data(&self) -> &BTreeMap<ShareableString, ComputedItem> {
        &self.data
    }

    /// Returns a reference to the computed item associated with the given key, if it exists.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ComputedItem> {
        self.data.get(&key.into())
    }

    /// Merges entries from `other` into `self`, skipping any keys that already exist.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub(crate) fn extend(&mut self, other: GlobalObjectComputedData) {
        for (key, item) in other.data {
            if self.data.contains_key(&key) {
                continue;
            }

            self.data.insert(key, item);
        }
    }

    /// Returns an iterator over the key-value pairs in the global object computed data.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn iter(&self) -> impl Iterator<Item = (&ShareableString, &ComputedItem)> {
        self.data.iter()
    }
}

/// Represents computed Parameter data for an object, mapping field names
/// to their corresponding computed data items.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterObjectComputedData {
    /// The map from a field name to its evaluated [`ComputedItem`].
    data: BTreeMap<ShareableString, ComputedItem>,
}

impl ParameterObjectComputedData {
    /// Creates a new `ParameterObjectComputedData` wrapping the given `data` map.
    pub(crate) const fn new(data: BTreeMap<ShareableString, ComputedItem>) -> Self {
        Self { data }
    }

    /// Returns a reference to the data of the parameter object computed data.
    #[must_use]
    pub const fn data(&self) -> &BTreeMap<ShareableString, ComputedItem> {
        &self.data
    }

    /// Returns a reference to the computed item associated with the given key, if it exists.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ComputedItem> {
        self.data.get(&key.into())
    }

    /// Returns an iterator over the key-value pairs in the parameter object computed data.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn iter(&self) -> impl Iterator<Item = (&ShareableString, &ComputedItem)> {
        self.data.iter()
    }
}

/// Represents computed Variable data for an object, mapping field names
/// to their corresponding computed data items.
#[derive(Debug, Clone, PartialEq)]
pub struct VariableObjectComputedData {
    /// The map from a field name to its evaluated [`ComputedItem`].
    data: BTreeMap<ShareableString, ComputedItem>,
}

impl VariableObjectComputedData {
    /// Creates a new `VariableObjectComputedData` wrapping the given `data` map.
    pub(crate) const fn new(data: BTreeMap<ShareableString, ComputedItem>) -> Self {
        Self { data }
    }

    /// Returns a reference to the data of the variable object computed data.
    #[must_use]
    pub const fn data(&self) -> &BTreeMap<ShareableString, ComputedItem> {
        &self.data
    }

    /// Returns a reference to the computed item associated with the given key, if it exists.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn get<S: Into<ShareableString>>(&self, key: S) -> Option<&ComputedItem> {
        self.data.get(&key.into())
    }

    /// Returns an iterator over the key-value pairs in the variable object computed data.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn iter(&self) -> impl Iterator<Item = (&ShareableString, &ComputedItem)> {
        self.data.iter()
    }
}
