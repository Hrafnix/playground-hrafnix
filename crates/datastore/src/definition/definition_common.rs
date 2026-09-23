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
}
