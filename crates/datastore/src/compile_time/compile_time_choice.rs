use crate::compile_time::compile_time_common::const_str_eq;
use crate::definition::{ChoiceDefinition, ChoiceItemDefinition};
use keys::store_key::ConstStoreKey;

/// Compile-time representation of a single choice item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChoiceItemCompileTime {
    /// Unique key identifying this choice item.
    id: ConstStoreKey,
    /// Human-readable label for this choice item.
    description: &'static str,
}

impl ChoiceItemCompileTime {
    /// Hidden backing constructor for `const_choice_item!("id", description)`.
    ///
    /// This is an implementation detail; call `const_choice_item!` instead.
    /// The macro validates its string literal as a [`ConstStoreKey`] before passing it
    /// as `id`. The id uniquely identifies the option and is compared against the
    /// parameter's stored value to determine the active choice. `description` is the
    /// human-readable label shown for the option.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(id: ConstStoreKey, description: &'static str) -> Self {
        Self { id, description }
    }

    /// Returns the ID of the choice item.
    #[must_use]
    pub const fn id(&self) -> ConstStoreKey {
        self.id
    }

    /// Returns the description of the choice item.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Converts this compile-time choice item into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> ChoiceItemDefinition {
        ChoiceItemDefinition::new(self.id, self.description)
    }
}

/// Compile-time representation of a choice parameter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChoiceCompileTime {
    /// Human-readable description of this choice parameter.
    description: &'static str,
    /// An ordered list of valid choices.
    choices: &'static [ChoiceItemCompileTime],
    /// Default value for this choice parameter.
    default_value: &'static str,
}

impl ChoiceCompileTime {
    /// Hidden backing constructor for `const_choice!(description, choices)`.
    ///
    /// This is an implementation detail; call `const_choice!` instead.
    /// `description` names the parameter and `choices` is the ordered slice of
    /// [`ChoiceItemCompileTime`] options. This arm creates a choice with no default
    /// value selected.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(
        description: &'static str,
        choices: &'static [ChoiceItemCompileTime],
    ) -> Self {
        Self::assert_unique_ids(choices);
        Self {
            description,
            choices,
            default_value: "",
        }
    }

    /// Hidden backing constructor for `const_choice!(description, choices, default = default_value)`.
    ///
    /// This is an implementation detail; call `const_choice!` instead.
    /// `description` names the parameter, `choices` is the ordered slice of
    /// [`ChoiceItemCompileTime`] options, and `default_value` is the id of the choice
    /// selected by default (must match one of the ids in `choices`).
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_default(
        description: &'static str,
        choices: &'static [ChoiceItemCompileTime],
        default_value: &'static str,
    ) -> Self {
        Self::assert_unique_ids(choices);
        Self {
            description,
            choices,
            default_value,
        }
    }

    /// Panics when two choices use the same id.
    const fn assert_unique_ids(choices: &[ChoiceItemCompileTime]) {
        let mut unchecked = choices;
        while let [choice, remaining @ ..] = unchecked {
            let mut candidates = remaining;
            while let [candidate, rest @ ..] = candidates {
                assert!(
                    !const_str_eq(choice.id.as_str(), candidate.id.as_str()),
                    "ChoiceCompileTime choice ids must be unique"
                );
                candidates = rest;
            }
            unchecked = remaining;
        }
    }

    /// Returns a reference to the list of choices.
    #[must_use]
    pub const fn choices(&self) -> &'static [ChoiceItemCompileTime] {
        self.choices
    }

    /// Returns true if the given value is a valid choice.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn contains(&self, value: &str) -> bool {
        self.choices
            .iter()
            .any(|choice| choice.id.as_str() == value)
    }

    /// Returns an iterator over the IDs of the choices.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn ids(&self) -> impl Iterator<Item = ConstStoreKey> + '_ {
        self.choices.iter().map(|choice| choice.id)
    }

    /// Returns an iterator over the descriptions of the choices.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn descriptions(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.choices.iter().map(|choice| choice.description)
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

    /// Converts this compile-time choice into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> ChoiceDefinition {
        let choices: Vec<ChoiceItemDefinition> = self
            .choices
            .iter()
            .map(|choice| choice.into_definition())
            .collect();

        if self.default_value.is_empty() {
            ChoiceDefinition::new(self.description, choices)
        } else {
            ChoiceDefinition::new_with_default(self.description, choices, self.default_value)
        }
    }
}

/// Creates a [`ChoiceItemCompileTime`], the compile-time metadata for a single selectable
/// option of a `const_choice!` parameter.
///
/// Expansion is wrapped in a `const` block, so both arguments must be const-compatible
/// (`'static`) expressions; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// const_choice_item!("id", description)
/// ```
///
/// # Arguments
/// - `"id"`: string literal uniquely identifying this option. The macro validates it as a
///   [`ConstStoreKey`] at compile time, so callers do not need to invoke `store_key!`.
///   The id is compared against the parameter's stored value to determine the active choice.
/// - `description`: `&'static str` human-readable label shown for this option.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::ChoiceItemCompileTime;
/// use datastore::prelude::*;
///
/// const SMALL: ChoiceItemCompileTime = const_choice_item!("small", "Small");
/// assert_eq!(SMALL.description(), "Small");
/// ```
#[macro_export]
macro_rules! const_choice_item {
    ($id:literal, $description:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ChoiceItemCompileTime::__new(
                $crate::prelude::store_key!($id),
                $description,
            )
        }
    };
}

/// Creates a [`ChoiceCompileTime`], the compile-time metadata for a single-select parameter
/// backed by an ordered list of [`ChoiceItemCompileTime`] options.
///
/// The declaration order of `choices` is preserved by [`ChoiceCompileTime::ids`],
/// [`ChoiceCompileTime::descriptions`], and [`ChoiceCompileTime::into_definition`].
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// const_choice!(description, choices)
/// const_choice!(description, choices, default = default_value)
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the parameter.
/// - `choices`: `&'static [ChoiceItemCompileTime]` ordered slice of options, typically built
///   with [`const_choice_item!`].
/// - `default_value` (optional): `&'static str` id of the choice selected by default; must
///   match one of the ids in `choices`. When omitted, the parameter has no default.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::{ChoiceCompileTime, ChoiceItemCompileTime};
/// use datastore::prelude::*;
///
/// const SIZES: &[ChoiceItemCompileTime] = &[
///     const_choice_item!("small", "Small"),
///     const_choice_item!("large", "Large"),
/// ];
/// const SIZE: ChoiceCompileTime = const_choice!("Size", SIZES, default = "small");
/// assert_eq!(SIZE.default_value(), "small");
/// assert!(SIZE.contains("large"));
///
/// let _definition = SIZE.into_definition();
/// ```
///
/// Duplicate choice ids are rejected at compile time:
/// ```compile_fail
/// use datastore::compile_time::{ChoiceCompileTime, ChoiceItemCompileTime};
/// use datastore::prelude::*;
///
/// const SIZES: &[ChoiceItemCompileTime] = &[
///     const_choice_item!("small", "Small"),
///     const_choice_item!("small", "Duplicate"),
/// ];
/// const SIZE: ChoiceCompileTime = const_choice!("Size", SIZES);
/// ```
#[macro_export]
macro_rules! const_choice {
    ($description:expr, $choices:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ChoiceCompileTime::__new($description, $choices)
        }
    };
    ($description:expr, $choices:expr, default = $default_value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ChoiceCompileTime::__new_with_default(
                $description,
                $choices,
                $default_value,
            )
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::store_key;

    #[test]
    #[allow(clippy::disallowed_methods)]
    fn hidden_constructors_run_at_runtime() {
        let choices = Box::leak(Box::new([
            ChoiceItemCompileTime::__new(store_key!("first"), std::hint::black_box("First")),
            ChoiceItemCompileTime::__new(store_key!("second"), std::hint::black_box("Second")),
        ]));
        let item = ChoiceItemCompileTime::__new(
            store_key!("runtime_choice"),
            std::hint::black_box("Runtime choice"),
        );
        let without_default = ChoiceCompileTime::__new(std::hint::black_box("Choice"), choices);
        let with_default = ChoiceCompileTime::__new_with_default(
            std::hint::black_box("Defaulted choice"),
            choices,
            "second",
        );

        assert_eq!(item.id(), store_key!("runtime_choice"));
        assert_eq!(item.description(), "Runtime choice");
        assert_eq!(without_default.choices(), choices);
        assert_eq!(without_default.default_value(), "");
        assert_eq!(with_default.default_value(), "second");
    }

    #[test]
    #[should_panic(expected = "ChoiceCompileTime choice ids must be unique")]
    fn choice_compile_time_rejects_duplicate_ids() {
        const DUPLICATES: &[ChoiceItemCompileTime] = &[
            const_choice_item!("duplicate", "First"),
            const_choice_item!("duplicate", "Second"),
        ];
        #[allow(clippy::disallowed_methods)]
        let _ = ChoiceCompileTime::__new(std::hint::black_box("Duplicates"), DUPLICATES);
    }
}
