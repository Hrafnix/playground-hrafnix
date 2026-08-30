use crate::common::{KEY_WORDS, is_valid_key_with_prefix};
use message::message::{Message, MessageCategory};
use shareable_string::{ShareableString, SharedStringStore};
use std::fmt::Display;

/// Compile-time assertion helper that panics with a message if the condition is false.
macro_rules! const_assert {
    ($x:expr, $msg:expr $(,)?) => {
        let _: () = ::core::assert!($x, $msg);
    };
}

/// Returns true if the port key is not empty and only contains valid characters.
/// The first character must be lowercase a-z.
/// The remaining characters may be lowercase a-z, digits 0-9, and underscores.
#[must_use]
pub const fn is_valid_port_key(s: &str) -> bool {
    is_valid_key_with_prefix(s, "")
}

/// Validates that a port key is not empty and only contains valid characters.
/// The first character must be lowercase a-z.
/// The remaining characters may be lowercase a-z, digits 0-9, and underscores.
#[cfg_attr(feature = "hotpath", hotpath::measure)]
fn validate_port_key(key: &ShareableString) -> Result<(), Message> {
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
    } else if is_valid_port_key(s) {
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

/// A validated port key that is known at compile-time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstPortKey(pub &'static str);

impl ConstPortKey {
    /// Creates a new `ConstPortKey` from a validated literal.
    /// Panics at compile-time if the key is invalid.
    ///
    /// Not part of the public API: use the `port_key!` macro instead,
    /// which wraps this in a `const { }` block so invalid keys are caught
    /// at compile-time rather than only when the code path runs.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(key: &'static str) -> Self {
        const_assert!(is_valid_port_key(key), "Invalid PortKey literal");
        Self(key)
    }

    /// Returns the string slice.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl Display for ConstPortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<&str> for ConstPortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ConstPortKey> for &str {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ConstPortKey) -> bool {
        *self == other.0
    }
}

impl PartialEq<String> for ConstPortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &String) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialEq<ConstPortKey> for String {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ConstPortKey) -> bool {
        self.as_str() == other.0
    }
}

impl PartialEq<ShareableString> for ConstPortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ShareableString) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialEq<ConstPortKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ConstPortKey) -> bool {
        self.as_str() == other.0
    }
}

impl PartialEq<PortKey> for ConstPortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &PortKey) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialEq<ConstPortKey> for PortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ConstPortKey) -> bool {
        self.as_str() == other.0
    }
}

impl PartialOrd<&str> for ConstPortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(*other)
    }
}

impl PartialOrd<ConstPortKey> for &str {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ConstPortKey) -> Option<std::cmp::Ordering> {
        (*self).partial_cmp(other.0)
    }
}

impl PartialOrd<String> for ConstPortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl PartialOrd<ConstPortKey> for String {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ConstPortKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.0)
    }
}

impl PartialOrd<ShareableString> for ConstPortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ShareableString) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl PartialOrd<ConstPortKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ConstPortKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.0)
    }
}

impl PartialOrd<PortKey> for ConstPortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &PortKey) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl PartialOrd<ConstPortKey> for PortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ConstPortKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.0)
    }
}

impl From<ConstPortKey> for PortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: ConstPortKey) -> Self {
        PortKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<&ConstPortKey> for PortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: &ConstPortKey) -> Self {
        PortKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<ConstPortKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: ConstPortKey) -> Self {
        ShareableString::from(value.0)
    }
}

impl From<&ConstPortKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: &ConstPortKey) -> Self {
        ShareableString::from(value.0)
    }
}

/// A validated port key.
/// Port keys must be non-empty and only contain lowercase a-z, digits 0-9, and underscores.
/// The first character must be a-z.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PortKey {
    /// The underlying validated port key string.
    pub key: ShareableString,
}

impl PortKey {
    /// Creates a new `PortKey` from a `ShareableString`.
    ///
    /// # Errors
    ///
    /// Returns an error message if the key is invalid.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn new(key: ShareableString) -> Result<Self, Message> {
        validate_port_key(&key)?;
        Ok(Self { key })
    }

    /// Creates a new `PortKey` from a `ShareableString` without validating the key.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided `ShareableString` is a valid port key
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

    /// Returns a new `PortKey` with its string interned through the given `SharedStringStore`.
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

impl PartialEq<ShareableString> for PortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &ShareableString) -> bool {
        self.key.as_ref() == other.as_ref()
    }
}

impl PartialEq<PortKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &PortKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<&str> for PortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<PortKey> for &str {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &PortKey) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<String> for PortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<PortKey> for String {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn eq(&self, other: &PortKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl AsRef<str> for PortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialOrd<&str> for PortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
}

impl PartialOrd<ShareableString> for PortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &ShareableString) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(other)
    }
}

impl PartialOrd<PortKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &PortKey) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl PartialOrd<PortKey> for &str {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &PortKey) -> Option<std::cmp::Ordering> {
        (*self).partial_cmp(other.as_str())
    }
}

impl PartialOrd<String> for PortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<PortKey> for String {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn partial_cmp(&self, other: &PortKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl Display for PortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl From<PortKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: PortKey) -> Self {
        value.key
    }
}

impl From<&PortKey> for ShareableString {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn from(value: &PortKey) -> Self {
        value.key.clone()
    }
}

impl std::borrow::Borrow<str> for PortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn borrow(&self) -> &str {
        self.key.as_str()
    }
}

impl std::borrow::Borrow<ShareableString> for PortKey {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn borrow(&self) -> &ShareableString {
        &self.key
    }
}

/// A macro to create a `ConstPortKey` from a string literal.
/// Validates the key at compile-time, regardless of whether the result
/// is bound with `let` or `const`.
#[macro_export]
macro_rules! port_key {
    ($key:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::port_key::ConstPortKey::__new($key)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use shareable_string::ShareableString;

    #[test]
    fn test_port_key_comparisons() {
        let sk = PortKey::new(ShareableString::new("key")).unwrap();
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
    fn test_const_port_key_comparisons() {
        let csk = port_key!("key");
        let sk = PortKey::new(ShareableString::new("key")).unwrap();
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
    fn test_is_valid_port_key() {
        assert!(is_valid_port_key("a"));
        assert!(is_valid_port_key("abc"));
        assert!(is_valid_port_key("a123"));
        assert!(is_valid_port_key("a_b_c"));
        assert!(is_valid_port_key("a_1_b_2"));

        assert!(!is_valid_port_key(""));
        assert!(!is_valid_port_key("1abc"));
        assert!(!is_valid_port_key("_abc"));
        assert!(!is_valid_port_key("Abc"));
        assert!(!is_valid_port_key("a-b"));
        assert!(!is_valid_port_key("a b"));
    }

    #[test]
    fn test_const_port_key() {
        const KEY: ConstPortKey = port_key!("valid_key");
        assert_eq!(KEY.as_str(), "valid_key");
        assert_eq!(format!("{KEY}"), "valid_key");

        let port_key: PortKey = KEY.into();
        assert_eq!(port_key.as_str(), "valid_key");

        let port_key_ref: PortKey = (&KEY).into();
        assert_eq!(port_key_ref.as_str(), "valid_key");
    }

    #[test]
    fn test_port_key_macro() {
        const KEY: ConstPortKey = port_key!("macro_key");
        assert_eq!(KEY.as_str(), "macro_key");
    }

    #[test]
    #[should_panic(expected = "Invalid PortKey literal")]
    fn test_const_port_key_invalid() {
        #[allow(clippy::disallowed_methods)]
        let _ = ConstPortKey::__new("Invalid");
    }

    #[test]
    fn test_port_key_from_runtime_string() {
        let s = String::from("runtime_key");
        let key = PortKey::new(s.into()).unwrap();
        assert_eq!(key.as_str(), "runtime_key");

        let invalid_s = String::from("Invalid");
        let result = PortKey::new(invalid_s.into());
        assert!(result.is_err());
    }

    #[test]
    fn test_port_key_as_shareable_string() {
        let key = port_key!("my_key");
        let port_key: PortKey = key.into();

        let shareable: &ShareableString = port_key.as_shareable_string();
        assert_eq!(shareable.as_str(), "my_key");

        let shareable_cloned: ShareableString = (&port_key).into();
        assert_eq!(shareable_cloned.as_str(), "my_key");
    }
}
