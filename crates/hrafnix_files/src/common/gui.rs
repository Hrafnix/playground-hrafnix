/// A position in drawing coordinates.
///
/// All fields are unsigned 32-bit integers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Position {
    /// Horizontal position.
    pub x: u32,
    /// Vertical position.
    pub y: u32,
}

/// Dimensions in drawing coordinates.
///
/// All fields are unsigned 16-bit integers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Dimensions {
    /// Width in drawing coordinates.
    pub width: u16,
    /// Height in drawing coordinates.
    pub height: u16,
}
