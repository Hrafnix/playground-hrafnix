use crate::definition::SeparatorDefinition;
use crate::frozen::SeparatorFrozen;
use crate::traits::TreePrint;

/// Represents a separator structural element in the editable data.
#[derive(Debug, Clone, PartialEq)]
pub struct SeparatorEditable {
    /// Definition metadata for this separator element.
    definition: SeparatorDefinition,
}

impl SeparatorEditable {
    /// Creates a new `SeparatorEditable` instance from a given `SeparatorFrozen` value.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new(frozen_separator: &SeparatorFrozen) -> Self {
        Self {
            definition: frozen_separator.definition().clone(),
        }
    }

    /// Converts the current `SeparatorEditable` instance into a `SeparatorFrozen` instance.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn freeze(&self) -> SeparatorFrozen {
        SeparatorFrozen::new_from_editable(self)
    }

    /// Returns a reference to the separator definition.
    #[must_use]
    pub const fn definition(&self) -> &SeparatorDefinition {
        &self.definition
    }
}

impl PartialEq<&SeparatorEditable> for SeparatorEditable {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&SeparatorEditable) -> bool {
        self == *other
    }
}

impl PartialEq<SeparatorEditable> for &SeparatorEditable {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &SeparatorEditable) -> bool {
        *self == other
    }
}

impl TreePrint for SeparatorEditable {
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
            "{}{}{} ({}) Separator",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
        )
    }
}
