use crate::common::gui::{Dimensions, Position};
use crate::foreground_decoration::data::{
    DecorationImage, ForegroundDecoration, ForegroundDecorationKind, ImageFit, TextStyle,
};
use serde::ser::Error as _;
use serde::{Serialize, Serializer};

/// A borrowed foreground decoration validated before its JSON is written.
#[derive(Debug)]
pub(crate) struct ForegroundDecorationDocument<'a>(&'a ForegroundDecoration);

impl<'a> From<&'a ForegroundDecoration> for ForegroundDecorationDocument<'a> {
    fn from(decoration: &'a ForegroundDecoration) -> Self {
        Self(decoration)
    }
}

impl Serialize for ForegroundDecorationDocument<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let decoration = self.0;
        decoration.validate().map_err(S::Error::custom)?;
        DecorationFields {
            id: decoration.id(),
            position: PositionDocument::from(&decoration.position()),
            dimensions: DimensionsDocument::from(&decoration.dimensions()),
            kind: ForegroundDecorationType::from(decoration.kind()),
        }
        .serialize(serializer)
    }
}

/// Named fields of a foreground decoration in the current JSON format.
#[derive(Debug, Serialize)]
struct DecorationFields<'a> {
    /// Identifier of the decoration.
    id: &'a str,
    /// Position.
    position: PositionDocument,
    /// Dimensions.
    dimensions: DimensionsDocument,
    /// Type tag and its applicable style or payload.
    #[serde(flatten)]
    kind: ForegroundDecorationType<'a>,
}

/// Foreground type tags and their corresponding style and payload schemas.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
enum ForegroundDecorationType<'a> {
    /// Text content.
    Text {
        /// Explicit font and color style, if supplied.
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<TextStyleDocument<'a>>,
        /// Displayed text.
        text: &'a str,
    },
    /// Image content without styling.
    Image {
        /// Source and fitting mode.
        image: ImageDocument<'a>,
    },
}

impl<'a> From<&'a ForegroundDecorationKind> for ForegroundDecorationType<'a> {
    fn from(kind: &'a ForegroundDecorationKind) -> Self {
        match kind {
            ForegroundDecorationKind::Text { text, style } => Self::Text {
                text,
                style: style.as_ref().map(TextStyleDocument::from),
            },
            ForegroundDecorationKind::Image(image) => Self::Image {
                image: ImageDocument::from(image),
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

/// Text font and color styling in the current JSON format.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
struct TextStyleDocument<'a> {
    /// Font size.
    #[serde(skip_serializing_if = "Option::is_none")]
    font_size: Option<f64>,
    /// Font weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    font_weight: Option<&'a str>,
    /// Text color.
    #[serde(skip_serializing_if = "Option::is_none")]
    text_color: Option<&'a str>,
}

impl<'a> From<&'a TextStyle> for TextStyleDocument<'a> {
    fn from(style: &'a TextStyle) -> Self {
        Self {
            font_size: style.font_size(),
            font_weight: style.font_weight(),
            text_color: style.text_color(),
        }
    }
}

/// Image source and fitting mode in the current JSON format.
#[derive(Debug, Serialize)]
struct ImageDocument<'a> {
    /// Path stored unchanged.
    path: &'a str,
    /// Fitting mode.
    fit: &'static str,
}

impl<'a> From<&'a DecorationImage> for ImageDocument<'a> {
    fn from(image: &'a DecorationImage) -> Self {
        Self {
            path: image.path().as_str(),
            fit: match image.fit() {
                ImageFit::Contain => "contain",
                ImageFit::Cover => "cover",
                ImageFit::Fill => "fill",
            },
        }
    }
}
