use std::fmt;

/// Trait for types that can be printed as a tree for debugging.
pub trait TreePrint {
    /// Prints the object as a tree with the given label and prefix.
    fn tree_print(
        &self,
        f: &mut fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> fmt::Result;

    /// Helper to get the correct prefix for the next level.
    fn child_prefix(prefix: &str, last: bool) -> String {
        format!("{}{}", prefix, if last { "    " } else { "│   " })
    }

    /// Helper to get the branch character.
    fn branch_char(last: bool) -> &'static str {
        if last { "└── " } else { "├── " }
    }
}
