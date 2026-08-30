use crate::common::{KEY_WORDS, is_valid_key_with_prefix};
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

/// Returns true if the key is not empty and only contains valid characters.
/// The first character must be lowercase a-z.
/// The remaining characters may be lowercase a-z, digits 0-9, and underscores.
#[must_use]
pub const fn is_valid_component_key(s: &str) -> bool {
    is_valid_key_with_prefix(s, "")
}

/// Validates that a key is not empty and only contains valid characters.
/// The first character must be lowercase a-z.
/// The remaining characters may be lowercase a-z, digits 0-9, and underscores.
#[cfg_attr(feature = "hotpath", hotpath::measure)]
fn validate_key(key: &ShareableString) -> Result<(), Message> {
    let s = key.as_str();

    for keyword in KEY_WORDS {
        if s == keyword {
            return Err(Message::error_with_param(
                MessageCategory::Component,
                "datastore_key_reserved",
                "key",
                s,
            ));
        }
    }

    for c in 'a'..='z' {
        if s.starts_with(&format!("{c}_")) {
            return Err(Message::error_with_param(
                MessageCategory::Component,
                "datastore_key_invalid_prefix",
                "key",
                s,
            ));
        }
    }

    if s.is_empty() {
        Err(Message::error(
            MessageCategory::Component,
            "datastore_key_empty",
        ))
    } else if is_valid_component_key(s) {
        Ok(())
    } else {
        Err(Message::error_with_param(
            MessageCategory::Component,
            "datastore_key_invalid_character",
            "key",
            s,
        ))
    }
}

/// A validated key that is known at compile-time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstComponentKey(pub &'static str);

impl ConstComponentKey {
    /// Creates a new `ConstComponentKey` from a validated literal.
    /// Panics at compile-time if the key is invalid.
    ///
    /// Not part of the public API: use the `component_key!` macro instead,
    /// which wraps this in a `const { }` block so invalid keys are caught
    /// at compile-time rather than only when the code path runs.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(key: &'static str) -> Self {
        const_assert!(is_valid_component_key(key), "Invalid ComponentKey literal");
        Self(key)
    }

    /// Returns the string slice.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl Display for ConstComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<&str> for ConstComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ConstComponentKey> for &str {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ConstComponentKey) -> bool {
        *self == other.0
    }
}

impl PartialEq<String> for ConstComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &String) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialEq<ConstComponentKey> for String {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ConstComponentKey) -> bool {
        self.as_str() == other.0
    }
}

impl PartialEq<ShareableString> for ConstComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ShareableString) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialEq<ConstComponentKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ConstComponentKey) -> bool {
        self.as_str() == other.0
    }
}

impl PartialEq<ComponentKey> for ConstComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ComponentKey) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialEq<ConstComponentKey> for ComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ConstComponentKey) -> bool {
        self.as_str() == other.0
    }
}

impl PartialOrd<&str> for ConstComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(*other)
    }
}

impl PartialOrd<ConstComponentKey> for &str {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ConstComponentKey) -> Option<std::cmp::Ordering> {
        (*self).partial_cmp(other.0)
    }
}

impl PartialOrd<String> for ConstComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl PartialOrd<ConstComponentKey> for String {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ConstComponentKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.0)
    }
}

impl PartialOrd<ShareableString> for ConstComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ShareableString) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl PartialOrd<ConstComponentKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ConstComponentKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.0)
    }
}

impl PartialOrd<ComponentKey> for ConstComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ComponentKey) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl PartialOrd<ConstComponentKey> for ComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ConstComponentKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.0)
    }
}

impl From<ConstComponentKey> for ComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: ConstComponentKey) -> Self {
        ComponentKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<&ConstComponentKey> for ComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: &ConstComponentKey) -> Self {
        ComponentKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<ConstComponentKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: ConstComponentKey) -> Self {
        ShareableString::from(value.0)
    }
}

impl From<&ConstComponentKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: &ConstComponentKey) -> Self {
        ShareableString::from(value.0)
    }
}

/// A validated key.
/// Keys must be non-empty and only contain lowercase a-z, digits 0-9, and underscores.
/// The first character must be a-z.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentKey {
    /// The underlying validated key string.
    pub key: ShareableString,
}

impl ComponentKey {
    /// Creates a new `ComponentKey` from a `ShareableString`.
    ///
    /// # Errors
    ///
    /// Returns an error message if the key is invalid.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new(key: ShareableString) -> Result<Self, Message> {
        validate_key(&key)?;
        Ok(Self { key })
    }

    /// Creates a new `ComponentKey` from a `ShareableString` without validating the key.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided `ShareableString` is a valid component key
    /// (starts without a reserved prefix and contains only valid characters).
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

    /// Returns a new `ComponentKey` with its string interned through the given `SharedStringStore`.
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

    /// Returns true if the key starts with the given prefix.
    #[must_use]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.key.as_str().starts_with(prefix)
    }
}

impl Serialize for ComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ComponentKey::new(ShareableString::from(s)).map_err(|message| {
            serde::de::Error::custom(message.translate_data().message_key().as_str())
        })
    }
}

impl PartialEq<ShareableString> for ComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ShareableString) -> bool {
        self.key.as_ref() == other.as_ref()
    }
}

impl PartialEq<ComponentKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ComponentKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<&str> for ComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<ComponentKey> for &str {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ComponentKey) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<String> for ComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<ComponentKey> for String {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ComponentKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl AsRef<str> for ComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialOrd<&str> for ComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
}

impl PartialOrd<ShareableString> for ComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ShareableString) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(other)
    }
}

impl PartialOrd<ComponentKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ComponentKey) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl PartialOrd<ComponentKey> for &str {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ComponentKey) -> Option<std::cmp::Ordering> {
        (*self).partial_cmp(other.as_str())
    }
}

impl PartialOrd<String> for ComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<ComponentKey> for String {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ComponentKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl Display for ComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl From<ComponentKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: ComponentKey) -> Self {
        value.key
    }
}

impl From<&ComponentKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: &ComponentKey) -> Self {
        value.key.clone()
    }
}

impl std::borrow::Borrow<str> for ComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn borrow(&self) -> &str {
        self.key.as_str()
    }
}

impl std::borrow::Borrow<ShareableString> for ComponentKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn borrow(&self) -> &ShareableString {
        &self.key
    }
}

/// A macro to create a `ConstComponentKey` from a string literal.
/// Validates the key at compile-time, regardless of whether the result
/// is bound with `let` or `const`.
#[macro_export]
macro_rules! component_key {
    ($key:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::component_key::ConstComponentKey::__new($key)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use shareable_string::ShareableString;

    #[test]
    fn test_component_key_comparisons() {
        let sk = ComponentKey::new(ShareableString::new("key")).unwrap();
        let ss = ShareableString::new("key");
        let s = "key";
        let string = String::from("key");

        assert_eq!(sk, ss);
        assert_eq!(ss, sk);
        assert_eq!(sk, s);
        assert_eq!(s, sk);
        assert_eq!(sk, string);
        assert_eq!(string, sk);

        assert!(sk >= ss);
        assert!(ss <= sk);
        assert!(sk >= s);
        assert!(s <= sk);
        assert!(sk >= string);
        assert!(string <= sk);
    }

    #[test]
    fn test_const_component_key_comparisons() {
        let csk = component_key!("key");
        let sk = ComponentKey::new(ShareableString::new("key")).unwrap();
        let ss = ShareableString::new("key");
        let s = "key";
        let string = String::from("key");

        assert_eq!(csk, s);
        assert_eq!(s, csk);
        assert_eq!(csk, string);
        assert_eq!(string, csk);
        assert_eq!(csk, ss);
        assert_eq!(ss, csk);
        assert_eq!(csk, sk);
        assert_eq!(sk, csk);

        assert!(csk >= s);
        assert!(s <= csk);
        assert!(csk >= string);
        assert!(string <= csk);
        assert!(csk >= ss);
        assert!(ss <= csk);
        assert!(csk >= sk);
        assert!(sk <= csk);
    }

    #[test]
    fn test_is_valid_component_key() {
        assert!(is_valid_component_key("a"));
        assert!(is_valid_component_key("abc"));
        assert!(is_valid_component_key("a123"));
        assert!(is_valid_component_key("a_b_c"));
        assert!(is_valid_component_key("a_1_b_2"));

        assert!(!is_valid_component_key(""));
        assert!(!is_valid_component_key("1abc"));
        assert!(!is_valid_component_key("_abc"));
        assert!(!is_valid_component_key("Abc"));
        assert!(!is_valid_component_key("a-b"));
        assert!(!is_valid_component_key("a b"));
    }

    #[test]
    fn test_const_component_key() {
        const KEY: ConstComponentKey = component_key!("valid_key");
        assert_eq!(KEY.as_str(), "valid_key");
        assert_eq!(format!("{KEY}"), "valid_key");

        let component_key: ComponentKey = KEY.into();
        assert_eq!(component_key.as_str(), "valid_key");

        let component_key_ref: ComponentKey = (&KEY).into();
        assert_eq!(component_key_ref.as_str(), "valid_key");
    }

    #[test]
    fn test_component_key_macro() {
        const KEY: ConstComponentKey = component_key!("macro_key");
        assert_eq!(KEY.as_str(), "macro_key");
    }

    #[test]
    #[should_panic(expected = "Invalid ComponentKey literal")]
    fn test_const_component_key_invalid() {
        #[allow(clippy::disallowed_methods)]
        let _ = ConstComponentKey::__new("Invalid");
    }

    #[test]
    fn test_component_key_from_runtime_string() {
        let s = String::from("runtime_key");
        let key = ComponentKey::new(s.into()).unwrap();
        assert_eq!(key.as_str(), "runtime_key");

        let invalid_s = String::from("Invalid");
        let result = ComponentKey::new(invalid_s.into());
        assert!(result.is_err());
    }

    #[test]
    fn test_component_key_as_shareable_string() {
        let key = component_key!("my_key");
        let component_key: ComponentKey = key.into();

        let shareable: &ShareableString = component_key.as_shareable_string();
        assert_eq!(shareable.as_str(), "my_key");

        let shareable_cloned: ShareableString = (&component_key).into();
        assert_eq!(shareable_cloned.as_str(), "my_key");
    }
}
