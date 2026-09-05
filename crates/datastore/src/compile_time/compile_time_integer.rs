use crate::definition::{
    IntegerConstraint as IntegerConstraintDefinition,
    IntegerConstraintEnum as IntegerConstraintEnumDefinition, IntegerDefinition,
};

/// Compile-time integer constraint variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Compile-time integer constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegerConstraint {
    /// The actual constraint variant (none, min, max, or range).
    pub(crate) constraint_enum: IntegerConstraintEnum,
}

impl IntegerConstraint {
    /// Creates a new constraint with no bounds.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            constraint_enum: IntegerConstraintEnum::None,
        }
    }

    /// Creates a new minimum constraint.
    #[must_use]
    pub const fn min(min: i64, inclusive: bool) -> Self {
        Self {
            constraint_enum: IntegerConstraintEnum::Min { min, inclusive },
        }
    }

    /// Creates a new maximum constraint.
    #[must_use]
    pub const fn max(max: i64, inclusive: bool) -> Self {
        Self {
            constraint_enum: IntegerConstraintEnum::Max { max, inclusive },
        }
    }

    /// Creates a new range constraint.
    #[must_use]
    pub const fn range(a: i64, b: i64, a_inclusive: bool, b_inclusive: bool) -> Self {
        let (min, max, min_inclusive, max_inclusive) = if a > b {
            (b, a, b_inclusive, a_inclusive)
        } else if a == b {
            (a, b, true, true)
        } else {
            (a, b, a_inclusive, b_inclusive)
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

    /// Converts this compile-time integer constraint into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> IntegerConstraintDefinition {
        match self.constraint_enum.into_definition() {
            IntegerConstraintEnumDefinition::None => IntegerConstraintDefinition::none(),
            IntegerConstraintEnumDefinition::Min { min, inclusive } => {
                IntegerConstraintDefinition::min(min, inclusive)
            }
            IntegerConstraintEnumDefinition::Max { max, inclusive } => {
                IntegerConstraintDefinition::max(max, inclusive)
            }
            IntegerConstraintEnumDefinition::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } => IntegerConstraintDefinition::range(min, max, min_inclusive, max_inclusive),
        }
    }
}

impl IntegerConstraintEnum {
    /// Converts this compile-time integer constraint variant into a runtime definition.
    #[must_use]
    pub const fn into_definition(self) -> IntegerConstraintEnumDefinition {
        match self {
            IntegerConstraintEnum::Min { min, inclusive } => {
                IntegerConstraintEnumDefinition::Min { min, inclusive }
            }
            IntegerConstraintEnum::Max { max, inclusive } => {
                IntegerConstraintEnumDefinition::Max { max, inclusive }
            }
            IntegerConstraintEnum::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } => IntegerConstraintEnumDefinition::Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            },
            IntegerConstraintEnum::None => IntegerConstraintEnumDefinition::None,
        }
    }
}

/// Compile-time representation of an integer parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegerCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Constraint applied to this compile-time value.
    constraint: IntegerConstraint,
    /// Default value for this compile-time value.
    default_value: &'static str,
}

impl IntegerCompileTime {
    /// Hidden backing constructor for `const_integer!(description)`.
    ///
    /// This is an implementation detail; call `const_integer!` instead.
    /// `description` names the parameter. This arm creates an integer with no
    /// constraint and no default value.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(description: &'static str) -> Self {
        #[allow(clippy::disallowed_methods)]
        Self::__new_with_constraint(description, IntegerConstraint::none())
    }

    /// Hidden backing constructor for `const_integer!(description, default = default_value)`.
    ///
    /// This is an implementation detail; call `const_integer!` instead.
    /// `description` names the parameter and `default_value` is the decimal string
    /// default. This arm creates an integer with no constraint.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_default(
        description: &'static str,
        default_value: &'static str,
    ) -> Self {
        #[allow(clippy::disallowed_methods)]
        Self::__new_with_constraint_and_default(
            description,
            IntegerConstraint::none(),
            default_value,
        )
    }

    /// Hidden backing constructor for `const_integer!(description, constraint = constraint)`.
    ///
    /// This is an implementation detail; call `const_integer!` instead.
    /// `description` names the parameter and `constraint` is the [`IntegerConstraint`]
    /// bound on the accepted value. This arm creates an integer with no default value.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_constraint(
        description: &'static str,
        constraint: IntegerConstraint,
    ) -> Self {
        Self {
            description,
            constraint,
            default_value: "",
        }
    }

    /// Hidden backing constructor for
    /// `const_integer!(description, constraint = constraint, default = default_value)`.
    ///
    /// This is an implementation detail; call `const_integer!` instead.
    /// `description` names the parameter, `constraint` is the [`IntegerConstraint`] bound
    /// on the accepted value, and `default_value` is the decimal string default.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_constraint_and_default(
        description: &'static str,
        constraint: IntegerConstraint,
        default_value: &'static str,
    ) -> Self {
        Self {
            description,
            constraint,
            default_value,
        }
    }

    /// Returns the description.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Returns the constraint.
    #[must_use]
    pub const fn constraint(&self) -> IntegerConstraintEnum {
        self.constraint.constraint_enum
    }

    /// Returns the default value.
    #[must_use]
    pub const fn default_value(&self) -> &'static str {
        self.default_value
    }

    /// Converts this compile-time integer into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> IntegerDefinition {
        let constraint = self.constraint.into_definition();
        if self.default_value.is_empty() {
            IntegerDefinition::new_with_constraint(self.description, constraint)
        } else {
            IntegerDefinition::new_with_constraint_and_default(
                self.description,
                constraint,
                self.default_value,
            )
        }
    }
}

/// Creates an [`IntegerCompileTime`], the compile-time metadata for an integer-valued
/// parameter, optionally bounded by an [`IntegerConstraint`].
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// const_integer!(description)
/// const_integer!(description, default = default_value)
/// const_integer!(description, constraint = constraint)
/// const_integer!(description, constraint = constraint, default = default_value)
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the parameter.
/// - `constraint` (optional): [`IntegerConstraint`] bound on the accepted value, built with
///   `IntegerConstraint::none()`, `IntegerConstraint::min(min, inclusive)`,
///   `IntegerConstraint::max(max, inclusive)`, or
///   `IntegerConstraint::range(a, b, a_inclusive, b_inclusive)`. When omitted, the value is
///   unconstrained.
/// - `default_value` (optional): `&'static str` decimal string default (e.g. `"10"`). When
///   omitted, the parameter has no default.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::{IntegerCompileTime, IntegerConstraint};
/// use datastore::prelude::*;
///
/// const COUNT: IntegerCompileTime = const_integer!(
///     "Item count",
///     constraint = IntegerConstraint::range(0, 100, true, true),
///     default = "10"
/// );
/// assert_eq!(COUNT.default_value(), "10");
///
/// let _definition = COUNT.into_definition();
/// ```
#[macro_export]
macro_rules! const_integer {
    ($description:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::IntegerCompileTime::__new($description)
        }
    };
    ($description:expr, default = $default_value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::IntegerCompileTime::__new_with_default(
                $description,
                $default_value,
            )
        }
    };
    ($description:expr, constraint = $constraint:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::IntegerCompileTime::__new_with_constraint(
                $description,
                $constraint,
            )
        }
    };
    ($description:expr, constraint = $constraint:expr, default = $default_value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::IntegerCompileTime::__new_with_constraint_and_default(
                $description,
                $constraint,
                $default_value,
            )
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_constraint_constructors_and_converters_cover_every_variant() {
        let constraints = [
            IntegerConstraint::none(),
            IntegerConstraint::min(std::hint::black_box(1), true),
            IntegerConstraint::max(std::hint::black_box(9), false),
            IntegerConstraint::range(std::hint::black_box(1), 9, true, false),
        ];

        assert_eq!(constraints[0].constraint_enum, IntegerConstraintEnum::None);
        assert_eq!(
            constraints[1].constraint_enum,
            IntegerConstraintEnum::Min {
                min: 1,
                inclusive: true
            }
        );
        assert_eq!(
            constraints[2].constraint_enum,
            IntegerConstraintEnum::Max {
                max: 9,
                inclusive: false
            }
        );
        assert_eq!(
            constraints[3].constraint_enum,
            IntegerConstraintEnum::Range {
                min: 1,
                max: 9,
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
    fn integer_range_normalizes_reversed_and_equal_bounds() {
        let reversed = IntegerConstraint::range(std::hint::black_box(9), 1, false, true);
        assert_eq!(
            reversed.constraint_enum,
            IntegerConstraintEnum::Range {
                min: 1,
                max: 9,
                min_inclusive: true,
                max_inclusive: false
            }
        );

        let equal = IntegerConstraint::range(std::hint::black_box(5), 5, false, false);
        assert_eq!(
            equal.constraint_enum,
            IntegerConstraintEnum::Range {
                min: 5,
                max: 5,
                min_inclusive: true,
                max_inclusive: true
            }
        );
    }

    #[test]
    #[allow(clippy::disallowed_methods)]
    fn hidden_constructors_run_at_runtime() {
        let plain = IntegerCompileTime::__new(std::hint::black_box("Plain"));
        let defaulted =
            IntegerCompileTime::__new_with_default(std::hint::black_box("Defaulted"), "10");
        let constrained = IntegerCompileTime::__new_with_constraint(
            std::hint::black_box("Constrained"),
            IntegerConstraint::min(0, true),
        );
        let constrained_defaulted = IntegerCompileTime::__new_with_constraint_and_default(
            std::hint::black_box("Both"),
            IntegerConstraint::max(100, false),
            "50",
        );

        assert_eq!(plain.constraint(), IntegerConstraintEnum::None);
        assert_eq!(defaulted.default_value(), "10");
        assert_eq!(
            constrained.constraint(),
            IntegerConstraintEnum::Min {
                min: 0,
                inclusive: true
            }
        );
        assert_eq!(
            constrained_defaulted.constraint(),
            IntegerConstraintEnum::Max {
                max: 100,
                inclusive: false
            }
        );
        assert_eq!(constrained_defaulted.default_value(), "50");
    }
}
