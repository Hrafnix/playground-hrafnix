use crate::background_decoration::data::{
    BackgroundDecoration, BackgroundDecorationKind, FrameStyle, RectangleStyle,
    RoundedRectangleStyle,
};
use crate::common::gui::{Dimensions, Position};
use serde::ser::Error as _;
use serde::{Serialize, Serializer};

/// A borrowed background decoration validated before its JSON is written.
#[derive(Debug)]
pub(crate) struct BackgroundDecorationDocument<'a>(&'a BackgroundDecoration);

impl<'a> From<&'a BackgroundDecoration> for BackgroundDecorationDocument<'a> {
    fn from(decoration: &'a BackgroundDecoration) -> Self {
        Self(decoration)
    }
}

impl Serialize for BackgroundDecorationDocument<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let decoration = self.0;
        decoration.validate().map_err(S::Error::custom)?;
        DecorationFields {
            id: decoration.id(),
            position: PositionDocument::from(&decoration.position()),
            dimensions: DimensionsDocument::from(&decoration.dimensions()),
            kind: BackgroundDecorationType::from(decoration.kind()),
        }
        .serialize(serializer)
    }
}

/// Named fields of a background decoration in the current JSON format.
#[derive(Debug, Serialize)]
struct DecorationFields<'a> {
    /// Identifier of the decoration.
    id: &'a str,
    /// Position.
    position: PositionDocument,
    /// Dimensions.
    dimensions: DimensionsDocument,
    /// Type tag and its applicable style.
    #[serde(flatten)]
    kind: BackgroundDecorationType<'a>,
}

/// Background type tags and their corresponding style schemas.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
enum BackgroundDecorationType<'a> {
    /// Rectangular shape.
    Rectangle {
        /// Explicit fill and outline style, if supplied.
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<RectangleStyleDocument<'a>>,
    },
    /// Rounded rectangular shape.
    RoundedRectangle {
        /// Explicit fill, outline, and corner style, if supplied.
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<RoundedRectangleStyleDocument<'a>>,
    },
    /// Rectangular outline.
    Frame {
        /// Explicit outline style, if supplied.
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<FrameStyleDocument<'a>>,
    },
}

impl<'a> From<&'a BackgroundDecorationKind> for BackgroundDecorationType<'a> {
    fn from(kind: &'a BackgroundDecorationKind) -> Self {
        match kind {
            BackgroundDecorationKind::Rectangle { style } => Self::Rectangle {
                style: style.as_ref().map(RectangleStyleDocument::from),
            },
            BackgroundDecorationKind::RoundedRectangle { style } => Self::RoundedRectangle {
                style: style.as_ref().map(RoundedRectangleStyleDocument::from),
            },
            BackgroundDecorationKind::Frame { style } => Self::Frame {
                style: style.as_ref().map(FrameStyleDocument::from),
            },
        }
    }
}

/// Position in the current JSON format.
#[derive(Debug, Serialize)]
struct PositionDocument {
    /// Horizontal position.
    x: u32,
    /// Vertical position.
    y: u32,
}

impl From<&Position> for PositionDocument {
    fn from(position: &Position) -> Self {
        Self {
            x: position.x,
            y: position.y,
        }
    }
}

/// Dimensions in the current JSON format.
#[derive(Debug, Serialize)]
struct DimensionsDocument {
    /// Width.
    width: u16,
    /// Height.
    height: u16,
}

impl From<&Dimensions> for DimensionsDocument {
    fn from(dimensions: &Dimensions) -> Self {
        Self {
            width: dimensions.width,
            height: dimensions.height,
        }
    }
}

/// Rectangle fill and outline styling in the current JSON format.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
struct RectangleStyleDocument<'a> {
    /// Fill color.
    #[serde(skip_serializing_if = "Option::is_none")]
    fill_color: Option<&'a str>,
    /// Outline color.
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_color: Option<&'a str>,
    /// Outline width.
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_width: Option<f64>,
}

impl<'a> From<&'a RectangleStyle> for RectangleStyleDocument<'a> {
    fn from(style: &'a RectangleStyle) -> Self {
        Self {
            fill_color: style.fill_color(),
            stroke_color: style.stroke_color(),
            stroke_width: style.stroke_width(),
        }
    }
}

/// Rounded rectangle fill, outline, and corner styling in the current JSON format.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
struct RoundedRectangleStyleDocument<'a> {
    /// Fill color.
    #[serde(skip_serializing_if = "Option::is_none")]
    fill_color: Option<&'a str>,
    /// Outline color.
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_color: Option<&'a str>,
    /// Outline width.
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_width: Option<f64>,
    /// Corner radius.
    #[serde(skip_serializing_if = "Option::is_none")]
    corner_radius: Option<f64>,
}

impl<'a> From<&'a RoundedRectangleStyle> for RoundedRectangleStyleDocument<'a> {
    fn from(style: &'a RoundedRectangleStyle) -> Self {
        Self {
            fill_color: style.fill_color(),
            stroke_color: style.stroke_color(),
            stroke_width: style.stroke_width(),
            corner_radius: style.corner_radius(),
        }
    }
}

/// Frame outline styling in the current JSON format.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
struct FrameStyleDocument<'a> {
    /// Outline color.
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_color: Option<&'a str>,
    /// Outline width.
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_width: Option<f64>,
}

impl<'a> From<&'a FrameStyle> for FrameStyleDocument<'a> {
    fn from(style: &'a FrameStyle) -> Self {
        Self {
            stroke_color: style.stroke_color(),
            stroke_width: style.stroke_width(),
        }
    }
}
