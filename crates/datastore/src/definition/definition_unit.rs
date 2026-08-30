use crate::traits::TreePrint;
use keys::UnitKey;
use shareable_string::{ShareableString, SharedStringStore};
use units::{UnitFamilyId, UnitId};

/// Definition for a choice-based parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct UnitDefinition {
    /// Human-readable description of this choice parameter.
    description: ShareableString,
    /// Unit family
    unit_family: UnitFamilyId,
    /// Default value for this choice parameter.
    default_value: ShareableString,
}

impl UnitDefinition {
    /// Creates a new `UnitDefinition` with the specified choices.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new<S: Into<ShareableString>>(description: S, unit_family: UnitFamilyId) -> Self {
        Self {
            description: description.into(),
            unit_family,
            default_value: ShareableString::default(),
        }
    }

    /// Creates a new `UnitDefinition` with the specified choices and default value.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        unit_family: UnitFamilyId,
        default_value: S2,
    ) -> Self {
        Self {
            description: description.into(),
            unit_family,
            default_value: default_value.into(),
        }
    }

    /// Returns a reference to the unit family.
    #[must_use]
    pub const fn unit_family(&self) -> UnitFamilyId {
        self.unit_family
    }

    /// Returns true if the given value is a valid units.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn contains<S: Into<ShareableString>>(&self, value: S) -> bool {
        let value = value.into();
        self.unit_family
            .unit_ids()
            .iter()
            .any(|unit_id| unit_id.string_id().as_str() == value)
    }

    /// Returns a vector of keys `UnitKey` for the units.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn keys(&self) -> Vec<UnitKey> {
        self.unit_family
            .unit_ids()
            .iter()
            .map(|unit_id| unit_id.string_id().into())
            .collect()
    }

    /// Returns a vector of Unit IDs for the units.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn ids(&self) -> Vec<UnitId> {
        self.unit_family.unit_ids().to_vec()
    }

    /// Returns a vector of descriptions for the units.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn descriptions(&self) -> Vec<ShareableString> {
        self.unit_family
            .unit_ids()
            .iter()
            .map(|unit_id| unit_id.description().into())
            .collect()
    }

    /// Returns a new `UnitDefinition` with strings laundered through the provided store.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            unit_family: self.unit_family,
            default_value: store.launder(&self.default_value),
        }
    }

    /// Returns the description of the parameter.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the description.
    #[must_use]
    pub const fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns the default value of the parameter.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn default_value(&self) -> ShareableString {
        self.default_value.clone()
    }

    /// Returns a reference to the default value.
    #[must_use]
    pub const fn default_value_ref(&self) -> &ShareableString {
        &self.default_value
    }
}

impl PartialEq<&UnitDefinition> for UnitDefinition {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&UnitDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<UnitDefinition> for &UnitDefinition {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &UnitDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for UnitDefinition {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Unit - default: \"{}\" [{}]",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.default_value,
            self.ids()
                .iter()
                .map(|unit_id| format!("{} ({})", unit_id.string_id(), unit_id.description()))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
