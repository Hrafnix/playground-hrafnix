use crate::common::gui::{Dimensions, Position};

/// A shape behind a model's or component's contents.
///
/// Fields are private; use the read-only accessors.
#[derive(Debug, Clone, PartialEq)]
pub struct BackgroundDecoration {
    /// Identifier of the decoration.
    id: String,
    /// Position in drawing coordinates.
    position: Position,
    /// Dimensions in drawing coordinates.
    dimensions: Dimensions,
    /// The shape of the decoration, including its type-specific style.
    kind: BackgroundDecorationKind,
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
    fill_color: Option<String>,
    /// Outline color.
    stroke_color: Option<String>,
    /// Outline width in drawing coordinates.
    stroke_width: Option<f64>,
}

/// Optional fill, outline, and corner styling for a rounded rectangle.
///
/// Omitted properties leave presentation defaults unspecified. Numeric properties
/// must be finite and non-negative.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoundedRectangleStyle {
    /// Fill color, such as `#f0f0f0`.
    fill_color: Option<String>,
    /// Outline color.
    stroke_color: Option<String>,
    /// Outline width in drawing coordinates.
    stroke_width: Option<f64>,
    /// Corner radius for rounded rectangles.
    corner_radius: Option<f64>,
}

/// Optional outline styling for a frame.
///
/// Omitted properties leave presentation defaults unspecified. Outline width must
/// be finite and non-negative.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FrameStyle {
    /// Outline color.
    stroke_color: Option<String>,
    /// Outline width in drawing coordinates.
    stroke_width: Option<f64>,
}

impl BackgroundDecoration {
    /// Creates a background decoration with geometry and type-specific styling.
    #[must_use]
    pub const fn new(
        id: String,
        position: Position,
        dimensions: Dimensions,
        kind: BackgroundDecorationKind,
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

    /// Returns the shape and its type-specific style.
    #[must_use]
    pub const fn kind(&self) -> &BackgroundDecorationKind {
        &self.kind
    }

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

impl RectangleStyle {
    /// Creates optional fill and outline styling for a rectangle.
    #[must_use]
    pub const fn new(
        fill_color: Option<String>,
        stroke_color: Option<String>,
        stroke_width: Option<f64>,
    ) -> Self {
        Self {
            fill_color,
            stroke_color,
            stroke_width,
        }
    }

    /// Returns the optional fill color.
    #[must_use]
    pub fn fill_color(&self) -> Option<&str> {
        self.fill_color.as_deref()
    }

    /// Returns the optional outline color.
    #[must_use]
    pub fn stroke_color(&self) -> Option<&str> {
        self.stroke_color.as_deref()
    }

    /// Returns the optional outline width in drawing coordinates.
    #[must_use]
    pub const fn stroke_width(&self) -> Option<f64> {
        self.stroke_width
    }
}

impl RoundedRectangleStyle {
    /// Creates optional fill, outline, and corner styling for a rounded rectangle.
    #[must_use]
    pub const fn new(
        fill_color: Option<String>,
        stroke_color: Option<String>,
        stroke_width: Option<f64>,
        corner_radius: Option<f64>,
    ) -> Self {
        Self {
            fill_color,
            stroke_color,
            stroke_width,
            corner_radius,
        }
    }

    /// Returns the optional fill color.
    #[must_use]
    pub fn fill_color(&self) -> Option<&str> {
        self.fill_color.as_deref()
    }

    /// Returns the optional outline color.
    #[must_use]
    pub fn stroke_color(&self) -> Option<&str> {
        self.stroke_color.as_deref()
    }

    /// Returns the optional outline width in drawing coordinates.
    #[must_use]
    pub const fn stroke_width(&self) -> Option<f64> {
        self.stroke_width
    }

    /// Returns the optional corner radius.
    #[must_use]
    pub const fn corner_radius(&self) -> Option<f64> {
        self.corner_radius
    }
}

impl FrameStyle {
    /// Creates optional outline styling for a frame.
    #[must_use]
    pub const fn new(stroke_color: Option<String>, stroke_width: Option<f64>) -> Self {
        Self {
            stroke_color,
            stroke_width,
        }
    }

    /// Returns the optional outline color.
    #[must_use]
    pub fn stroke_color(&self) -> Option<&str> {
        self.stroke_color.as_deref()
    }

    /// Returns the optional outline width in drawing coordinates.
    #[must_use]
    pub const fn stroke_width(&self) -> Option<f64> {
        self.stroke_width
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
