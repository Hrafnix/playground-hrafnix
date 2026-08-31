use crate::frozen::{GlobalObjectFrozen, ParameterObjectFrozen, VariableObjectFrozen};

/// A store holding a single parameter object, variable object, and global object in frozen form.
///
/// Use [`FrozenStore::merge`] to combine two stores: items already present in `self`
/// (including `Map` items) are kept unchanged, while items present in `other` but
/// absent from `self` are added at the top level of each object.
#[derive(Debug, Clone, PartialEq)]
pub struct FrozenStore {
    /// The frozen parameter object.
    parameter: ParameterObjectFrozen,
    /// The frozen variable object.
    variable: VariableObjectFrozen,
    /// The frozen global object.
    global: GlobalObjectFrozen,
}

impl FrozenStore {
    /// Creates a new `FrozenStore` from the given frozen objects.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new(
        parameter: ParameterObjectFrozen,
        variable: VariableObjectFrozen,
        global: GlobalObjectFrozen,
    ) -> Self {
        Self {
            parameter,
            variable,
            global,
        }
    }

    /// Returns a reference to the frozen parameter object.
    #[must_use]
    pub const fn parameter(&self) -> &ParameterObjectFrozen {
        &self.parameter
    }

    /// Returns a reference to the frozen variable object.
    #[must_use]
    pub const fn variable(&self) -> &VariableObjectFrozen {
        &self.variable
    }

    /// Returns a reference to the frozen global object.
    #[must_use]
    pub const fn global(&self) -> &GlobalObjectFrozen {
        &self.global
    }

    /// Creates a new `FrozenStore` by merging `self` with `other` at the top level.
    ///
    /// For each of the three object types (parameter, variable, global):
    /// - Items already present in `self` (including `Map` items) are kept unchanged.
    /// - Items present in `other` but absent from `self` are added.
    ///
    /// Maps are never deep-merged; they are treated as atomic items.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn merge(&self, other: &FrozenStore) -> FrozenStore {
        FrozenStore {
            parameter: self.parameter.merge_from(&other.parameter),
            variable: self.variable.merge_from(&other.variable),
            global: self.global.merge_from(&other.global),
        }
    }
}

impl PartialEq<&FrozenStore> for FrozenStore {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&FrozenStore) -> bool {
        self.parameter == other.parameter
            && self.variable == other.variable
            && self.global == other.global
    }
}

impl PartialEq<FrozenStore> for &FrozenStore {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &FrozenStore) -> bool {
        self.parameter == other.parameter
            && self.variable == other.variable
            && self.global == other.global
    }
}
