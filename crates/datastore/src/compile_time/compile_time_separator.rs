use crate::definition::SeparatorDefinition;

/// Compile-time representation of a separator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeparatorCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
}

impl SeparatorCompileTime {
    /// Hidden backing constructor for `separator_compile_time!(description)`.
    ///
    /// This is an implementation detail; call `separator_compile_time!` instead.
    /// `description` names the layout-only visual divider.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(description: &'static str) -> Self {
        Self { description }
    }

    /// Returns the description.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Converts this compile-time separator into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> SeparatorDefinition {
        SeparatorDefinition::new(self.description)
    }
}

/// Creates a [`SeparatorCompileTime`], the compile-time metadata for a layout-only visual
/// divider. It stores no value and exists purely to group related items in a UI.
///
/// Expansion is wrapped in a `const` block, so `description` must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// separator_compile_time!(description)
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the separator.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::SeparatorCompileTime;
/// use datastore::prelude::*;
///
/// const DIVIDER: SeparatorCompileTime = separator_compile_time!("General settings");
/// assert_eq!(DIVIDER.description(), "General settings");
///
/// let _definition = DIVIDER.into_definition();
/// ```
#[macro_export]
macro_rules! separator_compile_time {
    ($description:expr) => {
        const { $crate::compile_time::SeparatorCompileTime::__new($description) }
    };
}
