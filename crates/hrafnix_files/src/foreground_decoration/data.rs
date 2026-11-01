use crate::common::gui::{Dimensions, Position};
use camino::Utf8PathBuf;

/// Text or an image in front of a model's or component's contents.
///
/// Each variant stores its own identifier, geometry, and type-specific content.
/// Construct variants directly and use the accessors for their common fields.
#[derive(Debug, Clone, PartialEq)]
pub enum ForegroundDecoration {
    /// Text displayed within the decoration's bounds.
    ///
    /// Unset styling properties leave presentation defaults unspecified.
    Text {
        /// Identifier of the decoration.
        id: String,
        /// Position in drawing coordinates.
        position: Position,
        /// Dimensions in drawing coordinates.
        dimensions: Dimensions,
        /// The displayed text.
        text: String,
        /// Optional font size in hundredths of a drawing unit: `2000` represents `20.00`.
        font_size: Option<u16>,
        /// Optional font weight, such as `normal` or `bold`.
        font_weight: Option<String>,
        /// Optional text color.
        text_color: Option<String>,
    },
    /// An image displayed within the decoration's bounds.
    Image {
        /// Identifier of the decoration.
        id: String,
        /// Position in drawing coordinates.
        position: Position,
        /// Dimensions in drawing coordinates.
        dimensions: Dimensions,
        /// Image path, stored unchanged rather than opened or resolved during file I/O.
        path: Utf8PathBuf,
        /// How the image fits within the decoration's bounds.
        fit: ImageFit,
    },
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
    /// Returns the decoration's identifier.
    #[must_use]
    pub const fn id(&self) -> &str {
        match self {
            Self::Text { id, .. } | Self::Image { id, .. } => id.as_str(),
        }
    }

    /// Returns a copy of the position in drawing coordinates.
    #[must_use]
    pub const fn position(&self) -> Position {
        match self {
            Self::Text { position, .. } | Self::Image { position, .. } => *position,
        }
    }

    /// Returns a copy of the dimensions in drawing coordinates.
    #[must_use]
    pub const fn dimensions(&self) -> Dimensions {
        match self {
            Self::Text { dimensions, .. } | Self::Image { dimensions, .. } => *dimensions,
        }
    }
}
