use crate::StoreError;
use serde::{Deserialize, Serialize};
use shareable_string::store::SharedStringStore;
use shareable_string::string::ShareableString;
use std::fmt::Display;
use std::hash::Hash;

const KEY_WORDS: [&str; 2] = ["true", "false"];

macro_rules! const_assert {
    ($x:expr, $msg:expr $(,)?) => {
        let _: () = ::core::assert!($x, $msg);
    };
}

#[allow(
    clippy::indexing_slicing,
    reason = "All indexed access is guarded by explicit length and loop-bound checks."
)]
const fn is_valid_key_with_prefix(s: &str, prefix: &str) -> bool {
    let s_bytes = s.as_bytes();
    let prefix_bytes = prefix.as_bytes();

    if s_bytes.len() < prefix_bytes.len() {
        return false;
    }

    let mut i = 0;
    while i < prefix_bytes.len() {
        if s_bytes[i] != prefix_bytes[i] {
            return false;
        }
        i = i.saturating_add(1);
    }

    let rest = s_bytes.len().saturating_sub(prefix_bytes.len());
    if rest == 0 {
        return false;
    }

    let first_after_prefix = s_bytes[prefix_bytes.len()];
    if !first_after_prefix.is_ascii_lowercase() {
        return false;
    }

    if prefix_bytes.is_empty() {
        let mut i = 0;
        while i < KEY_WORDS.len() {
            let keyword = KEY_WORDS[i];
            let keyword_bytes = keyword.as_bytes();
            if s_bytes.len() == keyword_bytes.len() {
                let mut j = 0;
                let mut matches = true;
                while j < keyword_bytes.len() {
                    if s_bytes[j] != keyword_bytes[j] {
                        matches = false;
                        break;
                    }
                    j = j.saturating_add(1);
                }
                if matches {
                    return false;
                }
            }
            i = i.saturating_add(1);
        }
    }

    let mut i = prefix_bytes.len().saturating_add(1);
    while i < s_bytes.len() {
        let c = s_bytes[i];
        if !c.is_ascii_lowercase() && !c.is_ascii_digit() && c != b'_' {
            return false;
        }
        i = i.saturating_add(1);
    }
    true
}

// =====================================================================
// Store key section.
// =====================================================================

/// Returns true if the key is not empty and only contains valid characters.
/// The first character must be lowercase a-z.
/// Remaining characters may be lowercase a-z, digits 0-9, and underscores.
pub const fn is_valid_key(s: &str) -> bool {
    is_valid_key_with_prefix(s, "")
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
    ///
    /// Not part of the public API: use the `store_key!` macro instead,
    /// which wraps this in a `const { }` block so invalid keys are caught
    /// at compile-time rather than only when the code path runs.
    #[doc(hidden)]
    pub const fn __new(key: &'static str) -> Self {
        const_assert!(is_valid_key(key), "Invalid StoreKey literal");
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
    pub(crate) key: ShareableString,
}

impl StoreKey {
    /// Creates a new `StoreKey` from a `ShareableString`.
    /// Returns `StoreError::KeyEmpty` or `StoreError::KeyInvalidCharacter` if the key is invalid.
    pub fn new(key: ShareableString) -> Result<Self, StoreError> {
        validate_key(&key)?;
        Ok(Self { key })
    }

    /// Creates a new `StoreKey` from a `ShareableString` without validating the key.
    #[expect(unsafe_code)]
    pub(crate) unsafe fn new_unsafe(key: ShareableString) -> Self {
        Self { key }
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
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        let laundered_key = store.launder(self.key.clone());

        #[expect(unsafe_code)]
        unsafe {
            Self::new_unsafe(laundered_key)
        }
    }

    /// Returns the BLAKE3 hash of the key.
    pub fn current_blake3_hash(&self) -> [u8; 32] {
        self.key.current_blake3_hash()
    }

    /// Returns true if the key starts with the given prefix.
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
            $crate::key::ConstStoreKey::__new($key)
        }
    };
}

// =====================================================================
// Global key section.
// =====================================================================

/// Returns true if the key starts with g_ and the rest is a valid key.
pub const fn is_valid_global_key(s: &str) -> bool {
    is_valid_key_with_prefix(s, "g_")
}

fn validate_global_key(key: &ShareableString) -> Result<(), StoreError> {
    let s = key.as_str();
    if is_valid_global_key(s) {
        Ok(())
    } else if s.is_empty() {
        Err(StoreError::KeyEmpty)
    } else if !s.starts_with("g_") {
        Err(StoreError::KeyInvalidPrefix(s.to_string()))
    } else {
        Err(StoreError::KeyInvalidCharacter(s.to_string()))
    }
}

/// A validated global key that is known at compile-time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstGlobalKey(pub(crate) &'static str);

impl ConstGlobalKey {
    /// Creates a new `ConstGlobalKey` from a validated literal.
    /// Panics at compile-time if the key is invalid.
    ///
    /// Not part of the public API: use the `global_key!` macro instead,
    /// which wraps this in a `const { }` block so invalid keys are caught
    /// at compile-time rather than only when the code path runs.
    #[doc(hidden)]
    pub const fn __new(key: &'static str) -> Self {
        const_assert!(is_valid_global_key(key), "Invalid GlobalKey literal");
        Self(key)
    }

    /// Returns the string slice.
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl Display for ConstGlobalKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ConstGlobalKey> for GlobalKey {
    fn from(value: ConstGlobalKey) -> Self {
        GlobalKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<&ConstGlobalKey> for GlobalKey {
    fn from(value: &ConstGlobalKey) -> Self {
        GlobalKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<ConstGlobalKey> for ShareableString {
    fn from(value: ConstGlobalKey) -> Self {
        ShareableString::from(value.0)
    }
}

impl From<&ConstGlobalKey> for ShareableString {
    fn from(value: &ConstGlobalKey) -> Self {
        ShareableString::from(value.0)
    }
}

/// A validated global key.
/// Global keys must start with g_ and follow the rest of the StoreKey rules.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalKey {
    pub(crate) key: ShareableString,
}

impl GlobalKey {
    /// Creates a new `GlobalKey` from a `ShareableString`.
    /// Returns `StoreError::KeyEmpty`, `StoreError::KeyInvalidPrefix`, or `StoreError::KeyInvalidCharacter` if the key is invalid.
    pub fn new(key: ShareableString) -> Result<Self, StoreError> {
        validate_global_key(&key)?;
        Ok(Self { key })
    }

    /// Creates a new `GlobalKey` from a `ShareableString` without validating the key.
    #[expect(unsafe_code)]
    pub(crate) unsafe fn new_unsafe(key: ShareableString) -> Self {
        Self { key }
    }

    /// Returns the string slice.
    pub fn as_str(&self) -> &str {
        self.key.as_str()
    }

    /// Returns the underlying `ShareableString`.
    pub fn as_shareable_string(&self) -> &ShareableString {
        &self.key
    }

    /// Returns a new `GlobalKey` with its string interned through the given `SharedStringStore`.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        let laundered_key = store.launder(self.key.clone());

        #[expect(unsafe_code)]
        unsafe {
            Self::new_unsafe(laundered_key)
        }
    }

    /// Returns the BLAKE3 hash of the key.
    pub fn current_blake3_hash(&self) -> [u8; 32] {
        self.key.current_blake3_hash()
    }
}

impl Serialize for GlobalKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for GlobalKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        GlobalKey::new(ShareableString::from(s)).map_err(serde::de::Error::custom)
    }
}

impl AsRef<str> for GlobalKey {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<&str> for GlobalKey {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<GlobalKey> for &str {
    fn eq(&self, other: &GlobalKey) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<String> for GlobalKey {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<GlobalKey> for String {
    fn eq(&self, other: &GlobalKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<ShareableString> for GlobalKey {
    fn eq(&self, other: &ShareableString) -> bool {
        self.key.as_ref() == other.as_ref()
    }
}

impl PartialEq<GlobalKey> for ShareableString {
    fn eq(&self, other: &GlobalKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialOrd<&str> for GlobalKey {
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
}

impl PartialOrd<GlobalKey> for &str {
    fn partial_cmp(&self, other: &GlobalKey) -> Option<std::cmp::Ordering> {
        (*self).partial_cmp(other.as_str())
    }
}

impl PartialOrd<String> for GlobalKey {
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<GlobalKey> for String {
    fn partial_cmp(&self, other: &GlobalKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<ShareableString> for GlobalKey {
    fn partial_cmp(&self, other: &ShareableString) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(other)
    }
}

impl PartialOrd<GlobalKey> for ShareableString {
    fn partial_cmp(&self, other: &GlobalKey) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl PartialEq<ConstGlobalKey> for GlobalKey {
    fn eq(&self, other: &ConstGlobalKey) -> bool {
        self.as_str() == other.0
    }
}

impl PartialEq<GlobalKey> for ConstGlobalKey {
    fn eq(&self, other: &GlobalKey) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialOrd<ConstGlobalKey> for GlobalKey {
    fn partial_cmp(&self, other: &ConstGlobalKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.0)
    }
}

impl PartialOrd<GlobalKey> for ConstGlobalKey {
    fn partial_cmp(&self, other: &GlobalKey) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl Display for GlobalKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl From<GlobalKey> for ShareableString {
    fn from(value: GlobalKey) -> Self {
        value.key
    }
}

impl From<&GlobalKey> for ShareableString {
    fn from(value: &GlobalKey) -> Self {
        value.key.clone()
    }
}

impl std::borrow::Borrow<str> for GlobalKey {
    fn borrow(&self) -> &str {
        self.key.as_str()
    }
}

impl std::borrow::Borrow<ShareableString> for GlobalKey {
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
            $crate::key::ConstGlobalKey::__new($key)
        }
    };
}

// =====================================================================
// Parameter key section.
// =====================================================================

/// Returns true if the key starts with p_ and the rest is a valid key.
pub const fn is_valid_parameter_key(s: &str) -> bool {
    is_valid_key_with_prefix(s, "p_")
}

fn validate_parameter_key(key: &ShareableString) -> Result<(), StoreError> {
    let s = key.as_str();
    if is_valid_parameter_key(s) {
        Ok(())
    } else if s.is_empty() {
        Err(StoreError::KeyEmpty)
    } else if !s.starts_with("p_") {
        Err(StoreError::KeyInvalidPrefix(s.to_string()))
    } else {
        Err(StoreError::KeyInvalidCharacter(s.to_string()))
    }
}

/// A validated parameter key that is known at compile-time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstParameterKey(pub(crate) &'static str);

impl ConstParameterKey {
    /// Creates a new `ConstParameterKey` from a validated literal.
    /// Panics at compile-time if the key is invalid.
    ///
    /// Not part of the public API: use the `parameter_key!` macro instead,
    /// which wraps this in a `const { }` block so invalid keys are caught
    /// at compile-time rather than only when the code path runs.
    #[doc(hidden)]
    pub const fn __new(key: &'static str) -> Self {
        const_assert!(is_valid_parameter_key(key), "Invalid ParameterKey literal");
        Self(key)
    }

    /// Returns the string slice.
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl Display for ConstParameterKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ConstParameterKey> for ParameterKey {
    fn from(value: ConstParameterKey) -> Self {
        ParameterKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<&ConstParameterKey> for ParameterKey {
    fn from(value: &ConstParameterKey) -> Self {
        ParameterKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<ConstParameterKey> for ShareableString {
    fn from(value: ConstParameterKey) -> Self {
        ShareableString::from(value.0)
    }
}

impl From<&ConstParameterKey> for ShareableString {
    fn from(value: &ConstParameterKey) -> Self {
        ShareableString::from(value.0)
    }
}

/// A validated parameter key.
/// Parameter keys must start with p_ and follow the rest of the StoreKey rules.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParameterKey {
    pub(crate) key: ShareableString,
}

impl ParameterKey {
    /// Creates a new `ParameterKey` from a `ShareableString`.
    /// Returns `StoreError::KeyEmpty`, `StoreError::KeyInvalidPrefix`, or `StoreError::KeyInvalidCharacter` if the key is invalid.
    pub fn new(key: ShareableString) -> Result<Self, StoreError> {
        validate_parameter_key(&key)?;
        Ok(Self { key })
    }

    /// Creates a new `ParameterKey` from a `ShareableString` without validating the key.
    #[expect(unsafe_code)]
    pub(crate) unsafe fn new_unsafe(key: ShareableString) -> Self {
        ParameterKey { key }
    }

    /// Returns the string slice.
    pub fn as_str(&self) -> &str {
        self.key.as_str()
    }

    /// Returns the underlying `ShareableString`.
    pub fn as_shareable_string(&self) -> &ShareableString {
        &self.key
    }

    /// Returns a new `ParameterKey` with its string interned through the given `SharedStringStore`.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        let laundered_key = store.launder(self.key.clone());

        #[expect(unsafe_code)]
        unsafe {
            Self::new_unsafe(laundered_key)
        }
    }

    /// Returns the BLAKE3 hash of the key.
    pub fn current_blake3_hash(&self) -> [u8; 32] {
        self.key.current_blake3_hash()
    }
}

impl Serialize for ParameterKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ParameterKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ParameterKey::new(ShareableString::from(s)).map_err(serde::de::Error::custom)
    }
}

impl AsRef<str> for ParameterKey {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<&str> for ParameterKey {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<ParameterKey> for &str {
    fn eq(&self, other: &ParameterKey) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<String> for ParameterKey {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<ParameterKey> for String {
    fn eq(&self, other: &ParameterKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<ShareableString> for ParameterKey {
    fn eq(&self, other: &ShareableString) -> bool {
        self.key.as_ref() == other.as_ref()
    }
}

impl PartialEq<ParameterKey> for ShareableString {
    fn eq(&self, other: &ParameterKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialOrd<&str> for ParameterKey {
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
}

impl PartialOrd<ParameterKey> for &str {
    fn partial_cmp(&self, other: &ParameterKey) -> Option<std::cmp::Ordering> {
        (*self).partial_cmp(other.as_str())
    }
}

impl PartialOrd<String> for ParameterKey {
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<ParameterKey> for String {
    fn partial_cmp(&self, other: &ParameterKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<ShareableString> for ParameterKey {
    fn partial_cmp(&self, other: &ShareableString) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(other)
    }
}

impl PartialOrd<ParameterKey> for ShareableString {
    fn partial_cmp(&self, other: &ParameterKey) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl PartialEq<ConstParameterKey> for ParameterKey {
    fn eq(&self, other: &ConstParameterKey) -> bool {
        self.as_str() == other.0
    }
}

impl PartialEq<ParameterKey> for ConstParameterKey {
    fn eq(&self, other: &ParameterKey) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialOrd<ConstParameterKey> for ParameterKey {
    fn partial_cmp(&self, other: &ConstParameterKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.0)
    }
}

impl PartialOrd<ParameterKey> for ConstParameterKey {
    fn partial_cmp(&self, other: &ParameterKey) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl Display for ParameterKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl From<ParameterKey> for ShareableString {
    fn from(value: ParameterKey) -> Self {
        value.key
    }
}

impl From<&ParameterKey> for ShareableString {
    fn from(value: &ParameterKey) -> Self {
        value.key.clone()
    }
}

impl std::borrow::Borrow<str> for ParameterKey {
    fn borrow(&self) -> &str {
        self.key.as_str()
    }
}

impl std::borrow::Borrow<ShareableString> for ParameterKey {
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
            $crate::key::ConstParameterKey::__new($key)
        }
    };
}

// =====================================================================
// Variable key section.
// =====================================================================

/// Returns true if the key starts with v_ and the rest is a valid key.
pub const fn is_valid_variable_key(s: &str) -> bool {
    is_valid_key_with_prefix(s, "v_")
}

fn validate_variable_key(key: &ShareableString) -> Result<(), StoreError> {
    let s = key.as_str();
    if is_valid_variable_key(s) {
        Ok(())
    } else if s.is_empty() {
        Err(StoreError::KeyEmpty)
    } else if !s.starts_with("v_") {
        Err(StoreError::KeyInvalidPrefix(s.to_string()))
    } else {
        Err(StoreError::KeyInvalidCharacter(s.to_string()))
    }
}

/// A validated variable key that is known at compile-time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstVariableKey(pub(crate) &'static str);

impl ConstVariableKey {
    /// Creates a new `ConstVariableKey` from a validated literal.
    /// Panics at compile-time if the key is invalid.
    ///
    /// Not part of the public API: use the `variable_key!` macro instead,
    /// which wraps this in a `const { }` block so invalid keys are caught
    /// at compile-time rather than only when the code path runs.
    #[doc(hidden)]
    pub const fn __new(key: &'static str) -> Self {
        const_assert!(is_valid_variable_key(key), "Invalid VariableKey literal");
        Self(key)
    }

    /// Returns the string slice.
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl Display for ConstVariableKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ConstVariableKey> for VariableKey {
    fn from(value: ConstVariableKey) -> Self {
        VariableKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<&ConstVariableKey> for VariableKey {
    fn from(value: &ConstVariableKey) -> Self {
        VariableKey {
            key: ShareableString::from(value.0),
        }
    }
}

impl From<ConstVariableKey> for ShareableString {
    fn from(value: ConstVariableKey) -> Self {
        ShareableString::from(value.0)
    }
}

impl From<&ConstVariableKey> for ShareableString {
    fn from(value: &ConstVariableKey) -> Self {
        ShareableString::from(value.0)
    }
}

/// A validated variable key.
/// Variable keys must start with v_ and follow the rest of the StoreKey rules.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariableKey {
    pub(crate) key: ShareableString,
}

impl VariableKey {
    /// Creates a new `VariableKey` from a `ShareableString`.
    /// Returns `StoreError::KeyEmpty`, `StoreError::KeyInvalidPrefix`, or `StoreError::KeyInvalidCharacter` if the key is invalid.
    pub fn new(key: ShareableString) -> Result<Self, StoreError> {
        validate_variable_key(&key)?;
        Ok(Self { key })
    }

    /// Creates a new `VariableKey` from a `ShareableString` without validating the key.
    #[expect(unsafe_code)]
    pub(crate) unsafe fn new_unsafe(key: ShareableString) -> Self {
        Self { key }
    }

    /// Returns the string slice.
    pub fn as_str(&self) -> &str {
        self.key.as_str()
    }

    /// Returns the underlying `ShareableString`.
    pub fn as_shareable_string(&self) -> &ShareableString {
        &self.key
    }

    /// Returns a new `VariableKey` with its string interned through the given `SharedStringStore`.
    pub fn launder(&self, store: &SharedStringStore) -> Self {
        let laundered_key = store.launder(self.key.clone());

        #[expect(unsafe_code)]
        unsafe {
            Self::new_unsafe(laundered_key)
        }
    }

    /// Returns the BLAKE3 hash of the key.
    pub fn current_blake3_hash(&self) -> [u8; 32] {
        self.key.current_blake3_hash()
    }
}

impl Serialize for VariableKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for VariableKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        VariableKey::new(ShareableString::from(s)).map_err(serde::de::Error::custom)
    }
}

impl AsRef<str> for VariableKey {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<&str> for VariableKey {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<VariableKey> for &str {
    fn eq(&self, other: &VariableKey) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<String> for VariableKey {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<VariableKey> for String {
    fn eq(&self, other: &VariableKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<ShareableString> for VariableKey {
    fn eq(&self, other: &ShareableString) -> bool {
        self.key.as_ref() == other.as_ref()
    }
}

impl PartialEq<VariableKey> for ShareableString {
    fn eq(&self, other: &VariableKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialOrd<&str> for VariableKey {
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
}

impl PartialOrd<VariableKey> for &str {
    fn partial_cmp(&self, other: &VariableKey) -> Option<std::cmp::Ordering> {
        (*self).partial_cmp(other.as_str())
    }
}

impl PartialOrd<String> for VariableKey {
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<VariableKey> for String {
    fn partial_cmp(&self, other: &VariableKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<ShareableString> for VariableKey {
    fn partial_cmp(&self, other: &ShareableString) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(other)
    }
}

impl PartialOrd<VariableKey> for ShareableString {
    fn partial_cmp(&self, other: &VariableKey) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl PartialEq<ConstVariableKey> for VariableKey {
    fn eq(&self, other: &ConstVariableKey) -> bool {
        self.as_str() == other.0
    }
}

impl PartialEq<VariableKey> for ConstVariableKey {
    fn eq(&self, other: &VariableKey) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialOrd<ConstVariableKey> for VariableKey {
    fn partial_cmp(&self, other: &ConstVariableKey) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.0)
    }
}

impl PartialOrd<VariableKey> for ConstVariableKey {
    fn partial_cmp(&self, other: &VariableKey) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl Display for VariableKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl From<VariableKey> for ShareableString {
    fn from(value: VariableKey) -> Self {
        value.key
    }
}

impl From<&VariableKey> for ShareableString {
    fn from(value: &VariableKey) -> Self {
        value.key.clone()
    }
}

impl std::borrow::Borrow<str> for VariableKey {
    fn borrow(&self) -> &str {
        self.key.as_str()
    }
}

impl std::borrow::Borrow<ShareableString> for VariableKey {
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
            $crate::key::ConstVariableKey::__new($key)
        }
    };
}

// =====================================================================
// Cross-key relationships.
// =====================================================================

// Equality between GlobalKey and StoreKey
impl PartialEq<StoreKey> for GlobalKey {
    fn eq(&self, other: &StoreKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<GlobalKey> for StoreKey {
    fn eq(&self, other: &GlobalKey) -> bool {
        self.as_str() == other.as_str()
    }
}

// Equality between ParameterKey and StoreKey
impl PartialEq<StoreKey> for ParameterKey {
    fn eq(&self, other: &StoreKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<ParameterKey> for StoreKey {
    fn eq(&self, other: &ParameterKey) -> bool {
        self.as_str() == other.as_str()
    }
}

// Equality between VariableKey and StoreKey
impl PartialEq<StoreKey> for VariableKey {
    fn eq(&self, other: &StoreKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<VariableKey> for StoreKey {
    fn eq(&self, other: &VariableKey) -> bool {
        self.as_str() == other.as_str()
    }
}

// Equality between ParameterKey and VariableKey
impl PartialEq<VariableKey> for ParameterKey {
    fn eq(&self, other: &VariableKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<ParameterKey> for VariableKey {
    fn eq(&self, other: &ParameterKey) -> bool {
        self.as_str() == other.as_str()
    }
}

// Equality between Const types and other Key types
impl PartialEq<ConstGlobalKey> for StoreKey {
    fn eq(&self, other: &ConstGlobalKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<StoreKey> for ConstGlobalKey {
    fn eq(&self, other: &StoreKey) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialEq<ConstParameterKey> for StoreKey {
    fn eq(&self, other: &ConstParameterKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<StoreKey> for ConstParameterKey {
    fn eq(&self, other: &StoreKey) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialEq<ConstVariableKey> for StoreKey {
    fn eq(&self, other: &ConstVariableKey) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<StoreKey> for ConstVariableKey {
    fn eq(&self, other: &StoreKey) -> bool {
        self.0 == other.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_key_equality() {
        let p_key = ParameterKey::new(ShareableString::from("p_test")).unwrap();
        let g_key = GlobalKey::new(ShareableString::from("g_test")).unwrap();
        let v_key = VariableKey::new(ShareableString::from("v_test")).unwrap();
        let s_key_p = StoreKey::new(ShareableString::from("p_test")).unwrap();
        let s_key_g = StoreKey::new(ShareableString::from("g_test")).unwrap();
        let s_key_v = StoreKey::new(ShareableString::from("v_test")).unwrap();

        assert_eq!(p_key, s_key_p);
        assert_eq!(s_key_p, p_key);

        assert_eq!(v_key, s_key_v);
        assert_eq!(s_key_v, v_key);

        assert_eq!(g_key, s_key_g);
        assert_eq!(s_key_g, g_key);

        assert_eq!(p_key, p_key);
        assert_eq!(g_key, g_key);
        assert_eq!(v_key, v_key);

        assert_ne!(p_key, v_key);
        assert_ne!(v_key, p_key);
        assert_ne!(p_key, s_key_v);
        assert_ne!(v_key, s_key_p);
        assert_ne!(g_key, s_key_p);
        assert_ne!(g_key, s_key_v);

        // Const equality
        const CP: ConstParameterKey = parameter_key!("p_test");
        const CG: ConstGlobalKey = global_key!("g_test");
        const CV: ConstVariableKey = variable_key!("v_test");

        assert_eq!(CP, p_key);
        assert_eq!(p_key, CP);
        assert_eq!(CV, v_key);
        assert_eq!(v_key, CV);
        assert_eq!(CG, g_key);
        assert_eq!(g_key, CG);

        assert_eq!(CP, s_key_p);
        assert_eq!(s_key_p, CP);
        assert_eq!(CG, s_key_g);
        assert_eq!(s_key_g, CG);
        assert_eq!(CV, s_key_v);
        assert_eq!(s_key_v, CV);
    }

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
        let csk = store_key!("key");
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
    fn test_parameter_key() {
        let pk = ParameterKey::new(ShareableString::new("p_key")).unwrap();
        assert_eq!(pk.as_str(), "p_key");

        let pk2 = parameter_key!("p_const");
        assert_eq!(pk2.as_str(), "p_const");

        assert!(ParameterKey::new(ShareableString::new("key")).is_err());
        assert!(ParameterKey::new(ShareableString::new("v_key")).is_err());
    }

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
    fn test_variable_key() {
        let vk = VariableKey::new(ShareableString::new("v_key")).unwrap();
        assert_eq!(vk.as_str(), "v_key");

        let vk2 = variable_key!("v_const");
        assert_eq!(vk2.as_str(), "v_const");

        assert!(VariableKey::new(ShareableString::new("key")).is_err());
        assert!(VariableKey::new(ShareableString::new("p_key")).is_err());
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
        // Bypasses the `store_key!` macro on purpose: the macro forces
        // compile-time evaluation via a `const { }` block, which would turn
        // this invalid literal into a compiler error instead of the runtime
        // panic this test exercises.
        #[allow(clippy::disallowed_methods)]
        let _ = ConstStoreKey::__new("Invalid");
    }

    #[test]
    #[should_panic(expected = "Invalid GlobalKey literal")]
    fn test_const_global_key_invalid() {
        // Bypasses the `global_key!` macro on purpose: the macro forces
        // compile-time evaluation via a `const { }` block, which would turn
        // this invalid literal into a compiler error instead of the runtime
        // panic this test exercises.
        #[allow(clippy::disallowed_methods)]
        let _ = ConstGlobalKey::__new("Invalid");
    }

    #[test]
    #[should_panic(expected = "Invalid ParameterKey literal")]
    fn test_const_parameter_key_invalid() {
        // Bypasses the `parameter_key!` macro on purpose: the macro forces
        // compile-time evaluation via a `const { }` block, which would turn
        // this invalid literal into a compiler error instead of the runtime
        // panic this test exercises.
        #[allow(clippy::disallowed_methods)]
        let _ = ConstParameterKey::__new("Invalid");
    }

    #[test]
    #[should_panic(expected = "Invalid VariableKey literal")]
    fn test_const_variable_key_invalid() {
        // Bypasses the `variable_key!` macro on purpose: the macro forces
        // compile-time evaluation via a `const { }` block, which would turn
        // this invalid literal into a compiler error instead of the runtime
        // panic this test exercises.
        #[allow(clippy::disallowed_methods)]
        let _ = ConstVariableKey::__new("Invalid");
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
