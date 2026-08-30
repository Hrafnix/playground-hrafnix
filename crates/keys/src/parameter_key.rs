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

/// Returns true if the key starts with p_ and the rest is a valid key.
#[must_use]
pub const fn is_valid_parameter_key(s: &str) -> bool {
    is_valid_key_with_prefix(s, "p_")
}

/// Validates that a parameter key starts with `p_` and has valid remaining characters.
#[cfg_attr(feature = "hotpath", hotpath::measure)]
fn validate_parameter_key(key: &ShareableString) -> Result<(), Message> {
    let s = key.as_str();

    if s.is_empty() {
        Err(Message::error(
            MessageCategory::Datastore,
            "datastore_key_empty",
        ))
    } else if !s.starts_with("p_") {
        Err(Message::error_with_param(
            MessageCategory::Datastore,
            "datastore_key_invalid_prefix",
            "key",
            s,
        ))
    } else if is_valid_parameter_key(s) {
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

/// A validated parameter key that is known at compile-time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstParameterKey(pub &'static str);

impl ConstParameterKey {
    /// Creates a new `ConstParameterKey` from a validated literal.
    /// Panics at compile-time if the key is invalid.
    ///
    /// Not part of the public API: use the `parameter_key!` macro instead,
    /// which wraps this in a `const { }` block so invalid keys are caught
    /// at compile-time rather than only when the code path runs.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(key: &'static str) -> Self {
        const_assert!(is_valid_parameter_key(key), "Invalid ParameterKey literal");
        Self(key)
    }

    /// Returns the string slice.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl Display for ConstParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ConstParameterKey> for ParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: ConstParameterKey) -> Self {
        ParameterKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<&ConstParameterKey> for ParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: &ConstParameterKey) -> Self {
        ParameterKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<ConstParameterKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: ConstParameterKey) -> Self {
        ShareableString::from(value.0)
    }
}

impl From<&ConstParameterKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: &ConstParameterKey) -> Self {
        ShareableString::from(value.0)
    }
}

/// A validated parameter key.
/// Parameter keys must start with p_ and follow the rest of the `StoreKey` rules.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParameterKey {
    /// The underlying validated parameter key string (must start with `p_`).
    pub key: ShareableString,
}

impl ParameterKey {
    /// Creates a new `ParameterKey` from a `ShareableString`.
    ///
    /// # Errors
    ///
    /// Returns an error message if the key is invalid.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new(key: ShareableString) -> Result<Self, Message> {
        validate_parameter_key(&key)?;
        Ok(Self { key })
    }

    /// Creates a new `ParameterKey` from a `ShareableString` without validating the key.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided `ShareableString` is a valid parameter key (starts with `p_` and contains only valid characters).
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unsafe(key: ShareableString) -> Self {
        ParameterKey { key }
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

    /// Returns a new `ParameterKey` with its string interned through the given `SharedStringStore`.
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

impl AsRef<str> for ParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<&str> for ParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<ParameterKey> for &str {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ParameterKey) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<String> for ParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<ParameterKey> for String {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ParameterKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<ShareableString> for ParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ShareableString) -> bool {
        self.key.as_ref() == other.as_ref()
    }
}

impl PartialEq<ParameterKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ParameterKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialOrd<&str> for ParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
}

impl PartialOrd<ParameterKey> for &str {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ParameterKey) -> Option<std::cmp::Ordering> {
        (*self).partial_cmp(other.as_str())
    }
}

impl PartialOrd<String> for ParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<ParameterKey> for String {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ParameterKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<ShareableString> for ParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ShareableString) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(other)
    }
}

impl PartialOrd<ParameterKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ParameterKey) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl PartialEq<ConstParameterKey> for ParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ConstParameterKey) -> bool {
        self.as_str() == other.0
    }
}

impl PartialEq<ParameterKey> for ConstParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ParameterKey) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialOrd<ConstParameterKey> for ParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ConstParameterKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.0)
    }
}

impl PartialOrd<ParameterKey> for ConstParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ParameterKey) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl Display for ParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl From<ParameterKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: ParameterKey) -> Self {
        value.key
    }
}

impl From<&ParameterKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: &ParameterKey) -> Self {
        value.key.clone()
    }
}

impl PartialEq<crate::store_key::StoreKey> for ParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &crate::store_key::StoreKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<crate::variable_key::VariableKey> for ParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &crate::variable_key::VariableKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<crate::store_key::StoreKey> for ConstParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &crate::store_key::StoreKey) -> bool {
        self.0 == other.as_str()
    }
}

impl std::borrow::Borrow<str> for ParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn borrow(&self) -> &str {
        self.key.as_str()
    }
}

impl std::borrow::Borrow<ShareableString> for ParameterKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn borrow(&self) -> &ShareableString {
        &self.key
    }
}

/// A macro to create a `ConstParameterKey` from a string literal.
/// Validates the key at compile-time, regardless of whether the result
/// is bound with `let` or `const`.
#[macro_export]
macro_rules! parameter_key {
    ($key:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::parameter_key::ConstParameterKey::__new($key)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use shareable_string::ShareableString;

    #[test]
    fn test_parameter_key() {
        let pk = ParameterKey::new(ShareableString::new("p_key")).unwrap();
        assert_eq!(pk.as_str(), "p_key");

        let pk2 = parameter_key!("p_const");
        assert_eq!(pk2.as_str(), "p_const");

        assert!(ParameterKey::new(ShareableString::new("key")).is_err());
        assert!(ParameterKey::new(ShareableString::new("v_key")).is_err());
    }

    #[test]
    #[should_panic(expected = "Invalid ParameterKey literal")]
    fn test_const_parameter_key_invalid() {
        #[allow(clippy::disallowed_methods)]
        let _ = ConstParameterKey::__new("Invalid");
    }
}
