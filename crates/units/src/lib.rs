//! A library for working with units of measurement in Rust.
//! This crate provides a set of macros and types for defining and working with units of measurement,
//! including support for unit conversions and arithmetic operations.

/// Module containing the conversion logic for units of measurement.
pub mod conversion;
/// Module containing definitions for various units of measurement.
pub mod unit_definitions;

pub use conversion::*;
pub use unit_definitions::*;
