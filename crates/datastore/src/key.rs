use crate::StoreError;
use serde::{Deserialize, Serialize};
use shareable_string::store::SharedStringStore;
use shareable_string::string::ShareableString;
use std::fmt::Display;
use std::hash::Hash;

/// Returns true if the key is not empty and only contains valid characters.
/// The first character must be lowercase a-z.
/// Remaining characters may be lowercase a-z, digits 0-9, and underscores.
pub const fn is_valid_key(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let bytes = s.as_bytes();
    let first = bytes[0];
    if !first.is_ascii_lowercase() {
        return false;
    }

    let mut i = 1;
    while i < bytes.len() {
        let c = bytes[i];
        if !c.is_ascii_lowercase() && !c.is_ascii_digit() && c != b'_' {
            return false;
        }
        i += 1;
    }
    true
}

/// Validates that a key is not empty and only contains valid characters.
/// The first character must be lowercase a-z.
/// Remaining characters may be lowercase a-z, digits 0-9, and underscores.
fn validate_key(key: &ShareableString) -> Result<(), StoreError> {
    let s = key.as_str();
    if is_valid_key(s) {
        Ok(())
    } else if s.is_empty() {
        Err(StoreError::KeyEmpty)
    } else {
        Err(StoreError::KeyInvalidCharacter(s.to_string()))
    }
}

/// A validated key that is known at compile-time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstStoreKey(pub(crate) &'static str);

impl ConstStoreKey {
    /// Creates a new `ConstStoreKey` from a validated literal.
    /// Panics at compile-time if the key is invalid.
    pub const fn new(key: &'static str) -> Self {
        if !is_valid_key(key) {
            panic!("Invalid StoreKey literal");
        }
        Self(key)
    }

    /// Returns the string slice.
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl Display for ConstStoreKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<&str> for ConstStoreKey {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ConstStoreKey> for &str {
    fn eq(&self, other: &ConstStoreKey) -> bool {
        *self == other.0
    }
}

impl PartialEq<String> for ConstStoreKey {
    fn eq(&self, other: &String) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialEq<ConstStoreKey> for String {
    fn eq(&self, other: &ConstStoreKey) -> bool {
        self.as_str() == other.0
    }
}

impl PartialEq<ShareableString> for ConstStoreKey {
    fn eq(&self, other: &ShareableString) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialEq<ConstStoreKey> for ShareableString {
    fn eq(&self, other: &ConstStoreKey) -> bool {
        self.as_str() == other.0
    }
}

impl PartialEq<StoreKey> for ConstStoreKey {
    fn eq(&self, other: &StoreKey) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialEq<ConstStoreKey> for StoreKey {
    fn eq(&self, other: &ConstStoreKey) -> bool {
        self.as_str() == other.0
    }
}

impl PartialOrd<&str> for ConstStoreKey {
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(*other)
    }
}

impl PartialOrd<ConstStoreKey> for &str {
    fn partial_cmp(&self, other: &ConstStoreKey) -> Option<std::cmp::Ordering> {
        (*self).partial_cmp(other.0)
    }
}

impl PartialOrd<String> for ConstStoreKey {
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl PartialOrd<ConstStoreKey> for String {
    fn partial_cmp(&self, other: &ConstStoreKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.0)
    }
}

impl PartialOrd<ShareableString> for ConstStoreKey {
    fn partial_cmp(&self, other: &ShareableString) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl PartialOrd<ConstStoreKey> for ShareableString {
    fn partial_cmp(&self, other: &ConstStoreKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.0)
    }
}

impl PartialOrd<StoreKey> for ConstStoreKey {
    fn partial_cmp(&self, other: &StoreKey) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl PartialOrd<ConstStoreKey> for StoreKey {
    fn partial_cmp(&self, other: &ConstStoreKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.0)
    }
}

impl From<ConstStoreKey> for StoreKey {
    fn from(value: ConstStoreKey) -> Self {
        StoreKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<&ConstStoreKey> for StoreKey {
    fn from(value: &ConstStoreKey) -> Self {
        StoreKey {
            key: ShareableString::from(value.0),
        }
    }
}

/// A validated key.
/// Keys must be non-empty and only contain lowercase a-z, digits 0-9, and underscores.
/// The first character must be a-z.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StoreKey {
    pub(crate) key: ShareableString,
}

impl Serialize for StoreKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for StoreKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        StoreKey::new(ShareableString::from(s)).map_err(serde::de::Error::custom)
    }
}

impl StoreKey {
    /// Creates a new `StoreKey` from a `ShareableString`.
    /// Returns `StoreError::KeyEmpty` or `StoreError::KeyInvalidCharacter` if the key is invalid.
    pub fn new(key: ShareableString) -> Result<Self, StoreError> {
        validate_key(&key)?;
        Ok(StoreKey { key })
    }

    /// Creates a new `StoreKey` from a `ShareableString` without validating the key.
    #[expect(unsafe_code)]
    pub(crate) unsafe fn new_unsafe(key: ShareableString) -> Self {
        StoreKey { key }
    }

    /// Returns the string slice.
    pub fn as_str(&self) -> &str {
        self.key.as_str()
    }

    /// Returns the underlying `ShareableString`.
    pub fn as_shareable_string(&self) -> &ShareableString {
        &self.key
    }

    /// Returns a new `StoreKey` with its string interned through the given `SharedStringStore`.
    pub fn launder(&self, store: &SharedStringStore) -> StoreKey {
        let laundered_key = store.launder(self.key.clone());

        #[expect(unsafe_code)]
        unsafe {
            StoreKey::new_unsafe(laundered_key)
        }
    }

    /// Returns the BLAKE3 hash of the key.
    pub fn current_blake3_hash(&self) -> [u8; 32] {
        self.key.current_blake3_hash()
    }
}

impl PartialEq<ShareableString> for StoreKey {
    fn eq(&self, other: &ShareableString) -> bool {
        self.key.as_ref() == other.as_ref()
    }
}

impl PartialEq<StoreKey> for ShareableString {
    fn eq(&self, other: &StoreKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<&str> for StoreKey {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<StoreKey> for &str {
    fn eq(&self, other: &StoreKey) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<String> for StoreKey {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<StoreKey> for String {
    fn eq(&self, other: &StoreKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl AsRef<str> for StoreKey {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialOrd<&str> for StoreKey {
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
}

impl PartialOrd<ShareableString> for StoreKey {
    fn partial_cmp(&self, other: &ShareableString) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(other)
    }
}

impl PartialOrd<StoreKey> for ShareableString {
    fn partial_cmp(&self, other: &StoreKey) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl PartialOrd<StoreKey> for &str {
    fn partial_cmp(&self, other: &StoreKey) -> Option<std::cmp::Ordering> {
        (*self).partial_cmp(other.as_str())
    }
}

impl PartialOrd<String> for StoreKey {
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<StoreKey> for String {
    fn partial_cmp(&self, other: &StoreKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl Display for StoreKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl From<ConstStoreKey> for ShareableString {
    fn from(value: ConstStoreKey) -> Self {
        ShareableString::from(value.0)
    }
}

impl From<&ConstStoreKey> for ShareableString {
    fn from(value: &ConstStoreKey) -> Self {
        ShareableString::from(value.0)
    }
}

impl From<StoreKey> for ShareableString {
    fn from(value: StoreKey) -> Self {
        value.key
    }
}

impl From<&StoreKey> for ShareableString {
    fn from(value: &StoreKey) -> Self {
        value.key.clone()
    }
}

impl std::borrow::Borrow<str> for StoreKey {
    fn borrow(&self) -> &str {
        self.key.as_str()
    }
}

impl std::borrow::Borrow<ShareableString> for StoreKey {
    fn borrow(&self) -> &ShareableString {
        &self.key
    }
}

/// A macro to create a `ConstStoreKey` from a string literal.
/// Validates the key at compile-time.
#[macro_export]
macro_rules! store_key {
    ($key:expr) => {
        $crate::key::ConstStoreKey::new($key)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_key_comparisons() {
        let sk = StoreKey::new(ShareableString::new("key")).unwrap();
        let ss = ShareableString::new("key");
        let s = "key";
        let string = String::from("key");

        // PartialEq
        assert_eq!(sk, ss);
        assert_eq!(ss, sk);
        assert_eq!(sk, s);
        assert_eq!(s, sk);
        assert_eq!(sk, s);
        assert_eq!(s, sk);
        assert_eq!(sk, string);
        assert_eq!(string, sk);

        // PartialOrd
        assert!(sk >= ss);
        assert!(ss <= sk);
        assert!(sk >= s);
        assert!(s <= sk);
        assert!(sk >= s);
        assert!(s <= sk);
        assert!(sk >= string);
        assert!(string <= sk);
    }

    #[test]
    fn test_const_store_key_comparisons() {
        let csk = ConstStoreKey::new("key");
        let sk = StoreKey::new(ShareableString::new("key")).unwrap();
        let ss = ShareableString::new("key");
        let s = "key";
        let string = String::from("key");

        // PartialEq
        assert_eq!(csk, s);
        assert_eq!(s, csk);
        assert_eq!(csk, s);
        assert_eq!(s, csk);
        assert_eq!(csk, string);
        assert_eq!(string, csk);
        assert_eq!(csk, ss);
        assert_eq!(ss, csk);
        assert_eq!(csk, sk);
        assert_eq!(sk, csk);

        // PartialOrd
        assert!(csk >= s);
        assert!(s <= csk);
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
    fn test_is_valid_key() {
        assert!(is_valid_key("a"));
        assert!(is_valid_key("abc"));
        assert!(is_valid_key("a123"));
        assert!(is_valid_key("a_b_c"));
        assert!(is_valid_key("a_1_b_2"));

        assert!(!is_valid_key(""));
        assert!(!is_valid_key("1abc"));
        assert!(!is_valid_key("_abc"));
        assert!(!is_valid_key("Abc"));
        assert!(!is_valid_key("a-b"));
        assert!(!is_valid_key("a b"));
    }

    #[test]
    fn test_const_store_key() {
        const KEY: ConstStoreKey = ConstStoreKey::new("valid_key");
        assert_eq!(KEY.as_str(), "valid_key");
        assert_eq!(format!("{}", KEY), "valid_key");

        // From<ConstStoreKey>
        let store_key: StoreKey = KEY.into();
        assert_eq!(store_key.as_str(), "valid_key");

        // From<&ConstStoreKey>
        let store_key_ref: StoreKey = (&KEY).into();
        assert_eq!(store_key_ref.as_str(), "valid_key");
    }

    #[test]
    fn test_store_key_macro() {
        const KEY: ConstStoreKey = store_key!("macro_key");
        assert_eq!(KEY.as_str(), "macro_key");
    }

    #[test]
    #[should_panic(expected = "Invalid StoreKey literal")]
    fn test_const_store_key_invalid() {
        let _ = ConstStoreKey::new("Invalid");
    }

    #[test]
    fn test_store_key_from_runtime_string() {
        let s = String::from("runtime_key");
        let key = StoreKey::new(s.into()).unwrap();
        assert_eq!(key.as_str(), "runtime_key");

        let invalid_s = String::from("Invalid");
        let result = StoreKey::new(invalid_s.into());
        assert!(result.is_err());
    }

    #[test]
    fn test_store_key_as_shareable_string() {
        let key = store_key!("my_key");
        let store_key: StoreKey = key.into();

        let shareable: &ShareableString = store_key.as_shareable_string();
        assert_eq!(shareable.as_str(), "my_key");

        // From<&StoreKey> for ShareableString
        let shareable_cloned: ShareableString = (&store_key).into();
        assert_eq!(shareable_cloned.as_str(), "my_key");
    }
}
