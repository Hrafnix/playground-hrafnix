use crate::common::is_valid_key_with_prefix;
use errors::StoreError;
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::fmt::Display;

/// Compile-time assertion helper that panics with a message if the condition is false.
macro_rules! const_assert {
    ($x:expr, $msg:expr $(,)?) => {
        let _: () = ::core::assert!($x, $msg);
    };
}

/// Returns true if the key starts with v_ and the rest is a valid key.
#[must_use]
pub const fn is_valid_variable_key(s: &str) -> bool {
    is_valid_key_with_prefix(s, "v_")
}

/// Validates that a variable key starts with `v_` and has valid remaining characters.
#[hotpath::measure]
fn validate_variable_key(key: &ShareableString) -> Result<(), StoreError> {
    let s = key.as_str();
    if s.is_empty() {
        Err(StoreError::KeyEmpty)
    } else if !s.starts_with("v_") {
        Err(StoreError::KeyInvalidPrefix(s.to_string()))
    } else if is_valid_variable_key(s) {
        Ok(())
    } else {
        Err(StoreError::KeyInvalidCharacter(s.to_string()))
    }
}

/// A validated variable key that is known at compile-time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstVariableKey(pub &'static str);

impl ConstVariableKey {
    /// Creates a new `ConstVariableKey` from a validated literal.
    /// Panics at compile-time if the key is invalid.
    ///
    /// Not part of the public API: use the `variable_key!` macro instead,
    /// which wraps this in a `const { }` block so invalid keys are caught
    /// at compile-time rather than only when the code path runs.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(key: &'static str) -> Self {
        const_assert!(is_valid_variable_key(key), "Invalid VariableKey literal");
        Self(key)
    }

    /// Returns the string slice.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl Display for ConstVariableKey {
    #[hotpath::measure]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ConstVariableKey> for VariableKey {
    #[hotpath::measure]
    fn from(value: ConstVariableKey) -> Self {
        VariableKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<&ConstVariableKey> for VariableKey {
    #[hotpath::measure]
    fn from(value: &ConstVariableKey) -> Self {
        VariableKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<ConstVariableKey> for ShareableString {
    #[hotpath::measure]
    fn from(value: ConstVariableKey) -> Self {
        ShareableString::from(value.0)
    }
}

impl From<&ConstVariableKey> for ShareableString {
    #[hotpath::measure]
    fn from(value: &ConstVariableKey) -> Self {
        ShareableString::from(value.0)
    }
}

/// A validated variable key.
/// Variable keys must start with v_ and follow the rest of the `StoreKey` rules.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariableKey {
    /// The underlying validated variable key string (must start with `v_`).
    pub key: ShareableString,
}

impl VariableKey {
    /// Creates a new `VariableKey` from a `ShareableString`.
    ///
    /// # Errors
    ///
    /// Returns `StoreError::KeyEmpty`, `StoreError::KeyInvalidPrefix`, or `StoreError::KeyInvalidCharacter` if the key is invalid.
    #[hotpath::measure]
    pub fn new(key: ShareableString) -> Result<Self, StoreError> {
        validate_variable_key(&key)?;
        Ok(Self { key })
    }

    /// Creates a new `VariableKey` from a `ShareableString` without validating the key.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided `ShareableString` is a valid variable key (starts with `v_` and contains only valid characters).
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unsafe(key: ShareableString) -> Self {
        Self { key }
    }

    /// Returns the string slice.
    #[must_use]
    #[hotpath::measure]
    pub fn as_str(&self) -> &str {
        self.key.as_str()
    }

    /// Returns the underlying `ShareableString`.
    #[must_use]
    pub const fn as_shareable_string(&self) -> &ShareableString {
        &self.key
    }

    /// Returns a new `VariableKey` with its string interned through the given `SharedStringStore`.
    #[must_use]
    #[hotpath::measure]
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

impl Serialize for VariableKey {
    #[hotpath::measure]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for VariableKey {
    #[hotpath::measure]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        VariableKey::new(ShareableString::from(s)).map_err(serde::de::Error::custom)
    }
}

impl AsRef<str> for VariableKey {
    #[hotpath::measure]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<&str> for VariableKey {
    #[hotpath::measure]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<VariableKey> for &str {
    #[hotpath::measure]
    fn eq(&self, other: &VariableKey) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<String> for VariableKey {
    #[hotpath::measure]
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<VariableKey> for String {
    #[hotpath::measure]
    fn eq(&self, other: &VariableKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<ShareableString> for VariableKey {
    #[hotpath::measure]
    fn eq(&self, other: &ShareableString) -> bool {
        self.key.as_ref() == other.as_ref()
    }
}

impl PartialEq<VariableKey> for ShareableString {
    #[hotpath::measure]
    fn eq(&self, other: &VariableKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialOrd<&str> for VariableKey {
    #[hotpath::measure]
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
}

impl PartialOrd<VariableKey> for &str {
    #[hotpath::measure]
    fn partial_cmp(&self, other: &VariableKey) -> Option<std::cmp::Ordering> {
        (*self).partial_cmp(other.as_str())
    }
}

impl PartialOrd<String> for VariableKey {
    #[hotpath::measure]
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<VariableKey> for String {
    #[hotpath::measure]
    fn partial_cmp(&self, other: &VariableKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<ShareableString> for VariableKey {
    #[hotpath::measure]
    fn partial_cmp(&self, other: &ShareableString) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(other)
    }
}

impl PartialOrd<VariableKey> for ShareableString {
    #[hotpath::measure]
    fn partial_cmp(&self, other: &VariableKey) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl PartialEq<ConstVariableKey> for VariableKey {
    #[hotpath::measure]
    fn eq(&self, other: &ConstVariableKey) -> bool {
        self.as_str() == other.0
    }
}

impl PartialEq<VariableKey> for ConstVariableKey {
    #[hotpath::measure]
    fn eq(&self, other: &VariableKey) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialOrd<ConstVariableKey> for VariableKey {
    #[hotpath::measure]
    fn partial_cmp(&self, other: &ConstVariableKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.0)
    }
}

impl PartialOrd<VariableKey> for ConstVariableKey {
    #[hotpath::measure]
    fn partial_cmp(&self, other: &VariableKey) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl Display for VariableKey {
    #[hotpath::measure]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl From<VariableKey> for ShareableString {
    #[hotpath::measure]
    fn from(value: VariableKey) -> Self {
        value.key
    }
}

impl From<&VariableKey> for ShareableString {
    #[hotpath::measure]
    fn from(value: &VariableKey) -> Self {
        value.key.clone()
    }
}

impl PartialEq<crate::store_key::StoreKey> for VariableKey {
    #[hotpath::measure]
    fn eq(&self, other: &crate::store_key::StoreKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<crate::parameter_key::ParameterKey> for VariableKey {
    #[hotpath::measure]
    fn eq(&self, other: &crate::parameter_key::ParameterKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<crate::store_key::StoreKey> for ConstVariableKey {
    #[hotpath::measure]
    fn eq(&self, other: &crate::store_key::StoreKey) -> bool {
        self.0 == other.as_str()
    }
}

impl std::borrow::Borrow<str> for VariableKey {
    #[hotpath::measure]
    fn borrow(&self) -> &str {
        self.key.as_str()
    }
}

impl std::borrow::Borrow<ShareableString> for VariableKey {
    #[hotpath::measure]
    fn borrow(&self) -> &ShareableString {
        &self.key
    }
}

/// A macro to create a `ConstVariableKey` from a string literal.
/// Validates the key at compile-time, regardless of whether the result
/// is bound with `let` or `const`.
#[macro_export]
macro_rules! variable_key {
    ($key:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::variable_key::ConstVariableKey::__new($key)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use shareable_string::ShareableString;

    #[test]
    fn test_variable_key() {
        let vk = VariableKey::new(ShareableString::new("v_key")).unwrap();
        assert_eq!(vk.as_str(), "v_key");

        let vk2 = variable_key!("v_const");
        assert_eq!(vk2.as_str(), "v_const");

        assert!(VariableKey::new(ShareableString::new("key")).is_err());
        assert!(VariableKey::new(ShareableString::new("p_key")).is_err());
    }

    #[test]
    #[should_panic(expected = "Invalid VariableKey literal")]
    fn test_const_variable_key_invalid() {
        #[allow(clippy::disallowed_methods)]
        let _ = ConstVariableKey::__new("Invalid");
    }
}
