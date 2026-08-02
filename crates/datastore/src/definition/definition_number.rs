use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};

/// Definition for a number-based parameter constraint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NumberConstraint {
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
            constraint: NumberConstraint::None,
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
            constraint: NumberConstraint::None,
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
    pub fn constraint(&self) -> NumberConstraint {
        self.constraint.clone()
    }

    /// Returns a reference to the constraint.
    #[must_use]
    pub fn constraint_ref(&self) -> &NumberConstraint {
        &self.constraint
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
    pub fn description_ref(&self) -> &ShareableString {
        &self.description
    }

    /// Returns the default value of the parameter.
    #[must_use]
    pub fn default_value(&self) -> ShareableString {
        self.default_value.clone()
    }

    /// Returns a reference to the default value.
    #[must_use]
    pub fn default_value_ref(&self) -> &ShareableString {
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
        let constraint_str = match &self.constraint {
            NumberConstraint::Min { min, inclusive } => {
                format!(
                    " [Min({}, {})]",
                    format_number_value(*min),
                    if *inclusive { "inclusive" } else { "exclusive" }
                )
            }
            NumberConstraint::Max { max, inclusive } => {
                format!(
                    " [Max({}, {})]",
                    format_number_value(*max),
                    if *inclusive { "inclusive" } else { "exclusive" }
                )
            }
            NumberConstraint::Range {
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
            NumberConstraint::None => String::new(),
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
