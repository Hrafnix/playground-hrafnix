use crate::common::{KEY_WORDS, is_valid_key_with_prefix};
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

/// Returns true if the key is not empty and only contains valid characters.
/// The first character must be lowercase a-z.
/// The remaining characters may be lowercase a-z, digits 0-9, and underscores.
#[must_use]
pub const fn is_valid_key(s: &str) -> bool {
    is_valid_key_with_prefix(s, "")
}

/// Validates that a key is not empty and only contains valid characters.
/// The first character must be lowercase a-z.
/// The remaining characters may be lowercase a-z, digits 0-9, and underscores.
fn validate_key(key: &ShareableString) -> Result<(), StoreError> {
    let s = key.as_str();

    for keyword in KEY_WORDS {
        if s == keyword {
            return Err(StoreError::KeyReserved(s.to_string()));
        }
    }

    for c in 'a'..='z' {
        if s.starts_with(&format!("{c}_")) {
            return Err(StoreError::KeyInvalidPrefix(s.to_string()));
        }
    }

    if s.is_empty() {
        Err(StoreError::KeyEmpty)
    } else if is_valid_key(s) {
        Ok(())
    } else {
        Err(StoreError::KeyInvalidCharacter(s.to_string()))
    }
}

/// A validated key that is known at compile-time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstStoreKey(pub &'static str);

impl ConstStoreKey {
    /// Creates a new `ConstStoreKey` from a validated literal.
    /// Panics at compile-time if the key is invalid.
    ///
    /// Not part of the public API: use the `store_key!` macro instead,
    /// which wraps this in a `const { }` block so invalid keys are caught
    /// at compile-time rather than only when the code path runs.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(key: &'static str) -> Self {
        const_assert!(is_valid_key(key), "Invalid StoreKey literal");
        Self(key)
    }

    /// Returns the string slice.
    #[must_use]
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

/// A validated key.
/// Keys must be non-empty and only contain lowercase a-z, digits 0-9, and underscores.
/// The first character must be a-z.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StoreKey {
    /// The underlying validated key string.
    pub key: ShareableString,
}

impl StoreKey {
    /// Creates a new `StoreKey` from a `ShareableString`.
    ///
    /// # Errors
    ///
    /// Returns `StoreError::KeyEmpty` or `StoreError::KeyInvalidCharacter` if the key is invalid.
    pub fn new(key: ShareableString) -> Result<Self, StoreError> {
        validate_key(&key)?;
        Ok(Self { key })
    }

    /// Creates a new `StoreKey` from a `ShareableString` without validating the key.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided `ShareableString` is a valid store key (starts without a prefix and contains only valid characters).
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unsafe(key: ShareableString) -> Self {
        Self { key }
    }

    /// Returns the string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.key.as_str()
    }

    /// Returns the underlying `ShareableString`.
    #[must_use]
    pub const fn as_shareable_string(&self) -> &ShareableString {
        &self.key
    }

    /// Returns a new `StoreKey` with its string interned through the given `SharedStringStore`.
    #[must_use]
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
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.key.as_str().starts_with(prefix)
    }
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

impl PartialEq<crate::global_key::GlobalKey> for StoreKey {
    fn eq(&self, other: &crate::global_key::GlobalKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<crate::parameter_key::ParameterKey> for StoreKey {
    fn eq(&self, other: &crate::parameter_key::ParameterKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<crate::variable_key::VariableKey> for StoreKey {
    fn eq(&self, other: &crate::variable_key::VariableKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<crate::unit_key::UnitKey> for StoreKey {
    fn eq(&self, other: &crate::unit_key::UnitKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<crate::global_key::ConstGlobalKey> for StoreKey {
    fn eq(&self, other: &crate::global_key::ConstGlobalKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<crate::parameter_key::ConstParameterKey> for StoreKey {
    fn eq(&self, other: &crate::parameter_key::ConstParameterKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<crate::variable_key::ConstVariableKey> for StoreKey {
    fn eq(&self, other: &crate::variable_key::ConstVariableKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<crate::unit_key::ConstUnitKey> for StoreKey {
    fn eq(&self, other: &crate::unit_key::ConstUnitKey) -> bool {
        self.as_str() == other.as_str()
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
/// Validates the key at compile-time, regardless of whether the result
/// is bound with `let` or `const`.
#[macro_export]
macro_rules! store_key {
    ($key:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::store_key::ConstStoreKey::__new($key)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::global_key::GlobalKey;
    use crate::parameter_key::ParameterKey;
    use crate::unit_key::UnitKey;
    use crate::variable_key::VariableKey;
    use shareable_string::ShareableString;

    #[test]
    fn test_store_key_comparisons() {
        let sk = StoreKey::new(ShareableString::new("key")).unwrap();
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
    fn test_const_store_key_comparisons() {
        let csk = store_key!("key");
        let sk = StoreKey::new(ShareableString::new("key")).unwrap();
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
        const KEY: ConstStoreKey = store_key!("valid_key");
        assert_eq!(KEY.as_str(), "valid_key");
        assert_eq!(format!("{KEY}"), "valid_key");

        let store_key: StoreKey = KEY.into();
        assert_eq!(store_key.as_str(), "valid_key");

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
        #[allow(clippy::disallowed_methods)]
        let _ = ConstStoreKey::__new("Invalid");
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

        let shareable_cloned: ShareableString = (&store_key).into();
        assert_eq!(shareable_cloned.as_str(), "my_key");
    }

    #[test]
    fn test_cross_key_equality() {
        const CP: crate::parameter_key::ConstParameterKey = crate::parameter_key!("p_test");
        const CG: crate::global_key::ConstGlobalKey = crate::global_key!("g_test");
        const CV: crate::variable_key::ConstVariableKey = crate::variable_key!("v_test");
        const CU: crate::unit_key::ConstUnitKey = crate::unit_key!("u_test");

        let p_key = ParameterKey::new(ShareableString::from("p_test")).unwrap();
        let g_key = GlobalKey::new(ShareableString::from("g_test")).unwrap();
        let v_key = VariableKey::new(ShareableString::from("v_test")).unwrap();
        let u_key = UnitKey::new(ShareableString::from("u_test")).unwrap();
        let s_key = StoreKey::new(ShareableString::from("store_test")).unwrap();

        assert_eq!(p_key, p_key);
        assert_eq!(g_key, g_key);
        assert_eq!(v_key, v_key);
        assert_eq!(u_key, u_key);

        assert_ne!(p_key, v_key);
        assert_ne!(v_key, p_key);
        assert_ne!(p_key, s_key);
        assert_ne!(s_key, p_key);
        assert_ne!(g_key, s_key);
        assert_ne!(s_key, g_key);
        assert_ne!(v_key, s_key);
        assert_ne!(s_key, v_key);
        assert_ne!(u_key, s_key);
        assert_ne!(s_key, u_key);

        assert_eq!(CP, p_key);
        assert_eq!(p_key, CP);
        assert_eq!(CV, v_key);
        assert_eq!(v_key, CV);
        assert_eq!(CG, g_key);
        assert_eq!(g_key, CG);
        assert_eq!(CU, u_key);
        assert_eq!(u_key, CU);

        assert_ne!(CP, s_key);
        assert_ne!(s_key, CP);
        assert_ne!(CG, s_key);
        assert_ne!(s_key, CG);
        assert_ne!(CV, s_key);
        assert_ne!(s_key, CV);
        assert_ne!(CU, s_key);
        assert_ne!(s_key, CU);
    }
}
