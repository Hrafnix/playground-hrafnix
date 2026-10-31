use crate::common::gui::{Dimensions, Position};

/// A shape behind a model's or component's contents.
#[derive(Debug, Clone, PartialEq)]
pub struct BackgroundDecoration {
    /// Identifier of the decoration.
    pub id: String,
    /// Position in drawing coordinates.
    pub position: Position,
    /// Dimensions in drawing coordinates.
    pub dimensions: Dimensions,
    /// The shape of the decoration, including its type-specific style.
    pub kind: BackgroundDecorationKind,
}

/// The shapes allowed in a background layer, each with its own optional style.
///
/// Styles cannot be assigned to a different shape:
///
/// ```compile_fail
/// use hrafnix_files::background_decoration::data::{BackgroundDecorationKind, RectangleStyle};
///
/// let frame = BackgroundDecorationKind::Frame {
///     style: Some(RectangleStyle::default()),
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundDecorationKind {
    /// A rectangular shape.
    Rectangle {
        /// Fill and outline styling, if present.
        style: Option<RectangleStyle>,
    },
    /// A rectangle with rounded corners.
    RoundedRectangle {
        /// Fill, outline, and corner styling, if present.
        style: Option<RoundedRectangleStyle>,
    },
    /// A rectangular outline.
    Frame {
        /// Outline styling, if present.
        style: Option<FrameStyle>,
    },
}

/// Optional fill and outline styling for a rectangle.
///
/// Omitted properties leave presentation defaults unspecified. Numeric properties
/// must be finite and non-negative.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RectangleStyle {
    /// Fill color, such as `#f0f0f0`.
    pub fill_color: Option<String>,
    /// Outline color.
    pub stroke_color: Option<String>,
    /// Outline width in drawing coordinates.
    pub stroke_width: Option<f64>,
}

/// Optional fill, outline, and corner styling for a rounded rectangle.
///
/// Omitted properties leave presentation defaults unspecified. Numeric properties
/// must be finite and non-negative.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoundedRectangleStyle {
    /// Fill color, such as `#f0f0f0`.
    pub fill_color: Option<String>,
    /// Outline color.
    pub stroke_color: Option<String>,
    /// Outline width in drawing coordinates.
    pub stroke_width: Option<f64>,
    /// Corner radius for rounded rectangles.
    pub corner_radius: Option<f64>,
}

/// Optional outline styling for a frame.
///
/// Omitted properties leave presentation defaults unspecified. Outline width must
/// be finite and non-negative.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FrameStyle {
    /// Outline color.
    pub stroke_color: Option<String>,
    /// Outline width in drawing coordinates.
    pub stroke_width: Option<f64>,
}

impl BackgroundDecoration {
    /// Checks numeric styles when reading or writing to avoid invalid values
    /// and JSON's lossy serialization of non-finite floats as `null`.
    pub(super) fn validate(&self) -> Result<(), String> {
        match &self.kind {
            BackgroundDecorationKind::Rectangle { style: Some(style) } => {
                validate_non_negative("style.stroke-width", style.stroke_width)
            }
            BackgroundDecorationKind::RoundedRectangle { style: Some(style) } => {
                validate_non_negative("style.stroke-width", style.stroke_width)?;
                validate_non_negative("style.corner-radius", style.corner_radius)
            }
            BackgroundDecorationKind::Frame { style: Some(style) } => {
                validate_non_negative("style.stroke-width", style.stroke_width)
            }
            BackgroundDecorationKind::Rectangle { style: None }
            | BackgroundDecorationKind::RoundedRectangle { style: None }
            | BackgroundDecorationKind::Frame { style: None } => Ok(()),
        }
    }
}

/// Checks an optional numeric style property.
fn validate_non_negative(name: &str, value: Option<f64>) -> Result<(), String> {
    if value.is_none_or(|value| value.is_finite() && value >= 0.0) {
        Ok(())
    } else {
        Err(format!("{name} must be a finite, non-negative number"))
    }
}
