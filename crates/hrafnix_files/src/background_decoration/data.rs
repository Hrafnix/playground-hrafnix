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
/// Omitted properties leave presentation defaults unspecified. Outline width is
/// stored in hundredths of a drawing unit: `100` represents `1.00`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RectangleStyle {
    /// Fill color, such as `#f0f0f0`.
    fill_color: Option<String>,
    /// Outline color.
    stroke_color: Option<String>,
    /// Outline width in hundredths of a drawing unit.
    stroke_width: Option<u16>,
}

/// Optional fill, outline, and corner styling for a rounded rectangle.
///
/// Omitted properties leave presentation defaults unspecified. Numeric properties
/// are stored in hundredths of a drawing unit: `100` represents `1.00`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoundedRectangleStyle {
    /// Fill color, such as `#f0f0f0`.
    fill_color: Option<String>,
    /// Outline color.
    stroke_color: Option<String>,
    /// Outline width in hundredths of a drawing unit.
    stroke_width: Option<u16>,
    /// Corner radius in hundredths of a drawing unit.
    corner_radius: Option<u16>,
}

/// Optional outline styling for a frame.
///
/// Omitted properties leave presentation defaults unspecified. Outline width is
/// stored in hundredths of a drawing unit: `100` represents `1.00`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FrameStyle {
    /// Outline color.
    stroke_color: Option<String>,
    /// Outline width in hundredths of a drawing unit.
    stroke_width: Option<u16>,
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
}

impl RectangleStyle {
    /// Creates optional fill and outline styling for a rectangle.
    #[must_use]
    pub const fn new(
        fill_color: Option<String>,
        stroke_color: Option<String>,
        stroke_width: Option<u16>,
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

    /// Returns the optional outline width in hundredths of a drawing unit.
    #[must_use]
    pub const fn stroke_width(&self) -> Option<u16> {
        self.stroke_width
    }
}

impl RoundedRectangleStyle {
    /// Creates optional fill, outline, and corner styling for a rounded rectangle.
    #[must_use]
    pub const fn new(
        fill_color: Option<String>,
        stroke_color: Option<String>,
        stroke_width: Option<u16>,
        corner_radius: Option<u16>,
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

    /// Returns the optional outline width in hundredths of a drawing unit.
    #[must_use]
    pub const fn stroke_width(&self) -> Option<u16> {
        self.stroke_width
    }

    /// Returns the optional corner radius in hundredths of a drawing unit.
    #[must_use]
    pub const fn corner_radius(&self) -> Option<u16> {
        self.corner_radius
    }
}

impl FrameStyle {
    /// Creates optional outline styling for a frame.
    #[must_use]
    pub const fn new(stroke_color: Option<String>, stroke_width: Option<u16>) -> Self {
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

    /// Returns the optional outline width in hundredths of a drawing unit.
    #[must_use]
    pub const fn stroke_width(&self) -> Option<u16> {
        self.stroke_width
    }
}
