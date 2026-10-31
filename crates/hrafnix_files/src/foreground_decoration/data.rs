use crate::common::gui::{Dimensions, Position};
use camino::Utf8PathBuf;

/// Text or an image in front of a model's or component's contents.
#[derive(Debug, Clone, PartialEq)]
pub struct ForegroundDecoration {
    /// Identifier of the decoration.
    pub id: String,
    /// Position in drawing coordinates.
    pub position: Position,
    /// Dimensions in drawing coordinates.
    pub dimensions: Dimensions,
    /// The content of the decoration, including its type-specific style.
    pub kind: ForegroundDecorationKind,
}

/// The content allowed in a foreground layer.
#[derive(Debug, Clone, PartialEq)]
pub enum ForegroundDecorationKind {
    /// Text displayed within the decoration's bounds.
    Text {
        /// The displayed text.
        text: String,
        /// Font and text color styling, if present.
        style: Option<TextStyle>,
    },
    /// An image displayed within the decoration's bounds.
    Image(DecorationImage),
}

/// Optional font and color styling for text.
///
/// Omitted properties leave presentation defaults unspecified. Font size must be
/// finite and non-negative.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextStyle {
    /// Text font size.
    pub font_size: Option<f64>,
    /// Font weight, such as `normal` or `bold`.
    pub font_weight: Option<String>,
    /// Text color.
    pub text_color: Option<String>,
}

/// The source and fitting mode of an image decoration.
///
/// Images have no style:
///
/// ```compile_fail
/// use hrafnix_files::foreground_decoration::data::{DecorationImage, ImageFit, TextStyle};
///
/// let image = DecorationImage {
///     path: "image.svg".into(),
///     fit: ImageFit::Contain,
///     style: Some(TextStyle::default()),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecorationImage {
    /// Image path, stored unchanged rather than opened or resolved during file I/O.
    pub path: Utf8PathBuf,
    /// How the image fits within the decoration's bounds.
    pub fit: ImageFit,
}

/// How an image fits within its decoration's bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageFit {
    /// Preserve the aspect ratio and show the whole image within the bounds.
    Contain,
    /// Preserve the aspect ratio and cover the bounds, cropping if necessary.
    Cover,
    /// Stretch the image to fill the bounds.
    Fill,
}

impl ForegroundDecoration {
    /// Checks numeric text styling to avoid invalid values and JSON's lossy
    /// serialization of non-finite floats as `null`.
    pub(super) fn validate(&self) -> Result<(), String> {
        match &self.kind {
            ForegroundDecorationKind::Text {
                style: Some(style), ..
            } => validate_non_negative("style.font-size", style.font_size),
            ForegroundDecorationKind::Text { style: None, .. }
            | ForegroundDecorationKind::Image(_) => Ok(()),
        }
    }
}

/// Checks an optional numeric text style property.
fn validate_non_negative(name: &str, value: Option<f64>) -> Result<(), String> {
    if value.is_none_or(|value| value.is_finite() && value >= 0.0) {
        Ok(())
    } else {
        Err(format!("{name} must be a finite, non-negative number"))
    }
}
