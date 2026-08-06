use crate::definition::FileDefinition;
use crate::frozen::FileFrozen;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a file data value in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileEditable {
    definition: FileDefinition,
    value: ShareableString,
}

impl FileEditable {
    /// Creates a new `FileEditable` instance from a given `FileFrozen` value.
    #[must_use]
    pub fn new(frozen_file: &FileFrozen) -> Self {
        Self {
            definition: frozen_file.definition().clone(),
            value: frozen_file.value(),
        }
    }

    /// Converts the current `FileEditable` instance into a `FileFrozen` instance.
    #[must_use]
    pub fn freeze(&self) -> FileFrozen {
        FileFrozen::new_from_editable(self)
    }

    /// Returns the value as a `ShareableString`.
    #[must_use]
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the file definition.
    #[must_use]
    pub const fn definition(&self) -> &FileDefinition {
        &self.definition
    }

    /// Sets the value and updates the hash.
    pub fn set<S: Into<ShareableString>>(&mut self, value: S) {
        self.value = value.into();
    }
}

impl PartialEq<&FileEditable> for FileEditable {
    fn eq(&self, other: &&FileEditable) -> bool {
        self == *other
    }
}

impl PartialEq<FileEditable> for &FileEditable {
    fn eq(&self, other: &FileEditable) -> bool {
        *self == other
    }
}

impl TreePrint for FileEditable {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) File - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
