//! XML serialization and deserialization for simulation files.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

use datastore::definition::{FolderDefinition, SeparatorDefinition, TabDefinition};
use datastore::frozen::{FolderFrozen, SeparatorFrozen, TabFrozen};
use datastore::prelude::{
    BooleanDefinition, BooleanFrozen, FileDefinition, FileFrozen, IntegerDefinition, IntegerFrozen,
    ItemDefinitionType, ItemFrozen, NumberDefinition, NumberFrozen, NumberWithUnitsDefinition,
    NumberWithUnitsFrozen, ParameterKey, ParameterObjectDefinition, ParameterObjectFrozen,
    ShareableString, StringDefinition, StringFrozen, UnitDefinition, UnitFrozen, VariableKey,
    VariableObjectDefinition, VariableObjectFrozen,
};
use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use units::{UnitFamilyId, UnitId};

use crate::{
    AbsolutePoint, BuiltInComponent, ComponentDefinitionFile, ComponentInstance, ComponentMetadata,
    ComponentRectangle, Connection, ConnectionEndpoints, ConnectionLine, ConnectionStyle,
    FileBackedComponent, Identifier, InstantiatedComponent, ModelFile, ModelMetadata, ModelSetting,
    NormalizedCoordinate, Port, PortLocation, PortType, SourceBlake3Digest, ValidationError,
};

/// An error returned while reading or writing a simulation file XML document.
#[derive(Debug)]
pub enum XmlError {
    /// The input is not well-formed XML.
    Malformed(quick_xml::Error),
    /// XML violates the simulation file structure.
    InvalidStructure {
        /// A description of the structural problem.
        message: String,
    },
    /// A required XML attribute is absent.
    MissingAttribute {
        /// The element missing the attribute.
        element: String,
        /// The required attribute name.
        attribute: String,
    },
    /// An XML attribute cannot be converted to its required type.
    InvalidAttribute {
        /// The containing element.
        element: String,
        /// The attribute name.
        attribute: String,
        /// The rejected attribute value.
        value: String,
        /// The expected value format.
        expected: &'static str,
    },
    /// A datastore item cannot be represented faithfully by this XML format.
    UnsupportedItem {
        /// The datastore item type.
        item_type: String,
        /// Why the item cannot be represented.
        reason: String,
    },
    /// The parsed document violated a domain invariant.
    Validation(ValidationError),
}

impl Display for XmlError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Malformed(error) => write!(formatter, "malformed XML: {error}"),
            Self::InvalidStructure { message } => formatter.write_str(message),
            Self::MissingAttribute { element, attribute } => {
                write!(
                    formatter,
                    "element `{element}` is missing attribute `{attribute}`"
                )
            }
            Self::InvalidAttribute {
                element,
                attribute,
                value,
                expected,
            } => write!(
                formatter,
                "attribute `{attribute}` of element `{element}` has invalid value `{value}`; expected {expected}"
            ),
            Self::UnsupportedItem { item_type, reason } => {
                write!(
                    formatter,
                    "datastore item type `{item_type}` cannot be represented in simulation XML: {reason}"
                )
            }
            Self::Validation(error) => error.fmt(formatter),
        }
    }
}

impl Error for XmlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Malformed(error) => Some(error),
            Self::Validation(error) => Some(error),
            Self::InvalidStructure { .. }
            | Self::MissingAttribute { .. }
            | Self::InvalidAttribute { .. }
            | Self::UnsupportedItem { .. } => None,
        }
    }
}

impl From<quick_xml::Error> for XmlError {
    fn from(error: quick_xml::Error) -> Self {
        Self::Malformed(error)
    }
}

impl From<ValidationError> for XmlError {
    fn from(error: ValidationError) -> Self {
        Self::Validation(error)
    }
}

impl ModelFile {
    /// Serializes this model file as a UTF-8 XML document.
    ///
    /// # Errors
    ///
    /// Returns [`XmlError::UnsupportedItem`] if a datastore value has metadata
    /// that the simulation XML format cannot represent faithfully.
    pub fn to_xml_string(&self) -> Result<String, XmlError> {
        write_model(self)
    }

    /// Parses a model file from a UTF-8 XML document.
    ///
    /// # Errors
    ///
    /// Returns [`XmlError`] when XML is malformed, structurally invalid, or
    /// cannot be converted into concrete datastore objects.
    pub fn from_xml_str(xml: &str) -> Result<Self, XmlError> {
        parse_model(xml)
    }
}

impl ComponentDefinitionFile {
    /// Serializes this component definition as a UTF-8 XML document.
    ///
    /// # Errors
    ///
    /// Returns [`XmlError::UnsupportedItem`] if a datastore definition has
    /// metadata that the simulation XML format cannot represent faithfully.
    pub fn to_xml_string(&self) -> Result<String, XmlError> {
        write_component_definition(self)
    }

    /// Parses a component definition from a UTF-8 XML document.
    ///
    /// # Errors
    ///
    /// Returns [`XmlError`] when XML is malformed, structurally invalid, or
    /// cannot be converted into concrete datastore objects.
    pub fn from_xml_str(xml: &str) -> Result<Self, XmlError> {
        parse_component_definition(xml)
    }
}

/// Serializes a model file as a UTF-8 XML document.
///
/// # Errors
///
/// Returns [`XmlError::UnsupportedItem`] when a datastore value cannot be
/// represented faithfully by the simulation XML format.
pub fn model_to_xml_string(model: &ModelFile) -> Result<String, XmlError> {
    model.to_xml_string()
}

/// Parses a model file from a UTF-8 XML document.
///
/// # Errors
///
/// Returns [`XmlError`] if the document cannot be parsed as a valid model.
pub fn model_from_xml_str(xml: &str) -> Result<ModelFile, XmlError> {
    ModelFile::from_xml_str(xml)
}

/// Serializes a component definition as a UTF-8 XML document.
///
/// # Errors
///
/// Returns [`XmlError::UnsupportedItem`] when a datastore definition cannot be
/// represented faithfully by the simulation XML format.
pub fn component_definition_to_xml_string(
    component: &ComponentDefinitionFile,
) -> Result<String, XmlError> {
    component.to_xml_string()
}

/// Parses a component definition from a UTF-8 XML document.
///
/// # Errors
///
/// Returns [`XmlError`] if the document cannot be parsed as a valid component definition.
pub fn component_definition_from_xml_str(xml: &str) -> Result<ComponentDefinitionFile, XmlError> {
    ComponentDefinitionFile::from_xml_str(xml)
}

/// Writes a model document.
fn write_model(model: &ModelFile) -> Result<String, XmlError> {
    let mut output = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    open(
        &mut output,
        0,
        "model",
        &[
            ("file-version", model.metadata.file_version.to_string()),
            (
                "minimum-version",
                model.metadata.minimum_engine_version.clone(),
            ),
            ("built-version", model.metadata.built_engine_version.clone()),
            ("name", model.metadata.name.clone()),
        ],
    );
    write_settings(&mut output, &model.settings, 1);
    write_parameter_frozen_object(&mut output, "parameters", &model.parameter_object, 1)?;
    write_variable_frozen_object(&mut output, "variables", &model.variable_object, 1)?;
    write_components(&mut output, &model.components, 1)?;
    write_connections(&mut output, &model.connections, 1);
    close(&mut output, 0, "model");
    Ok(output)
}

/// Writes a component definition document.
fn write_component_definition(component: &ComponentDefinitionFile) -> Result<String, XmlError> {
    let mut output = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    let mut attributes = vec![
        ("file-version", component.metadata.file_version.to_string()),
        (
            "minimum-version",
            component.metadata.minimum_engine_version.clone(),
        ),
        (
            "recommended-version",
            component.metadata.recommended_engine_version.clone(),
        ),
        (
            "component-version",
            component.metadata.component_version.to_string(),
        ),
        ("name", component.metadata.name.clone()),
        ("length", component.metadata.length.to_string()),
        ("width", component.metadata.width.to_string()),
    ];
    if let Some(icon_path) = &component.metadata.icon_path {
        attributes.push(("icon", icon_path.clone()));
    }
    open(&mut output, 0, "component", &attributes);
    write_parameter_definition_object(&mut output, "parameters", &component.parameter_object, 1)?;
    write_variable_definition_object(&mut output, "variables", &component.variable_object, 1)?;
    write_ports(&mut output, &component.ports, 1);
    write_components(&mut output, &component.components, 1)?;
    write_connections(&mut output, &component.connections, 1);
    close(&mut output, 0, "component");
    Ok(output)
}

/// Writes the model settings collection.
fn write_settings(output: &mut String, settings: &[ModelSetting], depth: usize) {
    collection(output, depth, "model-settings", &[], |output| {
        for setting in settings {
            empty(
                output,
                depth.saturating_add(1),
                "setting",
                &[
                    ("key", setting.key.clone()),
                    ("value", setting.value.clone()),
                ],
            );
        }
    });
}

/// Writes a frozen parameter object.
fn write_parameter_frozen_object(
    output: &mut String,
    name: &str,
    object: &ParameterObjectFrozen,
    depth: usize,
) -> Result<(), XmlError> {
    write_frozen_object(
        output,
        name,
        object.definition().description_ref().as_str(),
        object
            .ordered_iter()
            .map(|(key, item)| (key.as_str(), item)),
        depth,
    )
}

/// Writes a frozen variable object.
fn write_variable_frozen_object(
    output: &mut String,
    name: &str,
    object: &VariableObjectFrozen,
    depth: usize,
) -> Result<(), XmlError> {
    write_frozen_object(
        output,
        name,
        object.definition().description_ref().as_str(),
        object
            .ordered_iter()
            .map(|(key, item)| (key.as_str(), item)),
        depth,
    )
}

/// Writes a frozen datastore object with its concrete item values.
fn write_frozen_object<'a>(
    output: &mut String,
    name: &str,
    description: &str,
    items: impl Iterator<Item = (&'a str, &'a ItemFrozen)>,
    depth: usize,
) -> Result<(), XmlError> {
    open(
        output,
        depth,
        name,
        &[("description", description.to_owned())],
    );
    for (key, item) in items {
        write_frozen_item(output, depth.saturating_add(1), key, item)?;
    }
    close(output, depth, name);
    Ok(())
}

/// Writes a parameter object definition.
fn write_parameter_definition_object(
    output: &mut String,
    name: &str,
    object: &ParameterObjectDefinition,
    depth: usize,
) -> Result<(), XmlError> {
    write_definition_object(
        output,
        name,
        object.description_ref().as_str(),
        object.iter().map(|(key, item)| (key.as_str(), item)),
        depth,
    )
}

/// Writes a variable object definition.
fn write_variable_definition_object(
    output: &mut String,
    name: &str,
    object: &VariableObjectDefinition,
    depth: usize,
) -> Result<(), XmlError> {
    write_definition_object(
        output,
        name,
        object.description_ref().as_str(),
        object.iter().map(|(key, item)| (key.as_str(), item)),
        depth,
    )
}

/// Writes a datastore object definition with its concrete item definitions.
fn write_definition_object<'a>(
    output: &mut String,
    name: &str,
    description: &str,
    items: impl Iterator<Item = (&'a str, &'a ItemDefinitionType)>,
    depth: usize,
) -> Result<(), XmlError> {
    open(
        output,
        depth,
        name,
        &[("description", description.to_owned())],
    );
    for (key, item) in items {
        write_definition_item(output, depth.saturating_add(1), key, item)?;
    }
    close(output, depth, name);
    Ok(())
}

/// Writes one concrete datastore item definition.
fn write_definition_item(
    output: &mut String,
    depth: usize,
    key: &str,
    item: &ItemDefinitionType,
) -> Result<(), XmlError> {
    let (item_type, description, default_value, mut attributes) = match item {
        ItemDefinitionType::Boolean(definition) => {
            validate_boolean_definition(definition.default_value_ref().as_str())?;
            (
                "boolean",
                definition.description_ref().as_str(),
                definition.default_value_ref().as_str(),
                Vec::new(),
            )
        }
        ItemDefinitionType::File(definition) => (
            "file",
            definition.description_ref().as_str(),
            definition.default_value_ref().as_str(),
            vec![
                (
                    "extension-filter",
                    definition.extension_filter_ref().to_string(),
                ),
                ("is-input", definition.is_input().to_string()),
            ],
        ),
        ItemDefinitionType::Folder(definition) => (
            "folder",
            definition.description_ref().as_str(),
            definition.default_value_ref().as_str(),
            vec![("is-input", definition.is_input().to_string())],
        ),
        ItemDefinitionType::Integer(definition) => {
            ensure_unconstrained_integer(definition.constraint_ref())?;
            (
                "integer",
                definition.description_ref().as_str(),
                definition.default_value_ref().as_str(),
                Vec::new(),
            )
        }
        ItemDefinitionType::Number(definition) => {
            ensure_unconstrained_number(definition.constraint_ref())?;
            (
                "number",
                definition.description_ref().as_str(),
                definition.default_value_ref().as_str(),
                Vec::new(),
            )
        }
        ItemDefinitionType::NumberWithUnits(definition) => {
            ensure_unconstrained_number(definition.constraint_ref())?;
            (
                "number-with-units",
                definition.description_ref().as_str(),
                definition.default_value_ref().as_str(),
                vec![(
                    "preferred-unit",
                    definition.preferred_units().string_id().as_str().to_owned(),
                )],
            )
        }
        ItemDefinitionType::String(definition) => (
            "string",
            definition.description_ref().as_str(),
            definition.default_value_ref().as_str(),
            Vec::new(),
        ),
        ItemDefinitionType::Unit(definition) => (
            "unit",
            definition.description_ref().as_str(),
            definition.default_value_ref().as_str(),
            vec![(
                "unit-family",
                unit_family_name(definition.unit_family()).to_owned(),
            )],
        ),
        ItemDefinitionType::Tab(definition) => {
            write_structural_item(output, depth, key, "tab", definition.description_ref());
            return Ok(());
        }
        ItemDefinitionType::Separator(definition) => {
            write_structural_item(
                output,
                depth,
                key,
                "separator",
                definition.description_ref(),
            );
            return Ok(());
        }
        ItemDefinitionType::Choice(_) => return Err(unsupported_item("choice", "choice lists")),
        ItemDefinitionType::Map(_) => {
            return Err(unsupported_item("map", "entry schemas and values"));
        }
        ItemDefinitionType::Table(_) => {
            return Err(unsupported_item("table", "column schemas and rows"));
        }
        ItemDefinitionType::TableWithUnits(_) => {
            return Err(unsupported_item(
                "table-with-units",
                "column schemas and rows",
            ));
        }
    };
    let mut item_attributes = item_attributes(key, item_type, description, default_value);
    item_attributes.append(&mut attributes);
    empty(output, depth, "item", &item_attributes);
    Ok(())
}

/// Writes one frozen datastore item, including its item definition metadata.
fn write_frozen_item(
    output: &mut String,
    depth: usize,
    key: &str,
    item: &ItemFrozen,
) -> Result<(), XmlError> {
    let (item_type, description, default_value, value, mut attributes) = match item {
        ItemFrozen::Boolean(value) => {
            validate_boolean_definition(value.definition().default_value_ref().as_str())?;
            validate_boolean_value(value.value().as_str(), "value")?;
            (
                "boolean",
                value.definition().description_ref().as_str(),
                value.definition().default_value_ref().as_str(),
                value.value().to_string(),
                Vec::new(),
            )
        }
        ItemFrozen::File(value) => (
            "file",
            value.definition().description_ref().as_str(),
            value.definition().default_value_ref().as_str(),
            value.value().to_string(),
            vec![
                (
                    "extension-filter",
                    value.definition().extension_filter_ref().to_string(),
                ),
                ("is-input", value.definition().is_input().to_string()),
            ],
        ),
        ItemFrozen::Folder(value) => (
            "folder",
            value.definition().description_ref().as_str(),
            value.definition().default_value_ref().as_str(),
            value.value().to_string(),
            vec![("is-input", value.definition().is_input().to_string())],
        ),
        ItemFrozen::Integer(value) => {
            ensure_unconstrained_integer(value.definition().constraint_ref())?;
            (
                "integer",
                value.definition().description_ref().as_str(),
                value.definition().default_value_ref().as_str(),
                value.value().to_string(),
                Vec::new(),
            )
        }
        ItemFrozen::Number(value) => {
            ensure_unconstrained_number(value.definition().constraint_ref())?;
            (
                "number",
                value.definition().description_ref().as_str(),
                value.definition().default_value_ref().as_str(),
                value.value().to_string(),
                Vec::new(),
            )
        }
        ItemFrozen::NumberWithUnits(value) => {
            ensure_unconstrained_number(value.definition().constraint_ref())?;
            (
                "number-with-units",
                value.definition().description_ref().as_str(),
                value.definition().default_value_ref().as_str(),
                value.value().to_string(),
                vec![
                    (
                        "preferred-unit",
                        value
                            .definition()
                            .preferred_units()
                            .string_id()
                            .as_str()
                            .to_owned(),
                    ),
                    ("units", value.units().to_string()),
                ],
            )
        }
        ItemFrozen::String(value) => (
            "string",
            value.definition().description_ref().as_str(),
            value.definition().default_value_ref().as_str(),
            value.value().to_string(),
            Vec::new(),
        ),
        ItemFrozen::Unit(value) => (
            "unit",
            value.definition().description_ref().as_str(),
            value.definition().default_value_ref().as_str(),
            value.value().to_string(),
            vec![(
                "unit-family",
                unit_family_name(value.definition().unit_family()).to_owned(),
            )],
        ),
        ItemFrozen::Tab(value) => {
            write_structural_item(
                output,
                depth,
                key,
                "tab",
                value.definition().description_ref(),
            );
            return Ok(());
        }
        ItemFrozen::Separator(value) => {
            write_structural_item(
                output,
                depth,
                key,
                "separator",
                value.definition().description_ref(),
            );
            return Ok(());
        }
        ItemFrozen::Choice(_) => return Err(unsupported_item("choice", "choice lists")),
        ItemFrozen::Map(_) => return Err(unsupported_item("map", "entry schemas and values")),
        ItemFrozen::Table(_) => return Err(unsupported_item("table", "column schemas and rows")),
        ItemFrozen::TableWithUnits(_) => {
            return Err(unsupported_item(
                "table-with-units",
                "column schemas and rows",
            ));
        }
    };
    let mut item_attributes = item_attributes(key, item_type, description, default_value);
    item_attributes.push(("value", value));
    item_attributes.append(&mut attributes);
    empty(output, depth, "item", &item_attributes);
    Ok(())
}

/// Writes a tab or separator item.
fn write_structural_item(
    output: &mut String,
    depth: usize,
    key: &str,
    item_type: &str,
    description: &ShareableString,
) {
    empty(
        output,
        depth,
        "item",
        &[
            ("key", key.to_owned()),
            ("type", item_type.to_owned()),
            ("description", description.to_string()),
        ],
    );
}

/// Constructs the shared attributes for a scalar datastore item.
fn item_attributes(
    key: &str,
    item_type: &str,
    description: &str,
    default_value: &str,
) -> Vec<(&'static str, String)> {
    let mut attributes = vec![
        ("key", key.to_owned()),
        ("type", item_type.to_owned()),
        ("description", description.to_owned()),
    ];
    if !default_value.is_empty() {
        attributes.push(("default", default_value.to_owned()));
    }
    attributes
}

/// Rejects integer constraints because the XML schema has no constraint representation.
fn ensure_unconstrained_integer(
    constraint: &datastore::prelude::IntegerConstraintEnum,
) -> Result<(), XmlError> {
    if matches!(constraint, datastore::prelude::IntegerConstraintEnum::None) {
        Ok(())
    } else {
        Err(unsupported_item(
            "integer",
            "integer constraints are not represented",
        ))
    }
}

/// Rejects number constraints because the XML schema has no constraint representation.
fn ensure_unconstrained_number(
    constraint: &datastore::prelude::NumberConstraintEnum,
) -> Result<(), XmlError> {
    if matches!(constraint, datastore::prelude::NumberConstraintEnum::None) {
        Ok(())
    } else {
        Err(unsupported_item(
            "number",
            "number constraints are not represented",
        ))
    }
}

/// Validates a datastore boolean default value before serializing it.
fn validate_boolean_definition(value: &str) -> Result<(), XmlError> {
    if value.is_empty() {
        Ok(())
    } else {
        validate_boolean_value(value, "default")
    }
}

/// Validates a datastore boolean value before serializing it.
fn validate_boolean_value(value: &str, attribute: &str) -> Result<(), XmlError> {
    if matches!(value, "true" | "false") {
        Ok(())
    } else {
        Err(XmlError::InvalidAttribute {
            element: "item".to_owned(),
            attribute: attribute.to_owned(),
            value: value.to_owned(),
            expected: "true or false",
        })
    }
}

/// Constructs a descriptive unsupported-item error.
fn unsupported_item(item_type: &str, reason: &str) -> XmlError {
    XmlError::UnsupportedItem {
        item_type: item_type.to_owned(),
        reason: reason.to_owned(),
    }
}

/// Writes a collection of ports.
fn write_ports(output: &mut String, ports: &[Port], depth: usize) {
    collection(output, depth, "ports", &[], |output| {
        for port in ports {
            open(
                output,
                depth.saturating_add(1),
                "port",
                &[
                    ("id", port.id.as_str().to_owned()),
                    ("type", port_type_name(port.port_type).to_owned()),
                ],
            );
            empty(
                output,
                depth.saturating_add(2),
                "location",
                &[
                    ("x", port.location.x.get().to_string()),
                    ("y", port.location.y.get().to_string()),
                ],
            );
            close(output, depth.saturating_add(1), "port");
        }
    });
}

/// Writes a collection of instantiated components.
fn write_components(
    output: &mut String,
    components: &[InstantiatedComponent],
    depth: usize,
) -> Result<(), XmlError> {
    open(output, depth, "components", &[]);
    for component in components {
        let (name, mut attributes) = match component {
            InstantiatedComponent::FileBacked(component) => (
                "component-file",
                vec![
                    ("id", component.instance.id.as_str().to_owned()),
                    ("component-version", component.component_version.to_string()),
                    ("source-blake3", component.source_hash.as_str().to_owned()),
                    ("path", component.path.clone()),
                ],
            ),
            InstantiatedComponent::BuiltIn(component) => (
                "component",
                vec![
                    ("id", component.instance.id.as_str().to_owned()),
                    ("type", component.component_type.clone()),
                    ("version", component.behavior_version.to_string()),
                ],
            ),
        };
        if let InstantiatedComponent::FileBacked(component) = component {
            if let Some(package) = &component.package {
                attributes.push(("package", package.clone()));
            }
        }
        open(output, depth.saturating_add(1), name, &attributes);
        write_instance_contents(output, component.instance(), depth.saturating_add(2))?;
        close(output, depth.saturating_add(1), name);
    }
    close(output, depth, "components");
    Ok(())
}

/// Writes the contents common to both component variants.
fn write_instance_contents(
    output: &mut String,
    instance: &ComponentInstance,
    depth: usize,
) -> Result<(), XmlError> {
    empty(
        output,
        depth,
        "location",
        &[
            ("x", instance.rectangle.x.to_string()),
            ("y", instance.rectangle.y.to_string()),
            ("width", instance.rectangle.width.to_string()),
            ("height", instance.rectangle.height.to_string()),
        ],
    );
    write_icon(output, instance.icon_path.as_deref(), depth);
    write_parameter_frozen_object(output, "parameters", &instance.parameter_object, depth)?;
    write_variable_frozen_object(output, "variables", &instance.variable_object, depth)?;
    write_ports(output, &instance.ports, depth);
    Ok(())
}

/// Writes an optional component icon.
fn write_icon(output: &mut String, icon_path: Option<&str>, depth: usize) {
    if let Some(path) = icon_path {
        empty(output, depth, "icon", &[("path", path.to_owned())]);
    }
}

/// Writes a collection of connections.
fn write_connections(output: &mut String, connections: &[Connection], depth: usize) {
    collection(output, depth, "connections", &[], |output| {
        for connection in connections {
            open(
                output,
                depth.saturating_add(1),
                "connection",
                &[
                    (
                        "from-component",
                        connection.endpoints.from_component.as_str().to_owned(),
                    ),
                    (
                        "from-port",
                        connection.endpoints.from_port.as_str().to_owned(),
                    ),
                    (
                        "to-component",
                        connection.endpoints.to_component.as_str().to_owned(),
                    ),
                    ("to-port", connection.endpoints.to_port.as_str().to_owned()),
                    ("style", connection_style_name(connection.style).to_owned()),
                ],
            );
            open(output, depth.saturating_add(2), "line", &[]);
            for point in connection.line.midpoints() {
                empty(
                    output,
                    depth.saturating_add(3),
                    "midpoint",
                    &[("x", point.x.to_string()), ("y", point.y.to_string())],
                );
            }
            close(output, depth.saturating_add(2), "line");
            close(output, depth.saturating_add(1), "connection");
        }
    });
}

/// Writes an enclosing collection element and its contents.
fn collection(
    output: &mut String,
    depth: usize,
    name: &str,
    attributes: &[(&str, String)],
    contents: impl FnOnce(&mut String),
) {
    open(output, depth, name, attributes);
    contents(output);
    close(output, depth, name);
}

/// Writes an opening XML element.
fn open(output: &mut String, depth: usize, name: &str, attributes: &[(&str, String)]) {
    indent(output, depth);
    output.push('<');
    output.push_str(name);
    write_attributes(output, attributes);
    output.push_str(">\n");
}

/// Writes an empty XML element.
fn empty(output: &mut String, depth: usize, name: &str, attributes: &[(&str, String)]) {
    indent(output, depth);
    output.push('<');
    output.push_str(name);
    write_attributes(output, attributes);
    output.push_str("/>\n");
}

/// Writes a closing XML element.
fn close(output: &mut String, depth: usize, name: &str) {
    indent(output, depth);
    output.push_str("</");
    output.push_str(name);
    output.push_str(">\n");
}

/// Writes indentation for an element depth.
fn indent(output: &mut String, depth: usize) {
    for _ in 0..depth {
        output.push_str("  ");
    }
}

/// Writes escaped XML attributes.
fn write_attributes(output: &mut String, attributes: &[(&str, String)]) {
    for (name, value) in attributes {
        output.push(' ');
        output.push_str(name);
        output.push_str("=\"");
        escape_attribute(output, value);
        output.push('"');
    }
}

/// Escapes a value for an XML attribute.
fn escape_attribute(output: &mut String, value: &str) {
    for character in value.chars() {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&apos;"),
            _ => output.push(character),
        }
    }
}

/// Parses a model document.
fn parse_model(xml: &str) -> Result<ModelFile, XmlError> {
    let mut cursor = Cursor::new(xml);
    let root = cursor.expect_start("model")?;
    let attributes = attributes(
        &root,
        "model",
        &["file-version", "minimum-version", "built-version", "name"],
    )?;
    let metadata = ModelMetadata::new(
        parse_u16(
            &required_attribute(&attributes, "model", "file-version")?,
            "model",
            "file-version",
        )?,
        required_attribute(&attributes, "model", "minimum-version")?,
        required_attribute(&attributes, "model", "built-version")?,
        required_attribute(&attributes, "model", "name")?,
    );
    let settings = parse_settings(&mut cursor)?;
    let parameter_object = parse_parameter_frozen_object(&mut cursor, "parameters")?;
    let variable_object = parse_variable_frozen_object(&mut cursor, "variables")?;
    let components = parse_components(&mut cursor)?;
    let connections = parse_connections(&mut cursor)?;
    cursor.expect_end("model")?;
    cursor.expect_eof()?;
    ModelFile::new(
        metadata,
        settings,
        parameter_object,
        variable_object,
        components,
        connections,
    )
    .map_err(Into::into)
}

/// Parses a component definition document.
fn parse_component_definition(xml: &str) -> Result<ComponentDefinitionFile, XmlError> {
    let mut cursor = Cursor::new(xml);
    let root = cursor.expect_start("component")?;
    let attributes = attributes(
        &root,
        "component",
        &[
            "file-version",
            "minimum-version",
            "recommended-version",
            "component-version",
            "name",
            "icon",
            "length",
            "width",
        ],
    )?;
    let metadata = ComponentMetadata::new(
        parse_u16(
            &required_attribute(&attributes, "component", "file-version")?,
            "component",
            "file-version",
        )?,
        required_attribute(&attributes, "component", "minimum-version")?,
        required_attribute(&attributes, "component", "recommended-version")?,
        parse_u16(
            &required_attribute(&attributes, "component", "component-version")?,
            "component",
            "component-version",
        )?,
        required_attribute(&attributes, "component", "name")?,
        parse_u16(
            &required_attribute(&attributes, "component", "length")?,
            "component",
            "length",
        )?,
        parse_u16(
            &required_attribute(&attributes, "component", "width")?,
            "component",
            "width",
        )?,
    )
    .with_icon_path(optional_attribute(&attributes, "icon"));
    let parameter_object = parse_parameter_definition_object(&mut cursor, "parameters")?;
    let variable_object = parse_variable_definition_object(&mut cursor, "variables")?;
    let ports = parse_ports(&mut cursor)?;
    let components = parse_components(&mut cursor)?;
    let connections = parse_connections(&mut cursor)?;
    cursor.expect_end("component")?;
    cursor.expect_eof()?;
    ComponentDefinitionFile::new(
        metadata,
        parameter_object,
        variable_object,
        ports,
        components,
        connections,
    )
    .map_err(Into::into)
}

/// Parses the model settings collection.
fn parse_settings(cursor: &mut Cursor<'_>) -> Result<Vec<ModelSetting>, XmlError> {
    parse_collection(cursor, "model-settings", |cursor, event| {
        let element = cursor.expect_empty_element(&event, "setting")?;
        let attributes = attributes(&element, "setting", &["key", "value"])?;
        Ok(ModelSetting::new(
            required_attribute(&attributes, "setting", "key")?,
            required_attribute(&attributes, "setting", "value")?,
        ))
    })
}

/// Parses a frozen parameter object.
fn parse_parameter_frozen_object(
    cursor: &mut Cursor<'_>,
    name: &str,
) -> Result<ParameterObjectFrozen, XmlError> {
    let (description, items) = parse_object(cursor, name, parse_frozen_item)?;
    let mut values = Vec::new();
    let mut keys = BTreeSet::new();
    for (key, item) in items {
        let key = parse_parameter_key(&key)?;
        if !keys.insert(key.clone()) {
            return Err(duplicate_object_key("parameter"));
        }
        values.push((key, item));
    }
    Ok(ParameterObjectFrozen::new_from_ordered_items(
        description,
        values,
    ))
}

/// Parses a frozen variable object.
fn parse_variable_frozen_object(
    cursor: &mut Cursor<'_>,
    name: &str,
) -> Result<VariableObjectFrozen, XmlError> {
    let (description, items) = parse_object(cursor, name, parse_frozen_item)?;
    let mut values = Vec::new();
    let mut keys = BTreeSet::new();
    for (key, item) in items {
        let key = parse_variable_key(&key)?;
        if !keys.insert(key.clone()) {
            return Err(duplicate_object_key("variable"));
        }
        values.push((key, item));
    }
    Ok(VariableObjectFrozen::new_from_ordered_items(
        description,
        values,
    ))
}

/// Parses a parameter object definition.
fn parse_parameter_definition_object(
    cursor: &mut Cursor<'_>,
    name: &str,
) -> Result<ParameterObjectDefinition, XmlError> {
    let (description, items) = parse_object(cursor, name, parse_definition_item)?;
    let mut keys = BTreeSet::new();
    let mut builder = ParameterObjectDefinition::builder(description);
    for (key, item) in items {
        if !keys.insert(key.clone()) {
            return Err(duplicate_object_key("parameter"));
        }
        builder.insert(parse_parameter_key(&key)?, item);
    }
    Ok(builder.finish())
}

/// Parses a variable object definition.
fn parse_variable_definition_object(
    cursor: &mut Cursor<'_>,
    name: &str,
) -> Result<VariableObjectDefinition, XmlError> {
    let (description, items) = parse_object(cursor, name, parse_definition_item)?;
    let mut keys = BTreeSet::new();
    let mut builder = VariableObjectDefinition::builder(description);
    for (key, item) in items {
        if !keys.insert(key.clone()) {
            return Err(duplicate_object_key("variable"));
        }
        builder.insert(parse_variable_key(&key)?, item);
    }
    Ok(builder.finish())
}

/// Parses a described datastore object and its item events.
fn parse_object<T>(
    cursor: &mut Cursor<'_>,
    name: &str,
    mut parse_item: impl FnMut(&mut Cursor<'_>, &Event<'static>) -> Result<T, XmlError>,
) -> Result<(String, Vec<T>), XmlError> {
    let event = cursor.next()?;
    let (start, is_empty) = match &event {
        Event::Empty(start) if start.name().as_ref() == name.as_bytes() => (start.to_owned(), true),
        Event::Start(start) if start.name().as_ref() == name.as_bytes() => {
            (start.to_owned(), false)
        }
        Event::Start(_)
        | Event::End(_)
        | Event::Empty(_)
        | Event::Text(_)
        | Event::CData(_)
        | Event::Comment(_)
        | Event::Decl(_)
        | Event::PI(_)
        | Event::DocType(_)
        | Event::Eof => return Err(unexpected_event(&event, name)),
    };
    let attributes = attributes(&start, name, &["description"])?;
    let description = required_attribute(&attributes, name, "description")?;
    if is_empty {
        return Ok((description, Vec::new()));
    }

    let mut values = Vec::new();
    loop {
        let event = cursor.next()?;
        if let Event::End(end) = &event {
            if end.name().as_ref() == name.as_bytes() {
                return Ok((description, values));
            }
        }
        values.push(parse_item(cursor, &event)?);
    }
}

/// Parses an item definition from an item XML event.
fn parse_definition_item(
    cursor: &mut Cursor<'_>,
    event: &Event<'static>,
) -> Result<(String, ItemDefinitionType), XmlError> {
    let element = cursor.expect_empty_element(event, "item")?;
    let attributes = item_xml_attributes(&element)?;
    let key = required_attribute(&attributes, "item", "key")?;
    let definition = parse_item_definition(&attributes, false)?;
    Ok((key, definition))
}

/// Parses a frozen item and its concrete definition from an item XML event.
fn parse_frozen_item(
    cursor: &mut Cursor<'_>,
    event: &Event<'static>,
) -> Result<(String, ItemFrozen), XmlError> {
    let element = cursor.expect_empty_element(event, "item")?;
    let attributes = item_xml_attributes(&element)?;
    let key = required_attribute(&attributes, "item", "key")?;
    let definition = parse_item_definition(&attributes, true)?;
    let item = match definition {
        ItemDefinitionType::Boolean(definition) => {
            ItemFrozen::Boolean(BooleanFrozen::new_with_value(
                definition,
                required_attribute(&attributes, "item", "value")?.into(),
            ))
        }
        ItemDefinitionType::File(definition) => ItemFrozen::File(FileFrozen::new_with_value(
            definition,
            required_attribute(&attributes, "item", "value")?.into(),
        )),
        ItemDefinitionType::Folder(definition) => ItemFrozen::Folder(FolderFrozen::new_with_value(
            definition,
            required_attribute(&attributes, "item", "value")?.into(),
        )),
        ItemDefinitionType::Integer(definition) => {
            ItemFrozen::Integer(IntegerFrozen::new_with_value(
                definition,
                required_attribute(&attributes, "item", "value")?.into(),
            ))
        }
        ItemDefinitionType::Number(definition) => ItemFrozen::Number(NumberFrozen::new_with_value(
            definition,
            required_attribute(&attributes, "item", "value")?.into(),
        )),
        ItemDefinitionType::NumberWithUnits(definition) => {
            ItemFrozen::NumberWithUnits(NumberWithUnitsFrozen::new_with_value(
                definition,
                required_attribute(&attributes, "item", "value")?.into(),
                required_attribute(&attributes, "item", "units")?.into(),
            ))
        }
        ItemDefinitionType::String(definition) => ItemFrozen::String(StringFrozen::new_with_value(
            definition,
            required_attribute(&attributes, "item", "value")?.into(),
        )),
        ItemDefinitionType::Unit(definition) => ItemFrozen::Unit(UnitFrozen::new_with_value(
            definition,
            required_attribute(&attributes, "item", "value")?.into(),
        )),
        ItemDefinitionType::Tab(definition) => ItemFrozen::Tab(TabFrozen::new(definition)),
        ItemDefinitionType::Separator(definition) => {
            ItemFrozen::Separator(SeparatorFrozen::new(definition))
        }
        ItemDefinitionType::Choice(_)
        | ItemDefinitionType::Map(_)
        | ItemDefinitionType::Table(_)
        | ItemDefinitionType::TableWithUnits(_) => {
            return Err(XmlError::InvalidStructure {
                message: "unsupported item definition reached frozen item construction".to_owned(),
            });
        }
    };
    Ok((key, item))
}

/// Parses a concrete datastore item definition from XML attributes.
fn parse_item_definition(
    attributes: &[(String, String)],
    frozen: bool,
) -> Result<ItemDefinitionType, XmlError> {
    let item_type = required_attribute(attributes, "item", "type")?;
    let description = required_attribute(attributes, "item", "description")?;
    let default_value = optional_attribute(attributes, "default").unwrap_or_default();
    match item_type.as_str() {
        "boolean" => {
            validate_item_attributes(
                attributes,
                if frozen {
                    &["key", "type", "description", "default", "value"]
                } else {
                    &["key", "type", "description", "default"]
                },
            )?;
            let definition = if default_value.is_empty() {
                BooleanDefinition::new(description)
            } else {
                BooleanDefinition::new_with_default(
                    description,
                    parse_boolean(&default_value, "default")?,
                )
            };
            if frozen {
                parse_boolean(&required_attribute(attributes, "item", "value")?, "value")?;
            }
            Ok(definition.into())
        }
        "file" => {
            validate_item_attributes(
                attributes,
                if frozen {
                    &[
                        "key",
                        "type",
                        "description",
                        "default",
                        "value",
                        "extension-filter",
                        "is-input",
                    ]
                } else {
                    &[
                        "key",
                        "type",
                        "description",
                        "default",
                        "extension-filter",
                        "is-input",
                    ]
                },
            )?;
            Ok(FileDefinition::new_with_default(
                description,
                required_attribute(attributes, "item", "extension-filter")?,
                parse_boolean(
                    &required_attribute(attributes, "item", "is-input")?,
                    "is-input",
                )?,
                default_value,
            )
            .into())
        }
        "folder" => {
            validate_item_attributes(
                attributes,
                if frozen {
                    &["key", "type", "description", "default", "value", "is-input"]
                } else {
                    &["key", "type", "description", "default", "is-input"]
                },
            )?;
            Ok(FolderDefinition::new_with_default(
                description,
                parse_boolean(
                    &required_attribute(attributes, "item", "is-input")?,
                    "is-input",
                )?,
                default_value,
            )
            .into())
        }
        "integer" => {
            validate_item_attributes(
                attributes,
                if frozen {
                    &["key", "type", "description", "default", "value"]
                } else {
                    &["key", "type", "description", "default"]
                },
            )?;
            Ok(IntegerDefinition::new_with_default(description, default_value).into())
        }
        "number" => {
            validate_item_attributes(
                attributes,
                if frozen {
                    &["key", "type", "description", "default", "value"]
                } else {
                    &["key", "type", "description", "default"]
                },
            )?;
            Ok(NumberDefinition::new_with_default(description, default_value).into())
        }
        "number-with-units" => {
            validate_item_attributes(
                attributes,
                if frozen {
                    &[
                        "key",
                        "type",
                        "description",
                        "default",
                        "value",
                        "preferred-unit",
                        "units",
                    ]
                } else {
                    &["key", "type", "description", "default", "preferred-unit"]
                },
            )?;
            Ok(NumberWithUnitsDefinition::new_with_default(
                description,
                default_value,
                parse_unit_id(&required_attribute(attributes, "item", "preferred-unit")?)?,
            )
            .into())
        }
        "string" => {
            validate_item_attributes(
                attributes,
                if frozen {
                    &["key", "type", "description", "default", "value"]
                } else {
                    &["key", "type", "description", "default"]
                },
            )?;
            Ok(StringDefinition::new_with_default(description, default_value).into())
        }
        "unit" => {
            validate_item_attributes(
                attributes,
                if frozen {
                    &[
                        "key",
                        "type",
                        "description",
                        "default",
                        "value",
                        "unit-family",
                    ]
                } else {
                    &["key", "type", "description", "default", "unit-family"]
                },
            )?;
            Ok(UnitDefinition::new_with_default(
                description,
                parse_unit_family(&required_attribute(attributes, "item", "unit-family")?)?,
                default_value,
            )
            .into())
        }
        "tab" => {
            validate_item_attributes(attributes, &["key", "type", "description"])?;
            Ok(TabDefinition::new(description).into())
        }
        "separator" => {
            validate_item_attributes(attributes, &["key", "type", "description"])?;
            Ok(SeparatorDefinition::new(description).into())
        }
        "choice" => Err(unsupported_item("choice", "choice lists")),
        "map" => Err(unsupported_item("map", "entry schemas and values")),
        "table" => Err(unsupported_item("table", "column schemas and rows")),
        "table-with-units" => Err(unsupported_item(
            "table-with-units",
            "column schemas and rows",
        )),
        _ => Err(XmlError::InvalidAttribute {
            element: "item".to_owned(),
            attribute: "type".to_owned(),
            value: item_type,
            expected: "a supported datastore item type",
        }),
    }
}

/// Parses the ports collection.
fn parse_ports(cursor: &mut Cursor<'_>) -> Result<Vec<Port>, XmlError> {
    parse_collection(cursor, "ports", |cursor, event| {
        let start = Cursor::expect_start_event(&event, "port")?;
        let attributes = attributes(&start, "port", &["id", "type"])?;
        let location = parse_port_location(cursor)?;
        cursor.expect_end("port")?;
        Ok(Port::new(
            parse_identifier(&required_attribute(&attributes, "port", "id")?)?,
            parse_port_type(
                &required_attribute(&attributes, "port", "type")?,
                "port",
                "type",
            )?,
            location,
        ))
    })
}

/// Parses a port location element.
fn parse_port_location(cursor: &mut Cursor<'_>) -> Result<PortLocation, XmlError> {
    let location = cursor.expect_empty("location")?;
    let attributes = attributes(&location, "location", &["x", "y"])?;
    Ok(PortLocation::new(
        parse_coordinate(
            &required_attribute(&attributes, "location", "x")?,
            "location",
            "x",
        )?,
        parse_coordinate(
            &required_attribute(&attributes, "location", "y")?,
            "location",
            "y",
        )?,
    ))
}

/// Parses the instantiated components collection.
fn parse_components(cursor: &mut Cursor<'_>) -> Result<Vec<InstantiatedComponent>, XmlError> {
    parse_collection(cursor, "components", |cursor, event| {
        let Event::Start(start) = &event else {
            return Err(unexpected_event(&event, "component-file or component"));
        };
        if !matches!(start.name().as_ref(), b"component-file" | b"component") {
            return Err(unexpected_event(&event, "component-file or component"));
        }
        let start = start.to_owned();
        let is_file_backed = start.name().as_ref() == b"component-file";
        let instance = parse_component_instance(cursor, &start)?;
        if is_file_backed {
            let attributes = attributes(
                &start,
                "component-file",
                &[
                    "id",
                    "component-version",
                    "source-blake3",
                    "path",
                    "package",
                ],
            )?;
            cursor.expect_end("component-file")?;
            Ok(InstantiatedComponent::FileBacked(FileBackedComponent::new(
                instance,
                parse_u16(
                    &required_attribute(&attributes, "component-file", "component-version")?,
                    "component-file",
                    "component-version",
                )?,
                SourceBlake3Digest::new(required_attribute(
                    &attributes,
                    "component-file",
                    "source-blake3",
                )?)?,
                required_attribute(&attributes, "component-file", "path")?,
                optional_attribute(&attributes, "package"),
            )))
        } else {
            let attributes = attributes(&start, "component", &["id", "type", "version"])?;
            cursor.expect_end("component")?;
            Ok(InstantiatedComponent::BuiltIn(BuiltInComponent::new(
                instance,
                required_attribute(&attributes, "component", "type")?,
                parse_u16(
                    &required_attribute(&attributes, "component", "version")?,
                    "component",
                    "version",
                )?,
            )))
        }
    })
}

/// Parses fields shared by file-backed and built-in component elements.
fn parse_component_instance(
    cursor: &mut Cursor<'_>,
    start: &BytesStart<'_>,
) -> Result<ComponentInstance, XmlError> {
    let element = if start.name().as_ref() == b"component-file" {
        "component-file"
    } else {
        "component"
    };
    let id = parse_identifier(&required_attribute(
        &attributes(
            start,
            element,
            &[
                "id",
                "component-version",
                "source-blake3",
                "path",
                "package",
                "type",
                "version",
            ],
        )?,
        element,
        "id",
    )?)?;
    let location = parse_component_rectangle(cursor)?;
    let icon_path = parse_optional_icon(cursor)?;
    let parameter_object = parse_parameter_frozen_object(cursor, "parameters")?;
    let variable_object = parse_variable_frozen_object(cursor, "variables")?;
    let ports = parse_ports(cursor)?;
    ComponentInstance::new(
        id,
        location,
        icon_path,
        parameter_object,
        variable_object,
        ports,
    )
    .map_err(Into::into)
}

/// Parses a component rectangle element.
fn parse_component_rectangle(cursor: &mut Cursor<'_>) -> Result<ComponentRectangle, XmlError> {
    let location = cursor.expect_empty("location")?;
    let attributes = attributes(&location, "location", &["x", "y", "width", "height"])?;
    Ok(ComponentRectangle::new(
        parse_u16(
            &required_attribute(&attributes, "location", "x")?,
            "location",
            "x",
        )?,
        parse_u16(
            &required_attribute(&attributes, "location", "y")?,
            "location",
            "y",
        )?,
        parse_u16(
            &required_attribute(&attributes, "location", "width")?,
            "location",
            "width",
        )?,
        parse_u16(
            &required_attribute(&attributes, "location", "height")?,
            "location",
            "height",
        )?,
    ))
}

/// Parses an optional component icon element.
fn parse_optional_icon(cursor: &mut Cursor<'_>) -> Result<Option<String>, XmlError> {
    let Some(icon) = cursor.expect_optional_empty("icon")? else {
        return Ok(None);
    };
    let attributes = attributes(&icon, "icon", &["path"])?;
    required_attribute(&attributes, "icon", "path").map(Some)
}

/// Parses the connections collection.
fn parse_connections(cursor: &mut Cursor<'_>) -> Result<Vec<Connection>, XmlError> {
    parse_collection(cursor, "connections", |cursor, event| {
        let start = Cursor::expect_start_event(&event, "connection")?;
        let attributes = attributes(
            &start,
            "connection",
            &[
                "from-component",
                "from-port",
                "to-component",
                "to-port",
                "style",
            ],
        )?;
        let endpoints = ConnectionEndpoints::new(
            parse_identifier(&required_attribute(
                &attributes,
                "connection",
                "from-component",
            )?)?,
            parse_identifier(&required_attribute(&attributes, "connection", "from-port")?)?,
            parse_identifier(&required_attribute(
                &attributes,
                "connection",
                "to-component",
            )?)?,
            parse_identifier(&required_attribute(&attributes, "connection", "to-port")?)?,
        );
        let line = parse_connection_line(cursor)?;
        cursor.expect_end("connection")?;
        Ok(Connection::new(
            endpoints,
            parse_connection_style(
                &required_attribute(&attributes, "connection", "style")?,
                "connection",
                "style",
            )?,
            line,
        ))
    })
}

/// Parses a connection line and its midpoints.
fn parse_connection_line(cursor: &mut Cursor<'_>) -> Result<ConnectionLine, XmlError> {
    cursor.expect_start("line")?;
    let mut midpoints = Vec::new();
    loop {
        let event = cursor.next()?;
        if let Event::End(end) = &event {
            if end.name().as_ref() == b"line" {
                break;
            }
        }
        let midpoint = cursor.expect_empty_element(&event, "midpoint")?;
        let attributes = attributes(&midpoint, "midpoint", &["x", "y"])?;
        midpoints.push(AbsolutePoint::new(
            parse_u16(
                &required_attribute(&attributes, "midpoint", "x")?,
                "midpoint",
                "x",
            )?,
            parse_u16(
                &required_attribute(&attributes, "midpoint", "y")?,
                "midpoint",
                "y",
            )?,
        ));
    }
    ConnectionLine::new(midpoints).map_err(Into::into)
}

/// Parses an optionally empty collection using its element parser.
fn parse_collection<T>(
    cursor: &mut Cursor<'_>,
    name: &str,
    mut parse_item: impl FnMut(&mut Cursor<'_>, Event<'static>) -> Result<T, XmlError>,
) -> Result<Vec<T>, XmlError> {
    if cursor.expect_collection_start(name)? {
        let mut values = Vec::new();
        loop {
            let event = cursor.next()?;
            if let Event::End(end) = &event {
                if end.name().as_ref() == name.as_bytes() {
                    return Ok(values);
                }
            }
            values.push(parse_item(cursor, event)?);
        }
    } else {
        Ok(Vec::new())
    }
}

/// Owns the streaming XML reader and its reusable event buffer.
struct Cursor<'a> {
    /// The underlying XML reader.
    reader: Reader<&'a [u8]>,
    /// Storage reused by the XML reader.
    buffer: Vec<u8>,
    /// The next event when an optional element was not present.
    pending: Option<Event<'static>>,
}

/// Determines whether XML text contains only insignificant whitespace.
fn is_whitespace_text(text: &quick_xml::events::BytesText<'_>) -> bool {
    let bytes: &[u8] = text.as_ref();
    bytes.iter().all(u8::is_ascii_whitespace)
}

impl<'a> Cursor<'a> {
    /// Creates a cursor for an XML document.
    fn new(xml: &'a str) -> Self {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(false);
        Self {
            reader,
            buffer: Vec::new(),
            pending: None,
        }
    }

    /// Returns the next meaningful XML event.
    fn next(&mut self) -> Result<Event<'static>, XmlError> {
        if let Some(event) = self.pending.take() {
            return Ok(event);
        }
        loop {
            self.buffer.clear();
            let event = self.reader.read_event_into(&mut self.buffer)?.into_owned();
            if matches!(
                &event,
                Event::Text(text) if is_whitespace_text(text)
            ) || matches!(
                &event,
                Event::Comment(_) | Event::Decl(_) | Event::PI(_) | Event::DocType(_)
            ) {
                continue;
            }
            if matches!(event, Event::Text(_) | Event::CData(_)) {
                return Err(XmlError::InvalidStructure {
                    message: "simulation file elements may not contain text".to_owned(),
                });
            }
            return Ok(event);
        }
    }

    /// Reads an optional empty element without consuming the following event.
    fn expect_optional_empty(
        &mut self,
        name: &str,
    ) -> Result<Option<BytesStart<'static>>, XmlError> {
        let event = self.next()?;
        if let Event::Empty(start) = &event {
            if start.name().as_ref() == name.as_bytes() {
                return Ok(Some(start.to_owned()));
            }
        }
        if let Event::Start(start) = &event {
            if start.name().as_ref() == name.as_bytes() {
                let start = start.to_owned();
                self.expect_end(name)?;
                return Ok(Some(start));
            }
        }
        self.pending = Some(event);
        Ok(None)
    }

    /// Reads the next event as a named opening element.
    fn expect_start(&mut self, name: &str) -> Result<BytesStart<'static>, XmlError> {
        let event = self.next()?;
        Self::expect_start_event(&event, name)
    }

    /// Validates an event as a named opening element.
    fn expect_start_event(
        event: &Event<'static>,
        name: &str,
    ) -> Result<BytesStart<'static>, XmlError> {
        if let Event::Start(start) = &event {
            if start.name().as_ref() == name.as_bytes() {
                return Ok(start.to_owned());
            }
        }
        Err(unexpected_event(event, name))
    }

    /// Reads the next event as a named empty element.
    fn expect_empty(&mut self, name: &str) -> Result<BytesStart<'static>, XmlError> {
        let event = self.next()?;
        self.expect_empty_element(&event, name)
    }

    /// Validates an event as a named empty element.
    fn expect_empty_element(
        &mut self,
        event: &Event<'static>,
        name: &str,
    ) -> Result<BytesStart<'static>, XmlError> {
        if let Event::Empty(start) = &event {
            if start.name().as_ref() == name.as_bytes() {
                return Ok(start.to_owned());
            }
        }
        if let Event::Start(start) = &event {
            if start.name().as_ref() == name.as_bytes() {
                self.expect_end(name)?;
                return Ok(start.to_owned());
            }
        }
        Err(unexpected_event(event, name))
    }

    /// Reads the start of a collection, reporting whether it is nonempty.
    fn expect_collection_start(&mut self, name: &str) -> Result<bool, XmlError> {
        let event = self.next()?;
        if let Event::Start(start) = &event {
            if start.name().as_ref() == name.as_bytes() {
                return Ok(true);
            }
        }
        if let Event::Empty(start) = &event {
            if start.name().as_ref() == name.as_bytes() {
                return Ok(false);
            }
        }
        Err(unexpected_event(&event, name))
    }

    /// Reads a named closing element.
    fn expect_end(&mut self, name: &str) -> Result<(), XmlError> {
        let event = self.next()?;
        if let Event::End(end) = &event {
            if end.name().as_ref() == name.as_bytes() {
                return Ok(());
            }
        }
        Err(unexpected_event(&event, &format!("closing {name}")))
    }

    /// Ensures no XML events follow the root element.
    fn expect_eof(&mut self) -> Result<(), XmlError> {
        let event = self.next()?;
        if matches!(event, Event::Eof) {
            Ok(())
        } else {
            Err(unexpected_event(&event, "end of document"))
        }
    }
}

/// Validates and collects allowed attributes from an element.
fn attributes(
    start: &BytesStart<'_>,
    element: &str,
    allowed: &[&str],
) -> Result<Vec<(String, String)>, XmlError> {
    let mut values = Vec::new();
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|error| XmlError::InvalidStructure {
            message: format!("invalid attributes on element `{element}`: {error}"),
        })?;
        let name = std::str::from_utf8(attribute.key.as_ref()).map_err(|_| {
            XmlError::InvalidStructure {
                message: format!("element `{element}` contains a non-UTF-8 attribute name"),
            }
        })?;
        if !allowed.contains(&name) {
            return Err(XmlError::InvalidStructure {
                message: format!("element `{element}` has unexpected attribute `{name}`"),
            });
        }
        if values.iter().any(|(existing, _)| existing == name) {
            return Err(XmlError::InvalidStructure {
                message: format!("element `{element}` has duplicate attribute `{name}`"),
            });
        }
        let value = attribute
            .unescape_value()
            .map_err(|error| XmlError::InvalidStructure {
                message: format!("element `{element}` has an invalid attribute value: {error}"),
            })?;
        values.push((name.to_owned(), value.into_owned()));
    }
    Ok(values)
}

/// Collects XML item attributes before validating them for an item type.
fn item_xml_attributes(start: &BytesStart<'_>) -> Result<Vec<(String, String)>, XmlError> {
    attributes(
        start,
        "item",
        &[
            "key",
            "type",
            "description",
            "default",
            "value",
            "extension-filter",
            "is-input",
            "preferred-unit",
            "units",
            "unit-family",
        ],
    )
}

/// Rejects attributes which do not apply to the parsed item type.
fn validate_item_attributes(
    attributes: &[(String, String)],
    allowed: &[&str],
) -> Result<(), XmlError> {
    for (name, _) in attributes {
        if !allowed.contains(&name.as_str()) {
            return Err(XmlError::InvalidStructure {
                message: format!("item has unexpected attribute `{name}` for its type"),
            });
        }
    }
    Ok(())
}

/// Gets a required attribute value.
fn required_attribute(
    attributes: &[(String, String)],
    element: &str,
    name: &str,
) -> Result<String, XmlError> {
    attributes
        .iter()
        .find(|(attribute, _)| attribute == name)
        .map(|(_, value)| value.clone())
        .ok_or_else(|| XmlError::MissingAttribute {
            element: element.to_owned(),
            attribute: name.to_owned(),
        })
}

/// Gets an optional attribute value.
fn optional_attribute(attributes: &[(String, String)], name: &str) -> Option<String> {
    attributes
        .iter()
        .find(|(attribute, _)| attribute == name)
        .map(|(_, value)| value.clone())
}

/// Parses an unsigned 16-bit integer attribute.
fn parse_u16(value: &str, element: &str, attribute: &str) -> Result<u16, XmlError> {
    value.parse().map_err(|_| XmlError::InvalidAttribute {
        element: element.to_owned(),
        attribute: attribute.to_owned(),
        value: value.to_owned(),
        expected: "an unsigned 16-bit integer",
    })
}

/// Parses a validated simulation identifier.
fn parse_identifier(value: &str) -> Result<Identifier, XmlError> {
    Identifier::new(value).map_err(Into::into)
}

/// Parses a validated datastore parameter key.
fn parse_parameter_key(value: &str) -> Result<ParameterKey, XmlError> {
    ParameterKey::new(ShareableString::from(value)).map_err(|_| XmlError::InvalidAttribute {
        element: "item".to_owned(),
        attribute: "key".to_owned(),
        value: value.to_owned(),
        expected: "a datastore parameter key (`p_` followed by [a-z][0-9a-z_]*)",
    })
}

/// Parses a validated datastore variable key.
fn parse_variable_key(value: &str) -> Result<VariableKey, XmlError> {
    VariableKey::new(ShareableString::from(value)).map_err(|_| XmlError::InvalidAttribute {
        element: "item".to_owned(),
        attribute: "key".to_owned(),
        value: value.to_owned(),
        expected: "a datastore variable key (`v_` followed by [a-z][0-9a-z_]*)",
    })
}

/// Constructs an error for a duplicate datastore object key.
fn duplicate_object_key(kind: &str) -> XmlError {
    XmlError::InvalidStructure {
        message: format!("duplicate {kind} object key"),
    }
}

/// Parses a boolean XML attribute.
fn parse_boolean(value: &str, attribute: &str) -> Result<bool, XmlError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(XmlError::InvalidAttribute {
            element: "item".to_owned(),
            attribute: attribute.to_owned(),
            value: value.to_owned(),
            expected: "true or false",
        }),
    }
}

/// Parses a datastore unit identifier.
fn parse_unit_id(value: &str) -> Result<UnitId, XmlError> {
    UnitId::from_unit_id_str(value).ok_or_else(|| XmlError::InvalidAttribute {
        element: "item".to_owned(),
        attribute: "preferred-unit".to_owned(),
        value: value.to_owned(),
        expected: "a datastore unit identifier",
    })
}

/// Parses a datastore unit family name.
fn parse_unit_family(value: &str) -> Result<UnitFamilyId, XmlError> {
    match value {
        "none" => Ok(UnitFamilyId::None),
        "area" => Ok(UnitFamilyId::Area),
        "current" => Ok(UnitFamilyId::Current),
        "length" => Ok(UnitFamilyId::Length),
        "luminous-intensity" => Ok(UnitFamilyId::LuminousIntensity),
        "amount" => Ok(UnitFamilyId::Amount),
        "temperature" => Ok(UnitFamilyId::Temperature),
        "time" => Ok(UnitFamilyId::Time),
        "volume" => Ok(UnitFamilyId::Volume),
        "mass" => Ok(UnitFamilyId::Mass),
        _ => Err(XmlError::InvalidAttribute {
            element: "item".to_owned(),
            attribute: "unit-family".to_owned(),
            value: value.to_owned(),
            expected: "a datastore unit family",
        }),
    }
}

/// Returns the XML spelling of a datastore unit family.
const fn unit_family_name(family: UnitFamilyId) -> &'static str {
    match family {
        UnitFamilyId::None => "none",
        UnitFamilyId::Area => "area",
        UnitFamilyId::Current => "current",
        UnitFamilyId::Length => "length",
        UnitFamilyId::LuminousIntensity => "luminous-intensity",
        UnitFamilyId::Amount => "amount",
        UnitFamilyId::Temperature => "temperature",
        UnitFamilyId::Time => "time",
        UnitFamilyId::Volume => "volume",
        UnitFamilyId::Mass => "mass",
    }
}

/// Parses a normalized coordinate attribute.
fn parse_coordinate(
    value: &str,
    element: &str,
    attribute: &str,
) -> Result<NormalizedCoordinate, XmlError> {
    let coordinate = value.parse().map_err(|_| XmlError::InvalidAttribute {
        element: element.to_owned(),
        attribute: attribute.to_owned(),
        value: value.to_owned(),
        expected: "a normalized finite decimal from 0 through 1",
    })?;
    NormalizedCoordinate::new(coordinate).map_err(Into::into)
}

/// Parses a port type attribute.
fn parse_port_type(value: &str, element: &str, attribute: &str) -> Result<PortType, XmlError> {
    match value {
        "input" => Ok(PortType::Input),
        "output" => Ok(PortType::Output),
        "bidirectional" => Ok(PortType::Bidirectional),
        _ => Err(XmlError::InvalidAttribute {
            element: element.to_owned(),
            attribute: attribute.to_owned(),
            value: value.to_owned(),
            expected: "input, output, or bidirectional",
        }),
    }
}

/// Parses a connection style attribute.
fn parse_connection_style(
    value: &str,
    element: &str,
    attribute: &str,
) -> Result<ConnectionStyle, XmlError> {
    match value {
        "solid" => Ok(ConnectionStyle::Solid),
        "dashed" => Ok(ConnectionStyle::Dashed),
        "dotted" => Ok(ConnectionStyle::Dotted),
        _ => Err(XmlError::InvalidAttribute {
            element: element.to_owned(),
            attribute: attribute.to_owned(),
            value: value.to_owned(),
            expected: "solid, dashed, or dotted",
        }),
    }
}

/// Returns the XML name of a port type.
const fn port_type_name(port_type: PortType) -> &'static str {
    match port_type {
        PortType::Input => "input",
        PortType::Output => "output",
        PortType::Bidirectional => "bidirectional",
    }
}

/// Returns the XML name of a connection style.
const fn connection_style_name(style: ConnectionStyle) -> &'static str {
    match style {
        ConnectionStyle::Solid => "solid",
        ConnectionStyle::Dashed => "dashed",
        ConnectionStyle::Dotted => "dotted",
    }
}

/// Constructs an error for an unexpected XML event.
fn unexpected_event(event: &Event<'_>, expected: &str) -> XmlError {
    XmlError::InvalidStructure {
        message: format!("expected {expected}, found {}", event_name(event)),
    }
}

/// Describes an XML event for an error message.
fn event_name(event: &Event<'_>) -> String {
    match event {
        Event::Start(start) | Event::Empty(start) => {
            String::from_utf8_lossy(start.name().as_ref()).into_owned()
        }
        Event::End(end) => format!("closing {}", String::from_utf8_lossy(end.name().as_ref())),
        Event::Eof => "end of document".to_owned(),
        Event::Text(_) => "text".to_owned(),
        Event::CData(_) => "CDATA".to_owned(),
        Event::Comment(_) => "comment".to_owned(),
        Event::Decl(_) => "declaration".to_owned(),
        Event::PI(_) => "processing instruction".to_owned(),
        Event::DocType(_) => "document type".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use datastore::definition::{FolderDefinition, SeparatorDefinition, TabDefinition};
    use datastore::prelude::{
        BooleanDefinition, ChoiceDefinition, FileDefinition, IntegerDefinition, NumberDefinition,
        NumberWithUnitsDefinition, ParameterKey, ParameterObjectDefinition, ParameterObjectFrozen,
        StringDefinition, UnitDefinition, VariableKey, VariableObjectDefinition,
        VariableObjectFrozen,
    };
    use units::{UnitFamilyId, UnitId};

    use super::*;

    /// Unwraps a result in test fixtures.
    fn valid<T>(result: Result<T, ValidationError>) -> T {
        match result {
            Ok(value) => value,
            Err(error) => panic!("test fixture must be valid: {error}"),
        }
    }

    /// Creates a valid simulation identifier.
    fn id(value: &str) -> Identifier {
        valid(Identifier::new(value))
    }

    /// Creates a valid normalized coordinate.
    fn coordinate(value: f64) -> NormalizedCoordinate {
        valid(NormalizedCoordinate::new(value))
    }

    /// Creates a valid datastore parameter key for a test fixture.
    fn parameter_key(value: &str) -> ParameterKey {
        let Ok(key) = ParameterKey::new(ShareableString::from(value)) else {
            panic!("test parameter key must be valid");
        };
        key
    }

    /// Creates a valid datastore variable key for a test fixture.
    fn variable_key(value: &str) -> VariableKey {
        let Ok(key) = VariableKey::new(ShareableString::from(value)) else {
            panic!("test variable key must be valid");
        };
        key
    }

    /// Creates one frozen parameter object fixture.
    fn parameter_object(description: &str) -> ParameterObjectFrozen {
        ParameterObjectFrozen::new(
            ParameterObjectDefinition::builder(description)
                .with(
                    parameter_key("p_enabled"),
                    BooleanDefinition::new_with_default("Enabled", true),
                )
                .finish(),
        )
    }

    /// Creates one frozen variable object fixture.
    fn variable_object(description: &str) -> VariableObjectFrozen {
        VariableObjectFrozen::new(
            VariableObjectDefinition::builder(description)
                .with(
                    variable_key("v_elapsed"),
                    NumberDefinition::new_with_default("Elapsed time", "0"),
                )
                .finish(),
        )
    }

    /// Creates one component instance fixture.
    fn instance(id_value: &str) -> ComponentInstance {
        valid(ComponentInstance::new(
            id(id_value),
            ComponentRectangle::new(1, 2, 30, 40),
            Some("icons/sensor \"&\".svg".to_owned()),
            parameter_object("Sensor parameters"),
            variable_object("Sensor variables"),
            vec![Port::new(
                id("output"),
                PortType::Output,
                PortLocation::new(coordinate(1.0), coordinate(0.5)),
            )],
        ))
    }

    /// Creates component fixtures.
    fn components() -> Vec<InstantiatedComponent> {
        vec![
            InstantiatedComponent::FileBacked(FileBackedComponent::new(
                instance("sensor"),
                2,
                valid(SourceBlake3Digest::new("ab".repeat(32))),
                "sensor & gauge.xml".to_owned(),
                Some("example.sensors@1.0".to_owned()),
            )),
            InstantiatedComponent::BuiltIn(BuiltInComponent::new(
                instance("clock"),
                "builtin.clock".to_owned(),
                1,
            )),
        ]
    }

    /// Creates connection fixtures.
    fn connections() -> Vec<Connection> {
        vec![Connection::new(
            ConnectionEndpoints::new(id("clock"), id("output"), id("sensor"), id("output")),
            ConnectionStyle::Dotted,
            valid(ConnectionLine::new(vec![AbsolutePoint::new(12, 34)])),
        )]
    }

    #[test]
    fn model_round_trips_concrete_datastore_objects_and_escapes_attributes() {
        let model = valid(ModelFile::new(
            ModelMetadata::new(
                1,
                "1.0".to_owned(),
                "1.2".to_owned(),
                "Heating & cooling".to_owned(),
            ),
            vec![ModelSetting::new(
                "label".to_owned(),
                "\"quoted\" & <value>".to_owned(),
            )],
            parameter_object("Model parameters"),
            variable_object("Model variables"),
            components(),
            connections(),
        ));

        let Ok(xml) = model.to_xml_string() else {
            panic!("model fixture must serialize");
        };
        assert!(xml.contains("Heating &amp; cooling"));
        assert!(xml.contains("&quot;quoted&quot; &amp; &lt;value&gt;"));
        assert!(xml.contains("path=\"icons/sensor &quot;&amp;&quot;.svg\""));
        assert!(matches!(ModelFile::from_xml_str(&xml), Ok(parsed) if parsed == model));
    }

    #[test]
    fn component_definition_round_trips_concrete_datastore_objects() {
        let component = valid(ComponentDefinitionFile::new(
            ComponentMetadata::new(
                1,
                "1.0".to_owned(),
                "1.1".to_owned(),
                3,
                "thermostat".to_owned(),
                180,
                90,
            )
            .with_icon_path(Some("thermostat.svg".to_owned())),
            ParameterObjectDefinition::builder("Thermostat parameters")
                .with(
                    parameter_key("p_target"),
                    NumberDefinition::new_with_default("Target temperature", "22"),
                )
                .finish(),
            VariableObjectDefinition::builder("Thermostat variables")
                .with(variable_key("v_state"), StringDefinition::new("State"))
                .finish(),
            vec![Port::new(
                id("input"),
                PortType::Bidirectional,
                PortLocation::new(coordinate(0.0), coordinate(0.5)),
            )],
            components(),
            connections(),
        ));

        let Ok(xml) = component.to_xml_string() else {
            panic!("component fixture must serialize");
        };
        assert!(xml.contains("icon=\"thermostat.svg\""));
        assert!(xml.contains("description=\"Thermostat parameters\""));
        assert!(matches!(
            ComponentDefinitionFile::from_xml_str(&xml),
            Ok(parsed) if parsed == component
        ));
    }

    #[test]
    fn writer_rejects_datastore_item_types_without_xml_representations() {
        let model = valid(ModelFile::new(
            ModelMetadata::new(1, "1.0".to_owned(), "1.0".to_owned(), "test".to_owned()),
            Vec::new(),
            ParameterObjectFrozen::new(
                ParameterObjectDefinition::builder("Parameters")
                    .with(
                        parameter_key("p_choice"),
                        ChoiceDefinition::new("Choice", Vec::new()),
                    )
                    .finish(),
            ),
            VariableObjectFrozen::new(VariableObjectDefinition::builder("Variables").finish()),
            Vec::new(),
            Vec::new(),
        ));

        assert!(matches!(
            model.to_xml_string(),
            Err(XmlError::UnsupportedItem { item_type, .. }) if item_type == "choice"
        ));
    }

    #[test]
    fn supported_datastore_item_types_round_trip() {
        let model = valid(ModelFile::new(
            ModelMetadata::new(1, "1.0".to_owned(), "1.0".to_owned(), "test".to_owned()),
            Vec::new(),
            ParameterObjectFrozen::new(
                ParameterObjectDefinition::builder("Parameters")
                    .with(
                        parameter_key("p_boolean"),
                        BooleanDefinition::new_with_default("Boolean", true),
                    )
                    .with(
                        parameter_key("p_file"),
                        FileDefinition::new_with_default("File", "*.csv", true, "input.csv"),
                    )
                    .with(
                        parameter_key("p_folder"),
                        FolderDefinition::new_with_default("Folder", false, "outputs"),
                    )
                    .with(
                        parameter_key("p_integer"),
                        IntegerDefinition::new_with_default("Integer", "42"),
                    )
                    .with(
                        parameter_key("p_number"),
                        NumberDefinition::new_with_default("Number", "1.5"),
                    )
                    .with(
                        parameter_key("p_number_with_units"),
                        NumberWithUnitsDefinition::new_with_default(
                            "Number with units",
                            "20",
                            UnitId::Temperature_Celsius,
                        ),
                    )
                    .with(
                        parameter_key("p_string"),
                        StringDefinition::new_with_default("String", "text"),
                    )
                    .with(
                        parameter_key("p_unit"),
                        UnitDefinition::new_with_default(
                            "Unit",
                            UnitFamilyId::Temperature,
                            "u_temperature_celsius",
                        ),
                    )
                    .with(parameter_key("p_tab"), TabDefinition::new("Tab"))
                    .with(
                        parameter_key("p_separator"),
                        SeparatorDefinition::new("Separator"),
                    )
                    .finish(),
            ),
            VariableObjectFrozen::new(VariableObjectDefinition::builder("Variables").finish()),
            Vec::new(),
            Vec::new(),
        ));

        let Ok(xml) = model.to_xml_string() else {
            panic!("supported datastore objects must serialize");
        };
        let Ok(parsed) = ModelFile::from_xml_str(&xml) else {
            panic!("serialized model must parse");
        };
        assert_eq!(parsed, model);
    }

    #[test]
    fn parser_rejects_unrepresentable_item_types_explicitly() {
        let xml = r#"
            <model file-version="1" minimum-version="1.0" built-version="1.0" name="test">
              <model-settings/>
              <parameters description="Parameters">
                <item key="p_choice" type="choice" description="Choice"/>
              </parameters>
              <variables description="Variables"/>
              <components/>
              <connections/>
            </model>
        "#;

        assert!(matches!(
            ModelFile::from_xml_str(xml),
            Err(XmlError::UnsupportedItem { item_type, .. }) if item_type == "choice"
        ));
    }

    #[test]
    fn parser_requires_datastore_object_descriptions_and_keys() {
        let xml = r#"
            <model file-version="1" minimum-version="1.0" built-version="1.0" name="test">
              <model-settings/>
              <parameters>
                <item key="target" type="number" description="Target" value="1"/>
              </parameters>
              <variables description="Variables"/>
              <components/>
              <connections/>
            </model>
        "#;

        assert!(matches!(
            ModelFile::from_xml_str(xml),
            Err(XmlError::MissingAttribute { element, attribute })
                if element == "parameters" && attribute == "description"
        ));
    }

    #[test]
    fn documented_examples_parse() {
        assert!(
            ModelFile::from_xml_str(include_str!("../../../docs/simulation-model.hxm")).is_ok()
        );
        assert!(
            ComponentDefinitionFile::from_xml_str(include_str!(
                "../../../docs/simulation-component.hrc"
            ))
            .is_ok()
        );
    }
}
