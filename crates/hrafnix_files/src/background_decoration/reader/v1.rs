use crate::background_decoration::data::{
    BackgroundDecoration, BackgroundDecorationKind, FrameStyle, RectangleStyle,
    RoundedRectangleStyle,
};
use crate::common::deserialize_object;
use crate::common::gui::{Dimensions, Position};
use serde::de::Error as _;
use serde::de::{DeserializeOwned, MapAccess, Visitor, value::MapDeserializer};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::fmt;

/// A version 1 background decoration with only the applicable style fields.
#[derive(Debug)]
pub(crate) struct BackgroundDecorationDocument(BackgroundDecoration);

impl<'de> Deserialize<'de> for BackgroundDecorationDocument {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let fields: DecorationFields = deserialize_object(deserializer)?;
        let kind = match fields.tag {
            BackgroundDecorationTag::Rectangle => BackgroundDecorationType::Rectangle {
                style: fields
                    .style
                    .map(StyleProperties::into_document)
                    .transpose()
                    .map_err(D::Error::custom)?,
            },
            BackgroundDecorationTag::RoundedRectangle => {
                BackgroundDecorationType::RoundedRectangle {
                    style: fields
                        .style
                        .map(StyleProperties::into_document)
                        .transpose()
                        .map_err(D::Error::custom)?,
                }
            }
            BackgroundDecorationTag::Frame => BackgroundDecorationType::Frame {
                style: fields
                    .style
                    .map(StyleProperties::into_document)
                    .transpose()
                    .map_err(D::Error::custom)?,
            },
        };
        let decoration = BackgroundDecoration {
            id: fields.id,
            position: fields.position.into(),
            dimensions: fields.dimensions.into(),
            kind: kind.into(),
        };
        decoration.validate().map_err(D::Error::custom)?;
        Ok(Self(decoration))
    }
}

impl From<BackgroundDecorationDocument> for BackgroundDecoration {
    fn from(document: BackgroundDecorationDocument) -> Self {
        document.0
    }
}

/// Streamed fields before the tag selects the type-specific background style.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DecorationFields {
    /// Identifier of the decoration.
    id: String,
    /// The shape discriminator.
    #[serde(rename = "type")]
    tag: BackgroundDecorationTag,
    /// Position as a named object.
    #[serde(deserialize_with = "deserialize_object")]
    position: PositionDocument,
    /// Dimensions as a named object.
    #[serde(deserialize_with = "deserialize_object")]
    dimensions: DimensionsDocument,
    /// Explicit style, if present, must be an object.
    #[serde(default, deserialize_with = "deserialize_optional_object")]
    style: Option<StyleProperties>,
}

/// Supported version 1 background decoration types.
#[derive(Debug)]
enum BackgroundDecorationType {
    /// Rectangular shape with fill and outline styling.
    Rectangle {
        /// Explicit style, if present.
        style: Option<RectangleStyleDocument>,
    },
    /// Rounded rectangular shape with corner styling.
    RoundedRectangle {
        /// Explicit style, if present.
        style: Option<RoundedRectangleStyleDocument>,
    },
    /// Rectangular outline.
    Frame {
        /// Explicit outline style, if present.
        style: Option<FrameStyleDocument>,
    },
}

impl From<BackgroundDecorationType> for BackgroundDecorationKind {
    fn from(kind: BackgroundDecorationType) -> Self {
        match kind {
            BackgroundDecorationType::Rectangle { style } => Self::Rectangle {
                style: style.map(RectangleStyle::from),
            },
            BackgroundDecorationType::RoundedRectangle { style } => Self::RoundedRectangle {
                style: style.map(RoundedRectangleStyle::from),
            },
            BackgroundDecorationType::Frame { style } => Self::Frame {
                style: style.map(FrameStyle::from),
            },
        }
    }
}

/// Background type tags, read separately so object field order is unrestricted.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum BackgroundDecorationTag {
    /// Rectangular shape.
    Rectangle,
    /// Rounded rectangular shape.
    RoundedRectangle,
    /// Rectangular outline.
    Frame,
}

/// Position in version 1.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PositionDocument {
    /// Horizontal position.
    x: u32,
    /// Vertical position.
    y: u32,
}

impl From<PositionDocument> for Position {
    fn from(document: PositionDocument) -> Self {
        Self {
            x: document.x,
            y: document.y,
        }
    }
}

/// Dimensions in version 1.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DimensionsDocument {
    /// Width.
    width: u16,
    /// Height.
    height: u16,
}

impl From<DimensionsDocument> for Dimensions {
    fn from(document: DimensionsDocument) -> Self {
        Self {
            width: document.width,
            height: document.height,
        }
    }
}

/// Rectangle fill and outline styling in version 1.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct RectangleStyleDocument {
    /// Fill color.
    #[serde(default, deserialize_with = "deserialize_optional")]
    fill_color: Option<String>,
    /// Outline color.
    #[serde(default, deserialize_with = "deserialize_optional")]
    stroke_color: Option<String>,
    /// Outline width.
    #[serde(default, deserialize_with = "deserialize_optional")]
    stroke_width: Option<f64>,
}

impl From<RectangleStyleDocument> for RectangleStyle {
    fn from(document: RectangleStyleDocument) -> Self {
        Self {
            fill_color: document.fill_color,
            stroke_color: document.stroke_color,
            stroke_width: document.stroke_width,
        }
    }
}

/// Rounded rectangle fill, outline, and corner styling in version 1.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct RoundedRectangleStyleDocument {
    /// Fill color.
    #[serde(default, deserialize_with = "deserialize_optional")]
    fill_color: Option<String>,
    /// Outline color.
    #[serde(default, deserialize_with = "deserialize_optional")]
    stroke_color: Option<String>,
    /// Outline width.
    #[serde(default, deserialize_with = "deserialize_optional")]
    stroke_width: Option<f64>,
    /// Corner radius.
    #[serde(default, deserialize_with = "deserialize_optional")]
    corner_radius: Option<f64>,
}

impl From<RoundedRectangleStyleDocument> for RoundedRectangleStyle {
    fn from(document: RoundedRectangleStyleDocument) -> Self {
        Self {
            fill_color: document.fill_color,
            stroke_color: document.stroke_color,
            stroke_width: document.stroke_width,
            corner_radius: document.corner_radius,
        }
    }
}

/// Frame outline styling in version 1.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct FrameStyleDocument {
    /// Outline color.
    #[serde(default, deserialize_with = "deserialize_optional")]
    stroke_color: Option<String>,
    /// Outline width.
    #[serde(default, deserialize_with = "deserialize_optional")]
    stroke_width: Option<f64>,
}

impl From<FrameStyleDocument> for FrameStyle {
    fn from(document: FrameStyleDocument) -> Self {
        Self {
            stroke_color: document.stroke_color,
            stroke_width: document.stroke_width,
        }
    }
}

/// Typed scalar properties retained until the background tag selects a schema.
///
/// Values are checked while streaming to preserve JSON diagnostic positions;
/// the selected style document then rejects inapplicable property names.
#[derive(Debug)]
struct StyleProperties(Vec<(&'static str, Value)>);

impl StyleProperties {
    /// Deserializes the collected properties through the selected style schema.
    fn into_document<T: DeserializeOwned>(self) -> Result<T, serde_json::Error> {
        T::deserialize(MapDeserializer::new(self.0.into_iter()))
    }
}

impl<'de> Deserialize<'de> for StyleProperties {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(StylePropertiesVisitor)
    }
}

/// Collects only known, correctly typed, non-duplicate background style properties.
struct StylePropertiesVisitor;

impl<'de> Visitor<'de> for StylePropertiesVisitor {
    type Value = StyleProperties;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        /// All version 1 background style fields; the selected schema restricts them.
        const FIELDS: &[&str] = &[
            "fill-color",
            "stroke-color",
            "stroke-width",
            "corner-radius",
        ];
        let mut properties = Vec::new();
        while let Some(name) = map.next_key::<String>()? {
            let field = FIELDS
                .iter()
                .copied()
                .find(|field| *field == name)
                .ok_or_else(|| A::Error::unknown_field(&name, FIELDS))?;
            if properties.iter().any(|(name, _)| *name == field) {
                return Err(A::Error::duplicate_field(field));
            }
            let value = match field {
                "stroke-width" | "corner-radius" => Value::Number(map.next_value()?),
                _ => Value::String(map.next_value()?),
            };
            properties.push((field, value));
        }
        Ok(StyleProperties(properties))
    }
}

/// Reads a present optional field without treating an explicit `null` as absence.
fn deserialize_optional<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(deserializer).map(Some)
}

/// Reads a present optional object without accepting positional arrays or `null`.
fn deserialize_optional_object<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    deserialize_object(deserializer).map(Some)
}
