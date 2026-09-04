use crate::compile_time::ItemCompileTime;
use crate::compile_time::compile_time_common::assert_unique_keys;
use crate::definition::ParameterObjectDefinition;
use keys::parameter_key::ConstParameterKey;

/// Compile-time representation of a parameter object.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParameterObjectCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Keyed items contained in this compile-time container.
    items: &'static [(ConstParameterKey, ItemCompileTime)],
}

impl ParameterObjectCompileTime {
    /// Hidden backing constructor for `parameter_object_compile_time!(description, items)`.
    ///
    /// This is an implementation detail; call `parameter_object_compile_time!` instead.
    /// `description` names the top-level object and `items` is the ordered slice of
    /// `(ConstParameterKey, ItemCompileTime)` key/item pairs.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(
        description: &'static str,
        items: &'static [(ConstParameterKey, ItemCompileTime)],
    ) -> Self {
        assert_unique_keys!(items, "ParameterObjectCompileTime item keys must be unique");
        Self { description, items }
    }

    /// Returns the description.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Returns the keyed items.
    #[must_use]
    pub const fn items(&self) -> &'static [(ConstParameterKey, ItemCompileTime)] {
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
    pub fn keys(&self) -> impl Iterator<Item = ConstParameterKey> + '_ {
        self.items.iter().map(|(key, _)| *key)
    }

    /// Returns an iterator over the entries.
    pub fn iter(&self) -> impl Iterator<Item = &(ConstParameterKey, ItemCompileTime)> + '_ {
        self.items.iter()
    }

    /// Converts this compile-time parameter object into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> ParameterObjectDefinition {
        self.items
            .iter()
            .fold(
                ParameterObjectDefinition::builder(self.description),
                |builder, (key, item)| builder.with(*key, item.into_definition()),
            )
            .finish()
    }
}

/// Creates a [`ParameterObjectCompileTime`], the compile-time metadata for a top-level
/// object of parameter-scoped items, keyed by [`ConstParameterKey`].
///
/// Declaration order of `items` is preserved by [`ParameterObjectCompileTime::keys`],
/// [`ParameterObjectCompileTime::iter`], and
/// [`ParameterObjectCompileTime::into_definition`].
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// parameter_object_compile_time!(description, items)
/// parameter_object_compile_time!(description, [("key", item), ...])
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the object.
/// - `items`: `&'static [(ConstParameterKey, ItemCompileTime)]` ordered slice of key/item
///   pairs, typically built with `parameter_key!` and `item_compile_time!`.
///
/// # Examples
/// ```rust
/// use datastore::prelude::*;
///
/// const SETTINGS: ParameterObjectCompileTime = parameter_object_compile_time!(
///     "Parameters",
///     [(
///         "p_thickness",
///         item_compile_time!(number = number_compile_time!("Thickness", default = "1")),
///     )],
/// );
/// assert_eq!(SETTINGS.count(), 1);
///
/// let _definition = SETTINGS.into_definition();
/// ```
///
/// Duplicate item keys are rejected at compile time:
/// ```compile_fail
/// use datastore::prelude::*;
///
/// const SETTINGS: ParameterObjectCompileTime = parameter_object_compile_time!(
///     "Settings",
///     [
///         (
///             "p_project_name",
///             item_compile_time!(string = string_compile_time!("Project name")),
///         ),
///         (
///             "p_project_name",
///             item_compile_time!(string = string_compile_time!("Duplicate")),
///         ),
///     ],
/// );
/// ```
#[macro_export]
macro_rules! parameter_object_compile_time {
    ($description:expr, [$(($key:literal, $item:expr $(,)?)),* $(,)?] $(,)?) => {
        const {
            $crate::compile_time::ParameterObjectCompileTime::__new(
                $description,
                &[
                    $(($crate::prelude::parameter_key!($key), $item)),*
                ],
            )
        }
    };
    ($description:expr, $items:expr) => {
        const { $crate::compile_time::ParameterObjectCompileTime::__new($description, $items) }
    };
}
