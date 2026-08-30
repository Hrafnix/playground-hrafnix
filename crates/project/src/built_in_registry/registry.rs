use crate::built_in_registry::BuiltInRegistryItem;
use crate::built_in_registry::signal::GAIN;
use keys::ConstComponentKey;
use phf::{Map, phf_map};

/// Registry of all built-in components.
#[derive(Debug)]
pub struct BuiltInRegistry {
    /// Components indexed by their stable identifiers.
    components: Map<&'static str, &'static BuiltInRegistryItem>,
}

impl BuiltInRegistry {
    /// Creates a registry from a component map.
    #[must_use]
    pub const fn new(components: Map<&'static str, &'static BuiltInRegistryItem>) -> Self {
        Self { components }
    }

    /// Returns the component with the supplied stable identifier.
    #[must_use]
    pub fn get(&self, id: ConstComponentKey) -> Option<&'static BuiltInRegistryItem> {
        self.components.get(id.as_str()).copied()
    }

    /// Returns the number of registered components.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.components.len()
    }

    /// Returns whether the registry contains no components.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Iterates over every registered component.
    pub fn iter(&self) -> impl Iterator<Item = &'static BuiltInRegistryItem> + '_ {
        self.components.values().copied()
    }
}

/// All built-in components in one map.
///
/// Duplicate identifiers are rejected by [`phf_map!`] at compile time.
pub static BUILT_IN_REGISTRY: BuiltInRegistry = BuiltInRegistry::new(phf_map! {
    /// Signal Components
    "gain" => &GAIN,
});

#[cfg(test)]
mod tests {
    use super::*;
    use keys::component_key;

    #[test]
    fn gain_is_registered() {
        let Some(gain) = BUILT_IN_REGISTRY.get(component_key!("gain")) else {
            panic!("gain must be registered");
        };

        assert_eq!(gain.id(), "gain");
        assert_eq!(gain.display_name(), "Gain");
        assert_eq!(gain.category().to_string(), "Signal");
        assert_eq!(gain.current().version(), 1);
        assert!(gain.previous().is_empty());
    }

    #[test]
    fn registry_exposes_all_components() {
        assert_eq!(BUILT_IN_REGISTRY.len(), 1);
        assert_eq!(BUILT_IN_REGISTRY.iter().count(), 1);
        assert!(!BUILT_IN_REGISTRY.is_empty());
    }
}
