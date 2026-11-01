use crate::common::gui::{Dimensions, Position};
use crate::foreground_decoration::data::{ForegroundDecoration, ImageFit};
use serde::Serialize;

/// The current JSON representation of a foreground decoration.
#[derive(Debug, Serialize)]
pub(crate) struct ForegroundDecorationDocument<'a> {
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

impl<'a> From<&'a ForegroundDecoration> for ForegroundDecorationDocument<'a> {
    fn from(decoration: &'a ForegroundDecoration) -> Self {
        Self {
            id: decoration.id(),
            position: PositionDocument::from(&decoration.position()),
            dimensions: DimensionsDocument::from(&decoration.dimensions()),
            kind: ForegroundDecorationType::from(decoration),
        }
    }
}

/// Foreground type tags and their corresponding style and payload schemas.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
enum ForegroundDecorationType<'a> {
    /// Text content.
    Text {
        /// Font and color style, omitted when all properties are unset.
        #[serde(skip_serializing_if = "TextStyleDocument::is_empty")]
        style: TextStyleDocument<'a>,
        /// Displayed text.
        text: &'a str,
    },
    /// Image content without styling.
    Image {
        /// Source and fitting mode.
        image: ImageDocument<'a>,
    },
}

impl<'a> From<&'a ForegroundDecoration> for ForegroundDecorationType<'a> {
    fn from(decoration: &'a ForegroundDecoration) -> Self {
        match decoration {
            ForegroundDecoration::Text {
                text,
                font_size,
                font_weight,
                text_color,
                ..
            } => Self::Text {
                text,
                style: TextStyleDocument {
                    font_size: *font_size,
                    font_weight: font_weight.as_deref(),
                    text_color: text_color.as_deref(),
                },
            },
            ForegroundDecoration::Image { path, fit, .. } => Self::Image {
                image: ImageDocument {
                    path: path.as_str(),
                    fit: ImageFitDocument::from(*fit),
                },
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
    /// Font size in stored hundredths.
    #[serde(skip_serializing_if = "Option::is_none")]
    font_size: Option<u16>,
    /// Font weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    font_weight: Option<&'a str>,
    /// Text color.
    #[serde(skip_serializing_if = "Option::is_none")]
    text_color: Option<&'a str>,
}

impl TextStyleDocument<'_> {
    /// Returns whether no text styling is supplied.
    const fn is_empty(&self) -> bool {
        self.font_size.is_none() && self.font_weight.is_none() && self.text_color.is_none()
    }
}

/// Image source and fitting mode in the current JSON format.
#[derive(Debug, Serialize)]
struct ImageDocument<'a> {
    /// Path stored unchanged.
    path: &'a str,
    /// Fitting mode.
    fit: ImageFitDocument,
}

/// Image fitting modes in the current JSON format.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum ImageFitDocument {
    /// Show the whole image.
    Contain,
    /// Cover the bounds.
    Cover,
    /// Stretch to the bounds.
    Fill,
}

impl From<ImageFit> for ImageFitDocument {
    fn from(fit: ImageFit) -> Self {
        match fit {
            ImageFit::Contain => Self::Contain,
            ImageFit::Cover => Self::Cover,
            ImageFit::Fill => Self::Fill,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serializes_without_wrapping_domain_data() {
        let document = ForegroundDecorationDocument {
            id: "label",
            position: PositionDocument { x: 10, y: 20 },
            dimensions: DimensionsDocument {
                width: 100,
                height: 50,
            },
            kind: ForegroundDecorationType::Text {
                text: "Title",
                style: TextStyleDocument {
                    font_size: Some(2050),
                    font_weight: Some("bold"),
                    text_color: Some("#000000"),
                },
            },
        };

        assert_eq!(
            serde_json::to_value(&document).unwrap(),
            json!({
                "id": "label",
                "position": {"x": 10, "y": 20},
                "dimensions": {"width": 100, "height": 50},
                "type": "text",
                "style": {
                    "font-size": 2050,
                    "font-weight": "bold",
                    "text-color": "#000000",
                },
                "text": "Title",
            })
        );
    }
}
