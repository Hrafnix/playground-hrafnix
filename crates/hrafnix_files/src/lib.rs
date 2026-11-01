//! Project, model, and component files.
//!
//! Background and foreground decorations are independent types:
//!
//! ```compile_fail
//! use hrafnix_files::background_decoration::data::BackgroundDecoration;
//! use hrafnix_files::foreground_decoration::data::ForegroundDecoration;
//!
//! fn wrong_layer(foreground: ForegroundDecoration) -> BackgroundDecoration {
//!     foreground
//! }
//! ```
//!
//! Both layers use shared [`Position`](crate::common::gui::Position) and
//! [`Dimensions`](crate::common::gui::Dimensions) types. Decoration, styling,
//! and codec types remain layer-specific.
//!
//! Numeric decoration styles use `u16` hundredths: `1234` represents `12.34`.
//! JSON stores these raw integers in `0..=65535`, representing drawing-unit values
//! in `0.00..=655.35`. Decimal and exponent-form numbers are rejected, including
//! whole-valued forms such as `20.0`. Older numeric decoration values must be
//! migrated to integer hundredths; the file version remains unchanged.
//! Positions, dimensions, and metadata versions remain unscaled integers.
//!
//! Foreground decorations use named `Text` and `Image` enum variants, each with
//! its own identifier, position, and dimensions.
//! Text styling fields are stored directly on the `Text` variant. JSON still nests
//! configured styling under `style`, omitting it when all properties are unset.
//! An empty text `style` object is read as unset styling and omitted when rewritten.
//! Image paths and fitting modes are stored directly on `Image`, while JSON
//! keeps them in the nested `image` payload.
//!
//! Writers use their own document structs and enums, borrowing text from data
//! values.
//!
//! Data structs have private fields. Construct them with `new(...)` and inspect
//! them through read-only accessors:
//!
//! ```
//! use hrafnix_files::component::data::ComponentFile;
//! use hrafnix_files::metadata::data::Metadata;
//!
//! let component = ComponentFile::new(Metadata::default(), Vec::new(), Vec::new());
//! assert_eq!(component.metadata().display_name(), "");
//! assert!(component.background_decorations().is_empty());
//! assert!(component.foreground_decorations().is_empty());
//! ```

// Test code favors clarity and brevity over the strictness we require of library code:
// panicking helpers (`unwrap`/`expect`/indexing/`panic!`) and approximate float comparisons
// are idiomatic and expected in tests.
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::panic,
        clippy::float_cmp,
        clippy::as_conversions,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::unreadable_literal,
        clippy::unnecessary_wraps,
        clippy::similar_names,
        clippy::arithmetic_side_effects
    )
)]

/// Background decorations for models and components.
pub mod background_decoration;
/// Shared file I/O, version detection, and diagnostics for components, models, and projects.
pub mod common;
/// Component definitions and behavior.
pub mod component;
/// Foreground decorations for models and components.
pub mod foreground_decoration;
/// Metadata shared by model and component files.
pub mod metadata;
/// Simulation model definitions.
pub mod model;
/// Projects that group models and components.
pub mod project;
