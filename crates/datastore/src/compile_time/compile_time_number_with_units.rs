use crate::compile_time::{NumberConstraint, NumberConstraintEnum};
use crate::definition::NumberWithUnitsDefinition;
use units::UnitId;

/// Compile-time representation of a number-with-units parameter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NumberWithUnitsCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Constraint applied to this compile-time value.
    constraint: NumberConstraint,
    /// Preferred units for this compile-time value.
    preferred_units: UnitId,
    /// Default value for this compile-time value.
    default_value: &'static str,
}

impl NumberWithUnitsCompileTime {
    /// Hidden backing constructor for `const_number_with_units!(description, preferred_units)`.
    ///
    /// This is an implementation detail; call `const_number_with_units!` instead.
    /// `description` names the parameter and `preferred_units` is the [`UnitId`] the value
    /// is displayed and, by default, entered in. This arm creates a number-with-units value
    /// with no constraint and no default value.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(description: &'static str, preferred_units: UnitId) -> Self {
        #[allow(clippy::disallowed_methods)]
        Self::__new_with_constraint(description, NumberConstraint::none(), preferred_units)
    }

    /// Hidden backing constructor for
    /// `const_number_with_units!(description, preferred_units, default = default_value)`.
    ///
    /// This is an implementation detail; call `const_number_with_units!` instead.
    /// `description` names the parameter, `default_value` is the decimal string default, and
    /// `preferred_units` is the [`UnitId`] the value is displayed and, by default, entered
    /// in. This arm creates a number-with-units value with no constraint.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_default(
        description: &'static str,
        default_value: &'static str,
        preferred_units: UnitId,
    ) -> Self {
        #[allow(clippy::disallowed_methods)]
        Self::__new_with_constraint_and_default(
            description,
            NumberConstraint::none(),
            default_value,
            preferred_units,
        )
    }

    /// Hidden backing constructor for
    /// `const_number_with_units!(description, preferred_units, constraint = constraint)`.
    ///
    /// This is an implementation detail; call `const_number_with_units!` instead.
    /// `description` names the parameter, `constraint` is the [`NumberConstraint`] bound on
    /// the accepted value (expressed in the parameter's base unit), and `preferred_units` is
    /// the [`UnitId`] the value is displayed and, by default, entered in. This arm creates a
    /// number-with-units value with no default value.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_constraint(
        description: &'static str,
        constraint: NumberConstraint,
        preferred_units: UnitId,
    ) -> Self {
        Self {
            description,
            constraint,
            preferred_units,
            default_value: "",
        }
    }

    /// Hidden backing constructor for
    /// `const_number_with_units!(description, preferred_units, constraint = constraint, default = default_value)`.
    ///
    /// This is an implementation detail; call `const_number_with_units!` instead.
    /// `description` names the parameter, `constraint` is the [`NumberConstraint`] bound on
    /// the accepted value (expressed in the parameter's base unit), `default_value` is the
    /// decimal string default, and `preferred_units` is the [`UnitId`] the value is
    /// displayed and, by default, entered in.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_constraint_and_default(
        description: &'static str,
        constraint: NumberConstraint,
        default_value: &'static str,
        preferred_units: UnitId,
    ) -> Self {
        Self {
            description,
            constraint,
            preferred_units,
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
    pub const fn constraint(&self) -> NumberConstraintEnum {
        self.constraint.constraint_enum
    }

    /// Returns the preferred units.
    #[must_use]
    pub const fn preferred_units(&self) -> UnitId {
        self.preferred_units
    }

    /// Returns the default value.
    #[must_use]
    pub const fn default_value(&self) -> &'static str {
        self.default_value
    }

    /// Converts this compile-time number-with-units into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> NumberWithUnitsDefinition {
        let constraint = self.constraint.into_definition();
        if self.default_value.is_empty() {
            NumberWithUnitsDefinition::new_with_constraint(
                self.description,
                constraint,
                self.preferred_units,
            )
        } else {
            NumberWithUnitsDefinition::new_with_constraint_and_default(
                self.description,
                constraint,
                self.default_value,
                self.preferred_units,
            )
        }
    }
}

/// Creates a [`NumberWithUnitsCompileTime`], the compile-time metadata for an `f64`-valued
/// parameter that is displayed and stored using a preferred unit, optionally bounded by a
/// [`NumberConstraint`].
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// const_number_with_units!(description, preferred_units)
/// const_number_with_units!(description, preferred_units, default = default_value)
/// const_number_with_units!(description, preferred_units, constraint = constraint)
/// const_number_with_units!(
///     description,
///     preferred_units,
///     constraint = constraint,
///     default = default_value,
/// )
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the parameter.
/// - `preferred_units`: [`UnitId`] (from the `units` crate) the unit the value is displayed
///   and, by default, entered in.
/// - `constraint` (optional): [`NumberConstraint`] bound on the accepted value, expressed in
///   the parameter's base unit, built with `NumberConstraint::none()`,
///   `NumberConstraint::min(min, inclusive)`, `NumberConstraint::max(max, inclusive)`, or
///   `NumberConstraint::range(a, b, a_inclusive, b_inclusive)`. When omitted, the value is
///   unconstrained.
/// - `default_value` (optional): `&'static str` decimal string default. When omitted, the
///   parameter has no default.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::{NumberConstraint, NumberWithUnitsCompileTime};
/// use datastore::prelude::*;
/// use units::UnitId;
///
/// const LENGTH: NumberWithUnitsCompileTime = const_number_with_units!(
///     "Length",
///     UnitId::Length_Meter,
///     constraint = NumberConstraint::min(0.0, true),
///     default = "1"
/// );
/// assert_eq!(LENGTH.preferred_units(), UnitId::Length_Meter);
///
/// let _definition = LENGTH.into_definition();
/// ```
#[macro_export]
macro_rules! const_number_with_units {
    ($description:expr, $preferred_units:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::NumberWithUnitsCompileTime::__new($description, $preferred_units)
        }
    };
    ($description:expr, $preferred_units:expr, default = $default_value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::NumberWithUnitsCompileTime::__new_with_default(
                $description,
                $default_value,
                $preferred_units,
            )
        }
    };
    ($description:expr, $preferred_units:expr, constraint = $constraint:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::NumberWithUnitsCompileTime::__new_with_constraint(
                $description,
                $constraint,
                $preferred_units,
            )
        }
    };
    ($description:expr, $preferred_units:expr, constraint = $constraint:expr, default = $default_value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::NumberWithUnitsCompileTime::__new_with_constraint_and_default(
                $description,
                $constraint,
                $default_value,
                $preferred_units,
            )
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::disallowed_methods)]
    fn hidden_constructors_run_at_runtime() {
        let unit = UnitId::Length_Meter;
        let plain = NumberWithUnitsCompileTime::__new(std::hint::black_box("Plain"), unit);
        let defaulted = NumberWithUnitsCompileTime::__new_with_default(
            std::hint::black_box("Defaulted"),
            "1.5",
            unit,
        );
        let constrained = NumberWithUnitsCompileTime::__new_with_constraint(
            std::hint::black_box("Constrained"),
            NumberConstraint::min(0.0, true),
            unit,
        );
        let constrained_defaulted = NumberWithUnitsCompileTime::__new_with_constraint_and_default(
            std::hint::black_box("Both"),
            NumberConstraint::max(100.0, false),
            "50",
            unit,
        );

        assert_eq!(plain.preferred_units(), unit);
        assert_eq!(plain.constraint(), NumberConstraintEnum::None);
        assert_eq!(defaulted.default_value(), "1.5");
        assert!(matches!(
            constrained.constraint(),
            NumberConstraintEnum::Min {
                min: 0.0,
                inclusive: true
            }
        ));
        assert!(matches!(
            constrained_defaulted.constraint(),
            NumberConstraintEnum::Max {
                max: 100.0,
                inclusive: false
            }
        ));
        assert_eq!(constrained_defaulted.default_value(), "50");
    }
}
