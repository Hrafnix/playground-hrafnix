use crate::definition::FolderDefinition;

/// Compile-time representation of a folder parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FolderCompileTime {
    /// Human-readable description for this compile-time value.
    description: &'static str,
    /// Whether this value is treated as input.
    is_input: bool,
    /// Default value for this compile-time value.
    default_value: &'static str,
}

impl FolderCompileTime {
    /// Hidden backing constructor for `const_folder!(description, is_input)`.
    ///
    /// This is an implementation detail; call `const_folder!` instead.
    /// `description` names the folder, and `is_input` controls whether the folder
    /// is treated as an input that can be chosen in a folder picker and included
    /// when archiving the project.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(description: &'static str, is_input: bool) -> Self {
        Self {
            description,
            is_input,
            default_value: "",
        }
    }

    /// Hidden backing constructor for `const_folder!(description, is_input, default = default_value)`.
    ///
    /// This is an implementation detail; call `const_folder!` instead.
    /// `description` names the folder, `is_input` controls whether it is treated
    /// as an input folder, and `default_value` provides the default folder path.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_default(
        description: &'static str,
        is_input: bool,
        default_value: &'static str,
    ) -> Self {
        Self {
            description,
            is_input,
            default_value,
        }
    }

    /// Returns the description.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Returns whether this value is treated as input.
    #[must_use]
    pub const fn is_input(&self) -> bool {
        self.is_input
    }

    /// Returns the default value.
    #[must_use]
    pub const fn default_value(&self) -> &'static str {
        self.default_value
    }

    /// Converts this compile-time folder into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> FolderDefinition {
        if self.default_value.is_empty() {
            FolderDefinition::new(self.description, self.is_input)
        } else {
            FolderDefinition::new_with_default(self.description, self.is_input, self.default_value)
        }
    }
}

/// Creates a [`FolderCompileTime`], the compile-time metadata for a folder-picking
/// parameter.
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// const_folder!(description, is_input)
/// const_folder!(description, is_input, default = default_value)
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the parameter.
/// - `is_input`: `bool` whether the folder is bundled when the project is archived and
///   whether folder-picker dialogs are used to select it.
/// - `default_value` (optional): `&'static str` default folder path. When omitted, the
///   parameter has no default.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::FolderCompileTime;
/// use datastore::prelude::*;
///
/// const OUTPUT_FOLDER: FolderCompileTime =
///     const_folder!("Output folder", false, default = "out");
/// assert!(!OUTPUT_FOLDER.is_input());
/// assert_eq!(OUTPUT_FOLDER.default_value(), "out");
///
/// let _definition = OUTPUT_FOLDER.into_definition();
/// ```
#[macro_export]
macro_rules! const_folder {
    ($description:expr, $is_input:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::FolderCompileTime::__new($description, $is_input)
        }
    };
    ($description:expr, $is_input:expr, default = $default_value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::FolderCompileTime::__new_with_default(
                $description,
                $is_input,
                $default_value,
            )
        }
    };
}
