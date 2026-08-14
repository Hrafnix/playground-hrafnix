use crate::definition::{NumberConstraint, NumberConstraintEnum};
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};

/// Definition for a number-based parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberDefinition {
    /// Human-readable description of this number parameter.
    description: ShareableString,
    /// Optional constraint (min, max, range, or none) applied to the value.
    constraint: NumberConstraint,
    /// Default value for this number parameter.
    default_value: ShareableString,
}

impl NumberDefinition {
    /// Creates a new number-based `NumberDefinition`.
    #[hotpath::measure]
    pub fn new<S1: Into<ShareableString>>(description: S1) -> Self {
        Self {
            description: description.into(),
            constraint: NumberConstraint::none(),
            default_value: ShareableString::default(),
        }
    }

    /// Creates a new number-based `NumberDefinition` with a default value.
    #[hotpath::measure]
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
    #[hotpath::measure]
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
    #[hotpath::measure]
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
    #[hotpath::measure]
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
    #[hotpath::measure]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        Self {
            description: store.launder(&self.description),
            constraint: self.constraint.clone(),
            default_value: store.launder(&self.default_value),
        }
    }

    /// Returns the description of the parameter.
    #[must_use]
    #[hotpath::measure]
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
    #[hotpath::measure]
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
    #[hotpath::measure]
    fn eq(&self, other: &&NumberDefinition) -> bool {
        self == *other
    }
}

impl PartialEq<NumberDefinition> for &NumberDefinition {
    #[hotpath::measure]
    fn eq(&self, other: &NumberDefinition) -> bool {
        *self == other
    }
}

/// Formats an `f64` for display, appending `.0` when the value has no
/// fractional part and is not in scientific notation.
#[hotpath::measure]
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
    #[hotpath::measure]
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
