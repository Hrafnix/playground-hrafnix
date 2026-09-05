use crate::definition::UnitDefinition;
use units::UnitFamilyId;

/// Compile-time representation of a unit parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Unit family for this compile-time value.
    unit_family: UnitFamilyId,
    /// Default value for this compile-time value.
    default_value: &'static str,
}

impl UnitCompileTime {
    /// Hidden backing constructor for `const_unit!(description, unit_family)`.
    ///
    /// This is an implementation detail; call `const_unit!` instead.
    /// `description` names the parameter and `unit_family` is the [`UnitFamilyId`] the
    /// value may be selected from. This arm creates a unit value with no default.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(description: &'static str, unit_family: UnitFamilyId) -> Self {
        Self {
            description,
            unit_family,
            default_value: "",
        }
    }

    /// Hidden backing constructor for
    /// `const_unit!(description, unit_family, default = default_value)`.
    ///
    /// This is an implementation detail; call `const_unit!` instead.
    /// `description` names the parameter, `unit_family` is the [`UnitFamilyId`] the value
    /// may be selected from, and `default_value` is the string id of the default unit
    /// (which must belong to `unit_family`).
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_default(
        description: &'static str,
        unit_family: UnitFamilyId,
        default_value: &'static str,
    ) -> Self {
        Self {
            description,
            unit_family,
            default_value,
        }
    }

    /// Returns the description.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Returns the unit family.
    #[must_use]
    pub const fn unit_family(&self) -> UnitFamilyId {
        self.unit_family
    }

    /// Returns the default value.
    #[must_use]
    pub const fn default_value(&self) -> &'static str {
        self.default_value
    }

    /// Converts this compile-time unit into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> UnitDefinition {
        if self.default_value.is_empty() {
            UnitDefinition::new(self.description, self.unit_family)
        } else {
            UnitDefinition::new_with_default(self.description, self.unit_family, self.default_value)
        }
    }
}

/// Creates a [`UnitCompileTime`], the compile-time metadata for a parameter whose value is
/// chosen among the units belonging to a single unit family.
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// const_unit!(description, unit_family)
/// const_unit!(description, unit_family, default = default_value)
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the parameter.
/// - `unit_family`: [`UnitFamilyId`] (from the `units` crate) family of units the value may
///   be selected from (e.g. `UnitFamilyId::Length`).
/// - `default_value` (optional): `&'static str` string id of the default unit (e.g.
///   `"u_length_meter"`), which must belong to `unit_family`. When omitted, the parameter
///   has no default.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::UnitCompileTime;
/// use datastore::prelude::*;
/// use units::UnitFamilyId;
///
/// const LENGTH_UNIT: UnitCompileTime = const_unit!(
///     "Preferred length unit",
///     UnitFamilyId::Length,
///     default = "u_length_meter"
/// );
/// assert_eq!(LENGTH_UNIT.unit_family(), UnitFamilyId::Length);
///
/// let _definition = LENGTH_UNIT.into_definition();
/// ```
#[macro_export]
macro_rules! const_unit {
    ($description:expr, $unit_family:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::UnitCompileTime::__new($description, $unit_family)
        }
    };
    ($description:expr, $unit_family:expr, default = $default_value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::UnitCompileTime::__new_with_default(
                $description,
                $unit_family,
                $default_value,
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
        let without_default =
            UnitCompileTime::__new(std::hint::black_box("Length"), UnitFamilyId::Length);
        let with_default = UnitCompileTime::__new_with_default(
            std::hint::black_box("Length"),
            UnitFamilyId::Length,
            "u_length_meter",
        );

        assert_eq!(without_default.unit_family(), UnitFamilyId::Length);
        assert_eq!(without_default.default_value(), "");
        assert_eq!(with_default.default_value(), "u_length_meter");
    }
}
