use crate::definition::FileDefinition;

/// Compile-time representation of a file parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileCompileTime {
    /// Human-readable description of this file parameter.
    description: &'static str,
    /// File-extension filter (e.g., `"*.csv"`) used in open-file dialogs.
    extension_filter: &'static str,
    /// Whether this file should be included when archiving the project and whether file dialogs should be used to select the file.
    is_input: bool,
    /// Default value for this file parameter.
    default_value: &'static str,
}

impl FileCompileTime {
    /// Hidden backing constructor for `const_file!(description, extension_filter, is_input)`.
    ///
    /// This is an implementation detail; call `const_file!` instead.
    /// `description` names the file parameter, `extension_filter` controls the
    /// dialog filter used to pick files, and `is_input` controls whether the file
    /// is treated as an input that can be chosen in a file picker and included
    /// when archiving the project.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(
        description: &'static str,
        extension_filter: &'static str,
        is_input: bool,
    ) -> Self {
        Self {
            description,
            extension_filter,
            is_input,
            default_value: "",
        }
    }

    /// Hidden backing constructor for `const_file!(description, extension_filter, is_input, default = default_value)`.
    ///
    /// This is an implementation detail; call `const_file!` instead.
    /// `description` names the file parameter, `extension_filter` controls the
    /// dialog filter used to pick files, `is_input` controls whether the file is
    /// treated as an input, and `default_value` provides the default file path.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new_with_default(
        description: &'static str,
        extension_filter: &'static str,
        is_input: bool,
        default_value: &'static str,
    ) -> Self {
        Self {
            description,
            extension_filter,
            is_input,
            default_value,
        }
    }

    /// Returns the extension filter.
    #[must_use]
    pub const fn extension_filter(&self) -> &'static str {
        self.extension_filter
    }

    /// Returns whether the file should be bundled when archived and whether file dialogs should be used to select the file.
    #[must_use]
    pub const fn is_input(&self) -> bool {
        self.is_input
    }

    /// Returns the description of the parameter.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Returns the default value of the parameter.
    #[must_use]
    pub const fn default_value(&self) -> &'static str {
        self.default_value
    }

    /// Converts this compile-time file into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> FileDefinition {
        if self.default_value.is_empty() {
            FileDefinition::new(self.description, self.extension_filter, self.is_input)
        } else {
            FileDefinition::new_with_default(
                self.description,
                self.extension_filter,
                self.is_input,
                self.default_value,
            )
        }
    }
}

/// Creates a [`FileCompileTime`], the compile-time metadata for a file-picking parameter.
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// const_file!(description, extension_filter, is_input)
/// const_file!(description, extension_filter, is_input, default = default_value)
/// ```
///
/// # Arguments
/// - `description`: `&'static str` human-readable description of the parameter.
/// - `extension_filter`: `&'static str` file-extension filter (e.g. `"*.csv"`) presented in
///   open-file dialogs.
/// - `is_input`: `bool` whether the file is bundled when the project is archived and
///   whether file-picker dialogs are used to select it.
/// - `default_value` (optional): `&'static str` default file path. When omitted, the
///   parameter has no default.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::FileCompileTime;
/// use datastore::prelude::*;
///
/// const INPUT_FILE: FileCompileTime =
///     const_file!("Input data file", "*.csv", true, default = "data.csv");
/// assert_eq!(INPUT_FILE.extension_filter(), "*.csv");
/// assert!(INPUT_FILE.is_input());
///
/// let _definition = INPUT_FILE.into_definition();
/// ```
#[macro_export]
macro_rules! const_file {
    ($description:expr, $extension_filter:expr, $is_input:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::FileCompileTime::__new($description, $extension_filter, $is_input)
        }
    };
    ($description:expr, $extension_filter:expr, $is_input:expr, default = $default_value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::FileCompileTime::__new_with_default(
                $description,
                $extension_filter,
                $is_input,
                $default_value,
            )
        }
    };
}
