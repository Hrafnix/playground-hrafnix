use crate::common::deserialize_object;
use crate::common::gui::{Dimensions, Position};
use crate::foreground_decoration::data::{ForegroundDecoration, ImageFit};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer};

/// A version 1 foreground decoration with only the applicable style and payload.
#[derive(Debug)]
pub(crate) struct ForegroundDecorationDocument(ForegroundDecoration);

impl<'de> Deserialize<'de> for ForegroundDecorationDocument {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let fields: DecorationFields = deserialize_object(deserializer)?;
        if fields.text.is_some() && !matches!(fields.tag, ForegroundDecorationTag::Text) {
            return Err(D::Error::custom(
                "`text` is only allowed on text decorations",
            ));
        }
        if fields.image.is_some() && !matches!(fields.tag, ForegroundDecorationTag::Image) {
            return Err(D::Error::custom(
                "`image` is only allowed on image decorations",
            ));
        }
        let decoration = match fields.tag {
            ForegroundDecorationTag::Text => {
                let style = fields.style.unwrap_or_default();
                ForegroundDecoration::Text {
                    id: fields.id,
                    position: fields.position.into(),
                    dimensions: fields.dimensions.into(),
                    text: fields.text.ok_or_else(|| D::Error::missing_field("text"))?,
                    font_size: style.font_size,
                    font_weight: style.font_weight,
                    text_color: style.text_color,
                }
            }
            ForegroundDecorationTag::Image => {
                if fields.style.is_some() {
                    return Err(D::Error::custom("image decorations cannot contain style"));
                }
                let image = fields
                    .image
                    .ok_or_else(|| D::Error::missing_field("image"))?;
                ForegroundDecoration::Image {
                    id: fields.id,
                    position: fields.position.into(),
                    dimensions: fields.dimensions.into(),
                    path: image.path.into(),
                    fit: image.fit.into(),
                }
            }
        };
        Ok(Self(decoration))
    }
}

impl From<ForegroundDecorationDocument> for ForegroundDecoration {
    fn from(document: ForegroundDecorationDocument) -> Self {
        document.0
    }
}

/// Streamed fields before the tag selects the foreground payload.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DecorationFields {
    /// Identifier of the decoration.
    id: String,
    /// The content discriminator.
    #[serde(rename = "type")]
    tag: ForegroundDecorationTag,
    /// Position as a named object.
    #[serde(deserialize_with = "deserialize_object")]
    position: PositionDocument,
    /// Dimensions as a named object.
    #[serde(deserialize_with = "deserialize_object")]
    dimensions: DimensionsDocument,
    /// Explicit text style, if present, must be an object.
    #[serde(default, deserialize_with = "deserialize_optional_object")]
    style: Option<TextStyleDocument>,
    /// Content required for text decorations.
    #[serde(default, deserialize_with = "deserialize_optional")]
    text: Option<String>,
    /// Source required for image decorations.
    #[serde(default, deserialize_with = "deserialize_optional_object")]
    image: Option<ImageDocument>,
}

/// Foreground type tags, read separately so object field order is unrestricted.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ForegroundDecorationTag {
    /// Text content.
    Text,
    /// Image content.
    Image,
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

/// Text font and color styling in version 1.
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct TextStyleDocument {
    /// Font size in stored hundredths.
    #[serde(default, deserialize_with = "deserialize_optional")]
    font_size: Option<u16>,
    /// Font weight.
    #[serde(default, deserialize_with = "deserialize_optional")]
    font_weight: Option<String>,
    /// Text color.
    #[serde(default, deserialize_with = "deserialize_optional")]
    text_color: Option<String>,
}

/// Image source and fitting mode in version 1.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ImageDocument {
    /// Path stored in the file.
    path: String,
    /// Fitting mode.
    fit: ImageFitDocument,
}

/// Supported image fitting modes in version 1.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ImageFitDocument {
    /// Show the whole image.
    Contain,
    /// Cover the bounds.
    Cover,
    /// Stretch to the bounds.
    Fill,
}

impl From<ImageFitDocument> for ImageFit {
    fn from(fit: ImageFitDocument) -> Self {
        match fit {
            ImageFitDocument::Contain => Self::Contain,
            ImageFitDocument::Cover => Self::Cover,
            ImageFitDocument::Fill => Self::Fill,
        }
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
