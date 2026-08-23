use crate::common::is_valid_key_with_prefix;
use message::message::{Message, MessageCategory};
use serde::{Deserialize, Serialize};
use shareable_string::{ShareableString, SharedStringStore};
use std::fmt::Display;

/// Compile-time assertion helper that panics with a message if the condition is false.
macro_rules! const_assert {
    ($x:expr, $msg:expr $(,)?) => {
        let _: () = ::core::assert!($x, $msg);
    };
}

/// Returns true if the key starts with g_ and the rest is a valid key.
#[must_use]
pub const fn is_valid_global_key(s: &str) -> bool {
    is_valid_key_with_prefix(s, "g_")
}

/// Validates that a global key starts with `g_` and has valid remaining characters.
#[hotpath::measure]
fn validate_global_key(key: &ShareableString) -> Result<(), Message> {
    let s = key.as_str();
    if !s.starts_with("g_") {
        Err(Message::error_with_param(
            MessageCategory::Datastore,
            "datastore_key_invalid_prefix",
            "key",
            s,
        ))
    } else if s.is_empty() {
        Err(Message::error(
            MessageCategory::Datastore,
            "datastore_key_empty",
        ))
    } else if is_valid_global_key(s) {
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

/// A validated global key that is known at compile-time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstGlobalKey(pub &'static str);

impl ConstGlobalKey {
    /// Creates a new `ConstGlobalKey` from a validated literal.
    /// Panics at compile-time if the key is invalid.
    ///
    /// Not part of the public API: use the `global_key!` macro instead,
    /// which wraps this in a `const { }` block so invalid keys are caught
    /// at compile-time rather than only when the code path runs.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(key: &'static str) -> Self {
        const_assert!(is_valid_global_key(key), "Invalid GlobalKey literal");
        Self(key)
    }

    /// Returns the string slice.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl Display for ConstGlobalKey {
    #[hotpath::measure]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ConstGlobalKey> for GlobalKey {
    #[hotpath::measure]
    fn from(value: ConstGlobalKey) -> Self {
        GlobalKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<&ConstGlobalKey> for GlobalKey {
    #[hotpath::measure]
    fn from(value: &ConstGlobalKey) -> Self {
        GlobalKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<ConstGlobalKey> for ShareableString {
    #[hotpath::measure]
    fn from(value: ConstGlobalKey) -> Self {
        ShareableString::from(value.0)
    }
}

impl From<&ConstGlobalKey> for ShareableString {
    #[hotpath::measure]
    fn from(value: &ConstGlobalKey) -> Self {
        ShareableString::from(value.0)
    }
}

/// A validated global key.
/// Global keys must start with g_ and follow the rest of the `StoreKey` rules.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalKey {
    /// The underlying validated global key string (must start with `g_`).
    pub key: ShareableString,
}

impl GlobalKey {
    /// Creates a new `GlobalKey` from a `ShareableString`.
    ///
    /// # Errors
    ///
    /// Returns an error message if the key is invalid.
    #[hotpath::measure]
    pub fn new(key: ShareableString) -> Result<Self, Message> {
        validate_global_key(&key)?;
        Ok(Self { key })
    }

    /// Creates a new `GlobalKey` from a `ShareableString` without validating the key.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided `ShareableString` is a valid global key (starts with `g_` and contains only valid characters).
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

    /// Returns a new `GlobalKey` with its string interned through the given `SharedStringStore`.
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

impl Serialize for GlobalKey {
    #[hotpath::measure]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for GlobalKey {
    #[hotpath::measure]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        GlobalKey::new(ShareableString::from(s)).map_err(|message| {
            serde::de::Error::custom(message.translate_data().message_key().as_str())
        })
    }
}

impl AsRef<str> for GlobalKey {
    #[hotpath::measure]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<&str> for GlobalKey {
    #[hotpath::measure]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<GlobalKey> for &str {
    #[hotpath::measure]
    fn eq(&self, other: &GlobalKey) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<String> for GlobalKey {
    #[hotpath::measure]
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<GlobalKey> for String {
    #[hotpath::measure]
    fn eq(&self, other: &GlobalKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<ShareableString> for GlobalKey {
    #[hotpath::measure]
    fn eq(&self, other: &ShareableString) -> bool {
        self.key.as_ref() == other.as_ref()
    }
}

impl PartialEq<GlobalKey> for ShareableString {
    #[hotpath::measure]
    fn eq(&self, other: &GlobalKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialOrd<&str> for GlobalKey {
    #[hotpath::measure]
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
}

impl PartialOrd<GlobalKey> for &str {
    #[hotpath::measure]
    fn partial_cmp(&self, other: &GlobalKey) -> Option<std::cmp::Ordering> {
        (*self).partial_cmp(other.as_str())
    }
}

impl PartialOrd<String> for GlobalKey {
    #[hotpath::measure]
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<GlobalKey> for String {
    #[hotpath::measure]
    fn partial_cmp(&self, other: &GlobalKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<ShareableString> for GlobalKey {
    #[hotpath::measure]
    fn partial_cmp(&self, other: &ShareableString) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(other)
    }
}

impl PartialOrd<GlobalKey> for ShareableString {
    #[hotpath::measure]
    fn partial_cmp(&self, other: &GlobalKey) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl PartialEq<ConstGlobalKey> for GlobalKey {
    #[hotpath::measure]
    fn eq(&self, other: &ConstGlobalKey) -> bool {
        self.as_str() == other.0
    }
}

impl PartialEq<GlobalKey> for ConstGlobalKey {
    #[hotpath::measure]
    fn eq(&self, other: &GlobalKey) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialOrd<ConstGlobalKey> for GlobalKey {
    #[hotpath::measure]
    fn partial_cmp(&self, other: &ConstGlobalKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.0)
    }
}

impl PartialOrd<GlobalKey> for ConstGlobalKey {
    #[hotpath::measure]
    fn partial_cmp(&self, other: &GlobalKey) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl Display for GlobalKey {
    #[hotpath::measure]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl From<GlobalKey> for ShareableString {
    #[hotpath::measure]
    fn from(value: GlobalKey) -> Self {
        value.key
    }
}

impl From<&GlobalKey> for ShareableString {
    #[hotpath::measure]
    fn from(value: &GlobalKey) -> Self {
        value.key.clone()
    }
}

impl PartialEq<crate::store_key::StoreKey> for GlobalKey {
    #[hotpath::measure]
    fn eq(&self, other: &crate::store_key::StoreKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<crate::store_key::StoreKey> for ConstGlobalKey {
    #[hotpath::measure]
    fn eq(&self, other: &crate::store_key::StoreKey) -> bool {
        self.0 == other.as_str()
    }
}

impl std::borrow::Borrow<str> for GlobalKey {
    #[hotpath::measure]
    fn borrow(&self) -> &str {
        self.key.as_str()
    }
}

impl std::borrow::Borrow<ShareableString> for GlobalKey {
    #[hotpath::measure]
    fn borrow(&self) -> &ShareableString {
        &self.key
    }
}

/// A macro to create a `ConstGlobalKey` from a string literal.
/// Validates the key at compile-time, regardless of whether the result
/// is bound with `let` or `const`.
#[macro_export]
macro_rules! global_key {
    ($key:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::global_key::ConstGlobalKey::__new($key)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use shareable_string::ShareableString;

    #[test]
    fn test_global_key() {
        let gk = GlobalKey::new(ShareableString::new("g_key")).unwrap();
        assert_eq!(gk.as_str(), "g_key");

        let gk2 = global_key!("g_const");
        assert_eq!(gk2.as_str(), "g_const");

        assert!(GlobalKey::new(ShareableString::new("key")).is_err());
        assert!(GlobalKey::new(ShareableString::new("p_key")).is_err());
        assert!(GlobalKey::new(ShareableString::new("v_key")).is_err());
    }

    #[test]
    #[should_panic(expected = "Invalid GlobalKey literal")]
    fn test_const_global_key_invalid() {
        #[allow(clippy::disallowed_methods)]
        let _ = ConstGlobalKey::__new("Invalid");
    }
}
