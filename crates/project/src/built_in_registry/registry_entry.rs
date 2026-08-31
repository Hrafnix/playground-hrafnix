use crate::BuiltInComponentTrait;
use crate::built_in_registry::Category;
use keys::ConstComponentKey;
use std::collections::HashMap;

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
    current: Box<dyn BuiltInComponentTrait>,
    /// Definitions superseded by the current definition.
    previous: HashMap<u8, Box<dyn BuiltInComponentTrait>>,
}

impl BuiltInRegistryItem {
    /// Creates a registry item from a current definition and its previous definitions.
    #[must_use]
    pub fn new(
        id: ConstComponentKey,
        display_name: &'static str,
        category: Category,
        current: Box<dyn BuiltInComponentTrait>,
        previous: HashMap<u8, Box<dyn BuiltInComponentTrait>>,
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
    pub fn current(&self) -> &dyn BuiltInComponentTrait {
        &*self.current
    }

    /// Returns the definitions superseded by the current definition.
    #[must_use]
    pub const fn previous(&self) -> &HashMap<u8, Box<dyn BuiltInComponentTrait>> {
        &self.previous
    }
}
