use crate::definition::{
    IntegerConstraint as IntegerConstraintDefinition,
    IntegerConstraintEnum as IntegerConstraintEnumDefinition, IntegerDefinition,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Compile-time integer constraint variants.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Compile-time integer constraint.
pub struct IntegerConstraint {
    /// The actual constraint variant (none, min, max, or range).
    pub(crate) constraint_enum: IntegerConstraintEnum,
}

impl IntegerConstraint {
    #[must_use]
    /// Creates a new constraint with no bounds.
    pub const fn none() -> Self {
        Self {
            constraint_enum: IntegerConstraintEnum::None,
        }
    }
    #[must_use]
    /// Creates a new minimum constraint.
    pub const fn min(min: i64, inclusive: bool) -> Self {
        Self {
            constraint_enum: IntegerConstraintEnum::Min { min, inclusive },
        }
    }
    #[must_use]
    /// Creates a new maximum constraint.
    pub const fn max(max: i64, inclusive: bool) -> Self {
        Self {
            constraint_enum: IntegerConstraintEnum::Max { max, inclusive },
        }
    }
    #[must_use]
    /// Creates a new range constraint.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Compile-time representation of an integer parameter.
pub struct IntegerCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Constraint applied to this compile-time value.
    constraint: IntegerConstraint,
    /// Default value for this compile-time value.
    default_value: &'static str,
}

impl IntegerCompileTime {
    /// Hidden backing constructor for `integer_compile_time!(description)`.
    ///
    /// This is an implementation detail; call `integer_compile_time!` instead.
    /// `description` names the parameter. This arm creates an integer with no
    /// constraint and no default value.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(description: &'static str) -> Self {
        Self::__new_with_constraint(description, IntegerConstraint::none())
    }

    /// Hidden backing constructor for `integer_compile_time!(description, default = default_value)`.
    ///
    /// This is an implementation detail; call `integer_compile_time!` instead.
    /// `description` names the parameter and `default_value` is the decimal string
    /// default. This arm creates an integer with no constraint.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_default(
        description: &'static str,
        default_value: &'static str,
    ) -> Self {
        Self::__new_with_constraint_and_default(
            description,
            IntegerConstraint::none(),
            default_value,
        )
    }

    /// Hidden backing constructor for `integer_compile_time!(description, constraint = constraint)`.
    ///
    /// This is an implementation detail; call `integer_compile_time!` instead.
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
    /// `integer_compile_time!(description, constraint = constraint, default = default_value)`.
    ///
    /// This is an implementation detail; call `integer_compile_time!` instead.
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
    #[must_use]
    /// Returns the description.
    pub const fn description(&self) -> &'static str {
        self.description
    }
    #[must_use]
    /// Returns the constraint.
    pub const fn constraint(&self) -> IntegerConstraintEnum {
        self.constraint.constraint_enum
    }
    #[must_use]
    /// Returns the default value.
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
/// integer_compile_time!(description)
/// integer_compile_time!(description, default = default_value)
/// integer_compile_time!(description, constraint = constraint)
/// integer_compile_time!(description, constraint = constraint, default = default_value)
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
/// const COUNT: IntegerCompileTime = integer_compile_time!(
///     "Item count",
///     constraint = IntegerConstraint::range(0, 100, true, true),
///     default = "10"
/// );
/// assert_eq!(COUNT.default_value(), "10");
///
/// let _definition = COUNT.into_definition();
/// ```
#[macro_export]
macro_rules! integer_compile_time {
    ($description:expr) => {
        const { $crate::compile_time::IntegerCompileTime::__new($description) }
    };
    ($description:expr, default = $default_value:expr) => {
        const {
            $crate::compile_time::IntegerCompileTime::__new_with_default(
                $description,
                $default_value,
            )
        }
    };
    ($description:expr, constraint = $constraint:expr) => {
        const {
            $crate::compile_time::IntegerCompileTime::__new_with_constraint(
                $description,
                $constraint,
            )
        }
    };
    ($description:expr, constraint = $constraint:expr, default = $default_value:expr) => {
        const {
            $crate::compile_time::IntegerCompileTime::__new_with_constraint_and_default(
                $description,
                $constraint,
                $default_value,
            )
        }
    };
}
