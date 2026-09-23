/// Definition for a number-based parameter constraint.
#[derive(Debug, Clone, PartialEq)]
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

/// Maximum distance, in units in the last place (ULPs), between two constraint bounds
/// for them to be considered equal when checking merge compatibility.
///
/// One ULP is one `f64::next_up` step, so this tolerance scales with the magnitude of
/// the bounds and absorbs rounding noise (e.g. `0.1 + 0.2` vs `0.3`) without treating
/// genuinely different values as equal.
const MERGE_BOUND_MAX_ULPS: u64 = 4;

/// Maps an `f64` onto a monotonic `u64` line so that adjacent floats differ by exactly one.
///
/// `-0.0` and `0.0` map to adjacent values.
const fn ordered_bits(value: f64) -> u64 {
    let bits = value.to_bits();
    if bits >> 63 == 0 {
        bits | (1 << 63)
    } else {
        !bits
    }
}

/// Returns true if `a` and `b` are within [`MERGE_BOUND_MAX_ULPS`] of each other.
const fn bounds_approx_eq(a: f64, b: f64) -> bool {
    ordered_bits(a).abs_diff(ordered_bits(b)) <= MERGE_BOUND_MAX_ULPS
}

/// Definition for an integer-based parameter constraint.
#[derive(Debug, Clone, PartialEq)]
pub struct NumberConstraint {
    /// The actual constraint variant (none, min, max, or range).
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
    ///
    /// If `min` is not finite (i.e., it is NaN or infinite), the constraint will be treated as `None`.
    #[must_use]
    pub const fn min(min: f64, inclusive: bool) -> Self {
        if !min.is_finite() {
            return Self {
                constraint_enum: NumberConstraintEnum::None,
            };
        }

        Self {
            constraint_enum: NumberConstraintEnum::Min { min, inclusive },
        }
    }

    /// Creates a new `NumberConstraint` with a maximum value constraint.
    ///
    /// If `max` is not finite (i.e., it is NaN or infinite), the constraint will be treated as `None`.
    #[must_use]
    pub const fn max(max: f64, inclusive: bool) -> Self {
        if !max.is_finite() {
            return Self {
                constraint_enum: NumberConstraintEnum::None,
            };
        }

        Self {
            constraint_enum: NumberConstraintEnum::Max { max, inclusive },
        }
    }

    /// Creates a new `NumberConstraint` with a range value constraint.
    ///
    /// If `value_1` is greater than `value_2`, the two values are swapped along with
    /// their corresponding inclusivity flags, so the resulting range is always valid.
    ///
    /// If the bounds and inclusivity flags describe a range containing no representable
    /// `f64` (e.g. `(1.0, 1.0]`, or `(x, y)` where `y` is the next float after `x`), both
    /// ends are made inclusive so the range is always satisfiable.
    ///
    /// If `value_1` or `value_2` are not finite (i.e., they are NaN or infinite), the constraint
    /// for that value will be treated as `None`.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn range(
        value_1: f64,
        value_2: f64,
        value_1_inclusive: bool,
        value_2_inclusive: bool,
    ) -> Self {
        let (min, max, min_inclusive, max_inclusive) = if value_1 >= value_2 {
            (value_2, value_1, value_2_inclusive, value_1_inclusive)
        } else {
            (value_1, value_2, value_1_inclusive, value_2_inclusive)
        };

        // Make sure that if both min and/or max are finite.
        // If infinite/NAN, we return a constraint that makes sense.
        if !min.is_finite() && !max.is_finite() {
            return Self::none();
        }

        if !min.is_finite() {
            return Self::max(max, max_inclusive);
        }

        if !max.is_finite() {
            return Self::min(min, min_inclusive);
        }

        // Check if the range contains at least one representable f64 value.
        let contains_value = match (min_inclusive, max_inclusive) {
            (true, true) => min <= max,
            (false, false) => min.next_up() < max,
            _ => min < max,
        };

        let (min_inclusive, max_inclusive) = if contains_value {
            (min_inclusive, max_inclusive)
        } else {
            (true, true)
        };

        Self {
            constraint_enum: NumberConstraintEnum::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            },
        }
    }

    /// Returns true if the constraints match, comparing bounds within
    /// a few ULPs (units in the last place) and inclusivity flags exactly.
    #[must_use]
    pub const fn is_merge_compatible(&self, other: &Self) -> bool {
        match (&self.constraint_enum, &other.constraint_enum) {
            (NumberConstraintEnum::None, NumberConstraintEnum::None) => true,
            (
                NumberConstraintEnum::Min {
                    min: a,
                    inclusive: ai,
                },
                NumberConstraintEnum::Min {
                    min: b,
                    inclusive: bi,
                },
            )
            | (
                NumberConstraintEnum::Max {
                    max: a,
                    inclusive: ai,
                },
                NumberConstraintEnum::Max {
                    max: b,
                    inclusive: bi,
                },
            ) => *ai == *bi && bounds_approx_eq(*a, *b),
            (
                NumberConstraintEnum::Range {
                    min: a_min,
                    max: a_max,
                    min_inclusive: a_min_inc,
                    max_inclusive: a_max_inc,
                },
                NumberConstraintEnum::Range {
                    min: b_min,
                    max: b_max,
                    min_inclusive: b_min_inc,
                    max_inclusive: b_max_inc,
                },
            ) => {
                *a_min_inc == *b_min_inc
                    && *a_max_inc == *b_max_inc
                    && bounds_approx_eq(*a_min, *b_min)
                    && bounds_approx_eq(*a_max, *b_max)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn ulps_up(value: f64, n: u64) -> f64 {
        f64::from_bits(value.to_bits() + n)
    }

    fn min(value: f64) -> NumberConstraint {
        NumberConstraint::min(value, true)
    }

    #[test]
    fn ordered_bits_is_monotonic_across_zero() {
        let values = [
            f64::MIN,
            -1.0,
            -f64::MIN_POSITIVE,
            -f64::from_bits(1),
            -0.0,
            0.0,
            f64::from_bits(1),
            f64::MIN_POSITIVE,
            1.0,
            f64::MAX,
        ];
        for pair in values.windows(2) {
            assert!(ordered_bits(pair[0]) < ordered_bits(pair[1]), "{pair:?}");
        }
        assert_eq!(ordered_bits(0.0) - ordered_bits(-0.0), 1);
        assert_eq!(ordered_bits(1.0_f64.next_up()) - ordered_bits(1.0), 1);
        assert_eq!(ordered_bits((-1.0_f64).next_up()) - ordered_bits(-1.0), 1);
    }

    #[test]
    fn bounds_approx_eq_uses_ulp_distance_at_every_magnitude() {
        for base in [1e-300, 1e-12, 1.0, 1e20, 1e300] {
            assert!(bounds_approx_eq(base, base));
            assert!(bounds_approx_eq(base, ulps_up(base, MERGE_BOUND_MAX_ULPS)));
            assert!(bounds_approx_eq(ulps_up(base, MERGE_BOUND_MAX_ULPS), base));
            assert!(!bounds_approx_eq(
                base,
                ulps_up(base, MERGE_BOUND_MAX_ULPS + 1)
            ));
            assert!(bounds_approx_eq(
                -base,
                -ulps_up(base, MERGE_BOUND_MAX_ULPS)
            ));
            assert!(!bounds_approx_eq(
                -base,
                -ulps_up(base, MERGE_BOUND_MAX_ULPS + 1)
            ));
        }
    }

    #[test]
    fn bounds_approx_eq_absorbs_rounding_noise_only() {
        assert!(bounds_approx_eq(0.1 + 0.2, 0.3));
        assert!(!bounds_approx_eq(1e-12, 1e-10));
        assert!(!bounds_approx_eq(1.0, 1.000_000_000_5));
        assert!(!bounds_approx_eq(0.3, 0.31));
    }

    #[test]
    fn bounds_approx_eq_handles_zero_and_sign() {
        assert!(bounds_approx_eq(-0.0, 0.0));
        assert!(bounds_approx_eq(-f64::from_bits(1), f64::from_bits(1)));
        assert!(!bounds_approx_eq(-1e-300, 1e-300));
        assert!(!bounds_approx_eq(-1.0, 1.0));
    }

    #[test]
    fn is_merge_compatible_applies_ulp_tolerance_to_every_bound() {
        let edge = ulps_up(1.0, MERGE_BOUND_MAX_ULPS);
        let past = ulps_up(1.0, MERGE_BOUND_MAX_ULPS + 1);

        assert!(min(1.0).is_merge_compatible(&min(edge)));
        assert!(!min(1.0).is_merge_compatible(&min(past)));

        let max = |value| NumberConstraint::max(value, true);
        assert!(max(1.0).is_merge_compatible(&max(edge)));
        assert!(!max(1.0).is_merge_compatible(&max(past)));

        let range = |lo, hi| NumberConstraint::range(lo, hi, true, true);
        assert!(range(0.0, 1.0).is_merge_compatible(&range(0.0, edge)));
        assert!(!range(0.0, 1.0).is_merge_compatible(&range(0.0, past)));
        assert!(range(1.0, 2.0).is_merge_compatible(&range(edge, 2.0)));
        assert!(!range(1.0, 2.0).is_merge_compatible(&range(past, 2.0)));
    }
}
