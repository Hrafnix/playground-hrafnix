use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};

/// Definition for a number-based parameter constraint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IntegerConstraint {
    /// Minimum value constraint.
    Min {
        /// Minimum value of the constraint.
        min: i64,
        /// Whether the minimum value is inclusive.
        inclusive: bool,
    },
    /// Maximum value constraint.
    Max {
        /// Maximum value of the constraint.
        max: i64,
        /// Whether the maximum value is inclusive.
        inclusive: bool,
    },
    /// Range value constraint.
    Range {
        /// Minimum value of the range.
        min: i64,
        /// Maximum value of the range.
        max: i64,
        /// Whether the minimum value is inclusive.
        min_inclusive: bool,
        /// Whether the maximum value is inclusive.
        max_inclusive: bool,
    },
    /// No constraint.
    None,
}

/// Definition for an integer-based parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegerDefinition {
    description: ShareableString,
    constraint: IntegerConstraint,
    default_value: ShareableString,
}

impl IntegerDefinition {
    /// Creates a new integer-based `IntegerDefinition`.
    pub fn new<S1: Into<ShareableString>>(description: S1) -> Self {
        Self {
            description: description.into(),
            constraint: IntegerConstraint::None,
            default_value: Default::default(),
        }
    }

    /// Creates a new integer-based `IntegerDefinition` with a default value.
    pub fn new_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        default_value: S2,
    ) -> Self {
        Self {
            description: description.into(),
            constraint: IntegerConstraint::None,
            default_value: default_value.into(),
        }
    }

    /// Creates a new integer-based `IntegerDefinition`.
    pub fn new_with_constraint<S: Into<ShareableString>>(
        description: S,
        constraint: IntegerConstraint,
    ) -> Self {
        Self {
            description: description.into(),
            constraint,
            default_value: Default::default(),
        }
    }

    /// Creates a new integer-based `IntegerDefinition` with a default value.
    pub fn new_with_constraint_and_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        constraint: IntegerConstraint,
        default_value: S2,
    ) -> Self {
        Self {
            description: description.into(),
            constraint,
            default_value: default_value.into(),
        }
    }

    /// Returns the constraint.
    pub fn constraint(&self) -> IntegerConstraint {
        self.constraint.clone()
    }

    /// Returns a reference to the constraint.
    pub fn constraint_ref(&self) -> &IntegerConstraint {
        &self.constraint
    }

    /// Returns a new `IntegerDefinition` with strings laundered through the provided store.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            constraint: self.constraint.clone(),
            default_value: store.launder(&self.default_value),
        }
    }

    /// Returns the description of the parameter.
    pub fn description(&self) -> ShareableString {
        self.description.clone()
    }

    /// Returns a reference to the description.
    pub fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns the default value of the parameter.
    pub fn default_value(&self) -> ShareableString {
        self.default_value.clone()
    }

    /// Returns a reference to the default value.
    pub fn default_value_ref(&self) -> &ShareableString {
        &self.default_value
    }
}

impl PartialEq<&IntegerDefinition> for IntegerDefinition {
    fn eq(&self, other: &&IntegerDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<IntegerDefinition> for &IntegerDefinition {
    fn eq(&self, other: &IntegerDefinition) -> bool {
        *self == other
    }
}

impl TreePrint for IntegerDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        let constraint_str = match &self.constraint {
            IntegerConstraint::Min { min, inclusive } => {
                format!(
                    " [Min({}, {})]",
                    *min,
                    if *inclusive { "inclusive" } else { "exclusive" }
                )
            }
            IntegerConstraint::Max { max, inclusive } => {
                format!(
                    " [Max({}, {})]",
                    *max,
                    if *inclusive { "inclusive" } else { "exclusive" }
                )
            }
            IntegerConstraint::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } => {
                let min_type = if *min_inclusive {
                    "inclusive"
                } else {
                    "exclusive"
                };
                let max_type = if *max_inclusive {
                    "inclusive"
                } else {
                    "exclusive"
                };
                format!(" [Range({}, {}, {}, {})]", *min, *max, min_type, max_type)
            }
            IntegerConstraint::None => "".to_string(),
        };

        writeln!(
            f,
            "{}{}{} ({}) Integer - default: \"{}\"{}",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.default_value,
            constraint_str
        )
    }
}
