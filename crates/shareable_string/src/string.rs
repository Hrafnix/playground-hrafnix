use blake3;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// An immutable, shareable string that includes a precomputed BLAKE3 hash.
/// It uses an `Arc<str>` for efficient sharing and memory management.
#[derive(Debug, Clone)]
pub struct ShareableString {
    data: Arc<str>,
    blake3_hash: [u8; 32],
}

impl ShareableString {
    /// Creates a new `ShareableString` from the given value and computes its BLAKE3 hash.
    pub fn new<S: Into<String>>(value: S) -> Self {
        let s: String = value.into();

        // Domain separation for leaf hashing.
        let mut h = blake3::Hasher::new();
        h.update(&[0x00]);
        h.update(s.as_bytes());
        let digest = h.finalize();

        let blake3_hash = *digest.as_bytes();

        Self {
            data: Arc::from(s),
            blake3_hash,
        }
    }

    /// Returns the precomputed BLAKE3 hash of the string.
    pub fn current_blake3_hash(&self) -> [u8; 32] {
        self.blake3_hash
    }

    /// Returns the string as a string slice.
    pub fn as_str(&self) -> &str {
        &self.data
    }

    /// Returns a reference to the underlying `Arc<str>`.
    pub fn as_arc(&self) -> &Arc<str> {
        &self.data
    }

    /// Returns true if both `ShareableString`s point to the same memory location.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.data, &other.data)
    }
}

impl Borrow<str> for ShareableString {
    fn borrow(&self) -> &str {
        &self.data
    }
}

impl Hash for ShareableString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl PartialEq for ShareableString {
    fn eq(&self, other: &Self) -> bool {
        self.ptr_eq(other) || (self.blake3_hash == other.blake3_hash && *self.data == *other.data)
    }
}

impl PartialEq<str> for ShareableString {
    fn eq(&self, other: &str) -> bool {
        &*self.data == other
    }
}

impl PartialEq<&str> for ShareableString {
    fn eq(&self, other: &&str) -> bool {
        &*self.data == *other
    }
}

impl PartialEq<ShareableString> for str {
    fn eq(&self, other: &ShareableString) -> bool {
        self == &*other.data
    }
}

impl PartialEq<ShareableString> for &str {
    fn eq(&self, other: &ShareableString) -> bool {
        *self == &*other.data
    }
}

impl PartialEq<String> for ShareableString {
    fn eq(&self, other: &String) -> bool {
        &*self.data == other.as_str()
    }
}

impl PartialEq<ShareableString> for String {
    fn eq(&self, other: &ShareableString) -> bool {
        self.as_str() == &*other.data
    }
}

impl From<String> for ShareableString {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&String> for ShareableString {
    fn from(value: &String) -> Self {
        Self::new(value.as_str())
    }
}

impl From<&str> for ShareableString {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl Eq for ShareableString {}

impl Ord for ShareableString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for ShareableString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<str> for ShareableString {
    fn partial_cmp(&self, other: &str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other)
    }
}

impl PartialOrd<&str> for ShareableString {
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
}

impl PartialOrd<ShareableString> for str {
    fn partial_cmp(&self, other: &ShareableString) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl PartialOrd<ShareableString> for &str {
    fn partial_cmp(&self, other: &ShareableString) -> Option<std::cmp::Ordering> {
        (*self).partial_cmp(other.as_str())
    }
}

impl PartialOrd<String> for ShareableString {
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<ShareableString> for String {
    fn partial_cmp(&self, other: &ShareableString) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl Serialize for ShareableString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.data)
    }
}

impl<'de> Deserialize<'de> for ShareableString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(ShareableString::new(s))
    }
}

impl Default for ShareableString {
    fn default() -> Self {
        Self::new("")
    }
}

impl AsRef<str> for ShareableString {
    fn as_ref(&self) -> &str {
        &self.data
    }
}

impl From<&ShareableString> for ShareableString {
    fn from(value: &ShareableString) -> Self {
        value.clone()
    }
}

impl std::fmt::Display for ShareableString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fxhashmap_lookup_by_str() {
        use rustc_hash::FxHashMap;
        let key = ShareableString::new("k");

        let mut map: FxHashMap<ShareableString, usize> = FxHashMap::default();
        map.insert(key.clone(), 123);

        assert_eq!(map.get("k"), Some(&123));
        assert_eq!(map.get("missing"), None);
    }

    #[test]
    fn test_display() {
        let s = ShareableString::new("hello");
        assert_eq!(format!("{s}"), "hello");
    }

    #[test]
    fn test_sorting_by_string_value() {
        let mut v = vec![
            ShareableString::new("b"),
            ShareableString::new("a"),
            ShareableString::new("c"),
        ];

        v.sort();

        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_clone_shares_arc() {
        let s1 = ShareableString::new("hello");
        let s2 = s1.clone();

        assert_eq!(s1, s2);
        assert!(Arc::ptr_eq(s1.as_arc(), s2.as_arc()));
        assert_eq!(s1.as_ref(), "hello");
    }

    #[test]
    fn test_equality_with_str_and_string() {
        let s = ShareableString::new("hello");

        // ShareableString with &str
        assert_eq!(s, "hello");
        assert_ne!(s, "world");

        // &str with ShareableString
        assert_eq!("hello", s);
        assert_ne!("world", s);

        // ShareableString with str
        assert_eq!(s, *"hello");
        assert_ne!(s, *"world");

        // str with ShareableString
        assert_eq!(*"hello", s);
        assert_ne!(*"world", s);

        // ShareableString with String
        assert_eq!(s, String::from("hello"));
        assert_ne!(s, String::from("world"));

        // String with ShareableString
        assert_eq!(String::from("hello"), s);
        assert_ne!(String::from("world"), s);

        // Explicit str slice with ShareableString
        let slice: &str = "hello";
        assert_eq!(slice, s);
    }

    #[test]
    fn test_partial_ord_with_str_and_string() {
        let s = ShareableString::new("m");

        // ShareableString with &str
        assert!(s > "a");
        assert!(s < "z");
        assert!(s <= "m");
        assert!(s >= "m");

        // &str with ShareableString
        assert!("a" < s);
        assert!("z" > s);
        assert!("m" <= s);
        assert!("m" >= s);

        // ShareableString with str
        assert!(s > *"a");
        assert!(s < *"z");
        assert!(s <= *"m");
        assert!(s >= *"m");

        // str with ShareableString
        assert!(*"a" < s);
        assert!(*"z" > s);
        assert!(*"m" <= s);
        assert!(*"m" >= s);

        // ShareableString with String
        assert!(s > String::from("a"));
        assert!(s < String::from("z"));

        // String with ShareableString
        assert!(String::from("a") < s);
        assert!(String::from("z") > s);
    }

    #[test]
    fn test_current_hash_stable_for_same_content() {
        let a1 = ShareableString::new("same");
        let a2 = ShareableString::new("same");
        let b = ShareableString::new("different");

        assert_eq!(a1.current_blake3_hash(), a2.current_blake3_hash());
        assert_ne!(a1.current_blake3_hash(), b.current_blake3_hash());
    }

    #[test]
    fn test_empty_and_unicode() {
        let empty = ShareableString::new("");
        assert_eq!(empty.as_ref(), "");
        assert_eq!(format!("{empty}"), "");

        let uni = ShareableString::new("🦀 ¡Hola! 🦀");
        assert_eq!(uni.as_ref(), "🦀 ¡Hola! 🦀");
        assert_eq!(format!("{uni}"), "🦀 ¡Hola! 🦀");
    }

    #[test]
    fn test_serde_serialization() {
        let s = ShareableString::new("serde test");
        let serialized = serde_json::to_string(&s).unwrap();
        assert_eq!(serialized, "\"serde test\"");

        let deserialized: ShareableString = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, s);
        assert_eq!(deserialized.current_blake3_hash(), s.current_blake3_hash());
    }

    #[test]
    fn test_deserialize_error_propagated_from_string() {
        // Each of these passes a non-string JSON Value directly into ShareableString's
        // Deserialize impl, causing `String::deserialize(deserializer)?` at line 191
        // to return an error that is propagated via `?`.
        let non_strings: &[serde_json::Value] = &[
            serde_json::json!(42),
            serde_json::json!(3.14),
            serde_json::json!(true),
            serde_json::json!(null),
            serde_json::json!(["a", "b"]),
            serde_json::json!({"key": "value"}),
        ];
        for value in non_strings {
            let result = serde_json::from_value::<ShareableString>(value.clone());
            assert!(result.is_err(), "expected error for value: {value}");
        }
    }

    #[test]
    fn test_from_traits() {
        let s1: ShareableString = "from str".into();
        let s2: ShareableString = String::from("from string").into();

        assert_eq!(s1, "from str");
        assert_eq!(s2, "from string");

        let s3_orig = String::from("from string ref");
        let s3: ShareableString = (&s3_orig).into();
        assert_eq!(s3, "from string ref");
    }

    #[test]
    fn test_default() {
        let s = ShareableString::default();
        assert_eq!(s, "");
        assert_eq!(
            s.current_blake3_hash(),
            ShareableString::new("").current_blake3_hash()
        );
    }

    #[test]
    fn test_as_ref_and_borrow() {
        use std::borrow::Borrow;
        let s = ShareableString::new("test");
        let r: &str = s.as_ref();
        let b: &str = s.borrow();

        assert_eq!(r, "test");
        assert_eq!(b, "test");
    }

    #[test]
    fn test_long_string() {
        let long_str = "a".repeat(10000);
        let s = ShareableString::new(&long_str);
        assert_eq!(s.as_str(), long_str);
        assert_eq!(s.current_blake3_hash().len(), 32);
    }

    #[test]
    fn test_special_characters() {
        let special = "!@#$%^&*()_+-=[]{}|;':\",./<>? \\";
        let s = ShareableString::new(special);
        assert_eq!(s.as_str(), special);
    }
}
