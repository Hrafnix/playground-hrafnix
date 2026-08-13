use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};

/// Definition for a number-based parameter constraint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IntegerConstraintEnum {
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

/// Definition for an integer-based parameter constraint.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct IntegerConstraint {
    /// The actual constraint variant (none, min, max, or range).
    pub(crate) constraint_enum: IntegerConstraintEnum,
}

impl IntegerConstraint {
    /// Creates a new `IntegerConstraint` with no constraint.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            constraint_enum: IntegerConstraintEnum::None,
        }
    }

    /// Creates a new `IntegerConstraint` with a minimum value constraint.
    #[must_use]
    pub const fn min(min: i64, inclusive: bool) -> Self {
        Self {
            constraint_enum: IntegerConstraintEnum::Min { min, inclusive },
        }
    }

    /// Creates a new `IntegerConstraint` with a maximum value constraint.
    #[must_use]
    pub const fn max(max: i64, inclusive: bool) -> Self {
        Self {
            constraint_enum: IntegerConstraintEnum::Max { max, inclusive },
        }
    }

    /// Creates a new `IntegerConstraint` with a range value constraint.
    ///
    /// If `value_1` is greater than `value_2`, the two values are swapped along with
    /// their corresponding inclusivity flags, so the resulting range is always valid.
    ///
    /// If `value_1` and `value_2` are equal, the resulting range is always inclusive on
    /// both ends (regardless of the passed inclusivity flags), so it always represents
    /// exactly that single value rather than a contradictory, unsatisfiable range.
    #[must_use]
    pub fn range(
        value_1: i64,
        value_2: i64,
        value_1_inclusive: bool,
        value_2_inclusive: bool,
    ) -> Self {
        let (min, max, min_inclusive, max_inclusive) = match value_1.cmp(&value_2) {
            std::cmp::Ordering::Equal => (value_1, value_2, true, true),
            std::cmp::Ordering::Greater => (value_2, value_1, value_2_inclusive, value_1_inclusive),
            std::cmp::Ordering::Less => (value_1, value_2, value_1_inclusive, value_2_inclusive),
        };

        Self {
            constraint_enum: IntegerConstraintEnum::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            },
        }
    }
}

impl<'de> Deserialize<'de> for IntegerConstraint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Mirrors the shape produced by the derived `Serialize` impl above, so the
        // on-the-wire format is unchanged; only construction is routed through the
        // same normalization logic as `IntegerConstraint::range` so a deserialized
        // `Range` can never end up with `min > max`.
        #[derive(Deserialize)]
        struct Raw {
            constraint_enum: IntegerConstraintEnum,
        }

        let raw = Raw::deserialize(deserializer)?;
        let constraint_enum = match raw.constraint_enum {
            IntegerConstraintEnum::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } => {
                return Ok(IntegerConstraint::range(
                    min,
                    max,
                    min_inclusive,
                    max_inclusive,
                ));
            }
            other @ (IntegerConstraintEnum::Min { .. }
            | IntegerConstraintEnum::Max { .. }
            | IntegerConstraintEnum::None) => other,
        };

        Ok(Self { constraint_enum })
    }
}

/// Definition for an integer-based parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegerDefinition {
    /// Human-readable description of this integer parameter.
    description: ShareableString,
    /// Optional constraint (min, max, range, or none) applied to the value.
    constraint: IntegerConstraint,
    /// Default value for this integer parameter.
    default_value: ShareableString,
}

impl IntegerDefinition {
    /// Creates a new integer-based `IntegerDefinition`.
    pub fn new<S1: Into<ShareableString>>(description: S1) -> Self {
        Self {
            description: description.into(),
            constraint: IntegerConstraint::none(),
            default_value: ShareableString::default(),
        }
    }

    /// Creates a new integer-based `IntegerDefinition` with a default value.
    pub fn new_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        default_value: S2,
    ) -> Self {
        Self {
            description: description.into(),
            constraint: IntegerConstraint::none(),
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
            default_value: ShareableString::default(),
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
    #[must_use]
    pub fn constraint(&self) -> IntegerConstraintEnum {
        self.constraint.constraint_enum.clone()
    }

    /// Returns a reference to the constraint.
    #[must_use]
    pub const fn constraint_ref(&self) -> &IntegerConstraintEnum {
        &self.constraint.constraint_enum
    }

    /// Returns a new `IntegerDefinition` with strings laundered through the provided store.
    #[must_use]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            constraint: self.constraint.clone(),
            default_value: store.launder(&self.default_value),
        }
    }

    /// Returns the description of the parameter.
    #[must_use]
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
    pub fn default_value(&self) -> ShareableString {
        self.default_value.clone()
    }

    /// Returns a reference to the default value.
    #[must_use]
    pub const fn default_value_ref(&self) -> &ShareableString {
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
        let constraint_str = match &self.constraint.constraint_enum {
            IntegerConstraintEnum::Min { min, inclusive } => {
                format!(
                    " [Min({}, {})]",
                    *min,
                    if *inclusive { "inclusive" } else { "exclusive" }
                )
            }
            IntegerConstraintEnum::Max { max, inclusive } => {
                format!(
                    " [Max({}, {})]",
                    *max,
                    if *inclusive { "inclusive" } else { "exclusive" }
                )
            }
            IntegerConstraintEnum::Range {
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
            IntegerConstraintEnum::None => String::new(),
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
