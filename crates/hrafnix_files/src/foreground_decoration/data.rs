use crate::common::gui::{Dimensions, Position};
use camino::{Utf8Path, Utf8PathBuf};

/// Text or an image in front of a model's or component's contents.
///
/// Fields are private; use the read-only accessors.
#[derive(Debug, Clone, PartialEq)]
pub struct ForegroundDecoration {
    /// Identifier of the decoration.
    id: String,
    /// Position in drawing coordinates.
    position: Position,
    /// Dimensions in drawing coordinates.
    dimensions: Dimensions,
    /// The content of the decoration, including its type-specific style.
    kind: ForegroundDecorationKind,
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
    font_size: Option<f64>,
    /// Font weight, such as `normal` or `bold`.
    font_weight: Option<String>,
    /// Text color.
    text_color: Option<String>,
}

/// The source and fitting mode of an image decoration.
///
/// Images have no style:
///
/// ```compile_fail
/// use hrafnix_files::foreground_decoration::data::{DecorationImage, ImageFit, TextStyle};
///
/// let image = DecorationImage::new(
///     "image.svg".into(),
///     ImageFit::Contain,
///     Some(TextStyle::default()),
/// );
/// ```
///
/// Fields are private; use the read-only accessors.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecorationImage {
    /// Image path, stored unchanged rather than opened or resolved during file I/O.
    path: Utf8PathBuf,
    /// How the image fits within the decoration's bounds.
    fit: ImageFit,
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
    /// Creates a foreground decoration with geometry and type-specific content.
    #[must_use]
    pub const fn new(
        id: String,
        position: Position,
        dimensions: Dimensions,
        kind: ForegroundDecorationKind,
    ) -> Self {
        Self {
            id,
            position,
            dimensions,
            kind,
        }
    }

    /// Returns the decoration's identifier.
    #[must_use]
    pub const fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Returns a copy of the position in drawing coordinates.
    #[must_use]
    pub const fn position(&self) -> Position {
        self.position
    }

    /// Returns a copy of the dimensions in drawing coordinates.
    #[must_use]
    pub const fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    /// Returns the content and its type-specific style.
    #[must_use]
    pub const fn kind(&self) -> &ForegroundDecorationKind {
        &self.kind
    }

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

impl TextStyle {
    /// Creates optional font and color styling for text.
    #[must_use]
    pub const fn new(
        font_size: Option<f64>,
        font_weight: Option<String>,
        text_color: Option<String>,
    ) -> Self {
        Self {
            font_size,
            font_weight,
            text_color,
        }
    }

    /// Returns the optional font size.
    #[must_use]
    pub const fn font_size(&self) -> Option<f64> {
        self.font_size
    }

    /// Returns the optional font weight.
    #[must_use]
    pub fn font_weight(&self) -> Option<&str> {
        self.font_weight.as_deref()
    }

    /// Returns the optional text color.
    #[must_use]
    pub fn text_color(&self) -> Option<&str> {
        self.text_color.as_deref()
    }
}

impl DecorationImage {
    /// Creates an image source and fitting mode without opening or resolving the path.
    #[must_use]
    pub const fn new(path: Utf8PathBuf, fit: ImageFit) -> Self {
        Self { path, fit }
    }

    /// Returns the image path as stored in the file.
    #[must_use]
    pub fn path(&self) -> &Utf8Path {
        &self.path
    }

    /// Returns how the image fits within the decoration's bounds.
    #[must_use]
    pub const fn fit(&self) -> ImageFit {
        self.fit
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
