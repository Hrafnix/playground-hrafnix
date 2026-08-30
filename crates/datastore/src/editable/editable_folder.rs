use crate::definition::FolderDefinition;
use crate::frozen::FolderFrozen;
use crate::traits::TreePrint;
use shareable_string::ShareableString;

/// Represents a folder data value in the editable data.
#[derive(Debug, Clone, PartialEq)]
pub struct FolderEditable {
    /// Definition metadata for this folder value.
    definition: FolderDefinition,
    /// Current value for this folder data, stored as a `ShareableString`.
    value: ShareableString,
}

impl FolderEditable {
    /// Creates a new `FolderEditable` instance from a given `FolderFrozen` value.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new(frozen_folder: &FolderFrozen) -> Self {
        Self {
            definition: frozen_folder.definition().clone(),
            value: frozen_folder.value(),
        }
    }

    /// Converts the current `FolderEditable` instance into a `FolderFrozen` instance.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn freeze(&self) -> FolderFrozen {
        FolderFrozen::new_from_editable(self)
    }

    /// Returns the value as a `ShareableString`.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the folder definition.
    #[must_use]
    pub const fn definition(&self) -> &FolderDefinition {
        &self.definition
    }

    /// Sets the value and updates the hash.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn set<S: Into<ShareableString>>(&mut self, value: S) {
        self.value = value.into();
    }
}

impl PartialEq<&FolderEditable> for FolderEditable {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&FolderEditable) -> bool {
        self == *other
    }
}

impl PartialEq<FolderEditable> for &FolderEditable {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &FolderEditable) -> bool {
        *self == other
    }
}

impl TreePrint for FolderEditable {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Folder - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.value,
        )
    }
}
