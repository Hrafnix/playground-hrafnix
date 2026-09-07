//! Shared domain types used by model and component definition files.

use std::error::Error;
use std::fmt::{self, Display, Formatter};

use datastore::prelude::{ParameterObjectFrozen, VariableObjectFrozen};

/// An error returned when a simulation file invariant is violated.
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// An identifier does not match `[a-z][0-9a-z_]*`.
    InvalidIdentifier {
        /// The rejected identifier.
        value: String,
    },
    /// A source digest is not 64 lowercase hexadecimal characters.
    InvalidSourceBlake3Digest {
        /// The rejected digest.
        value: String,
    },
    /// A port coordinate is non-finite or outside the inclusive range `0..=1`.
    InvalidNormalizedCoordinate {
        /// The rejected coordinate.
        value: f64,
    },
    /// A connection line has no midpoint.
    EmptyConnectionLine,
    /// A collection contains the same identifier more than once.
    DuplicateIdentifier {
        /// The collection in which the duplicate occurred.
        kind: IdentifierCollection,
        /// The duplicated identifier.
        identifier: Identifier,
    },
    /// A connection refers to a component that is not in the containing file.
    UnknownConnectionComponent {
        /// The component identifier used by the endpoint.
        identifier: Identifier,
    },
}

impl Display for ValidationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { value } => {
                write!(
                    formatter,
                    "invalid identifier `{value}`; expected [a-z][0-9a-z_]*"
                )
            }
            Self::InvalidSourceBlake3Digest { value } => write!(
                formatter,
                "invalid BLAKE3 digest `{value}`; expected 64 lowercase hexadecimal characters"
            ),
            Self::InvalidNormalizedCoordinate { value } => {
                write!(
                    formatter,
                    "invalid normalized coordinate `{value}`; expected 0..=1"
                )
            }
            Self::EmptyConnectionLine => {
                formatter.write_str("a connection line needs at least one midpoint")
            }
            Self::DuplicateIdentifier { kind, identifier } => {
                write!(formatter, "duplicate {kind} identifier `{identifier}`")
            }
            Self::UnknownConnectionComponent { identifier } => {
                write!(
                    formatter,
                    "connection refers to unknown component `{identifier}`"
                )
            }
        }
    }
}

impl Error for ValidationError {}

/// A collection that enforces unique identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentifierCollection {
    /// Ports of one component.
    Ports,
    /// Instantiated components in one file.
    Components,
}

impl Display for IdentifierCollection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Ports => "port",
            Self::Components => "component",
        };
        formatter.write_str(name)
    }
}

/// A schema identifier constrained to `[a-z][0-9a-z_]*`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier(String);

impl Identifier {
    /// Creates a validated identifier.
    ///
    /// # Errors
    ///
    /// Returns [`ValidationError::InvalidIdentifier`] when the value does not
    /// match `[a-z][0-9a-z_]*`.
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        let mut characters = value.bytes();
        let Some(first) = characters.next() else {
            return Err(ValidationError::InvalidIdentifier { value });
        };

        if !first.is_ascii_lowercase()
            || !characters.all(|character| {
                character.is_ascii_lowercase() || character.is_ascii_digit() || character == b'_'
            })
        {
            return Err(ValidationError::InvalidIdentifier { value });
        }

        Ok(Self(value))
    }

    /// Returns the identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its text.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Display for Identifier {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<String> for Identifier {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Identifier {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// A validated source BLAKE3 digest represented as lowercase hexadecimal text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceBlake3Digest(String);

impl SourceBlake3Digest {
    /// Creates a digest from exactly 64 lowercase hexadecimal characters.
    ///
    /// # Errors
    ///
    /// Returns [`ValidationError::InvalidSourceBlake3Digest`] when the value is
    /// not 64 lowercase hexadecimal characters.
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        if value.len() != 64
            || !value
                .bytes()
                .all(|character| character.is_ascii_digit() || matches!(character, b'a'..=b'f'))
        {
            return Err(ValidationError::InvalidSourceBlake3Digest { value });
        }
        Ok(Self(value))
    }

    /// Returns the hexadecimal digest text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the digest and returns its hexadecimal text.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Display for SourceBlake3Digest {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<String> for SourceBlake3Digest {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for SourceBlake3Digest {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// An absolute point in the component canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AbsolutePoint {
    /// Horizontal coordinate.
    pub x: u16,
    /// Vertical coordinate.
    pub y: u16,
}

impl AbsolutePoint {
    /// Creates an absolute point.
    #[must_use]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

/// An absolute component rectangle in the component canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentRectangle {
    /// Left coordinate.
    pub x: u16,
    /// Top coordinate.
    pub y: u16,
    /// Rectangle width.
    pub width: u16,
    /// Rectangle height.
    pub height: u16,
}

impl ComponentRectangle {
    /// Creates a component rectangle.
    #[must_use]
    pub const fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// A finite port coordinate in the inclusive normalized range `0..=1`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NormalizedCoordinate(f64);

impl NormalizedCoordinate {
    /// Creates a normalized coordinate.
    ///
    /// # Errors
    ///
    /// Returns [`ValidationError::InvalidNormalizedCoordinate`] when the value
    /// is non-finite or outside the inclusive range `0..=1`.
    pub fn new(value: f64) -> Result<Self, ValidationError> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(ValidationError::InvalidNormalizedCoordinate { value });
        }
        Ok(Self(value))
    }

    /// Returns the coordinate value.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl TryFrom<f64> for NormalizedCoordinate {
    type Error = ValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// The normalized position of a port in a component rectangle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PortLocation {
    /// Horizontal normalized coordinate.
    pub x: NormalizedCoordinate,
    /// Vertical normalized coordinate.
    pub y: NormalizedCoordinate,
}

impl PortLocation {
    /// Creates a port location from validated coordinates.
    #[must_use]
    pub const fn new(x: NormalizedCoordinate, y: NormalizedCoordinate) -> Self {
        Self { x, y }
    }
}

/// The direction in which a port permits connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortType {
    /// An input port.
    Input,
    /// An output port.
    Output,
    /// A bidirectional port.
    Bidirectional,
}

/// A named, positioned component port.
#[derive(Debug, Clone, PartialEq)]
pub struct Port {
    /// The port identifier.
    pub id: Identifier,
    /// The port direction.
    pub port_type: PortType,
    /// The port's normalized location.
    pub location: PortLocation,
}

impl Port {
    /// Creates a port.
    #[must_use]
    pub const fn new(id: Identifier, port_type: PortType, location: PortLocation) -> Self {
        Self {
            id,
            port_type,
            location,
        }
    }
}

/// Fields shared by file-backed and built-in component instances.
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentInstance {
    /// The instance identifier.
    pub id: Identifier,
    /// The instance rectangle.
    pub rectangle: ComponentRectangle,
    /// The optional path to the component icon.
    pub icon_path: Option<String>,
    /// Frozen parameter object values.
    pub parameter_object: ParameterObjectFrozen,
    /// Frozen variable object values.
    pub variable_object: VariableObjectFrozen,
    /// Exposed ports.
    pub ports: Vec<Port>,
}

impl ComponentInstance {
    /// Creates an instance after validating port identifier uniqueness.
    ///
    /// # Errors
    ///
    /// Returns [`ValidationError`] when a port ID is duplicated.
    pub fn new(
        id: Identifier,
        rectangle: ComponentRectangle,
        icon_path: Option<String>,
        parameter_object: ParameterObjectFrozen,
        variable_object: VariableObjectFrozen,
        ports: Vec<Port>,
    ) -> Result<Self, ValidationError> {
        validate_port_ids(&ports)?;
        Ok(Self {
            id,
            rectangle,
            icon_path,
            parameter_object,
            variable_object,
            ports,
        })
    }
}

/// A component instance backed by a standalone component definition file.
#[derive(Debug, Clone, PartialEq)]
pub struct FileBackedComponent {
    /// Fields shared by all component instances.
    pub instance: ComponentInstance,
    /// The component definition format version.
    pub component_version: u16,
    /// The BLAKE3 digest of the source component file.
    pub source_hash: SourceBlake3Digest,
    /// The component file path.
    pub path: String,
    /// The optional package that supplied the component.
    pub package: Option<String>,
}

impl FileBackedComponent {
    /// Creates a file-backed component instance.
    #[must_use]
    pub const fn new(
        instance: ComponentInstance,
        component_version: u16,
        source_hash: SourceBlake3Digest,
        path: String,
        package: Option<String>,
    ) -> Self {
        Self {
            instance,
            component_version,
            source_hash,
            path,
            package,
        }
    }
}

/// A component instance implemented by the simulation engine.
#[derive(Debug, Clone, PartialEq)]
pub struct BuiltInComponent {
    /// Fields shared by all component instances.
    pub instance: ComponentInstance,
    /// The engine-defined component type.
    pub component_type: String,
    /// The behavior version of the built-in component.
    pub behavior_version: u16,
}

impl BuiltInComponent {
    /// Creates a built-in component instance.
    #[must_use]
    pub const fn new(
        instance: ComponentInstance,
        component_type: String,
        behavior_version: u16,
    ) -> Self {
        Self {
            instance,
            component_type,
            behavior_version,
        }
    }
}

/// A component instance that is either file-backed or built in.
#[derive(Debug, Clone, PartialEq)]
pub enum InstantiatedComponent {
    /// A component loaded from a standalone component definition file.
    FileBacked(FileBackedComponent),
    /// A component provided by the simulation engine.
    BuiltIn(BuiltInComponent),
}

impl InstantiatedComponent {
    /// Returns the common instance fields.
    #[must_use]
    pub const fn instance(&self) -> &ComponentInstance {
        match self {
            Self::FileBacked(component) => &component.instance,
            Self::BuiltIn(component) => &component.instance,
        }
    }
}

/// The line style of a connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionStyle {
    /// A solid line.
    Solid,
    /// A dashed line.
    Dashed,
    /// A dotted line.
    Dotted,
}

/// The source and destination identifiers for a connection.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConnectionEndpoints {
    /// The source component identifier.
    pub from_component: Identifier,
    /// The source port identifier.
    pub from_port: Identifier,
    /// The destination component identifier.
    pub to_component: Identifier,
    /// The destination port identifier.
    pub to_port: Identifier,
}

impl ConnectionEndpoints {
    /// Creates connection endpoints.
    #[must_use]
    pub const fn new(
        from_component: Identifier,
        from_port: Identifier,
        to_component: Identifier,
        to_port: Identifier,
    ) -> Self {
        Self {
            from_component,
            from_port,
            to_component,
            to_port,
        }
    }
}

/// An ordered, non-empty sequence of connection midpoints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionLine {
    /// The ordered, non-empty midpoint sequence.
    midpoints: Vec<AbsolutePoint>,
}

impl ConnectionLine {
    /// Creates a connection line with at least one midpoint.
    ///
    /// # Errors
    ///
    /// Returns [`ValidationError::EmptyConnectionLine`] when no midpoint is
    /// supplied.
    pub fn new(midpoints: Vec<AbsolutePoint>) -> Result<Self, ValidationError> {
        if midpoints.is_empty() {
            return Err(ValidationError::EmptyConnectionLine);
        }
        Ok(Self { midpoints })
    }

    /// Returns the ordered line midpoints.
    #[must_use]
    pub fn midpoints(&self) -> &[AbsolutePoint] {
        &self.midpoints
    }

    /// Consumes the line and returns its ordered midpoints.
    #[must_use]
    pub fn into_midpoints(self) -> Vec<AbsolutePoint> {
        self.midpoints
    }
}

/// A styled connection between two component ports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connection {
    /// The source and destination endpoints.
    pub endpoints: ConnectionEndpoints,
    /// The line style.
    pub style: ConnectionStyle,
    /// The ordered line midpoints.
    pub line: ConnectionLine,
}

impl Connection {
    /// Creates a connection.
    #[must_use]
    pub const fn new(
        endpoints: ConnectionEndpoints,
        style: ConnectionStyle,
        line: ConnectionLine,
    ) -> Self {
        Self {
            endpoints,
            style,
            line,
        }
    }
}

/// Validates that port identifiers are unique.
pub(crate) fn validate_port_ids(ports: &[Port]) -> Result<(), ValidationError> {
    validate_unique(
        ports.iter().map(|port| &port.id),
        IdentifierCollection::Ports,
    )
}

/// Validates child component identifiers and connection component references.
pub(crate) fn validate_components(
    components: &[InstantiatedComponent],
    connections: &[Connection],
) -> Result<(), ValidationError> {
    validate_unique(
        components.iter().map(|component| &component.instance().id),
        IdentifierCollection::Components,
    )?;

    for connection in connections {
        for identifier in [
            &connection.endpoints.from_component,
            &connection.endpoints.to_component,
        ] {
            if !components
                .iter()
                .any(|component| component.instance().id == *identifier)
            {
                return Err(ValidationError::UnknownConnectionComponent {
                    identifier: (*identifier).clone(),
                });
            }
        }
    }

    Ok(())
}

/// Validates that one sequence of identifiers contains no duplicates.
fn validate_unique<'a>(
    identifiers: impl Iterator<Item = &'a Identifier>,
    kind: IdentifierCollection,
) -> Result<(), ValidationError> {
    let mut seen = std::collections::BTreeSet::new();
    for identifier in identifiers {
        if !seen.insert(identifier) {
            return Err(ValidationError::DuplicateIdentifier {
                kind,
                identifier: identifier.clone(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use datastore::prelude::{
        ParameterObjectDefinition, ParameterObjectFrozen, VariableObjectDefinition,
        VariableObjectFrozen,
    };

    use super::*;

    #[test]
    fn identifier_accepts_schema_pattern() {
        assert!(matches!(
            Identifier::new("heater_2"),
            Ok(identifier) if identifier.as_str() == "heater_2"
        ));
        assert!(matches!(
            Identifier::new("Heater"),
            Err(ValidationError::InvalidIdentifier { .. })
        ));
        assert!(matches!(
            Identifier::new("2heater"),
            Err(ValidationError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn source_digest_requires_lowercase_hex() {
        let digest = "ab".repeat(32);
        assert_eq!(
            SourceBlake3Digest::new(digest.clone()).map(|value| value.as_str().to_owned()),
            Ok(digest)
        );
        assert!(matches!(
            SourceBlake3Digest::new("A".repeat(64)),
            Err(ValidationError::InvalidSourceBlake3Digest { .. })
        ));
    }

    #[test]
    fn normalized_coordinate_rejects_non_finite_and_out_of_range_values() {
        assert!(matches!(
            NormalizedCoordinate::new(0.5),
            Ok(value) if (value.get() - 0.5).abs() < f64::EPSILON
        ));
        assert!(NormalizedCoordinate::new(-0.01).is_err());
        assert!(NormalizedCoordinate::new(f64::NAN).is_err());
    }

    #[test]
    fn component_instance_rejects_duplicate_ports() {
        let Ok(port_id) = Identifier::new("temperature") else {
            panic!("test identifier must be valid");
        };
        let Ok(origin) = NormalizedCoordinate::new(0.0) else {
            panic!("test coordinate must be valid");
        };
        let Ok(center) = NormalizedCoordinate::new(0.5) else {
            panic!("test coordinate must be valid");
        };
        let Ok(heater) = Identifier::new("heater") else {
            panic!("test identifier must be valid");
        };
        let location = PortLocation::new(origin, center);
        let ports = vec![
            Port::new(port_id.clone(), PortType::Input, location),
            Port::new(port_id, PortType::Output, location),
        ];

        assert!(matches!(
            ComponentInstance::new(
                heater,
                ComponentRectangle::new(0, 0, 100, 50),
                None,
                ParameterObjectFrozen::new(
                    ParameterObjectDefinition::builder("Parameters").finish()
                ),
                VariableObjectFrozen::new(VariableObjectDefinition::builder("Variables").finish()),
                ports,
            ),
            Err(ValidationError::DuplicateIdentifier {
                kind: IdentifierCollection::Ports,
                ..
            })
        ));
    }

    #[test]
    fn connection_line_requires_a_midpoint() {
        assert_eq!(
            ConnectionLine::new(Vec::new()),
            Err(ValidationError::EmptyConnectionLine)
        );
    }
}
