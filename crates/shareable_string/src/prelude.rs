//! Convenience re-exports for common types.
//!
//! Using the prelude allows you to quickly import everything you need:
//!
//! ```rust
//! use shareable_string::prelude::*;
//! ```

// Core types
pub use crate::ShareableString;

// Interning store
pub use crate::SharedStringStore;

// Translations
pub use crate::{SharedStringTranslationMap, TranslateMessage};
