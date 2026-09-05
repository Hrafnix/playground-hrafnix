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
    /// Hidden backing constructor for `const_parameter_object!(description, items)`.
    ///
    /// This is an implementation detail; call `const_parameter_object!` instead.
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
/// const_parameter_object!(description, items)
/// const_parameter_object!(description, [("key", item), ...])
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the object.
/// - `items`: `&'static [(ConstParameterKey, ItemCompileTime)]` ordered slice of key/item
///   pairs, typically built with `parameter_key!` and `const_item!`.
///
/// # Examples
/// ```rust
/// use datastore::prelude::*;
///
/// const SETTINGS: ParameterObjectCompileTime = const_parameter_object!(
///     "Parameters",
///     [(
///         "p_thickness",
///         const_item!(number = const_number!("Thickness", default = "1")),
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
/// const SETTINGS: ParameterObjectCompileTime = const_parameter_object!(
///     "Settings",
///     [
///         (
///             "p_project_name",
///             const_item!(string = const_string!("Project name")),
///         ),
///         (
///             "p_project_name",
///             const_item!(string = const_string!("Duplicate")),
///         ),
///     ],
/// );
/// ```
#[macro_export]
macro_rules! const_parameter_object {
    ($description:expr, [$(($key:literal, $item:expr $(,)?)),* $(,)?] $(,)?) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ParameterObjectCompileTime::__new(
                $description,
                &[
                    $(($crate::prelude::parameter_key!($key), $item)),*
                ],
            )
        }
    };
    ($description:expr, $items:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ParameterObjectCompileTime::__new($description, $items)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{const_item, const_string, parameter_key};

    #[test]
    #[allow(clippy::disallowed_methods)]
    fn hidden_constructor_runs_at_runtime() {
        const ITEMS: &[(ConstParameterKey, ItemCompileTime)] = &[(
            parameter_key!("p_name"),
            const_item!(string = const_string!("Name")),
        )];
        let object = ParameterObjectCompileTime::__new(std::hint::black_box("Parameters"), ITEMS);

        assert_eq!(object.description(), "Parameters");
        assert_eq!(object.items(), ITEMS);
    }

    #[test]
    #[should_panic(expected = "ParameterObjectCompileTime item keys must be unique")]
    fn parameter_object_compile_time_rejects_duplicate_keys() {
        const DUPLICATES: &[(ConstParameterKey, ItemCompileTime)] = &[
            (
                parameter_key!("p_duplicate"),
                const_item!(string = const_string!("First")),
            ),
            (
                parameter_key!("p_duplicate"),
                const_item!(string = const_string!("Second")),
            ),
        ];
        #[allow(clippy::disallowed_methods)]
        let _ = ParameterObjectCompileTime::__new(std::hint::black_box("Duplicates"), DUPLICATES);
    }
}
