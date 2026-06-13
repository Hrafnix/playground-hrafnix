use std::fmt;

/// Internal trait for common store operations related to hashing.
pub(crate) trait CommonStoreTraitInternal {
    /// Returns the current shared BLAKE3 hash.
    fn current_shared_hash(&self) -> [u8; 32];
    /// Recomputes the local BLAKE3 hash from the current data without syncing to shared storage.
    fn update_current_hash(&mut self);
    /// Syncs the shared hash with the current local hash, making changes visible to other handles.
    fn update_shared_hash(&mut self);
    /// Clears the current shared hash.
    fn clear_shared_hash(&mut self);
    /// Returns `true` if the local data has changed since the last sync with the store.
    fn has_changed(&self) -> bool;
    /// Returns `true` if the shared storage backing this value still exists (non-zero hash).
    fn is_valid(&self) -> bool;
}

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
    fn next_prefix(prefix: &str, last: bool) -> String {
        format!("{}{}", prefix, if last { "    " } else { "│   " })
    }

    /// Helper to get the branch character.
    fn branch_char(prefix: &str, last: bool) -> &'static str {
        if prefix.is_empty() {
            ""
        } else if last {
            "└── "
        } else {
            "├── "
        }
    }

    /// Returns a `TreeDisplay` for the given item.
    fn tree_display(&self, label: &str) -> TreeDisplay<'_, Self>
    where
        Self: Sized,
    {
        TreeDisplay {
            item: self,
            label: label.to_string(),
        }
    }
}

/// Wrapper for displaying a `TreePrint` object.
#[derive(Debug)]
pub struct TreeDisplay<'a, T: TreePrint> {
    /// The item to print.
    pub item: &'a T,
    /// The label for the root of the tree.
    pub label: String,
}

impl<'a, T: TreePrint> fmt::Display for TreeDisplay<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.item.tree_print(f, &self.label, "", true)
    }
}
