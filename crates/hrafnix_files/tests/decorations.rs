//! Integration tests for background and foreground decoration persistence.

#![allow(clippy::unwrap_used, clippy::indexing_slicing)]

#[allow(dead_code)]
mod common;

use camino::Utf8Path;
use common::{METADATA, TestDirectory};
use hrafnix_files::background_decoration::data::{
    BackgroundDecoration, BackgroundDecorationKind, FrameStyle, RectangleStyle,
    RoundedRectangleStyle,
};
use hrafnix_files::common::gui::{Dimensions, Position};
use hrafnix_files::component::{
    data::ComponentFile, reader::read_component_file, writer::write_component_file,
};
use hrafnix_files::foreground_decoration::data::{
    DecorationImage, ForegroundDecoration, ForegroundDecorationKind, ImageFit, TextStyle,
};
use hrafnix_files::metadata::data::Metadata;
use hrafnix_files::model::{data::ModelFile, reader::read_model_file, writer::write_model_file};
use hrafnix_message::message::Message;
use serde_json::{Value, json};
use std::fs;

/// The three background decorations from the original schema example.
const BACKGROUND: &str = r##"[
    {
        "id": "background", "type": "rectangle",
        "position": {"x": 0, "y": 0},
        "dimensions": {"width": 500, "height": 200},
        "style": {"fill-color": "#f0f0f0", "stroke-color": "#000000", "stroke-width": 1}
    },
    {
        "id": "sensor_region", "type": "rounded-rectangle",
        "position": {"x": 140, "y": 0},
        "dimensions": {"width": 140, "height": 70},
        "style": {
            "fill-color": "#e8f4ff", "stroke-color": "#93c5fd",
            "stroke-width": 1, "corner-radius": 8
        }
    },
    {
        "id": "heater_frame", "type": "frame",
        "position": {"x": 310, "y": 0},
        "dimensions": {"width": 140, "height": 70},
        "style": {"stroke-color": "#f97316", "stroke-width": 2}
    }
]"##;

/// The two foreground decorations from the original schema example.
const FOREGROUND: &str = r##"[
    {
        "id": "title", "type": "text",
        "position": {"x": 10, "y": 10},
        "dimensions": {"width": 480, "height": 30},
        "style": {"font-size": 20, "font-weight": "bold", "text-color": "#000000"},
        "text": "Heater Simulation Model"
    },
    {
        "id": "model_icon", "type": "image",
        "position": {"x": 455, "y": 10},
        "dimensions": {"width": 30, "height": 30},
        "image": {"path": "heater.svg", "fit": "contain"}
    }
]"##;

/// Both ordered decoration layers, independent of the file's root type.
#[derive(Debug, Clone, Default, PartialEq)]
struct Layers {
    /// Decorations behind the contents.
    background: Vec<BackgroundDecoration>,
    /// Decorations in front of the contents.
    foreground: Vec<ForegroundDecoration>,
}

/// File formats that share the decoration schema.
#[derive(Debug, Clone, Copy)]
enum FileKind {
    /// A model file.
    Model,
    /// A component file.
    Component,
}

impl FileKind {
    /// Returns the JSON root key.
    const fn root(self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Component => "component",
        }
    }

    /// Reads the layers through the public file reader.
    fn read(self, path: &Utf8Path) -> Result<Layers, Message> {
        match self {
            Self::Model => read_model_file(path).map(|file| Layers {
                background: file.background_decorations().to_vec(),
                foreground: file.foreground_decorations().to_vec(),
            }),
            Self::Component => read_component_file(path).map(|file| Layers {
                background: file.background_decorations().to_vec(),
                foreground: file.foreground_decorations().to_vec(),
            }),
        }
    }

    /// Writes the layers through the public file writer.
    fn write(self, path: &Utf8Path, layers: &Layers) -> Result<(), Message> {
        match self {
            Self::Model => write_model_file(
                path,
                &ModelFile::new(
                    Metadata::default(),
                    layers.background.clone(),
                    layers.foreground.clone(),
                ),
            ),
            Self::Component => write_component_file(
                path,
                &ComponentFile::new(
                    Metadata::default(),
                    layers.background.clone(),
                    layers.foreground.clone(),
                ),
            ),
        }
    }
}

/// Returns the expected data for the original schema example.
fn example_layers() -> Layers {
    Layers {
        background: vec![
            BackgroundDecoration::new(
                "background".to_owned(),
                position(0, 0),
                dimensions(500, 200),
                BackgroundDecorationKind::Rectangle {
                    style: Some(RectangleStyle::new(
                        Some("#f0f0f0".to_owned()),
                        Some("#000000".to_owned()),
                        Some(1.0),
                    )),
                },
            ),
            BackgroundDecoration::new(
                "sensor_region".to_owned(),
                position(140, 0),
                dimensions(140, 70),
                BackgroundDecorationKind::RoundedRectangle {
                    style: Some(RoundedRectangleStyle::new(
                        Some("#e8f4ff".to_owned()),
                        Some("#93c5fd".to_owned()),
                        Some(1.0),
                        Some(8.0),
                    )),
                },
            ),
            BackgroundDecoration::new(
                "heater_frame".to_owned(),
                position(310, 0),
                dimensions(140, 70),
                BackgroundDecorationKind::Frame {
                    style: Some(FrameStyle::new(Some("#f97316".to_owned()), Some(2.0))),
                },
            ),
        ],
        foreground: vec![
            ForegroundDecoration::new(
                "title".to_owned(),
                position(10, 10),
                dimensions(480, 30),
                ForegroundDecorationKind::Text {
                    text: "Heater Simulation Model".to_owned(),
                    style: Some(TextStyle::new(
                        Some(20.0),
                        Some("bold".to_owned()),
                        Some("#000000".to_owned()),
                    )),
                },
            ),
            ForegroundDecoration::new(
                "model_icon".to_owned(),
                position(455, 10),
                dimensions(30, 30),
                ForegroundDecorationKind::Image(DecorationImage::new(
                    "heater.svg".into(),
                    ImageFit::Contain,
                )),
            ),
        ],
    }
}

/// Constructs a position without obscuring example decoration data.
const fn position(x: u32, y: u32) -> Position {
    Position { x, y }
}

/// Constructs dimensions without obscuring example decoration data.
const fn dimensions(width: u16, height: u16) -> Dimensions {
    Dimensions { width, height }
}

/// Wraps extra root fields with the required file version and metadata.
fn document(kind: FileKind, extra: &str) -> String {
    format!(
        "{{\"file-version\":1,\"{}\":{{\"metadata\":{METADATA}{extra}}}}}",
        kind.root()
    )
}

#[test]
fn file_accessors_preserve_metadata_and_drawing_order() {
    let layers = example_layers();
    let metadata = common::metadata("model");
    let model = ModelFile::new(
        metadata.clone(),
        layers.background.clone(),
        layers.foreground.clone(),
    );
    let component = ComponentFile::new(
        metadata.clone(),
        layers.background.clone(),
        layers.foreground.clone(),
    );

    assert_eq!(metadata.display_name(), "Example");
    assert_eq!(metadata.description(), "An example model.");
    assert_eq!(metadata.author(), "Hrafnix");
    assert_eq!(metadata.minimum_version_major(), 1);
    assert_eq!(metadata.minimum_version_minor(), 2);
    assert_eq!(model.metadata(), &metadata);
    assert_eq!(component.metadata(), &metadata);
    assert_eq!(model.background_decorations(), layers.background.as_slice());
    assert_eq!(model.foreground_decorations(), layers.foreground.as_slice());
    assert_eq!(
        component.background_decorations(),
        layers.background.as_slice()
    );
    assert_eq!(
        component.foreground_decorations(),
        layers.foreground.as_slice()
    );
}

#[test]
fn decoration_accessors_expose_geometry_payloads_and_styles() {
    let layers = example_layers();
    let rectangle = &layers.background[0];
    assert_eq!(rectangle.id(), "background");
    assert_eq!(rectangle.position(), position(0, 0));
    assert_eq!(rectangle.dimensions(), dimensions(500, 200));
    let BackgroundDecorationKind::Rectangle { style: Some(style) } = rectangle.kind() else {
        panic!("expected a styled rectangle");
    };
    assert_eq!(style.fill_color(), Some("#f0f0f0"));
    assert_eq!(style.stroke_color(), Some("#000000"));
    assert_eq!(style.stroke_width(), Some(1.0));

    let BackgroundDecorationKind::RoundedRectangle { style: Some(style) } =
        layers.background[1].kind()
    else {
        panic!("expected a styled rounded rectangle");
    };
    assert_eq!(style.fill_color(), Some("#e8f4ff"));
    assert_eq!(style.stroke_color(), Some("#93c5fd"));
    assert_eq!(style.stroke_width(), Some(1.0));
    assert_eq!(style.corner_radius(), Some(8.0));

    let BackgroundDecorationKind::Frame { style: Some(style) } = layers.background[2].kind() else {
        panic!("expected a styled frame");
    };
    assert_eq!(style.stroke_color(), Some("#f97316"));
    assert_eq!(style.stroke_width(), Some(2.0));

    let text_decoration = &layers.foreground[0];
    assert_eq!(text_decoration.id(), "title");
    assert_eq!(text_decoration.position(), position(10, 10));
    assert_eq!(text_decoration.dimensions(), dimensions(480, 30));
    let ForegroundDecorationKind::Text {
        text,
        style: Some(style),
    } = text_decoration.kind()
    else {
        panic!("expected styled text");
    };
    assert_eq!(text, "Heater Simulation Model");
    assert_eq!(style.font_size(), Some(20.0));
    assert_eq!(style.font_weight(), Some("bold"));
    assert_eq!(style.text_color(), Some("#000000"));

    let ForegroundDecorationKind::Image(image) = layers.foreground[1].kind() else {
        panic!("expected an image");
    };
    assert_eq!(image.path(), Utf8Path::new("heater.svg"));
    assert_eq!(image.fit(), ImageFit::Contain);
}

#[test]
fn default_metadata_and_style_accessors_preserve_absent_values() {
    let metadata = Metadata::default();
    assert_eq!(metadata.display_name(), "");
    assert_eq!(metadata.description(), "");
    assert_eq!(metadata.author(), "");
    assert_eq!(metadata.minimum_version_major(), 0);
    assert_eq!(metadata.minimum_version_minor(), 0);

    let rectangle = RectangleStyle::default();
    assert_eq!(rectangle.fill_color(), None);
    assert_eq!(rectangle.stroke_color(), None);
    assert_eq!(rectangle.stroke_width(), None);
    let rounded_rectangle = RoundedRectangleStyle::default();
    assert_eq!(rounded_rectangle.fill_color(), None);
    assert_eq!(rounded_rectangle.stroke_color(), None);
    assert_eq!(rounded_rectangle.stroke_width(), None);
    assert_eq!(rounded_rectangle.corner_radius(), None);
    let frame = FrameStyle::default();
    assert_eq!(frame.stroke_color(), None);
    assert_eq!(frame.stroke_width(), None);
    let text = TextStyle::default();
    assert_eq!(text.font_size(), None);
    assert_eq!(text.font_weight(), None);
    assert_eq!(text.text_color(), None);
}

#[test]
fn shared_gui_geometry_round_trips_in_both_decoration_layers() {
    let directory = TestDirectory::new();
    let path = directory.path.join("shared-gui-geometry.json");
    let position = Position { x: 10, y: 20 };
    let dimensions = Dimensions {
        width: 100,
        height: 50,
    };
    let background = BackgroundDecoration::new(
        "background".to_owned(),
        position,
        dimensions,
        BackgroundDecorationKind::Rectangle { style: None },
    );
    let foreground = ForegroundDecoration::new(
        "label".to_owned(),
        background.position(),
        background.dimensions(),
        ForegroundDecorationKind::Text {
            text: "Title".to_owned(),
            style: None,
        },
    );
    let mut copied_position = background.position();
    let mut copied_dimensions = foreground.dimensions();
    copied_position.x = 99;
    copied_dimensions.width = 99;
    assert_eq!(background.position(), position);
    assert_eq!(foreground.dimensions(), dimensions);
    assert_ne!(background.position(), copied_position);
    assert_ne!(foreground.dimensions(), copied_dimensions);
    let layers = Layers {
        background: vec![background],
        foreground: vec![foreground],
    };
    for kind in [FileKind::Model, FileKind::Component] {
        kind.write(&path, &layers).unwrap();
        assert_eq!(kind.read(&path).unwrap(), layers);
    }
}

#[test]
fn all_decoration_types_round_trip_in_their_layers_for_both_file_kinds() {
    let directory = TestDirectory::new();
    let path = directory.path.join("decorations.json");
    for kind in [FileKind::Model, FileKind::Component] {
        let expected = example_layers();
        fs::write(
            &path,
            document(
                kind,
                &format!(
                    ",\"background-decorations\":{BACKGROUND},\"foreground-decorations\":{FOREGROUND}"
                ),
            ),
        )
        .unwrap();

        let layers = kind.read(&path).unwrap();
        assert_eq!(layers, expected);
        kind.write(&path, &layers).unwrap();
        assert_eq!(kind.read(&path).unwrap(), expected);

        let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        let root = &written[kind.root()];
        assert_eq!(written["file-version"], 1);
        assert!(root["background-decorations"].is_array());
        assert!(root["foreground-decorations"].is_array());
        let image = root["foreground-decorations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|decoration| decoration["type"] == "image")
            .unwrap();
        assert!(image.get("style").is_none());
        assert!(image.get("text").is_none());
        assert_eq!(
            image["image"],
            json!({"path": "heater.svg", "fit": "contain"})
        );
    }
}

#[test]
fn missing_empty_and_independent_layers_are_supported() {
    let directory = TestDirectory::new();
    let path = directory.path.join("layers.json");
    for kind in [FileKind::Model, FileKind::Component] {
        for extra in [
            "",
            ",\"background-decorations\":[]",
            ",\"foreground-decorations\":[]",
            ",\"background-decorations\":[],\"foreground-decorations\":[]",
        ] {
            fs::write(&path, document(kind, extra)).unwrap();
            assert_eq!(kind.read(&path).unwrap(), Layers::default());
            kind.write(&path, &Layers::default()).unwrap();
            let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
            assert_eq!(written[kind.root()].as_object().unwrap().len(), 1);
            assert!(written[kind.root()]["metadata"].is_object());
        }
        for (layer, entries) in [
            ("background-decorations", BACKGROUND),
            ("foreground-decorations", FOREGROUND),
        ] {
            fs::write(&path, document(kind, &format!(",\"{layer}\":{entries}"))).unwrap();
            let mut expected = example_layers();
            if layer == "background-decorations" {
                expected.foreground.clear();
            } else {
                expected.background.clear();
            }
            assert_eq!(kind.read(&path).unwrap(), expected);
        }
    }
}

#[test]
fn integer_geometry_styles_text_and_all_image_fits_are_preserved() {
    let directory = TestDirectory::new();
    let path = directory.path.join("integer-geometry.json");
    for kind in [FileKind::Model, FileKind::Component] {
        for fit in [ImageFit::Contain, ImageFit::Cover, ImageFit::Fill] {
            let layers = Layers {
                background: vec![BackgroundDecoration::new(
                    "shape".to_owned(),
                    position(0, u32::MAX),
                    dimensions(0, u16::MAX),
                    BackgroundDecorationKind::RoundedRectangle {
                        style: Some(RoundedRectangleStyle::default()),
                    },
                )],
                foreground: vec![
                    ForegroundDecoration::new(
                        "image".to_owned(),
                        position(u32::MAX, 0),
                        dimensions(u16::MAX, 0),
                        ForegroundDecorationKind::Image(DecorationImage::new(
                            "images/missing \"heater\".svg".into(),
                            fit,
                        )),
                    ),
                    ForegroundDecoration::new(
                        "text".to_owned(),
                        position(0, 0),
                        dimensions(0, 0),
                        ForegroundDecorationKind::Text {
                            text: "Heater\n\"status\" \\ \u{1f525}".to_owned(),
                            style: Some(TextStyle::new(Some(20.25), Some("500".to_owned()), None)),
                        },
                    ),
                ],
            };
            kind.write(&path, &layers).unwrap();
            assert_eq!(kind.read(&path).unwrap(), layers);
            let written: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
            assert_eq!(
                written[kind.root()]["background-decorations"][0]["style"],
                json!({})
            );
            assert_eq!(
                written[kind.root()]["background-decorations"][0]["position"],
                json!({"x": 0, "y": u32::MAX})
            );
            assert_eq!(
                written[kind.root()]["background-decorations"][0]["dimensions"],
                json!({"width": 0, "height": u16::MAX})
            );
            assert_eq!(
                written[kind.root()]["foreground-decorations"][0]["position"],
                json!({"x": u32::MAX, "y": 0})
            );
            assert_eq!(
                written[kind.root()]["foreground-decorations"][0]["dimensions"],
                json!({"width": u16::MAX, "height": 0})
            );
        }
    }
}

#[test]
fn styles_are_specific_to_each_decoration_kind_in_both_file_kinds() {
    let directory = TestDirectory::new();
    let path = directory.path.join("type-specific-styles.json");
    let properties = json!({
        "fill-color": "#f0f0f0",
        "stroke-color": "#000000",
        "stroke-width": 0.5,
        "corner-radius": 0.0,
        "font-size": 20.25,
        "font-weight": "bold",
        "text-color": "#ffffff"
    });
    let cases: [(&str, &str, &[&str]); 5] = [
        (
            "background-decorations",
            "rectangle",
            &["fill-color", "stroke-color", "stroke-width"],
        ),
        (
            "background-decorations",
            "rounded-rectangle",
            &[
                "fill-color",
                "stroke-color",
                "stroke-width",
                "corner-radius",
            ],
        ),
        (
            "background-decorations",
            "frame",
            &["stroke-color", "stroke-width"],
        ),
        (
            "foreground-decorations",
            "text",
            &["font-size", "font-weight", "text-color"],
        ),
        ("foreground-decorations", "image", &[]),
    ];
    for kind in [FileKind::Model, FileKind::Component] {
        for (layer, tag, allowed) in cases {
            let mut entry = json!({
                "id": "a",
                "type": tag,
                "position": {"x": 0, "y": 0},
                "dimensions": {"width": 10, "height": 20}
            });
            if tag == "text" {
                entry["text"] = json!("Title");
            } else if tag == "image" {
                entry["image"] = json!({"path": "heater.svg", "fit": "contain"});
            }
            let applicable_style = Value::Object(
                allowed
                    .iter()
                    .map(|&name| (name.to_owned(), properties[name].clone()))
                    .collect(),
            );
            for style in [None, Some(json!({})), Some(applicable_style)] {
                let mut candidate = entry.clone();
                if let Some(style) = &style {
                    candidate["style"] = style.clone();
                }
                let source = document(kind, &format!(",\"{layer}\":[{candidate}]"));
                if tag == "image" && style.is_some() {
                    assert_data_error(kind, &path, &source);
                } else {
                    fs::write(&path, source).unwrap();
                    let layers = kind.read(&path).unwrap();
                    kind.write(&path, &layers).unwrap();
                    assert_eq!(kind.read(&path).unwrap(), layers);
                    let written: Value =
                        serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
                    assert_eq!(written[kind.root()][layer][0], candidate);
                }
            }
            for (name, value) in properties.as_object().unwrap() {
                if allowed.contains(&name.as_str()) {
                    continue;
                }
                let mut invalid = entry.clone();
                invalid["style"] = json!({(name): value});
                let error = assert_data_error(
                    kind,
                    &path,
                    &document(kind, &format!(",\"{layer}\":[{invalid}]")),
                );
                let detail = error.extra_detail().unwrap().as_str();
                if tag == "image"
                    && ["font-size", "font-weight", "text-color"].contains(&name.as_str())
                {
                    assert!(detail.contains("image decorations cannot contain style"));
                } else {
                    assert!(detail.contains(&format!("unknown field `{name}`")));
                }
            }
        }
    }
}

/// Checks that JSON errors retain localized diagnostics and file positions.
fn assert_json_error(kind: FileKind, path: &Utf8Path, source: &str, key: &str) -> Message {
    fs::write(path, source).unwrap();
    let error = kind.read(path).unwrap_err();
    assert_eq!(
        error.translate_data().message_key().as_str(),
        key,
        "{source}"
    );
    for name in ["line", "column"] {
        assert!(
            error.translate_data().message_params()[name]
                .as_str()
                .parse::<usize>()
                .unwrap()
                > 0,
            "{source}"
        );
    }
    assert_eq!(
        error.translate_data().message_params()["path"].as_str(),
        path.as_str()
    );
    assert!(error.extra_detail().is_some(), "{source}");
    error
}

/// Checks that invalid data is distinguished from malformed JSON syntax.
fn assert_data_error(kind: FileKind, path: &Utf8Path, source: &str) -> Message {
    assert_json_error(kind, path, source, "files_json_data")
}

#[test]
fn style_value_errors_preserve_json_positions_before_and_after_the_type_tag() {
    let directory = TestDirectory::new();
    let path = directory.path.join("style-positions.json");
    for kind in [FileKind::Model, FileKind::Component] {
        for (layer, tag, field, invalid) in [
            ("background-decorations", "rectangle", "fill-color", "42"),
            (
                "background-decorations",
                "frame",
                "stroke-width",
                r#""wide""#,
            ),
            (
                "background-decorations",
                "rounded-rectangle",
                "corner-radius",
                "null",
            ),
            ("foreground-decorations", "text", "font-size", r#""large""#),
        ] {
            let style = format!("\"style\":{{\n  \"{field}\": {invalid}\n}}");
            let type_tag = format!("\"type\":\"{tag}\"");
            let payload = if tag == "text" {
                r#","text":"Title""#
            } else {
                ""
            };
            for style_first in [false, true] {
                let fields = if style_first {
                    [style.as_str(), type_tag.as_str()]
                } else {
                    [type_tag.as_str(), style.as_str()]
                }
                .join(",\n");
                let source = document(
                    kind,
                    &format!(
                        ",\"{layer}\":[{{\"id\":\"a\",\"position\":{{\"x\":0,\"y\":0}},\"dimensions\":{{\"width\":10,\"height\":20}},\n{fields}{payload}\n}}]"
                    ),
                );
                let error = assert_data_error(kind, &path, &source);
                let (line_index, line) = source
                    .lines()
                    .enumerate()
                    .find(|(_, line)| line.contains(&format!("\"{field}\":")))
                    .unwrap();
                assert_eq!(
                    error.translate_data().message_params()["line"]
                        .as_str()
                        .parse::<usize>()
                        .unwrap(),
                    line_index.saturating_add(1),
                    "{source}"
                );
                assert_eq!(
                    error.translate_data().message_params()["column"]
                        .as_str()
                        .parse::<usize>()
                        .unwrap(),
                    line.len(),
                    "{source}"
                );
            }
        }
    }
}

#[test]
fn geometry_integer_fields_preserve_bounds_and_reject_invalid_values() {
    let directory = TestDirectory::new();
    let path = directory.path.join("integer-bounds.json");
    for kind in [FileKind::Model, FileKind::Component] {
        for (layer, tag, payload) in [
            ("background-decorations", "frame", ""),
            ("foreground-decorations", "text", r#","text":"title""#),
        ] {
            for (coordinate, extent) in [(0, 0), (u32::MAX, u16::MAX)] {
                let entry = format!(
                    r#"{{"id":"a","type":"{tag}","position":{{"x":{coordinate},"y":{coordinate}}},"dimensions":{{"width":{extent},"height":{extent}}}{payload}}}"#
                );
                fs::write(&path, document(kind, &format!(",\"{layer}\":[{entry}]"))).unwrap();
                let layers = kind.read(&path).unwrap();
                kind.write(&path, &layers).unwrap();
                assert_eq!(kind.read(&path).unwrap(), layers);
                let written: Value =
                    serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
                assert_eq!(
                    written[kind.root()][layer][0],
                    serde_json::from_str::<Value>(&entry).unwrap()
                );
            }
            for (field, overflow) in [
                ("x", "4294967296"),
                ("y", "4294967296"),
                ("width", "65536"),
                ("height", "65536"),
            ] {
                for invalid in ["-1", "0.5", "1.0", "1e0", overflow] {
                    let position = ["x", "y"]
                        .map(|name| {
                            let value = if name == field { invalid } else { "0" };
                            format!("\"{name}\":{value}")
                        })
                        .join(",");
                    let dimensions = ["width", "height"]
                        .map(|name| {
                            let value = if name == field { invalid } else { "0" };
                            format!("\"{name}\":{value}")
                        })
                        .join(",");
                    let entry = format!(
                        r#"{{"id":"a","type":"{tag}","position":{{{position}}},"dimensions":{{{dimensions}}}{payload}}}"#
                    );
                    assert_data_error(
                        kind,
                        &path,
                        &document(kind, &format!(",\"{layer}\":[{entry}]")),
                    );
                }
            }
        }
    }
}

#[test]
fn legacy_location_geometry_is_rejected_in_both_layers_and_file_kinds() {
    let directory = TestDirectory::new();
    let path = directory.path.join("legacy-location.json");
    for kind in [FileKind::Model, FileKind::Component] {
        for (layer, tag, payload) in [
            ("background-decorations", "frame", ""),
            ("foreground-decorations", "text", r#","text":"title""#),
        ] {
            let entry = format!(
                r#"{{"id":"a","type":"{tag}","location":{{"x":0,"y":0,"width":10,"height":20}}{payload}}}"#
            );
            let error = assert_data_error(
                kind,
                &path,
                &document(kind, &format!(",\"{layer}\":[{entry}]")),
            );
            assert!(
                error
                    .extra_detail()
                    .unwrap()
                    .as_str()
                    .contains("unknown field `location`")
            );
        }
    }
}

/// Produces malformed position and dimension objects and fields.
fn invalid_geometry_entries(valid: &Value) -> Vec<Value> {
    let mut invalid_entries = Vec::new();
    for (field, value) in [
        ("position", json!([0, 0])),
        ("dimensions", json!([10, 20])),
        ("position", Value::Null),
        ("dimensions", Value::Null),
        ("position", json!({"x": 0, "y": 0, "unknown": 0})),
        (
            "dimensions",
            json!({"width": 10, "height": 20, "unknown": 0}),
        ),
        ("position", json!({"x": 0, "y": 0, "width": 10})),
        ("dimensions", json!({"width": 10, "height": 20, "x": 0})),
    ] {
        let mut entry = valid.clone();
        entry[field] = value;
        invalid_entries.push(entry);
    }
    for (object, fields) in [
        ("position", ["x", "y"]),
        ("dimensions", ["width", "height"]),
    ] {
        for field in fields {
            for value in [Value::Null, json!("0")] {
                let mut entry = valid.clone();
                entry[object][field] = value;
                invalid_entries.push(entry);
            }
            let mut entry = valid.clone();
            entry[object].as_object_mut().unwrap().remove(field);
            invalid_entries.push(entry);
        }
    }
    invalid_entries
}

/// Produces malformed common fields without invalidating the baseline layer's kind.
fn invalid_common_entries(valid: &Value) -> Vec<Value> {
    let mut invalid_entries = vec![
        Value::Null,
        json!([]),
        json!(["frame", "frame", {"x": 0, "y": 0}, {"width": 10, "height": 20}]),
        json!("frame"),
    ];
    for field in ["id", "type", "position", "dimensions"] {
        let mut entry = valid.clone();
        entry.as_object_mut().unwrap().remove(field);
        invalid_entries.push(entry);
    }
    for (field, value) in [
        ("id", json!(1)),
        ("type", json!("circle")),
        ("unknown", json!(true)),
        (
            "location",
            json!({"x": 0, "y": 0, "width": 10, "height": 20}),
        ),
        ("style", json!([null, null, null, null, null, null, null])),
        ("style", Value::Null),
        ("style", json!({"unknown": 1})),
        ("style", json!({"stroke-width": "1"})),
        ("style", json!({"fill-color": 1})),
    ] {
        let mut entry = valid.clone();
        entry[field] = value;
        invalid_entries.push(entry);
    }
    invalid_entries.extend(invalid_geometry_entries(valid));
    for field in [
        "fill-color",
        "stroke-color",
        "stroke-width",
        "corner-radius",
        "font-size",
        "font-weight",
        "text-color",
    ] {
        let mut entry = valid.clone();
        entry["style"] = json!({(field): null});
        invalid_entries.push(entry);
    }
    for field in ["stroke-width", "corner-radius", "font-size"] {
        let mut entry = valid.clone();
        entry["style"] = json!({(field): -1});
        invalid_entries.push(entry);
    }
    invalid_entries
}

#[test]
fn malformed_layer_and_decoration_objects_are_rejected() {
    let directory = TestDirectory::new();
    let path = directory.path.join("invalid.json");
    let geometry = r#""position":{"x":0,"y":0},"dimensions":{"width":1,"height":1}"#;
    for kind in [FileKind::Model, FileKind::Component] {
        for (layer, tag, payload, numeric_field) in [
            ("background-decorations", "frame", "", "stroke-width"),
            (
                "foreground-decorations",
                "text",
                r#","text":"title""#,
                "font-size",
            ),
        ] {
            let valid: Value = serde_json::from_str(&format!(
                r#"{{"id":"a","type":"{tag}",{geometry}{payload}}}"#,
            ))
            .unwrap();
            for value in [Value::Null, json!({}), json!(""), json!(1)] {
                assert_data_error(
                    kind,
                    &path,
                    &document(kind, &format!(",\"{layer}\":{value}")),
                );
            }
            for entry in invalid_common_entries(&valid) {
                assert_data_error(
                    kind,
                    &path,
                    &document(kind, &format!(",\"{layer}\":[{entry}]")),
                );
            }
            for entry in [
                format!(r#"{{"id":"a","id":"b","type":"{tag}",{geometry}{payload}}}"#),
                format!(r#"{{"id":"a","type":"{tag}","type":"{tag}",{geometry}{payload}}}"#),
                format!(
                    r#"{{"id":"a","type":"{tag}","position":{{"x":0,"x":1,"y":0}},"dimensions":{{"width":1,"height":1}}{payload}}}"#
                ),
                format!(
                    r#"{{"id":"a","type":"{tag}","position":{{"x":0,"y":0}},"dimensions":{{"width":1,"width":2,"height":1}}{payload}}}"#
                ),
                format!(
                    r#"{{"id":"a","type":"{tag}",{geometry},"position":{{"x":0,"y":0}}{payload}}}"#
                ),
                format!(
                    r#"{{"id":"a","type":"{tag}",{geometry},"dimensions":{{"width":1,"height":1}}{payload}}}"#
                ),
                format!(
                    r#"{{"id":"a","type":"{tag}",{geometry},"style":{{"{numeric_field}":1,"{numeric_field}":2}}{payload}}}"#
                ),
            ] {
                assert_data_error(
                    kind,
                    &path,
                    &document(kind, &format!(",\"{layer}\":[{entry}]")),
                );
            }
            assert_data_error(
                kind,
                &path,
                &document(kind, &format!(",\"{layer}\":[],\"{layer}\":[]")),
            );
        }
    }
}

#[test]
fn every_decoration_kind_is_rejected_in_the_wrong_layer() {
    let directory = TestDirectory::new();
    let path = directory.path.join("wrong-layer.json");
    for kind in [FileKind::Model, FileKind::Component] {
        for (layer, entries) in [
            ("background-decorations", FOREGROUND),
            ("foreground-decorations", BACKGROUND),
        ] {
            let entries: Vec<Value> = serde_json::from_str(entries).unwrap();
            for entry in entries {
                assert_data_error(
                    kind,
                    &path,
                    &document(kind, &format!(",\"{layer}\":[{entry}]")),
                );
                let tag = entry["type"].as_str().unwrap();
                // Read the tag first to isolate layer errors from foreign style/payload fields.
                let fields = entry
                    .as_object()
                    .unwrap()
                    .iter()
                    .filter(|(name, _)| name.as_str() != "type")
                    .map(|(name, value)| format!("{}:{value}", json!(name)))
                    .collect::<Vec<_>>()
                    .join(",");
                let tag_first = format!(r#"{{"type":{},{fields}}}"#, json!(tag));
                let error = assert_data_error(
                    kind,
                    &path,
                    &document(kind, &format!(",\"{layer}\":[{tag_first}]")),
                );
                assert!(
                    error
                        .extra_detail()
                        .unwrap()
                        .as_str()
                        .contains(&format!("unknown variant `{tag}`")),
                    "{error:?}"
                );
            }
        }
    }
}

#[test]
fn type_specific_payloads_are_required_and_cannot_be_mixed() {
    let directory = TestDirectory::new();
    let path = directory.path.join("payloads.json");
    let base = json!({
        "id": "a",
        "position": {"x": 0, "y": 0},
        "dimensions": {"width": 10, "height": 20}
    });
    let mut invalid_foreground = Vec::new();
    for value in [
        None,
        Some(Value::Null),
        Some(json!(["heater.svg", "contain"])),
        Some(json!({"path": "heater.svg"})),
        Some(json!({"fit": "contain"})),
        Some(json!({"path": 1, "fit": "contain"})),
        Some(json!({"path": "heater.svg", "fit": "stretch"})),
        Some(json!({"path": "heater.svg", "fit": "contain", "unknown": true})),
    ] {
        let mut entry = base.clone();
        entry["type"] = json!("image");
        if let Some(value) = value {
            entry["image"] = value;
        }
        invalid_foreground.push(entry);
    }
    for value in [None, Some(Value::Null), Some(json!(42))] {
        let mut entry = base.clone();
        entry["type"] = json!("text");
        if let Some(value) = value {
            entry["text"] = value;
        }
        invalid_foreground.push(entry);
    }
    let mut mixed = base.clone();
    mixed["text"] = json!("title");
    mixed["image"] = json!({"path": "heater.svg", "fit": "contain"});
    for tag in ["text", "image"] {
        mixed["type"] = json!(tag);
        invalid_foreground.push(mixed.clone());
    }
    for kind in [FileKind::Model, FileKind::Component] {
        for tag in ["rectangle", "rounded-rectangle", "frame"] {
            for (field, value) in [
                ("text", json!("not a background payload")),
                ("text", Value::Null),
                ("image", json!({"path": "heater.svg", "fit": "contain"})),
                ("image", Value::Null),
            ] {
                let mut entry = base.clone();
                entry["type"] = json!(tag);
                entry[field] = value;
                assert_data_error(
                    kind,
                    &path,
                    &document(kind, &format!(",\"background-decorations\":[{entry}]")),
                );
            }
        }
        for entry in &invalid_foreground {
            assert_data_error(
                kind,
                &path,
                &document(kind, &format!(",\"foreground-decorations\":[{entry}]")),
            );
        }
        let mut null_fit = base.clone();
        null_fit["type"] = json!("image");
        null_fit["image"] = json!({"path": "heater.svg", "fit": null});
        assert_json_error(
            kind,
            &path,
            &document(kind, &format!(",\"foreground-decorations\":[{null_fit}]")),
            "files_json_syntax",
        );
        for entry in [
            r#"{"id":"a","type":"text","position":{"x":0,"y":0},"dimensions":{"width":1,"height":1},"text":"a","text":"b"}"#,
            r#"{"id":"a","type":"image","position":{"x":0,"y":0},"dimensions":{"width":1,"height":1},"image":{"path":"a.svg","path":"b.svg","fit":"contain"}}"#,
            r#"{"id":"a","type":"image","position":{"x":0,"y":0},"dimensions":{"width":1,"height":1},"image":{"path":"a.svg","fit":"contain","fit":"cover"}}"#,
        ] {
            assert_data_error(
                kind,
                &path,
                &document(kind, &format!(",\"foreground-decorations\":[{entry}]")),
            );
        }
    }
}

/// Creates layers with one numeric style property to exercise every validation path.
type WithNumber = fn(f64) -> Layers;

#[test]
fn invalid_numeric_styles_never_replace_existing_files() {
    let directory = TestDirectory::new();
    let path = directory.path.join("preserved.json");
    let numeric_fields: [(&str, WithNumber); 5] = [
        ("style.stroke-width", |v| Layers {
            background: vec![BackgroundDecoration::new(
                "rectangle".to_owned(),
                Position::default(),
                Dimensions::default(),
                BackgroundDecorationKind::Rectangle {
                    style: Some(RectangleStyle::new(None, None, Some(v))),
                },
            )],
            ..Layers::default()
        }),
        ("style.stroke-width", |v| Layers {
            background: vec![BackgroundDecoration::new(
                "rounded-rectangle".to_owned(),
                Position::default(),
                Dimensions::default(),
                BackgroundDecorationKind::RoundedRectangle {
                    style: Some(RoundedRectangleStyle::new(None, None, Some(v), None)),
                },
            )],
            ..Layers::default()
        }),
        ("style.corner-radius", |v| Layers {
            background: vec![BackgroundDecoration::new(
                "rounded-rectangle".to_owned(),
                Position::default(),
                Dimensions::default(),
                BackgroundDecorationKind::RoundedRectangle {
                    style: Some(RoundedRectangleStyle::new(None, None, None, Some(v))),
                },
            )],
            ..Layers::default()
        }),
        ("style.stroke-width", |v| Layers {
            background: vec![BackgroundDecoration::new(
                "frame".to_owned(),
                Position::default(),
                Dimensions::default(),
                BackgroundDecorationKind::Frame {
                    style: Some(FrameStyle::new(None, Some(v))),
                },
            )],
            ..Layers::default()
        }),
        ("style.font-size", |v| Layers {
            foreground: vec![ForegroundDecoration::new(
                "text".to_owned(),
                Position::default(),
                Dimensions::default(),
                ForegroundDecorationKind::Text {
                    text: "Title".to_owned(),
                    style: Some(TextStyle::new(Some(v), None, None)),
                },
            )],
            ..Layers::default()
        }),
    ];
    for kind in [FileKind::Model, FileKind::Component] {
        for (name, with_number) in numeric_fields {
            for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0] {
                let layers = with_number(value);
                fs::write(&path, "original contents").unwrap();

                let error = kind.write(&path, &layers).unwrap_err();

                assert_eq!(
                    error.translate_data().message_key().as_str(),
                    "files_json_serialization"
                );
                assert_eq!(
                    error.translate_data().message_params()["path"].as_str(),
                    path.as_str()
                );
                assert!(error.extra_detail().unwrap().as_str().contains(name));
                assert_eq!(fs::read_to_string(&path).unwrap(), "original contents");
                assert_eq!(directory.entries(), ["preserved.json"]);
            }
        }
    }
}
