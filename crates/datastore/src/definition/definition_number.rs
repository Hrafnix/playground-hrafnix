use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};

/// Definition for a number-based parameter constraint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NumberConstraintEnum {
    /// Minimum value constraint.
    Min {
        /// Minimum value of the constraint.
        min: f64,
        /// Whether the minimum value is inclusive.
        inclusive: bool,
    },
    /// Maximum value constraint.
    Max {
        /// Maximum value of the constraint.
        max: f64,
        /// Whether the maximum value is inclusive.
        inclusive: bool,
    },
    /// Range value constraint.
    Range {
        /// Minimum value of the range.
        min: f64,
        /// Maximum value of the range.
        max: f64,
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
pub struct NumberConstraint {
    pub(crate) constraint_enum: NumberConstraintEnum,
}

impl NumberConstraint {
    /// Creates a new `NumberConstraint` with no constraint.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            constraint_enum: NumberConstraintEnum::None,
        }
    }

    /// Creates a new `NumberConstraint` with a minimum value constraint.
    #[must_use]
    pub const fn min(min: f64, inclusive: bool) -> Self {
        Self {
            constraint_enum: NumberConstraintEnum::Min { min, inclusive },
        }
    }

    /// Creates a new `NumberConstraint` with a maximum value constraint.
    #[must_use]
    pub const fn max(max: f64, inclusive: bool) -> Self {
        Self {
            constraint_enum: NumberConstraintEnum::Max { max, inclusive },
        }
    }

    /// Creates a new `NumberConstraint` with a range value constraint.
    ///
    /// If `value_1` is greater than `value_2`, the two values are swapped along with
    /// their corresponding inclusivity flags so the resulting range is always valid.
    ///
    /// If `value_1` and `value_2` are equal (or within a hair's breadth of it due to
    /// floating-point imprecision), the range is widened symmetrically by `f64::EPSILON`
    /// so `min` and `max` never end up equal.
    #[must_use]
    pub fn range(
        value_1: f64,
        value_2: f64,
        value_1_inclusive: bool,
        value_2_inclusive: bool,
    ) -> Self {
        let (mut min, mut max, min_inclusive, max_inclusive) = if value_1 >= value_2 {
            (value_2, value_1, value_2_inclusive, value_1_inclusive)
        } else {
            (value_1, value_2, value_1_inclusive, value_2_inclusive)
        };

        // If the range is degenerate (or within a hair's breadth of it due to
        // floating-point imprecision), widen it symmetrically by `f64::EPSILON`
        // so `min` and `max` never end up equal.
        if (max - min).abs() < f64::EPSILON {
            min -= f64::EPSILON;
            max += f64::EPSILON;
        }

        Self {
            constraint_enum: NumberConstraintEnum::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            },
        }
    }
}

impl<'de> Deserialize<'de> for NumberConstraint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Mirrors the shape produced by the derived `Serialize` impl above, so the
        // on-the-wire format is unchanged; only construction is routed through the
        // same normalization logic as `NumberConstraint::range` so a deserialized
        // `Range` can never end up with `min > max`.
        #[derive(Deserialize)]
        struct Raw {
            constraint_enum: NumberConstraintEnum,
        }

        let raw = Raw::deserialize(deserializer)?;
        let constraint_enum = match raw.constraint_enum {
            NumberConstraintEnum::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } => {
                return Ok(NumberConstraint::range(
                    min,
                    max,
                    min_inclusive,
                    max_inclusive,
                ));
            }
            other => other,
        };

        Ok(Self { constraint_enum })
    }
}

/// Definition for a number-based parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberDefinition {
    description: ShareableString,
    constraint: NumberConstraint,
    default_value: ShareableString,
}

impl NumberDefinition {
    /// Creates a new number-based `NumberDefinition`.
    pub fn new<S1: Into<ShareableString>>(description: S1) -> Self {
        Self {
            description: description.into(),
            constraint: NumberConstraint::none(),
            default_value: ShareableString::default(),
        }
    }

    /// Creates a new number-based `NumberDefinition` with a default value.
    pub fn new_with_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        default_value: S2,
    ) -> Self {
        Self {
            description: description.into(),
            constraint: NumberConstraint::none(),
            default_value: default_value.into(),
        }
    }

    /// Creates a new number-based `NumberDefinition`.
    pub fn new_with_constraint<S: Into<ShareableString>>(
        description: S,
        constraint: NumberConstraint,
    ) -> Self {
        Self {
            description: description.into(),
            constraint,
            default_value: ShareableString::default(),
        }
    }

    /// Creates a new number-based `NumberDefinition` with a default value.
    pub fn new_with_constraint_and_default<S1: Into<ShareableString>, S2: Into<ShareableString>>(
        description: S1,
        constraint: NumberConstraint,
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
    pub fn constraint(&self) -> NumberConstraintEnum {
        self.constraint.constraint_enum.clone()
    }

    /// Returns a reference to the constraint.
    #[must_use]
    pub const fn constraint_ref(&self) -> &NumberConstraintEnum {
        &self.constraint.constraint_enum
    }

    /// Returns a new `NumberDefinition` with strings laundered through the provided store.
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

impl PartialEq<&NumberDefinition> for NumberDefinition {
    fn eq(&self, other: &&NumberDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<NumberDefinition> for &NumberDefinition {
    fn eq(&self, other: &NumberDefinition) -> bool {
        *self == other
    }
}

fn format_number_value(value: f64) -> String {
    if !value.is_finite() {
        return value.to_string();
    }

    let mut formatted = value.to_string();
    if !formatted.contains('.') && !formatted.contains('e') && !formatted.contains('E') {
        formatted.push_str(".0");
    }

    formatted
}

impl TreePrint for NumberDefinition {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        let constraint_str = match &self.constraint.constraint_enum {
            NumberConstraintEnum::Min { min, inclusive } => {
                format!(
                    " [Min({}, {})]",
                    format_number_value(*min),
                    if *inclusive { "inclusive" } else { "exclusive" }
                )
            }
            NumberConstraintEnum::Max { max, inclusive } => {
                format!(
                    " [Max({}, {})]",
                    format_number_value(*max),
                    if *inclusive { "inclusive" } else { "exclusive" }
                )
            }
            NumberConstraintEnum::Range {
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
                format!(
                    " [Range({}, {}, {}, {})]",
                    format_number_value(*min),
                    format_number_value(*max),
                    min_type,
                    max_type
                )
            }
            NumberConstraintEnum::None => String::new(),
        };

        writeln!(
            f,
            "{}{}{} ({}) Number - default: \"{}\"{}",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.default_value,
            constraint_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::format_number_value;

    #[test]
    fn format_number_value_keeps_fractional_values() {
        assert_eq!(format_number_value(1.52), "1.52");
    }

    #[test]
    fn format_number_value_adds_single_decimal_for_integers() {
        assert_eq!(format_number_value(1.0), "1.0");
        assert_eq!(format_number_value(42.0), "42.0");
    }
}
