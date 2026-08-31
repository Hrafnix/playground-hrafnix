use crate::built_in_registry::BuiltInRegistryItem;
use crate::built_in_registry::signal::{ADD, CONSTANT, DELAY, GAIN};
use crate::built_in_registry::translational::{FIXED_BOUNDARY, MASS, SPRING};
use keys::ConstComponentKey;
use phf::{Map, phf_map};
use std::sync::LazyLock;

/// Registry of all built-in components.
#[derive(Debug)]
pub struct BuiltInRegistry {
    /// Components indexed by their stable identifiers.
    components: Map<&'static str, &'static LazyLock<BuiltInRegistryItem>>,
}

impl BuiltInRegistry {
    /// Creates a registry from a component map.
    #[must_use]
    pub const fn new(
        components: Map<&'static str, &'static LazyLock<BuiltInRegistryItem>>,
    ) -> Self {
        Self { components }
    }

    /// Returns the component with the supplied stable identifier.
    #[must_use]
    pub fn get(&self, id: ConstComponentKey) -> Option<&'static BuiltInRegistryItem> {
        self.components.get(id.as_str()).map(|item| &***item)
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
        self.components.values().map(|item| &***item)
    }
}

/// All built-in components in one map.
///
/// Duplicate identifiers are rejected by [`phf_map!`] at compile time.
pub static BUILT_IN_REGISTRY: BuiltInRegistry = BuiltInRegistry::new(phf_map! {
    /// Signal Components
    "add" => &ADD,
    "constant" => &CONSTANT,
    "delay" => &DELAY,
    "gain" => &GAIN,
    /// Translational Components
    "translational_fixed_boundary" => &FIXED_BOUNDARY,
    "translational_mass" => &MASS,
    "translational_spring" => &SPRING,
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
        assert_eq!(gain.current().definition().version(), 1);
        assert!(gain.previous().is_empty());
    }

    #[test]
    fn signal_library_exposes_component_schemas() {
        let Some(constant) = BUILT_IN_REGISTRY.get(component_key!("constant")) else {
            panic!("constant must be registered");
        };
        assert_eq!(constant.category().to_string(), "Signal/Sources");
        assert_eq!(constant.current().definition().parameters().count(), 1);
        assert_eq!(constant.current().definition().ports().len(), 1);

        let Some(add) = BUILT_IN_REGISTRY.get(component_key!("add")) else {
            panic!("add must be registered");
        };
        assert_eq!(add.category().to_string(), "Signal/Math");
        assert_eq!(add.current().definition().ports().len(), 3);

        let Some(delay) = BUILT_IN_REGISTRY.get(component_key!("delay")) else {
            panic!("delay must be registered");
        };
        assert_eq!(delay.category().to_string(), "Signal/Control");
        assert_eq!(delay.current().definition().variables().count(), 1);
    }

    #[test]
    fn registry_exposes_all_components() {
        assert_eq!(BUILT_IN_REGISTRY.len(), 7);
        assert_eq!(BUILT_IN_REGISTRY.iter().count(), 7);
        assert!(!BUILT_IN_REGISTRY.is_empty());
    }
}
