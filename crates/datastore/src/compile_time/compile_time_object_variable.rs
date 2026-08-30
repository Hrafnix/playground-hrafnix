use crate::compile_time::ItemCompileTime;
use crate::compile_time::compile_time_common::assert_unique_keys;
use crate::definition::VariableObjectDefinition;
use keys::variable_key::ConstVariableKey;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Compile-time representation of a variable object.
pub struct VariableObjectCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Keyed items contained in this compile-time container.
    items: &'static [(ConstVariableKey, ItemCompileTime)],
}

impl VariableObjectCompileTime {
    /// Hidden backing constructor for `variable_object_compile_time!(description, items)`.
    ///
    /// This is an implementation detail; call `variable_object_compile_time!` instead.
    /// `description` names the top-level object and `items` is the ordered slice of
    /// `(ConstVariableKey, ItemCompileTimeType)` key/item pairs.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(
        description: &'static str,
        items: &'static [(ConstVariableKey, ItemCompileTime)],
    ) -> Self {
        assert_unique_keys!(items, "VariableObjectCompileTime item keys must be unique");
        Self { description, items }
    }

    #[must_use]
    /// Returns the description.
    pub const fn description(&self) -> &'static str {
        self.description
    }
    #[must_use]
    /// Returns the keyed items.
    pub const fn items(&self) -> &'static [(ConstVariableKey, ItemCompileTime)] {
        self.items
    }
    #[must_use]
    /// Returns the number of entries.
    pub const fn count(&self) -> usize {
        self.items.len()
    }
    #[must_use]
    /// Returns true if the given value is present.
    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
    #[must_use]
    /// Returns the value associated with the given key.
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
/// variable_object_compile_time!(description, items)
/// variable_object_compile_time!(description, [("key", item), ...])
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the object.
/// - `items`: `&'static [(ConstVariableKey, ItemCompileTimeType)]` ordered slice of
///   key/item pairs.
/// - `"key"`: variable-key string literal. In the inline form, each key is validated by
///   `variable_key!` internally.
/// - `item`: [`ItemCompileTime`] expression, typically built with `item_compile_time!`.
///
/// # Examples
/// ```rust
/// use datastore::prelude::*;
///
/// const RESULTS: VariableObjectCompileTime = variable_object_compile_time!(
///     "Results",
///     [(
///         "v_result",
///         item_compile_time!(number = number_compile_time!("Result")),
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
/// const SETTINGS: VariableObjectCompileTime = variable_object_compile_time!(
///     "Settings",
///     [
///         (
///             "v_project_name",
///             item_compile_time!(string = string_compile_time!("Project name")),
///         ),
///         (
///             "v_project_name",
///             item_compile_time!(string = string_compile_time!("Duplicate")),
///         ),
///     ],
/// );
/// ```
#[macro_export]
macro_rules! variable_object_compile_time {
    ($description:expr, [$(($key:literal, $item:expr $(,)?)),* $(,)?] $(,)?) => {
        const {
            $crate::compile_time::VariableObjectCompileTime::__new(
                $description,
                &[
                    $(($crate::prelude::variable_key!($key), $item)),*
                ],
            )
        }
    };
    ($description:expr, $items:expr) => {
        const { $crate::compile_time::VariableObjectCompileTime::__new($description, $items) }
    };
}
