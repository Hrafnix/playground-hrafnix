use crate::BuiltInComponentDefinition;
use crate::built_in_registry::Category;
use keys::ConstComponentKey;

/// A built-in component's registry metadata and available definitions.
#[derive(Debug)]
pub struct BuiltInRegistryItem {
    /// Stable identifier shared by all versions of the component.
    id: ConstComponentKey,
    /// Human-readable component name.
    display_name: &'static str,
    /// Category under which the component is listed.
    category: Category,
    /// Current component definition.
    current: &'static BuiltInComponentDefinition,
    /// Definitions superseded by the current definition.
    previous: &'static [BuiltInRegistryItem],
}

impl BuiltInRegistryItem {
    /// Creates a registry item from a current definition and its previous definitions.
    #[must_use]
    pub const fn new(
        id: ConstComponentKey,
        display_name: &'static str,
        category: Category,
        current: &'static BuiltInComponentDefinition,
        previous: &'static [BuiltInRegistryItem],
    ) -> Self {
        Self {
            id,
            display_name,
            category,
            current,
            previous,
        }
    }

    /// Returns the component identifier.
    #[must_use]
    pub const fn id(&self) -> ConstComponentKey {
        self.id
    }

    /// Returns the component display name.
    #[must_use]
    pub const fn display_name(&self) -> &'static str {
        self.display_name
    }

    /// Returns the component category.
    #[must_use]
    pub const fn category(&self) -> &Category {
        &self.category
    }

    /// Returns the current component definition.
    #[must_use]
    pub const fn current(&self) -> &'static BuiltInComponentDefinition {
        self.current
    }

    /// Returns the definitions superseded by the current definition.
    #[must_use]
    pub const fn previous(&self) -> &'static [BuiltInRegistryItem] {
        self.previous
    }
}
