use crate::compile_time::ItemCompileTimeType;
use crate::compile_time::compile_time_common::assert_unique_keys;
use crate::definition::ParameterObjectDefinition;
use keys::parameter_key::ConstParameterKey;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Compile-time representation of a parameter object.
pub struct ParameterObjectCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Keyed items contained in this compile-time container.
    items: &'static [(ConstParameterKey, ItemCompileTimeType)],
}

impl ParameterObjectCompileTime {
    /// Hidden backing constructor for `parameter_object_compile_time!(description, items)`.
    ///
    /// This is an implementation detail; call `parameter_object_compile_time!` instead.
    /// `description` names the top-level object and `items` is the ordered slice of
    /// `(ConstParameterKey, ItemCompileTimeType)` key/item pairs, typically built with the
    /// `parameter_key!` macro and `item_compile_time!`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(
        description: &'static str,
        items: &'static [(ConstParameterKey, ItemCompileTimeType)],
    ) -> Self {
        assert_unique_keys!(items, "ParameterObjectCompileTime item keys must be unique");
        Self { description, items }
    }

    #[must_use]
    /// Returns the description.
    pub const fn description(&self) -> &'static str {
        self.description
    }
    #[must_use]
    /// Returns the keyed items.
    pub const fn items(&self) -> &'static [(ConstParameterKey, ItemCompileTimeType)] {
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
    pub fn keys(&self) -> impl Iterator<Item = ConstParameterKey> + '_ {
        self.items.iter().map(|(key, _)| *key)
    }
    /// Returns an iterator over the entries.
    pub fn iter(&self) -> impl Iterator<Item = &(ConstParameterKey, ItemCompileTimeType)> + '_ {
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
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the object.
/// - `items`: `&'static [(ConstParameterKey, ItemCompileTimeType)]` ordered slice of
///   key/item pairs, typically built with the `parameter_key!` macro and
///   `item_compile_time!`.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::{ItemCompileTimeType, ParameterObjectCompileTime};
/// use datastore::prelude::*;
///
/// const PARAMETERS: &[(ConstParameterKey, ItemCompileTimeType)] = &[(
///     parameter_key!("p_thickness"),
///     item_compile_time!(number = number_compile_time!("Thickness", default = "1")),
/// )];
/// const SETTINGS: ParameterObjectCompileTime =
///     parameter_object_compile_time!("Parameters", PARAMETERS);
/// assert_eq!(SETTINGS.count(), 1);
///
/// let _definition = SETTINGS.into_definition();
/// ```
///
/// Duplicate item keys are rejected at compile time:
/// ```compile_fail
/// use datastore::compile_time::ParameterObjectCompileTime;
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
    ($description:expr, $items:expr) => {
        const { $crate::compile_time::ParameterObjectCompileTime::__new($description, $items) }
    };
}
