//! Integration test root module.
//!
//! Declares sub-modules containing tests organized by topic.
//! Each sub-module focuses on a specific area of the crate's functionality:
//!
//! - [`definition`] – tests for datastore definition types and their builders, ensuring
//!   they correctly represent the intended structures and properties.
//!
//! - [`static_store`] – tests for converting a dynamic `Store` into a `StaticStore` and back,
//!   verifying data fidelity across the round-trip for all container kinds.
//!
//! - [`store`] – tests for the dynamic store, covering proxy access, error handling,
//!   object copying, data recovery, and JSON serialization / deserialization.

mod definition;
mod static_store;
mod store;
