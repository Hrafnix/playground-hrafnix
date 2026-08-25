use crate::component::{ComponentDefinition, ComponentTypeId};
use crate::diagnostic::{Diagnostic, DiagnosticCategory, DiagnosticSeverity, EntityReference};
use crate::identity::ComponentId;
use std::collections::BTreeMap;

/// Registry insertion failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    /// A definition with this stable type ID is already registered.
    DuplicateTypeId(ComponentTypeId),
}

/// Deterministic catalog of installed built-in component definitions.
#[derive(Debug, Clone, Default)]
pub struct ComponentRegistry {
    /// Definitions ordered by stable type ID.
    definitions: BTreeMap<ComponentTypeId, ComponentDefinition>,
}

impl ComponentRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            definitions: BTreeMap::new(),
        }
    }

    /// Registers a built-in definition.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::DuplicateTypeId`] rather than replacing an
    /// installed definition implicitly.
    pub fn register(&mut self, definition: ComponentDefinition) -> Result<(), RegistryError> {
        let type_id = definition.type_id.clone();
        if self.definitions.contains_key(&type_id) {
            return Err(RegistryError::DuplicateTypeId(type_id));
        }
        self.definitions.insert(type_id, definition);
        Ok(())
    }

    /// Returns a registered definition by stable type ID.
    #[must_use]
    pub fn get(&self, type_id: &ComponentTypeId) -> Option<&ComponentDefinition> {
        self.definitions.get(type_id)
    }

    /// Returns registered definitions in stable type-ID order.
    pub fn iter(&self) -> impl Iterator<Item = &ComponentDefinition> {
        self.definitions.values()
    }

    /// Resolves a built-in reference or returns a stable validation diagnostic.
    ///
    /// # Errors
    ///
    /// Returns a diagnostic when the requested built-in is not installed.
    pub fn require(
        &self,
        type_id: &ComponentTypeId,
        instance_id: ComponentId,
    ) -> Result<&ComponentDefinition, Diagnostic> {
        self.get(type_id).ok_or_else(|| {
            Diagnostic::new(
                DiagnosticSeverity::Error,
                DiagnosticCategory::Validation,
                Some(EntityReference::Component(instance_id)),
                Some("component".into()),
                "simulation_registry_unknown_builtin",
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{ComponentRegistry, RegistryError};
    use crate::component::{
        ComponentCapabilities, ComponentCapability, ComponentDefinition, ComponentTypeId,
        SemanticVersion,
    };
    use crate::diagnostic::{DiagnosticCategory, DiagnosticSeverity, EntityReference};
    use crate::identity::ComponentId;

    /// Creates a minimal deterministic registry definition.
    fn definition(type_id: &str) -> ComponentDefinition {
        ComponentDefinition {
            type_id: ComponentTypeId::new(type_id).unwrap(),
            version: SemanticVersion {
                major: 1,
                minor: 0,
                patch: 0,
            },
            display_name: "Gain".into(),
            category: "Signal/Math".into(),
            aliases: vec![],
            tags: vec!["deterministic".into()],
            documentation: "Multiplies a signal.".into(),
            parameters: vec![],
            ports: vec![],
            capabilities: ComponentCapabilities::new([
                ComponentCapability::DirectFeedthrough,
                ComponentCapability::Deterministic,
            ]),
            deprecation: None,
        }
    }

    #[test]
    fn registers_and_resolves_definition_without_implicit_replacement() {
        let mut registry = ComponentRegistry::new();
        let definition = definition("signal.gain");
        let type_id = definition.type_id.clone();

        assert_eq!(registry.register(definition.clone()), Ok(()));
        assert_eq!(registry.get(&type_id), Some(&definition));
        assert_eq!(
            registry.register(definition),
            Err(RegistryError::DuplicateTypeId(type_id))
        );
    }

    #[test]
    fn unknown_builtin_returns_stable_entity_diagnostic() {
        let type_id = ComponentTypeId::new("signal.missing").unwrap();
        let component_id = ComponentId::from_raw(19);
        let diagnostic = ComponentRegistry::new()
            .require(&type_id, component_id)
            .unwrap_err();

        assert_eq!(diagnostic.severity(), DiagnosticSeverity::Error);
        assert_eq!(diagnostic.category(), DiagnosticCategory::Validation);
        assert_eq!(
            diagnostic.entity(),
            Some(EntityReference::Component(component_id))
        );
        assert_eq!(
            diagnostic.message_key().as_str(),
            "simulation_registry_unknown_builtin"
        );
    }
}
