use crate::definition::TabDefinition;
use crate::frozen::TabFrozen;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};

/// Represents a tab structural element in the editable data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TabEditable {
    /// Definition metadata for this tab element.
    definition: TabDefinition,
}

impl TabEditable {
    /// Creates a new `TabEditable` instance from a given `TabFrozen` value.
    #[must_use]
    #[hotpath::measure]
    pub fn new(frozen_tab: &TabFrozen) -> Self {
        Self {
            definition: frozen_tab.definition().clone(),
        }
    }

    /// Converts the current `TabEditable` instance into a `TabFrozen` instance.
    #[must_use]
    #[hotpath::measure]
    pub fn freeze(&self) -> TabFrozen {
        TabFrozen::new_from_editable(self)
    }

    /// Returns a reference to the tab definition.
    #[must_use]
    pub const fn definition(&self) -> &TabDefinition {
        &self.definition
    }
}

impl PartialEq<&TabEditable> for TabEditable {
    #[hotpath::measure]
    fn eq(&self, other: &&TabEditable) -> bool {
        self == *other
    }
}

impl PartialEq<TabEditable> for &TabEditable {
    #[hotpath::measure]
    fn eq(&self, other: &TabEditable) -> bool {
        *self == other
    }
}

impl TreePrint for TabEditable {
    #[hotpath::measure]
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Tab",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
        )
    }
}
