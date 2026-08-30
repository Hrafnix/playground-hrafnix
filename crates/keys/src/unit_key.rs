use crate::common::is_valid_key_with_prefix;
use message::message::{Message, MessageCategory};
use shareable_string::{ShareableString, SharedStringStore};
use std::fmt::Display;

/// Compile-time assertion helper that panics with a message if the condition is false.
macro_rules! const_assert {
    ($x:expr, $msg:expr $(,)?) => {
        let _: () = ::core::assert!($x, $msg);
    };
}

/// Returns whether a string is a valid unit key.
#[must_use]
pub const fn is_valid_unit_key(s: &str) -> bool {
    is_valid_key_with_prefix(s, "u_")
}

/// Validates that a unit key starts with `u_` and has valid remaining characters.
#[cfg_attr(feature = "hotpath", hotpath::measure)]
fn validate_unit_key(key: &ShareableString) -> Result<(), Message> {
    let s = key.as_str();

    if s.is_empty() {
        Err(Message::error(
            MessageCategory::Datastore,
            "datastore_key_empty",
        ))
    } else if !s.starts_with("u_") {
        Err(Message::error_with_param(
            MessageCategory::Datastore,
            "datastore_key_invalid_prefix",
            "key",
            s,
        ))
    } else if is_valid_unit_key(s) {
        Ok(())
    } else {
        Err(Message::error_with_param(
            MessageCategory::Datastore,
            "datastore_key_invalid_character",
            "key",
            s,
        ))
    }
}

/// A validated unit key that is known at compile-time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstUnitKey(pub &'static str);

impl ConstUnitKey {
    /// Creates a new `ConstUnitKey` from a validated literal.
    /// Panics at compile-time if the key is invalid.
    ///
    /// Not part of the public API: use the `unit_key!` macro instead,
    /// which wraps this in a `const { }` block so invalid keys are caught
    /// at compile-time rather than only when the code path runs.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(key: &'static str) -> Self {
        const_assert!(is_valid_unit_key(key), "Invalid UnitKey literal");
        Self(key)
    }

    /// Returns the string slice.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl Display for ConstUnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ConstUnitKey> for UnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: ConstUnitKey) -> Self {
        Self {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<&ConstUnitKey> for UnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: &ConstUnitKey) -> Self {
        Self {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<ConstUnitKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: ConstUnitKey) -> Self {
        ShareableString::from(value.0)
    }
}

impl From<&ConstUnitKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: &ConstUnitKey) -> Self {
        ShareableString::from(value.0)
    }
}

/// A validated unit key.
/// Unit keys must start with `u_` and follow the rest of the `StoreKey` rules.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitKey {
    /// The underlying validated unit key string (must start with `u_`).
    pub key: ShareableString,
}

impl UnitKey {
    /// Creates a new `UnitKey` from a `ShareableString`.
    ///
    /// # Errors
    ///
    /// Returns an error message if the key is invalid.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new(key: ShareableString) -> Result<Self, Message> {
        validate_unit_key(&key)?;
        Ok(Self { key })
    }

    /// Creates a new `UnitKey` from a `ShareableString` without validating the key.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided `ShareableString` is a valid unit key (starts with `u_` and contains only valid characters).
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unsafe(key: ShareableString) -> Self {
        Self { key }
    }

    /// Returns the string slice.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn as_str(&self) -> &str {
        self.key.as_str()
    }

    /// Returns the underlying `ShareableString`.
    #[must_use]
    pub const fn as_shareable_string(&self) -> &ShareableString {
        &self.key
    }

    /// Returns a new `UnitKey` with its string interned through the given `SharedStringStore`.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        let laundered_key = store.launder(self.key.clone());

        #[expect(unsafe_code)]
        unsafe {
            Self::new_unsafe(laundered_key)
        }
    }

    /// Returns the BLAKE3 hash of the key.
    #[must_use]
    pub const fn current_blake3_hash(&self) -> [u8; 32] {
        self.key.current_blake3_hash()
    }
}

impl AsRef<str> for UnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<&str> for UnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<UnitKey> for &str {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &UnitKey) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<String> for UnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<UnitKey> for String {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &UnitKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<ShareableString> for UnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ShareableString) -> bool {
        self.key.as_ref() == other.as_ref()
    }
}

impl PartialEq<UnitKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &UnitKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialOrd<&str> for UnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
}

impl PartialOrd<UnitKey> for &str {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &UnitKey) -> Option<std::cmp::Ordering> {
        (*self).partial_cmp(other.as_str())
    }
}

impl PartialOrd<String> for UnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<UnitKey> for String {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &UnitKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<ShareableString> for UnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ShareableString) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(other)
    }
}

impl PartialOrd<UnitKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &UnitKey) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl PartialEq<ConstUnitKey> for UnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ConstUnitKey) -> bool {
        self.as_str() == other.0
    }
}

impl PartialEq<UnitKey> for ConstUnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &UnitKey) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialOrd<ConstUnitKey> for UnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ConstUnitKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.0)
    }
}

impl PartialOrd<UnitKey> for ConstUnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &UnitKey) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl Display for UnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl From<UnitKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: UnitKey) -> Self {
        value.key
    }
}

impl From<&UnitKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: &UnitKey) -> Self {
        value.key.clone()
    }
}

impl PartialEq<crate::store_key::StoreKey> for UnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &crate::store_key::StoreKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<crate::store_key::StoreKey> for ConstUnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &crate::store_key::StoreKey) -> bool {
        self.0 == other.as_str()
    }
}

impl std::borrow::Borrow<str> for UnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn borrow(&self) -> &str {
        self.key.as_str()
    }
}

impl std::borrow::Borrow<ShareableString> for UnitKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn borrow(&self) -> &ShareableString {
        &self.key
    }
}

/// A macro to create a `ConstUnitKey` from a string literal.
/// Validates the key at compile-time, regardless of whether the result
/// is bound with `let` or `const`.
#[macro_export]
macro_rules! unit_key {
    ($key:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::unit_key::ConstUnitKey::__new($key)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use shareable_string::ShareableString;

    #[test]
    fn test_unit_key() {
        let uk = UnitKey::new(ShareableString::new("u_key")).unwrap();
        assert_eq!(uk.as_str(), "u_key");

        let uk2 = unit_key!("u_const");
        assert_eq!(uk2.as_str(), "u_const");

        assert!(UnitKey::new(ShareableString::new("key")).is_err());
        assert!(UnitKey::new(ShareableString::new("p_key")).is_err());
        assert!(UnitKey::new(ShareableString::new("v_key")).is_err());
    }

    #[test]
    #[should_panic(expected = "Invalid UnitKey literal")]
    fn test_const_unit_key_invalid() {
        #[allow(clippy::disallowed_methods)]
        let _ = ConstUnitKey::__new("Invalid");
    }
}
