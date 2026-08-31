use crate::{BuiltInComponentDefinition, IconDefinition, PortDefinition};
use datastore::prelude::{
    ParameterObjectFrozen, SharedStringStore, VariableObjectFrozen, editable_set_value,
};
use expression_engine::prelude::{
    ExpressionEngine, Message, ParameterObjectComputedData, ParameterObjectInputData,
    VariableObjectComputedData, VariableObjectInputData,
};
use keys::ConstComponentKey;

/// An active component initialized from a component definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    /// Static definition used to create this component.
    definition: &'static BuiltInComponentDefinition,
    /// Current frozen parameter values.
    parameters: ParameterObjectFrozen,
    /// Current frozen variable values.
    variables: VariableObjectFrozen,
}

impl Component {
    /// Creates an editable component with the definition's default state.
    #[must_use]
    pub fn new(definition: &'static BuiltInComponentDefinition) -> Self {
        Self {
            definition,
            parameters: ParameterObjectFrozen::new(definition.parameters().into_definition()),
            variables: VariableObjectFrozen::new(definition.variables().into_definition()),
        }
    }

    /// Returns the component identifier.
    #[must_use]
    pub const fn id(&self) -> ConstComponentKey {
        self.definition.id()
    }

    /// Returns the component definition version.
    #[must_use]
    pub const fn version(&self) -> u8 {
        self.definition.version()
    }

    /// Returns the component display name.
    #[must_use]
    pub const fn display_name(&self) -> &'static str {
        self.definition.display_name()
    }

    /// Returns the component's frozen parameters.
    #[must_use]
    pub const fn parameters(&self) -> &ParameterObjectFrozen {
        &self.parameters
    }

    /// Returns a mutable reference to the component's frozen parameters.
    #[must_use]
    pub const fn parameters_mut(&mut self) -> &mut ParameterObjectFrozen {
        &mut self.parameters
    }

    /// Sets a parameter value or expression while preserving an immutable snapshot.
    ///
    /// # Errors
    /// Returns a datastore message when the key is unknown or cannot accept a scalar value.
    pub fn set_parameter_expression(&mut self, key: &str, expression: &str) -> Result<(), Message> {
        let mut editable = self.parameters.thaw();
        editable_set_value(&mut editable, key, expression)?;
        self.parameters = editable.freeze();
        Ok(())
    }

    /// Returns the component's frozen variables.
    #[must_use]
    pub const fn variables(&self) -> &VariableObjectFrozen {
        &self.variables
    }

    /// Returns a mutable reference to the component's frozen variables.
    #[must_use]
    pub const fn variables_mut(&mut self) -> &mut VariableObjectFrozen {
        &mut self.variables
    }

    /// Sets a variable value or expression while preserving an immutable snapshot.
    ///
    /// # Errors
    /// Returns a datastore message when the key is unknown or cannot accept a scalar value.
    pub fn set_variable_expression(&mut self, key: &str, expression: &str) -> Result<(), Message> {
        let mut editable = self.variables.thaw();
        editable_set_value(&mut editable, key, expression)?;
        self.variables = editable.freeze();
        Ok(())
    }

    /// Returns the component icon.
    #[must_use]
    pub const fn icon(&self) -> IconDefinition {
        self.definition.icon()
    }

    /// Returns the component ports.
    #[must_use]
    pub const fn ports(&self) -> &'static [PortDefinition] {
        self.definition.ports()
    }

    /// Returns a new `Component` instance with laundered parameters and variables.
    pub fn launder(&mut self, store: &mut SharedStringStore) {
        self.parameters = self.parameters.launder(store);
        self.variables = self.variables.launder(store);
    }

    /// Evaluates the component's parameter and variable expressions.
    ///
    /// # Errors
    ///
    /// Returns expression evaluation messages when either object cannot be evaluated.
    pub fn evaluate(
        &self,
        engine: &ExpressionEngine,
    ) -> Result<(ParameterObjectComputedData, VariableObjectComputedData), Vec<Message>> {
        let parameters =
            engine.evaluate_parameters(&ParameterObjectInputData::new(&self.parameters))?;
        let variables = engine
            .evaluate_variables(&parameters, &VariableObjectInputData::new(&self.variables))?;
        Ok((parameters, variables))
    }
}

/// Common access to editable component state.
pub trait ComponentTrait {
    /// Returns the component identifier.
    fn id(&self) -> ConstComponentKey;

    /// Returns the component definition version.
    fn version(&self) -> u8;

    /// Returns the component display name.
    fn display_name(&self) -> &'static str;

    /// Returns the component's frozen parameters.
    fn parameters(&self) -> &ParameterObjectFrozen;

    /// Returns a mutable reference to the component's frozen parameters.
    fn parameters_mut(&mut self) -> &mut ParameterObjectFrozen;

    /// Returns the component's frozen variables.
    fn variables(&self) -> &VariableObjectFrozen;

    /// Returns a mutable reference to the component's frozen variables.
    fn variables_mut(&mut self) -> &mut VariableObjectFrozen;

    /// Returns the component icon.
    fn icon(&self) -> IconDefinition;

    /// Returns the component ports.
    fn ports(&self) -> &'static [PortDefinition];
}

impl From<&'static BuiltInComponentDefinition> for Component {
    fn from(definition: &'static BuiltInComponentDefinition) -> Self {
        Self::new(definition)
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
