use crate::compile_time::ItemCompileTimeType;
use crate::compile_time::compile_time_common::assert_unique_keys;
use crate::definition::GlobalObjectDefinition;
use keys::global_key::ConstGlobalKey;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Compile-time representation of a global object.
pub struct GlobalObjectCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Keyed items contained in this compile-time container.
    items: &'static [(ConstGlobalKey, ItemCompileTimeType)],
}

impl GlobalObjectCompileTime {
    /// Hidden backing constructor for `global_object_compile_time!(description, items)`.
    ///
    /// This is an implementation detail; call `global_object_compile_time!` instead.
    /// `description` names the top-level object and `items` is the ordered slice of
    /// `(ConstGlobalKey, ItemCompileTimeType)` key/item pairs, typically built with the
    /// `global_key!` macro and `item_compile_time!`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(
        description: &'static str,
        items: &'static [(ConstGlobalKey, ItemCompileTimeType)],
    ) -> Self {
        assert_unique_keys!(items, "GlobalObjectCompileTime item keys must be unique");
        Self { description, items }
    }

    #[must_use]
    /// Returns the description.
    pub const fn description(&self) -> &'static str {
        self.description
    }
    #[must_use]
    /// Returns the keyed items.
    pub const fn items(&self) -> &'static [(ConstGlobalKey, ItemCompileTimeType)] {
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
    pub fn get(&self, key: &str) -> Option<&ItemCompileTimeType> {
        self.items
            .iter()
            .find_map(|(item_key, item)| (item_key.as_str() == key).then_some(item))
    }
    /// Returns an iterator over the keys.
    pub fn keys(&self) -> impl Iterator<Item = ConstGlobalKey> + '_ {
        self.items.iter().map(|(key, _)| *key)
    }
    /// Returns an iterator over the entries.
    pub fn iter(&self) -> impl Iterator<Item = &(ConstGlobalKey, ItemCompileTimeType)> + '_ {
        self.items.iter()
    }
    /// Converts this compile-time global object into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> GlobalObjectDefinition {
        self.items
            .iter()
            .fold(
                GlobalObjectDefinition::builder(self.description),
                |builder, (key, item)| builder.with(*key, item.into_definition()),
            )
            .finish()
    }
}

/// Creates a [`GlobalObjectCompileTime`], the compile-time metadata for a top-level object
/// of global-scoped items, keyed by [`ConstGlobalKey`].
///
/// Declaration order of `items` is preserved by [`GlobalObjectCompileTime::keys`],
/// [`GlobalObjectCompileTime::iter`], and [`GlobalObjectCompileTime::into_definition`].
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// global_object_compile_time!(description, items)
/// global_object_compile_time!(description, [("key", item), ...])
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the object.
/// - `items`: `&'static [(ConstGlobalKey, ItemCompileTimeType)]` ordered slice of key/item
///   pairs, typically built with `global_key!` and `item_compile_time!`.
/// - `"key"`: global-key string literal. In the inline form, each key is validated by
///   `global_key!` internally, so callers do not need to invoke that macro themselves.
/// - `item`: [`ItemCompileTimeType`] expression, typically built with `item_compile_time!`.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::GlobalObjectCompileTime;
/// use datastore::prelude::*;
///
/// const SETTINGS: GlobalObjectCompileTime = global_object_compile_time!(
///     "Settings",
///     [(
///         "g_project_name",
///         item_compile_time!(string = string_compile_time!("Project name")),
///     )],
/// );
/// assert_eq!(SETTINGS.count(), 1);
///
/// let _definition = SETTINGS.into_definition();
/// ```
///
/// Duplicate item keys are rejected at compile time:
/// ```compile_fail
/// use datastore::compile_time::GlobalObjectCompileTime;
/// use datastore::prelude::*;
///
/// const SETTINGS: GlobalObjectCompileTime = global_object_compile_time!(
///     "Settings",
///     [
///         (
///             "g_project_name",
///             item_compile_time!(string = string_compile_time!("Project name")),
///         ),
///         (
///             "g_project_name",
///             item_compile_time!(string = string_compile_time!("Duplicate")),
///         ),
///     ],
/// );
/// ```
#[macro_export]
macro_rules! global_object_compile_time {
    ($description:expr, [$(($key:literal, $item:expr $(,)?)),* $(,)?] $(,)?) => {
        const {
            $crate::compile_time::GlobalObjectCompileTime::__new(
                $description,
                &[
                    $(($crate::prelude::global_key!($key), $item)),*
                ],
            )
        }
    };
    ($description:expr, $items:expr) => {
        const { $crate::compile_time::GlobalObjectCompileTime::__new($description, $items) }
    };
}
