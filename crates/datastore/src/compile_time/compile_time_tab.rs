use crate::definition::TabDefinition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Compile-time representation of a tab.
pub struct TabCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
}

impl TabCompileTime {
    /// Hidden backing constructor for `tab_compile_time!(description)`.
    ///
    /// This is an implementation detail; call `tab_compile_time!` instead.
    /// `description` names the layout-only tab heading.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(description: &'static str) -> Self {
        Self { description }
    }

    #[must_use]
    /// Returns the description.
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Converts this compile-time tab into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> TabDefinition {
        TabDefinition::new(self.description)
    }
}

/// Creates a [`TabCompileTime`], the compile-time metadata for a layout-only tab heading.
/// It stores no value and exists purely to group related items under a named tab in a UI.
///
/// Expansion is wrapped in a `const` block, so `description` must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// tab_compile_time!(description)
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the tab.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::TabCompileTime;
/// use datastore::prelude::*;
///
/// const GENERAL: TabCompileTime = tab_compile_time!("General");
/// assert_eq!(GENERAL.description(), "General");
///
/// let _definition = GENERAL.into_definition();
/// ```
#[macro_export]
macro_rules! tab_compile_time {
    ($description:expr) => {
        const { $crate::compile_time::TabCompileTime::__new($description) }
    };
}
