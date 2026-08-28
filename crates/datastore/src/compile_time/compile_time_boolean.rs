use crate::definition::BooleanDefinition;
use crate::traits::TreePrint;

/// Compile-time representation of a boolean parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BooleanCompileTime {
    /// Human-readable description of this boolean parameter.
    description: &'static str,
    /// Label displayed when the value is `true`.
    true_description: &'static str,
    /// Label displayed when the value is `false`.
    false_description: &'static str,
    /// Default value for this boolean parameter.
    default_value: &'static str,
}

impl BooleanCompileTime {
    /// Hidden backing constructor for `boolean_compile_time!(description)`.
    ///
    /// This is an implementation detail; call `boolean_compile_time!` instead.
    /// `description` names the parameter and this arm creates a boolean with no
    /// default value.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(description: &'static str) -> Self {
        Self {
            description,
            true_description: "True",
            false_description: "False",
            default_value: "",
        }
    }

    /// Hidden backing constructor for `boolean_compile_time!(description, default = default_value)`.
    ///
    /// This is an implementation detail; call `boolean_compile_time!` instead.
    /// `description` names the parameter and `default_value` selects the initial
    /// `true`/`false` state.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_default(description: &'static str, default_value: bool) -> Self {
        Self {
            description,
            true_description: "True",
            false_description: "False",
            default_value: if default_value { "true" } else { "false" },
        }
    }

    /// Returns the IDs of the choices.
    #[must_use]
    pub const fn ids() -> [&'static str; 2] {
        ["true", "false"]
    }

    /// Returns the descriptions of the choices.
    #[must_use]
    pub const fn descriptions(&self) -> [&'static str; 2] {
        [self.true_description, self.false_description]
    }

    /// Returns the description of the parameter.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Returns the default value of the parameter.
    #[must_use]
    pub const fn default_value(&self) -> &'static str {
        self.default_value
    }

    /// Converts this compile-time boolean into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> BooleanDefinition {
        if self.default_value.is_empty() {
            BooleanDefinition::new(self.description)
        } else {
            BooleanDefinition::new_with_default(self.description, self.default_value == "true")
        }
    }
}

impl PartialEq<&BooleanCompileTime> for BooleanCompileTime {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&BooleanCompileTime) -> bool {
        self == *other
    }
}

impl PartialEq<BooleanCompileTime> for &BooleanCompileTime {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &BooleanCompileTime) -> bool {
        *self == other
    }
}

impl TreePrint for BooleanCompileTime {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Boolean - default: \"{}\" [{}]",
            prefix,
            Self::branch_char(last),
            label,
            self.description,
            self.default_value,
            [self.true_description, self.false_description]
                .iter()
                .map(|choice| format!("{} ({})", choice.to_lowercase(), choice))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

/// Creates a [`BooleanCompileTime`], the compile-time metadata for a two-state
/// (`true`/`false`) parameter.
///
/// The resulting value always exposes exactly two choice ids, `"true"` and `"false"`,
/// labeled `"True"` and `"False"`. Call [`BooleanCompileTime::into_definition`] to convert
/// it into a runtime `BooleanDefinition`.
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// boolean_compile_time!(description)
/// boolean_compile_time!(description, default = default_value)
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the parameter.
/// - `default_value` (optional): `bool` value used as the default when the parameter is
///   left unset. When omitted, the parameter has no default.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::BooleanCompileTime;
/// use datastore::prelude::*;
///
/// const ENABLE_FEATURE: BooleanCompileTime =
///     boolean_compile_time!("Enable feature", default = true);
/// assert_eq!(ENABLE_FEATURE.description(), "Enable feature");
/// assert_eq!(ENABLE_FEATURE.default_value(), "true");
///
/// let _definition = ENABLE_FEATURE.into_definition();
/// ```
#[macro_export]
macro_rules! boolean_compile_time {
    ($description:expr) => {
        const { $crate::compile_time::BooleanCompileTime::__new($description) }
    };
    ($description:expr, default = $default_value:expr) => {
        const {
            $crate::compile_time::BooleanCompileTime::__new_with_default(
                $description,
                $default_value,
            )
        }
    };
}
