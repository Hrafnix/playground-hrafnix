use crate::component::{ComponentDefinition, ComponentFactory, ComponentTypeId, SemanticVersion};
use crate::diagnostic::{Diagnostic, DiagnosticCategory, DiagnosticSeverity, EntityReference};
use crate::identity::ComponentId;
use std::collections::BTreeMap;
use std::sync::Arc;

/// Registry insertion failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    /// A definition with this stable type ID and version is already registered.
    DuplicateVersion(ComponentTypeId, SemanticVersion),
}

/// Deterministic catalog of installed built-in component definitions.
#[derive(Debug, Clone, Default)]
pub struct ComponentRegistry {
    /// Definitions ordered by stable type ID and semantic version.
    definitions: BTreeMap<(ComponentTypeId, SemanticVersion), ComponentDefinition>,
    /// Runtime factories installed for executable definitions.
    factories: BTreeMap<(ComponentTypeId, SemanticVersion), Arc<dyn ComponentFactory>>,
}

impl ComponentRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            definitions: BTreeMap::new(),
            factories: BTreeMap::new(),
        }
    }

    /// Registers a built-in definition.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::DuplicateVersion`] rather than replacing an
    /// installed version implicitly.
    pub fn register(&mut self, definition: ComponentDefinition) -> Result<(), RegistryError> {
        let key = (definition.type_id.clone(), definition.version);
        if self.definitions.contains_key(&key) {
            return Err(RegistryError::DuplicateVersion(key.0, key.1));
        }
        self.definitions.insert(key, definition);
        Ok(())
    }

    /// Registers executable metadata and its runtime factory atomically.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::DuplicateVersion`] without changing the registry
    /// when the stable type ID and version are already installed.
    pub fn register_with_factory(
        &mut self,
        definition: ComponentDefinition,
        factory: Arc<dyn ComponentFactory>,
    ) -> Result<(), RegistryError> {
        let key = (definition.type_id.clone(), definition.version);
        self.register(definition)?;
        self.factories.insert(key, factory);
        Ok(())
    }

    /// Returns the latest registered definition for a stable type ID.
    #[must_use]
    pub fn get(&self, type_id: &ComponentTypeId) -> Option<&ComponentDefinition> {
        self.definitions
            .range(
                (type_id.clone(), SemanticVersion::MIN)..=(type_id.clone(), SemanticVersion::MAX),
            )
            .next_back()
            .map(|(_, definition)| definition)
    }

    /// Returns an exact registered definition by stable type ID and version.
    #[must_use]
    pub fn get_version(
        &self,
        type_id: &ComponentTypeId,
        version: SemanticVersion,
    ) -> Option<&ComponentDefinition> {
        self.definitions.get(&(type_id.clone(), version))
    }

    /// Returns registered definitions in stable type-ID order.
    pub fn iter(&self) -> impl Iterator<Item = &ComponentDefinition> {
        self.definitions.values()
    }

    /// Returns the runtime factory installed for the latest built-in version.
    #[must_use]
    pub fn factory(&self, type_id: &ComponentTypeId) -> Option<&Arc<dyn ComponentFactory>> {
        self.factories
            .range(
                (type_id.clone(), SemanticVersion::MIN)..=(type_id.clone(), SemanticVersion::MAX),
            )
            .next_back()
            .map(|(_, factory)| factory)
    }

    /// Returns the runtime factory installed for an exact built-in version.
    #[must_use]
    pub fn factory_version(
        &self,
        type_id: &ComponentTypeId,
        version: SemanticVersion,
    ) -> Option<&Arc<dyn ComponentFactory>> {
        self.factories.get(&(type_id.clone(), version))
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

    /// Resolves an exact built-in version or returns a stable validation diagnostic.
    ///
    /// # Errors
    ///
    /// Returns a diagnostic when the requested built-in version is not installed.
    pub fn require_version(
        &self,
        type_id: &ComponentTypeId,
        version: SemanticVersion,
        instance_id: ComponentId,
    ) -> Result<&ComponentDefinition, Diagnostic> {
        self.get_version(type_id, version).ok_or_else(|| {
            Diagnostic::new(
                DiagnosticSeverity::Error,
                DiagnosticCategory::Validation,
                Some(EntityReference::Component(instance_id)),
                Some("component".into()),
                "simulation_registry_unknown_builtin_version",
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{ComponentRegistry, RegistryError};
    use crate::component::{
        ComponentBehavior, ComponentCapabilities, ComponentCapability, ComponentDefinition,
        ComponentFactory, ComponentTypeId, ComponentUpdate, RuntimeValues, SemanticVersion,
        StepContext,
    };
    use crate::diagnostic::{DiagnosticCategory, DiagnosticSeverity, EntityReference};
    use crate::identity::ComponentId;
    use std::sync::Arc;

    #[derive(Debug)]
    struct TestFactory;

    impl ComponentFactory for TestFactory {
        fn create(
            &self,
            _component_id: ComponentId,
            _parameters: &RuntimeValues,
        ) -> Result<Box<dyn ComponentBehavior>, crate::diagnostic::Diagnostic> {
            Ok(Box::new(TestBehavior))
        }
    }

    #[derive(Debug)]
    struct TestBehavior;

    impl ComponentBehavior for TestBehavior {
        fn initialize(
            &self,
            _context: StepContext,
            _parameters: &RuntimeValues,
        ) -> Result<ComponentUpdate, crate::diagnostic::Diagnostic> {
            Ok(ComponentUpdate::default())
        }

        fn evaluate(
            &self,
            _context: StepContext,
            _parameters: &RuntimeValues,
            _inputs: &RuntimeValues,
            _state: &RuntimeValues,
        ) -> Result<ComponentUpdate, crate::diagnostic::Diagnostic> {
            Ok(ComponentUpdate::default())
        }
    }

    /// Creates a minimal deterministic registry definition.
    fn definition(type_id: &str, major: u16) -> ComponentDefinition {
        ComponentDefinition {
            type_id: ComponentTypeId::new(type_id).unwrap(),
            version: SemanticVersion {
                major,
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
            appearance: Default::default(),
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
        let definition = definition("signal.gain", 1);
        let type_id = definition.type_id.clone();

        assert_eq!(registry.register(definition.clone()), Ok(()));
        assert_eq!(registry.get(&type_id), Some(&definition));
        assert_eq!(
            registry.register(definition),
            Err(RegistryError::DuplicateVersion(
                type_id,
                SemanticVersion {
                    major: 1,
                    minor: 0,
                    patch: 0,
                }
            ))
        );
    }

    #[test]
    fn retains_multiple_versions_and_resolves_latest_or_exact() {
        let mut registry = ComponentRegistry::new();
        let version_one = definition("signal.gain", 1);
        let version_two = definition("signal.gain", 2);
        let type_id = version_one.type_id.clone();

        registry.register(version_two.clone()).unwrap();
        registry.register(version_one.clone()).unwrap();

        assert_eq!(registry.get(&type_id), Some(&version_two));
        assert_eq!(
            registry.get_version(&type_id, version_one.version),
            Some(&version_one)
        );
        assert_eq!(registry.iter().count(), 2);
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

    #[test]
    fn executable_registration_retains_factory_without_breaking_metadata_lookup() {
        let mut registry = ComponentRegistry::new();
        let definition = definition("signal.gain", 1);
        let type_id = definition.type_id.clone();

        registry
            .register_with_factory(definition.clone(), Arc::new(TestFactory))
            .unwrap();

        assert_eq!(registry.get(&type_id), Some(&definition));
        assert!(registry.factory(&type_id).is_some());
    }
}
