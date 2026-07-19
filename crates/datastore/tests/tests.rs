//! Integration test root module.
//!
//! Declares sub-modules containing tests organized by topic.
//! Each sub-module focuses on a specific area of the crate's functionality:
//!
//! - [`definition`] – tests for datastore definition types and their builders, ensuring
//!   they correctly represent the intended structures and parameters.
//!
//! - [`store`] – tests for the dynamic store, covering proxy access, error handling,
//!   object copying, data recovery, and JSON serialization / deserialization.

mod definition;
mod editable;
mod frozen;
