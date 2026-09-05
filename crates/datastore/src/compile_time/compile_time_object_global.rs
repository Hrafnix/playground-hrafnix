use crate::compile_time::ItemCompileTime;
use crate::compile_time::compile_time_common::assert_unique_keys;
use crate::definition::GlobalObjectDefinition;
use keys::global_key::ConstGlobalKey;

/// Compile-time representation of a global object.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlobalObjectCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Keyed items contained in this compile-time container.
    items: &'static [(ConstGlobalKey, ItemCompileTime)],
}

impl GlobalObjectCompileTime {
    /// Hidden backing constructor for `const_global_object!(description, items)`.
    ///
    /// This is an implementation detail; call `const_global_object!` instead.
    /// `description` names the top-level object and `items` is the ordered slice of
    /// `(ConstGlobalKey, ItemCompileTime)` key/item pairs, typically built with the
    /// `global_key!` macro and `const_item!`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(
        description: &'static str,
        items: &'static [(ConstGlobalKey, ItemCompileTime)],
    ) -> Self {
        assert_unique_keys!(items, "GlobalObjectCompileTime item keys must be unique");
        Self { description, items }
    }

    /// Returns the description.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Returns the keyed items.
    #[must_use]
    pub const fn items(&self) -> &'static [(ConstGlobalKey, ItemCompileTime)] {
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
    pub fn keys(&self) -> impl Iterator<Item = ConstGlobalKey> + '_ {
        self.items.iter().map(|(key, _)| *key)
    }

    /// Returns an iterator over the entries.
    pub fn iter(&self) -> impl Iterator<Item = &(ConstGlobalKey, ItemCompileTime)> + '_ {
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
/// const_global_object!(description, items)
/// const_global_object!(description, [("key", item), ...])
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the object.
/// - `items`: `&'static [(ConstGlobalKey, ItemCompileTime)]` ordered slice of key/item
///   pairs, typically built with `global_key!` and `const_item!`.
///
/// # Examples
/// ```rust
/// use datastore::prelude::*;
///
/// const SETTINGS: GlobalObjectCompileTime = const_global_object!(
///     "Settings",
///     [(
///         "g_project_name",
///         const_item!(string = const_string!("Project name")),
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
/// const SETTINGS: GlobalObjectCompileTime = const_global_object!(
///     "Settings",
///     [
///         (
///             "g_project_name",
///             const_item!(string = const_string!("Project name")),
///         ),
///         (
///             "g_project_name",
///             const_item!(string = const_string!("Duplicate")),
///         ),
///     ],
/// );
/// ```
#[macro_export]
macro_rules! const_global_object {
    ($description:expr, [$(($key:literal, $item:expr $(,)?)),* $(,)?] $(,)?) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::GlobalObjectCompileTime::__new(
                $description,
                &[
                    $(($crate::prelude::global_key!($key), $item)),*
                ],
            )
        }
    };
    ($description:expr, $items:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::GlobalObjectCompileTime::__new($description, $items)
        }
    };
}
