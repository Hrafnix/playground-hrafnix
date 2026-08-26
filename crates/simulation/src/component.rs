use crate::diagnostic::Diagnostic;
use crate::identity::{ComponentId, RunId};
use crate::parameter::ParameterValueType;
use crate::value::RuntimeValue;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use units::UnitId;

/// Stable, namespaced identity of a built-in component type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ComponentTypeId(ShareableString);

impl ComponentTypeId {
    /// Creates a component type ID.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidComponentTypeId`] when `value` is empty or contains
    /// whitespace. Namespaced values such as `signal.gain` are supported.
    pub fn new(value: impl Into<ShareableString>) -> Result<Self, InvalidComponentTypeId> {
        let value = value.into();
        if value.as_str().is_empty() || value.as_str().chars().any(char::is_whitespace) {
            return Err(InvalidComponentTypeId);
        }
        Ok(Self(value))
    }

    /// Returns the stable string representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for ComponentTypeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Invalid built-in component type identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidComponentTypeId;

/// Semantic version used by built-in component definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SemanticVersion {
    /// Breaking-change version.
    pub major: u16,
    /// Backward-compatible feature version.
    pub minor: u16,
    /// Backward-compatible fix version.
    pub patch: u16,
}

impl SemanticVersion {
    /// Lowest representable semantic version.
    pub const MIN: Self = Self {
        major: u16::MIN,
        minor: u16::MIN,
        patch: u16::MIN,
    };

    /// Highest representable semantic version.
    pub const MAX: Self = Self {
        major: u16::MAX,
        minor: u16::MAX,
        patch: u16::MAX,
    };
}

/// Direction of a signal port in a component interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortDirection {
    /// Value enters the component.
    Input,
    /// Value leaves the component.
    Output,
}

/// Parameter metadata exposed by a built-in or custom component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterDefinition {
    /// Stable key within the component type.
    pub key: ShareableString,
    /// User-facing name.
    pub display_name: ShareableString,
    /// Parameter documentation.
    pub description: ShareableString,
    /// Value shape accepted after expression evaluation.
    pub value_type: ParameterValueType,
    /// Calculator expression used when an instance has no override.
    pub default_expression: ShareableString,
}

/// Port metadata exposed by a built-in or custom component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortDefinition {
    /// Stable key within the component type.
    pub key: ShareableString,
    /// User-facing name.
    pub display_name: ShareableString,
    /// Port documentation.
    pub description: ShareableString,
    /// Input or output direction.
    pub direction: PortDirection,
    /// Runtime value shape.
    pub value_type: ParameterValueType,
    /// Canonical unit when the port carries a scalar quantity.
    pub unit: Option<UnitId>,
    /// Whether an input connection is required.
    pub required: bool,
}

/// One scheduling or runtime trait declared by a component definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentCapability {
    /// Component owns state across timesteps.
    Stateful,
    /// Current outputs depend on current inputs.
    DirectFeedthrough,
    /// Component output depends directly on simulation time.
    TimeDependent,
    /// Component explicitly breaks a direct-feedthrough cycle.
    AlgebraicLoopBreak,
    /// Component is expected to produce deterministic results.
    Deterministic,
}

/// Extensible set of scheduling and runtime traits.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ComponentCapabilities(pub BTreeSet<ComponentCapability>);

impl ComponentCapabilities {
    /// Creates a capability set from an iterator of flags.
    #[must_use]
    pub fn new(capabilities: impl IntoIterator<Item = ComponentCapability>) -> Self {
        Self(capabilities.into_iter().collect())
    }

    /// Returns whether this set contains `capability`.
    #[must_use]
    pub fn contains(&self, capability: ComponentCapability) -> bool {
        self.0.contains(&capability)
    }
}

/// Optional migration guidance for a deprecated component type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deprecation {
    /// Explanation shown to users.
    pub message: ShareableString,
    /// Preferred replacement when one exists.
    pub replacement: Option<ComponentTypeId>,
}

/// Immutable metadata registered for one built-in component type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentDefinition {
    /// Stable built-in type identity.
    pub type_id: ComponentTypeId,
    /// Interface and behavior version.
    pub version: SemanticVersion,
    /// User-facing name.
    pub display_name: ShareableString,
    /// Library category path.
    pub category: ShareableString,
    /// Alternate search names.
    pub aliases: Vec<ShareableString>,
    /// Search and capability tags.
    pub tags: Vec<ShareableString>,
    /// Component documentation.
    pub documentation: ShareableString,
    /// Public parameters in display order.
    pub parameters: Vec<ParameterDefinition>,
    /// Public ports in display order.
    pub ports: Vec<PortDefinition>,
    /// Runtime and scheduling capabilities.
    pub capabilities: ComponentCapabilities,
    /// Optional deprecation guidance.
    pub deprecation: Option<Deprecation>,
}

/// Stable-keyed runtime parameters, inputs, outputs, or owned state.
pub type RuntimeValues = BTreeMap<ShareableString, RuntimeValue>;

/// Immutable context supplied during initialization and stepping.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StepContext {
    /// Identity of the current run.
    pub run_id: RunId,
    /// Current simulation-grid time.
    pub time: f64,
    /// Fixed transition duration.
    pub timestep: f64,
    /// Current sample index, starting at zero.
    pub step_index: u64,
}

/// Outputs and owned state computed before an atomic runtime commit.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ComponentUpdate {
    /// Complete output values produced by the component.
    pub outputs: RuntimeValues,
    /// Complete next owned state.
    pub next_state: RuntimeValues,
}

/// Configured component algorithm whose mutable state remains runtime-owned.
pub trait ComponentBehavior: fmt::Debug + Send + Sync {
    /// Computes initialized outputs and state.
    ///
    /// # Errors
    ///
    /// Returns a runtime diagnostic when initialization cannot produce valid values.
    fn initialize(
        &self,
        context: StepContext,
        parameters: &RuntimeValues,
    ) -> Result<ComponentUpdate, Diagnostic>;

    /// Computes one update without mutating committed runtime state.
    ///
    /// # Errors
    ///
    /// Returns a runtime diagnostic without exposing partial outputs or state.
    fn evaluate(
        &self,
        context: StepContext,
        parameters: &RuntimeValues,
        inputs: &RuntimeValues,
        state: &RuntimeValues,
    ) -> Result<ComponentUpdate, Diagnostic>;

    /// Finalizes one completed or otherwise terminal component run.
    ///
    /// # Errors
    ///
    /// Returns a runtime diagnostic when finalization fails.
    fn finalize(
        &self,
        _context: StepContext,
        _parameters: &RuntimeValues,
        _state: &RuntimeValues,
    ) -> Result<(), Diagnostic> {
        Ok(())
    }
}

/// Factory for one configured runtime behavior instance.
pub trait ComponentFactory: fmt::Debug + Send + Sync {
    /// Creates a fresh behavior from evaluated configuration values.
    ///
    /// # Errors
    ///
    /// Returns a component-scoped diagnostic when configuration cannot be compiled.
    fn create(
        &self,
        component_id: ComponentId,
        parameters: &RuntimeValues,
    ) -> Result<Box<dyn ComponentBehavior>, Diagnostic>;
}

#[cfg(test)]
mod tests {
    use super::ComponentTypeId;

    #[test]
    fn component_type_id_accepts_namespace_and_rejects_whitespace() {
        assert_eq!(
            ComponentTypeId::new("signal.gain").unwrap().as_str(),
            "signal.gain"
        );
        assert!(ComponentTypeId::new("signal gain").is_err());
    }
}
