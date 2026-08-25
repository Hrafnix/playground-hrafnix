use crate::diagnostic::{Diagnostic, DiagnosticCategory, DiagnosticSeverity};
use crate::document::{
    COMPONENT_SCHEMA_VERSION, CustomComponentDocument, MODEL_SCHEMA_VERSION, ModelDocument,
    SchemaVersion,
};
use serde::Deserialize;

/// Last custom-component schema accepted through an explicit migration.
const COMPONENT_SCHEMA_VERSION_1_0: SchemaVersion = SchemaVersion { major: 1, minor: 0 };

/// Minimal envelope read before deserializing a complete document.
#[derive(Debug, Deserialize)]
struct VersionEnvelope {
    /// Header containing the document schema version.
    header: VersionHeader,
}

/// Minimal header needed for schema dispatch.
#[derive(Debug, Deserialize)]
struct VersionHeader {
    /// Persisted schema version.
    schema_version: SchemaVersion,
}

/// Parses a native JSON model after enforcing its schema version.
///
/// # Errors
///
/// Returns stable diagnostics for malformed JSON and unsupported schemas.
pub fn load_model_json(source: &str) -> Result<ModelDocument, Diagnostic> {
    enforce_version(source, MODEL_SCHEMA_VERSION)?;
    serde_json::from_str(source).map_err(malformed_document)
}

/// Parses a native JSON custom component after enforcing its schema version.
///
/// # Errors
///
/// Returns stable diagnostics for malformed JSON and unsupported schemas.
pub fn load_custom_component_json(source: &str) -> Result<CustomComponentDocument, Diagnostic> {
    let mut value: serde_json::Value = serde_json::from_str(source).map_err(malformed_document)?;
    let envelope: VersionEnvelope =
        serde_json::from_value(value.clone()).map_err(malformed_document)?;
    if envelope.header.schema_version == COMPONENT_SCHEMA_VERSION_1_0 {
        migrate_component_1_0_to_1_1(&mut value)?;
    } else if envelope.header.schema_version != COMPONENT_SCHEMA_VERSION {
        return Err(unsupported_schema());
    }
    serde_json::from_value(value).map_err(malformed_document)
}

/// Serializes a model using stable pretty-printed native JSON.
///
/// # Errors
///
/// Returns the serializer error if a value cannot be represented as JSON.
pub fn save_model_json(document: &ModelDocument) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(document)
}

/// Serializes a custom component using stable pretty-printed native JSON.
///
/// # Errors
///
/// Returns the serializer error if a value cannot be represented as JSON.
pub fn save_custom_component_json(
    document: &CustomComponentDocument,
) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(document)
}

/// Reads and checks the version envelope shared by native documents.
fn enforce_version(source: &str, supported: SchemaVersion) -> Result<(), Diagnostic> {
    let envelope: VersionEnvelope = serde_json::from_str(source).map_err(malformed_document)?;
    if envelope.header.schema_version != supported {
        return Err(unsupported_schema());
    }
    Ok(())
}

/// Adds parameter mappings and an audit record to a schema 1.0 component value.
fn migrate_component_1_0_to_1_1(value: &mut serde_json::Value) -> Result<(), Diagnostic> {
    let object = value.as_object_mut().ok_or_else(malformed_value)?;
    object
        .entry("parameter_mappings")
        .or_insert_with(|| serde_json::json!([]));
    let header = object
        .get_mut("header")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or_else(malformed_value)?;
    header.insert(
        "schema_version".into(),
        serde_json::to_value(COMPONENT_SCHEMA_VERSION).map_err(malformed_document)?,
    );
    let migrations = header
        .get_mut("migrations")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or_else(malformed_value)?;
    migrations.push(serde_json::json!({
        "from": COMPONENT_SCHEMA_VERSION_1_0,
        "to": COMPONENT_SCHEMA_VERSION,
        "migration_id": "simulation_component_1_0_to_1_1_parameter_mappings"
    }));
    Ok(())
}

/// Creates the stable unsupported-schema diagnostic.
fn unsupported_schema() -> Diagnostic {
    Diagnostic::new(
        DiagnosticSeverity::Error,
        DiagnosticCategory::Validation,
        None,
        Some("schema_version".into()),
        "simulation_persistence_unsupported_schema",
    )
}

/// Creates the stable malformed-document diagnostic without a serde error.
fn malformed_value() -> Diagnostic {
    Diagnostic::new(
        DiagnosticSeverity::Error,
        DiagnosticCategory::Validation,
        None,
        None,
        "simulation_persistence_malformed_document",
    )
}

/// Converts serde failures into a stable public diagnostic.
fn malformed_document(_error: serde_json::Error) -> Diagnostic {
    Diagnostic::new(
        DiagnosticSeverity::Error,
        DiagnosticCategory::Validation,
        None,
        None,
        "simulation_persistence_malformed_document",
    )
}

#[cfg(test)]
mod tests {
    use super::{
        load_custom_component_json, load_model_json, save_custom_component_json, save_model_json,
    };
    use crate::component::{ComponentTypeId, ParameterDefinition, PortDefinition, PortDirection};
    use crate::document::{
        ArtifactRevision, COMPONENT_SCHEMA_VERSION, CanvasPosition, ComponentInstance,
        ComponentReference, ComponentTestCase, Composition, Connection, CustomComponentDocument,
        DependencyLock, DocumentHeader, LoggingPolicy, MODEL_SCHEMA_VERSION, ModelDocument,
        PortEndpoint, ProbeDefinition, PublicPortDefinition, PublicPortMapping, SimulationSettings,
        StateDeclaration,
    };
    use crate::identity::{ComponentId, ConnectionId, DocumentId, PortId, ProbeId, SystemId};
    use crate::parameter::ParameterValueType;
    use crate::timing::FixedStepSemantics;
    use std::collections::BTreeMap;
    use units::UnitId;

    /// Creates shared deterministic metadata for round-trip fixtures.
    fn header(document_id: u128, component: bool) -> DocumentHeader {
        DocumentHeader {
            schema_version: if component {
                COMPONENT_SCHEMA_VERSION
            } else {
                MODEL_SCHEMA_VERSION
            },
            document_id: DocumentId::from_raw(document_id),
            title: "Fixture".into(),
            description: "Round-trip fixture".into(),
            author: "tests".into(),
            created_at: "2026-08-24T00:00:00Z".into(),
            updated_at: "2026-08-24T00:00:00Z".into(),
            migrations: vec![],
        }
    }

    /// Creates a representative built-in instance.
    fn instance() -> ComponentInstance {
        ComponentInstance {
            id: ComponentId::from_raw(3),
            name: "gain".into(),
            component: ComponentReference::BuiltIn {
                type_id: ComponentTypeId::new("signal.gain").unwrap(),
            },
            parameter_overrides: BTreeMap::from([("gain".into(), "2.0".into())]),
            enabled: true,
            position: CanvasPosition { x: 10.0, y: 20.0 },
        }
    }

    /// Creates a representative custom-component reference.
    fn custom_instance() -> ComponentInstance {
        ComponentInstance {
            id: ComponentId::from_raw(5),
            name: "controller".into(),
            component: ComponentReference::Custom {
                document_id: DocumentId::from_raw(50),
                revision: ArtifactRevision("1.0.0".into()),
                source: "components/controller.json".into(),
            },
            parameter_overrides: BTreeMap::new(),
            enabled: true,
            position: CanvasPosition { x: 30.0, y: 20.0 },
        }
    }

    /// Creates a representative model containing references, probes, settings, and lock data.
    fn model_fixture() -> ModelDocument {
        ModelDocument {
            header: header(1, false),
            root: Composition {
                system_id: SystemId::from_raw(2),
                components: vec![instance(), custom_instance()],
                connections: vec![Connection {
                    id: ConnectionId::from_raw(6),
                    source: PortEndpoint {
                        component_id: ComponentId::from_raw(3),
                        port_key: "out".into(),
                    },
                    target: PortEndpoint {
                        component_id: ComponentId::from_raw(5),
                        port_key: "in".into(),
                    },
                    label: Some("control signal".into()),
                    route: vec![CanvasPosition { x: 20.0, y: 20.0 }],
                }],
                annotations: BTreeMap::from([("note".into(), "fixture".into())]),
            },
            simulation: SimulationSettings {
                start_time: 0.0,
                stop_time: 1.0,
                timestep: 0.1,
                maximum_steps: 100,
                random_seed: 42,
                logging: LoggingPolicy::EveryStep,
                semantics: FixedStepSemantics::default(),
            },
            probes: vec![ProbeDefinition {
                id: ProbeId::from_raw(4),
                target: PortEndpoint {
                    component_id: ComponentId::from_raw(5),
                    port_key: "out".into(),
                },
                display_name: "output".into(),
                plot_group: Some("main".into()),
            }],
            dependencies: vec![DependencyLock {
                document_id: DocumentId::from_raw(50),
                revision: ArtifactRevision("1.0.0".into()),
                checksum: "abc123".into(),
                source: "components/controller.json".into(),
            }],
        }
    }

    /// Creates a representative custom component with a strict public/private boundary.
    fn component_fixture() -> CustomComponentDocument {
        let parameter = ParameterDefinition {
            key: "gain".into(),
            display_name: "Gain".into(),
            description: "Multiplier".into(),
            value_type: ParameterValueType::Scalar,
            default_expression: "1.0".into(),
        };
        let port = PublicPortDefinition {
            id: PortId::from_raw(12),
            definition: PortDefinition {
                key: "out".into(),
                display_name: "Output".into(),
                description: "Signal output".into(),
                direction: PortDirection::Output,
                value_type: ParameterValueType::Scalar,
                unit: Some(UnitId::None),
                required: false,
            },
        };

        CustomComponentDocument {
            header: header(10, true),
            revision: ArtifactRevision("1.0.0".into()),
            public_parameters: vec![parameter],
            public_ports: vec![port],
            implementation: Composition {
                system_id: SystemId::from_raw(11),
                components: vec![instance()],
                connections: vec![],
                annotations: BTreeMap::new(),
            },
            port_mappings: vec![PublicPortMapping {
                public_port_id: PortId::from_raw(12),
                internal: PortEndpoint {
                    component_id: ComponentId::from_raw(3),
                    port_key: "out".into(),
                },
            }],
            parameter_mappings: vec![],
            state: vec![StateDeclaration {
                key: "previous".into(),
                value_type: ParameterValueType::Scalar,
                initial_expression: "0.0".into(),
            }],
            test_cases: vec![ComponentTestCase {
                name: "identity".into(),
                parameter_overrides: BTreeMap::new(),
                simulation: None,
                inputs: BTreeMap::new(),
                expected_outputs: vec![],
                expected_behavior: "Output follows input".into(),
            }],
            dependencies: vec![],
            documentation: "Reusable gain wrapper".into(),
            replacement: None,
        }
    }

    #[test]
    fn model_fixture_has_exact_json_round_trip() {
        let json = save_model_json(&model_fixture()).unwrap();
        let loaded = load_model_json(&json).unwrap();

        assert_eq!(save_model_json(&loaded).unwrap(), json);
        assert_eq!(loaded, model_fixture());
    }

    #[test]
    fn custom_component_fixture_has_exact_json_round_trip() {
        let json = save_custom_component_json(&component_fixture()).unwrap();
        let loaded = load_custom_component_json(&json).unwrap();

        assert_eq!(save_custom_component_json(&loaded).unwrap(), json);
        assert_eq!(loaded, component_fixture());
    }

    #[test]
    fn unsupported_model_schema_has_stable_diagnostic() {
        let mut value = serde_json::to_value(model_fixture()).unwrap();
        value["header"]["schema_version"]["major"] = serde_json::json!(99);
        let diagnostic = load_model_json(&value.to_string()).unwrap_err();

        assert_eq!(
            diagnostic.message_key().as_str(),
            "simulation_persistence_unsupported_schema"
        );
        assert_eq!(diagnostic.field().unwrap().as_str(), "schema_version");
    }

    #[test]
    fn unsupported_component_schema_has_stable_diagnostic() {
        let mut value = serde_json::to_value(component_fixture()).unwrap();
        value["header"]["schema_version"]["minor"] = serde_json::json!(99);
        let diagnostic = load_custom_component_json(&value.to_string()).unwrap_err();

        assert_eq!(
            diagnostic.message_key().as_str(),
            "simulation_persistence_unsupported_schema"
        );
        assert_eq!(diagnostic.field().unwrap().as_str(), "schema_version");
    }

    #[test]
    fn migrates_component_schema_1_0_with_audit_record() {
        let mut value = serde_json::to_value(component_fixture()).unwrap();
        value["header"]["schema_version"]["minor"] = serde_json::json!(0);
        value["header"]["migrations"] = serde_json::json!([]);
        value.as_object_mut().unwrap().remove("parameter_mappings");

        let migrated = load_custom_component_json(&value.to_string()).unwrap();

        assert_eq!(migrated.header.schema_version, COMPONENT_SCHEMA_VERSION);
        assert!(migrated.parameter_mappings.is_empty());
        assert_eq!(
            migrated.header.migrations[0].migration_id.as_str(),
            "simulation_component_1_0_to_1_1_parameter_mappings"
        );
    }

    #[test]
    fn malformed_document_has_stable_diagnostic() {
        let diagnostic = load_model_json("not json").unwrap_err();

        assert_eq!(
            diagnostic.message_key().as_str(),
            "simulation_persistence_malformed_document"
        );
    }
}
