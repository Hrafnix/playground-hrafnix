use crate::compile_time::ItemCompileTime;
use crate::compile_time::compile_time_common::assert_unique_keys;
use crate::definition::VariableObjectDefinition;
use keys::variable_key::ConstVariableKey;

/// Compile-time representation of a variable object.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VariableObjectCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Keyed items contained in this compile-time container.
    items: &'static [(ConstVariableKey, ItemCompileTime)],
}

impl VariableObjectCompileTime {
    /// Hidden backing constructor for `const_variable_object!(description, items)`.
    ///
    /// This is an implementation detail; call `const_variable_object!` instead.
    /// `description` names the top-level object and `items` is the ordered slice of
    /// `(ConstVariableKey, ItemCompileTime)` key/item pairs.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(
        description: &'static str,
        items: &'static [(ConstVariableKey, ItemCompileTime)],
    ) -> Self {
        assert_unique_keys!(items, "VariableObjectCompileTime item keys must be unique");
        Self { description, items }
    }

    /// Returns the description.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Returns the keyed items.
    #[must_use]
    pub const fn items(&self) -> &'static [(ConstVariableKey, ItemCompileTime)] {
        self.items
    }

    /// Returns the number of entries.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the given value is present.
    #[must_use]
    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Returns the value associated with the given key.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&ItemCompileTime> {
        self.items
            .iter()
            .find_map(|(item_key, item)| (item_key.as_str() == key).then_some(item))
    }

    /// Returns an iterator over the keys.
    pub fn keys(&self) -> impl Iterator<Item = ConstVariableKey> + '_ {
        self.items.iter().map(|(key, _)| *key)
    }

    /// Returns an iterator over the entries.
    pub fn iter(&self) -> impl Iterator<Item = &(ConstVariableKey, ItemCompileTime)> + '_ {
        self.items.iter()
    }

    /// Converts this compile-time variable object into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> VariableObjectDefinition {
        self.items
            .iter()
            .fold(
                VariableObjectDefinition::builder(self.description),
                |builder, (key, item)| builder.with(*key, item.into_definition()),
            )
            .finish()
    }
}

/// Creates a [`VariableObjectCompileTime`], the compile-time metadata for a top-level
/// object of variable-scoped items, keyed by [`ConstVariableKey`].
///
/// Declaration order of `items` is preserved by [`VariableObjectCompileTime::keys`],
/// [`VariableObjectCompileTime::iter`], and
/// [`VariableObjectCompileTime::into_definition`].
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// const_variable_object!(description, items)
/// const_variable_object!(description, [("key", item), ...])
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the object.
/// - `items`: `&'static [(ConstVariableKey, ItemCompileTime)]` ordered slice of key/item
///   pairs, typically built with `variable_key!` and `const_item!`.
///
/// # Examples
/// ```rust
/// use datastore::prelude::*;
///
/// const RESULTS: VariableObjectCompileTime = const_variable_object!(
///     "Results",
///     [(
///         "v_result",
///         const_item!(number = const_number!("Result")),
///     )],
/// );
/// assert_eq!(RESULTS.count(), 1);
///
/// let _definition = RESULTS.into_definition();
/// ```
///
/// Duplicate item keys are rejected at compile time:
/// ```compile_fail
/// use datastore::prelude::*;
///
/// const SETTINGS: VariableObjectCompileTime = const_variable_object!(
///     "Settings",
///     [
///         (
///             "v_project_name",
///             const_item!(string = const_string!("Project name")),
///         ),
///         (
///             "v_project_name",
///             const_item!(string = const_string!("Duplicate")),
///         ),
///     ],
/// );
/// ```
#[macro_export]
macro_rules! const_variable_object {
    ($description:expr, [$(($key:literal, $item:expr $(,)?)),* $(,)?] $(,)?) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::VariableObjectCompileTime::__new(
                $description,
                &[
                    $(($crate::prelude::variable_key!($key), $item)),*
                ],
            )
        }
    };
    ($description:expr, $items:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::VariableObjectCompileTime::__new($description, $items)
        }
    };
}
