use crate::definition::StringDefinition;

/// Compile-time representation of a string parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Default value for this compile-time value.
    default_value: &'static str,
}

impl StringCompileTime {
    /// Hidden backing constructor for `string_compile_time!(description)`.
    ///
    /// This is an implementation detail; call `string_compile_time!` instead.
    /// `description` names the parameter and this arm creates a string with no
    /// default value.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(description: &'static str) -> Self {
        Self {
            description,
            default_value: "",
        }
    }

    /// Hidden backing constructor for `string_compile_time!(description, default = default_value)`.
    ///
    /// This is an implementation detail; call `string_compile_time!` instead.
    /// `description` names the parameter and `default_value` supplies the default
    /// text returned when the parameter is unset.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_default(
        description: &'static str,
        default_value: &'static str,
    ) -> Self {
        Self {
            description,
            default_value,
        }
    }

    /// Returns the description.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Returns the default value.
    #[must_use]
    pub const fn default_value(&self) -> &'static str {
        self.default_value
    }

    /// Converts this compile-time string into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> StringDefinition {
        if self.default_value.is_empty() {
            StringDefinition::new(self.description)
        } else {
            StringDefinition::new_with_default(self.description, self.default_value)
        }
    }
}

/// Creates a [`StringCompileTime`], the compile-time metadata for a free-form text
/// parameter.
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// string_compile_time!(description)
/// string_compile_time!(description, default = default_value)
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the parameter.
/// - `default_value` (optional): `&'static str` default text value. When omitted, the
///   parameter has no default.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::StringCompileTime;
/// use datastore::prelude::*;
///
/// const NAME: StringCompileTime = string_compile_time!("Name", default = "Untitled");
/// assert_eq!(NAME.default_value(), "Untitled");
///
/// let _definition = NAME.into_definition();
/// ```
#[macro_export]
macro_rules! string_compile_time {
    ($description:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::StringCompileTime::__new($description)
        }
    };
    ($description:expr, default = $default_value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::StringCompileTime::__new_with_default(
                $description,
                $default_value,
            )
        }
    };
}
