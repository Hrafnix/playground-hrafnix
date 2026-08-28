use crate::compile_time::{NumberConstraint, NumberConstraintEnum};
use crate::definition::NumberDefinition;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Compile-time representation of a number parameter.
pub struct NumberCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Constraint applied to this compile-time value.
    constraint: NumberConstraint,
    /// Default value for this compile-time value.
    default_value: &'static str,
}

impl NumberCompileTime {
    /// Hidden backing constructor for `number_compile_time!(description)`.
    ///
    /// This is an implementation detail; call `number_compile_time!` instead.
    /// `description` names the parameter. This arm creates a number with no
    /// constraint and no default value.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(description: &'static str) -> Self {
        Self::__new_with_constraint(description, NumberConstraint::none())
    }

    /// Hidden backing constructor for `number_compile_time!(description, default = default_value)`.
    ///
    /// This is an implementation detail; call `number_compile_time!` instead.
    /// `description` names the parameter and `default_value` is the decimal string
    /// default. This arm creates a number with no constraint.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_default(
        description: &'static str,
        default_value: &'static str,
    ) -> Self {
        Self::__new_with_constraint_and_default(
            description,
            NumberConstraint::none(),
            default_value,
        )
    }

    /// Hidden backing constructor for `number_compile_time!(description, constraint = constraint)`.
    ///
    /// This is an implementation detail; call `number_compile_time!` instead.
    /// `description` names the parameter and `constraint` is the [`NumberConstraint`]
    /// bound on the accepted value. This arm creates a number with no default value.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_constraint(
        description: &'static str,
        constraint: NumberConstraint,
    ) -> Self {
        Self {
            description,
            constraint,
            default_value: "",
        }
    }

    /// Hidden backing constructor for
    /// `number_compile_time!(description, constraint = constraint, default = default_value)`.
    ///
    /// This is an implementation detail; call `number_compile_time!` instead.
    /// `description` names the parameter, `constraint` is the [`NumberConstraint`] bound
    /// on the accepted value, and `default_value` is the decimal string default.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_constraint_and_default(
        description: &'static str,
        constraint: NumberConstraint,
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
    pub const fn constraint(&self) -> NumberConstraintEnum {
        self.constraint.constraint_enum
    }
    #[must_use]
    /// Returns the default value.
    pub const fn default_value(&self) -> &'static str {
        self.default_value
    }

    /// Converts this compile-time number into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> NumberDefinition {
        let constraint = self.constraint.into_definition();
        if self.default_value.is_empty() {
            NumberDefinition::new_with_constraint(self.description, constraint)
        } else {
            NumberDefinition::new_with_constraint_and_default(
                self.description,
                constraint,
                self.default_value,
            )
        }
    }
}

/// Creates a [`NumberCompileTime`], the compile-time metadata for an `f64`-valued
/// parameter, optionally bounded by a [`NumberConstraint`].
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// number_compile_time!(description)
/// number_compile_time!(description, default = default_value)
/// number_compile_time!(description, constraint = constraint)
/// number_compile_time!(description, constraint = constraint, default = default_value)
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the parameter.
/// - `constraint` (optional): [`NumberConstraint`] bound on the accepted value, built with
///   `NumberConstraint::none()`, `NumberConstraint::min(min, inclusive)`,
///   `NumberConstraint::max(max, inclusive)`, or
///   `NumberConstraint::range(a, b, a_inclusive, b_inclusive)`. When omitted, the value is
///   unconstrained.
/// - `default_value` (optional): `&'static str` decimal string default (e.g. `"1.5"`). When
///   omitted, the parameter has no default.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::{NumberCompileTime, NumberConstraint};
/// use datastore::prelude::*;
///
/// const WEIGHT: NumberCompileTime = number_compile_time!(
///     "Weight",
///     constraint = NumberConstraint::min(0.0, true),
///     default = "1.5"
/// );
/// assert_eq!(WEIGHT.default_value(), "1.5");
///
/// let _definition = WEIGHT.into_definition();
/// ```
#[macro_export]
macro_rules! number_compile_time {
    ($description:expr) => {
        const { $crate::compile_time::NumberCompileTime::__new($description) }
    };
    ($description:expr, default = $default_value:expr) => {
        const {
            $crate::compile_time::NumberCompileTime::__new_with_default(
                $description,
                $default_value,
            )
        }
    };
    ($description:expr, constraint = $constraint:expr) => {
        const {
            $crate::compile_time::NumberCompileTime::__new_with_constraint(
                $description,
                $constraint,
            )
        }
    };
    ($description:expr, constraint = $constraint:expr, default = $default_value:expr) => {
        const {
            $crate::compile_time::NumberCompileTime::__new_with_constraint_and_default(
                $description,
                $constraint,
                $default_value,
            )
        }
    };
}
