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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_string_equality_covers_equal_different_and_length_mismatch() {
        assert!(const_str_eq(
            std::hint::black_box("same"),
            std::hint::black_box("same")
        ));
        assert!(!const_str_eq(
            std::hint::black_box("left"),
            std::hint::black_box("lest")
        ));
        assert!(!const_str_eq(
            std::hint::black_box("short"),
            std::hint::black_box("shorter")
        ));
    }

    #[test]
    fn number_constraint_constructors_and_converters_cover_every_variant() {
        let constraints = [
            NumberConstraint::none(),
            NumberConstraint::min(std::hint::black_box(1.0), true),
            NumberConstraint::max(std::hint::black_box(9.0), false),
            NumberConstraint::range(std::hint::black_box(1.0), 9.0, true, false),
        ];

        assert_eq!(constraints[0].constraint_enum, NumberConstraintEnum::None);
        assert_eq!(
            constraints[1].constraint_enum,
            NumberConstraintEnum::Min {
                min: 1.0,
                inclusive: true
            }
        );
        assert_eq!(
            constraints[2].constraint_enum,
            NumberConstraintEnum::Max {
                max: 9.0,
                inclusive: false
            }
        );
        assert_eq!(
            constraints[3].constraint_enum,
            NumberConstraintEnum::Range {
                min: 1.0,
                max: 9.0,
                min_inclusive: true,
                max_inclusive: false
            }
        );

        for constraint in constraints {
            let expected = constraint.constraint_enum.into_definition();
            assert_eq!(constraint.into_definition().constraint_enum, expected);
        }
    }

    #[test]
    fn number_range_normalizes_reversed_and_degenerate_bounds() {
        let reversed = NumberConstraint::range(std::hint::black_box(9.0), 1.0, false, true);
        assert_eq!(
            reversed.constraint_enum,
            NumberConstraintEnum::Range {
                min: 1.0,
                max: 9.0,
                min_inclusive: true,
                max_inclusive: false
            }
        );

        for value in [0.0, -0.0, 1.0, -1.0] {
            let range = NumberConstraint::range(std::hint::black_box(value), value, false, true);
            let NumberConstraintEnum::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } = range.constraint_enum
            else {
                panic!("expected a range");
            };
            assert!(min < value);
            assert!(max > value);
            assert!(min_inclusive);
            assert!(!max_inclusive);
        }

        let infinite = NumberConstraint::range(
            std::hint::black_box(f64::INFINITY),
            f64::INFINITY,
            true,
            false,
        );
        assert_eq!(
            infinite.constraint_enum,
            NumberConstraintEnum::Range {
                min: f64::INFINITY,
                max: f64::INFINITY,
                min_inclusive: false,
                max_inclusive: true
            }
        );
    }
}
