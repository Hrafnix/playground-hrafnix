use crate::{BuiltInComponentDefinition, IconDefinition, PortDefinition};
use datastore::prelude::{ParameterObjectFrozen, ShareableString, VariableObjectFrozen};
use keys::ConstComponentKey;

/// An active component initialized from a component definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    /// Stable identifier shared by all versions of the component.
    id: ConstComponentKey,
    /// Version of the component definition used to create this component.
    version: u32,
    /// Human-readable component name.
    display_name: ShareableString,
    /// Current frozen parameter values.
    parameters: ParameterObjectFrozen,
    /// Current frozen variable values.
    variables: VariableObjectFrozen,
    /// Component icon.
    icon: IconDefinition,
    /// Component ports.
    ports: &'static [PortDefinition],
}

impl Component {
    /// Returns the component identifier.
    #[must_use]
    pub const fn id(&self) -> ConstComponentKey {
        self.id
    }

    /// Returns the component definition version.
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }

    /// Returns the component display name.
    #[must_use]
    pub const fn display_name(&self) -> &ShareableString {
        &self.display_name
    }

    /// Returns the component's frozen parameters.
    #[must_use]
    pub const fn parameters(&self) -> &ParameterObjectFrozen {
        &self.parameters
    }

    /// Returns the component's frozen variables.
    #[must_use]
    pub const fn variables(&self) -> &VariableObjectFrozen {
        &self.variables
    }

    /// Returns the component icon.
    #[must_use]
    pub const fn icon(&self) -> IconDefinition {
        self.icon
    }

    /// Returns the component ports.
    #[must_use]
    pub const fn ports(&self) -> &'static [PortDefinition] {
        self.ports
    }
}

impl From<&BuiltInComponentDefinition> for Component {
    fn from(definition: &BuiltInComponentDefinition) -> Self {
        Self {
            id: definition.id(),
            version: definition.version(),
            display_name: definition.display_name().into(),
            parameters: ParameterObjectFrozen::new(definition.parameters().into_definition()),
            variables: VariableObjectFrozen::new(definition.variables().into_definition()),
            icon: definition.icon(),
            ports: definition.ports(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::built_in_registry::signal::gain_v1::gain_definition::GAIN_V1;

    #[test]
    fn built_in_definition_creates_active_component_with_default_state() {
        let component = Component::from(&GAIN_V1);

        assert_eq!(component.id(), "gain");
        assert_eq!(component.version(), 1);
        assert_eq!(component.display_name(), "Gain");
        assert_eq!(component.parameters().iter().count(), 1);
        assert!(component.parameters().get("p_gain").is_some());
        assert_eq!(component.variables().iter().count(), 0);
        assert_eq!(component.icon(), GAIN_V1.icon());
        assert_eq!(component.ports(), GAIN_V1.ports());
    }
}
