use crate::definition::{
    NumberConstraint as NumberConstraintDefinition,
    NumberConstraintEnum as NumberConstraintEnumDefinition,
};

/// Compares strings in const contexts.
pub(crate) const fn const_str_eq(left: &str, right: &str) -> bool {
    let mut left = left.as_bytes();
    let mut right = right.as_bytes();

    loop {
        match (left, right) {
            ([], []) => return true,
            ([left_byte, left_rest @ ..], [right_byte, right_rest @ ..]) => {
                if *left_byte != *right_byte {
                    return false;
                }
                left = left_rest;
                right = right_rest;
            }
            _ => return false,
        }
    }
}

/// Asserts that a const slice of keyed tuples contains no duplicate keys.
macro_rules! assert_unique_keys {
    ($items:expr, $message:literal) => {
        let mut unchecked = $items;
        while let [(key, _), remaining @ ..] = unchecked {
            let mut candidates = remaining;
            while let [(candidate_key, _), rest @ ..] = candidates {
                assert!(
                    !$crate::compile_time::compile_time_common::const_str_eq(
                        key.as_str(),
                        candidate_key.as_str(),
                    ),
                    $message
                );
                candidates = rest;
            }
            unchecked = remaining;
        }
    };
}

pub(crate) use assert_unique_keys;

/// Compile-time number constraint variants.
#[derive(Debug, Clone, Copy, PartialEq)]
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

/// Compile-time number constraint.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NumberConstraint {
    /// The actual constraint variant (none, min, max, or range).
    pub(crate) constraint_enum: NumberConstraintEnum,
}

impl NumberConstraint {
    /// Creates a new constraint with no bounds.
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
    /// their corresponding inclusivity flags, so the resulting range is always valid.
    ///
    /// If `value_1` and `value_2` are equal (or within a hair's breadth of it due to
    /// floating-point imprecision), the range is widened symmetrically by `f64::EPSILON`
    /// so `min` and `max` never end up equal.
    #[must_use]
    pub const fn range(
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
        let min_bits = min.to_bits();
        if min_bits == max.to_bits() && min.is_finite() {
            if min_bits == 0 || min_bits == (1_u64 << 63) {
                min = f64::from_bits((1_u64 << 63) | 1);
                max = f64::from_bits(1);
            } else if min_bits & (1_u64 << 63) == 0 {
                min = f64::from_bits(min_bits.wrapping_sub(1));
                max = f64::from_bits(min_bits.wrapping_add(1));
            } else {
                min = f64::from_bits(min_bits.wrapping_add(1));
                max = f64::from_bits(min_bits.wrapping_sub(1));
            }
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

    /// Converts this compile-time number constraint into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> NumberConstraintDefinition {
        match self.constraint_enum.into_definition() {
            NumberConstraintEnumDefinition::None => NumberConstraintDefinition::none(),
            NumberConstraintEnumDefinition::Min { min, inclusive } => {
                NumberConstraintDefinition::min(min, inclusive)
            }
            NumberConstraintEnumDefinition::Max { max, inclusive } => {
                NumberConstraintDefinition::max(max, inclusive)
            }
            NumberConstraintEnumDefinition::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } => NumberConstraintDefinition::range(min, max, min_inclusive, max_inclusive),
        }
    }
}

impl NumberConstraintEnum {
    /// Converts this compile-time number constraint variant into a runtime definition.
    #[must_use]
    pub const fn into_definition(self) -> NumberConstraintEnumDefinition {
        match self {
            NumberConstraintEnum::Min { min, inclusive } => {
                NumberConstraintEnumDefinition::Min { min, inclusive }
            }
            NumberConstraintEnum::Max { max, inclusive } => {
                NumberConstraintEnumDefinition::Max { max, inclusive }
            }
            NumberConstraintEnum::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } => NumberConstraintEnumDefinition::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            },
            NumberConstraintEnum::None => NumberConstraintEnumDefinition::None,
        }
    }
}
