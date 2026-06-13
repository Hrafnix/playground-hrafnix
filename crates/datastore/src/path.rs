use crate::StoreError;
use crate::key::StoreKey;
use std::fmt::{Display, Formatter};

/// A path to a piece of data within the store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorePath {
    object_key: StoreKey,
    segments: Vec<StoreKey>,
}

impl<S1, S2> From<(S1, S2)> for StorePath
where
    S1: Into<StoreKey>,
    S2: Into<StoreKey>,
{
    fn from((s1, s2): (S1, S2)) -> Self {
        StorePath::new(s1).with_segment(s2)
    }
}

impl<S1, S2, S3> From<(S1, S2, S3)> for StorePath
where
    S1: Into<StoreKey>,
    S2: Into<StoreKey>,
    S3: Into<StoreKey>,
{
    fn from((s1, s2, s3): (S1, S2, S3)) -> Self {
        StorePath::new(s1).with_segment(s2).with_segment(s3)
    }
}

impl<S1, S2, S3, S4> From<(S1, S2, S3, S4)> for StorePath
where
    S1: Into<StoreKey>,
    S2: Into<StoreKey>,
    S3: Into<StoreKey>,
    S4: Into<StoreKey>,
{
    fn from((s1, s2, s3, s4): (S1, S2, S3, S4)) -> Self {
        StorePath::new(s1)
            .with_segment(s2)
            .with_segment(s3)
            .with_segment(s4)
    }
}

impl<S1, S2, S3, S4, S5> From<(S1, S2, S3, S4, S5)> for StorePath
where
    S1: Into<StoreKey>,
    S2: Into<StoreKey>,
    S3: Into<StoreKey>,
    S4: Into<StoreKey>,
    S5: Into<StoreKey>,
{
    fn from((s1, s2, s3, s4, s5): (S1, S2, S3, S4, S5)) -> Self {
        StorePath::new(s1)
            .with_segment(s2)
            .with_segment(s3)
            .with_segment(s4)
            .with_segment(s5)
    }
}

impl Display for StorePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.object_key)?;
        for seg in &self.segments {
            write!(f, "/{}", seg)?;
        }
        Ok(())
    }
}

impl StorePath {
    /// Creates a new `StorePath` pointing to an object.
    pub fn new(object_key: impl Into<StoreKey>) -> Self {
        Self {
            object_key: object_key.into(),
            segments: vec![],
        }
    }

    /// Returns the object key part of the path.
    pub fn object_key(&self) -> &StoreKey {
        &self.object_key
    }

    /// Returns the segments of the path after the object key.
    pub fn segments(&self) -> &Vec<StoreKey> {
        &self.segments
    }

    /// Parses a string into a `StorePath`.
    ///
    /// The string should be in the format `object/segment1/segment2/...`.
    pub fn parse(s: &str) -> Result<Self, StoreError> {
        if s.is_empty() {
            return Err(StoreError::KeyEmpty);
        }
        let mut parts = s.split('/');
        let object_key = parts.next().ok_or(StoreError::KeyEmpty)?;
        let mut segments = Vec::new();

        for part in parts {
            if part.is_empty() {
                return Err(StoreError::InvalidPathSegment(part.to_string()));
            }
            segments.push(StoreKey::new(part.into())?);
        }

        Ok(StorePath {
            object_key: StoreKey::new(object_key.into())?,
            segments,
        })
    }

    /// Pushes a segment key onto the path and returns the new path.
    pub fn with_segment(mut self, key: impl Into<StoreKey>) -> Self {
        self.segments.push(key.into());
        self
    }

    /// Returns a path that points only to the object.
    pub fn get_object(&self) -> Self {
        Self {
            object_key: self.object_key.clone(),
            segments: vec![],
        }
    }

    /// Returns the last key in the path (either the object key or the last segment's key).
    pub fn get_last_key(&self) -> StoreKey {
        self.segments
            .last()
            .cloned()
            .unwrap_or_else(|| self.object_key.clone())
    }
}

impl PartialEq<&StorePath> for StorePath {
    fn eq(&self, other: &&StorePath) -> bool {
        self == *other
    }
}

impl PartialEq<StorePath> for &StorePath {
    fn eq(&self, other: &StorePath) -> bool {
        *self == other
    }
}

/// A macro to create a [`StorePath`] ergonomically.
///
/// The first argument is the object key. Each additional `/`-separated argument
/// appends a segment to the path.
///
/// # Examples
///
/// ```
/// use datastore::path;
/// let p = path!("obj" / "prop" / "nested");
/// assert_eq!(p.to_string(), "obj/prop/nested");
///
/// let p2 = path!("my_obj");
/// assert_eq!(p2.to_string(), "my_obj");
/// ```
#[macro_export]
macro_rules! path {
    ($obj:tt $(/ $seg:tt)+) => {{
        let mut p = $crate::path::StorePath::new($crate::store_key!($obj));
        $(
            p = p.with_segment($crate::store_key!($seg));
        )+
        p
    }};
    ($obj:tt) => {
        $crate::path::StorePath::new($crate::store_key!($obj))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store_key;

    #[test]
    fn test_valid_paths() {
        // Object path
        let path = StorePath::new(store_key!("obj"));
        assert_eq!(path.to_string(), "obj");

        // Property path
        let path = StorePath::new(store_key!("obj")).with_segment(store_key!("prop"));
        assert_eq!(path.to_string(), "obj/prop");

        // Map entry path
        let path = StorePath::new(store_key!("obj"))
            .with_segment(store_key!("prop"))
            .with_segment(store_key!("key"));
        assert_eq!(path.to_string(), "obj/prop/key");

        // Struct item path from property
        let path = StorePath::new(store_key!("obj"))
            .with_segment(store_key!("prop"))
            .with_segment(store_key!("item"));
        assert_eq!(path.to_string(), "obj/prop/item");

        // Struct item path from map entry
        let path = StorePath::new(store_key!("obj"))
            .with_segment(store_key!("prop"))
            .with_segment(store_key!("key"))
            .with_segment(store_key!("item"));
        assert_eq!(path.to_string(), "obj/prop/key/item");
    }

    #[test]
    fn test_ergonomic_paths() {
        // From string
        let p0: StorePath = StorePath::parse("obj/prop/key").unwrap();
        assert_eq!(p0.to_string(), "obj/prop/key");

        // From tuple
        let p1: StorePath = (store_key!("obj"), store_key!("prop")).into();
        assert_eq!(p1.to_string(), "obj/prop");

        let p2: StorePath = (store_key!("obj"), store_key!("prop"), store_key!("key")).into();
        assert_eq!(p2.to_string(), "obj/prop/key");

        let p3: StorePath = (
            store_key!("obj"),
            store_key!("prop"),
            store_key!("key"),
            store_key!("item"),
        )
            .into();
        assert_eq!(p3.to_string(), "obj/prop/key/item");

        let p4: StorePath = (
            store_key!("obj"),
            store_key!("prop"),
            store_key!("key"),
            store_key!("item"),
            store_key!("nested"),
        )
            .into();
        assert_eq!(p4.to_string(), "obj/prop/key/item/nested");
    }

    #[test]
    fn test_path_macro() {
        let p = path!("obj" / "prop" / "key");
        assert_eq!(p.to_string(), "obj/prop/key");

        let p2 = path!("my_obj");
        assert_eq!(p2.to_string(), "my_obj");
    }

    #[test]
    fn test_parse_path() {
        let p = StorePath::parse("obj/prop/key").unwrap();
        assert_eq!(p.to_string(), "obj/prop/key");

        let p2 = StorePath::parse("obj/prop/key/item").unwrap();
        assert_eq!(p2.to_string(), "obj/prop/key/item");

        // Any number of segments is valid
        let p3 = StorePath::parse("obj/prop/key/item/extra").unwrap();
        assert_eq!(p3.to_string(), "obj/prop/key/item/extra");

        let err = StorePath::parse("").unwrap_err();
        assert_eq!(err, StoreError::KeyEmpty);

        let err = StorePath::parse("obj/").unwrap_err();
        assert!(matches!(err, StoreError::InvalidPathSegment(_)));

        let err = StorePath::parse("obj//prop").unwrap_err();
        assert!(matches!(err, StoreError::InvalidPathSegment(_)));
    }

    #[test]
    fn test_get_object() {
        let path = StorePath::parse("obj/prop/key").unwrap();
        let obj_path = path.get_object();
        assert_eq!(obj_path.to_string(), "obj");
        assert!(obj_path.segments().is_empty());
    }

    #[test]
    fn test_get_last_key_object() {
        let path = StorePath::parse("obj").unwrap();
        let obj_path = path.get_last_key();
        assert_eq!(obj_path.to_string(), "obj");
    }

    #[test]
    fn test_get_last_key_full_path() {
        let path = StorePath::parse("obj/prop/key").unwrap();
        let obj_path = path.get_last_key();
        assert_eq!(obj_path.to_string(), "key");
    }

    #[test]
    fn test_path_equality() {
        let p1 = path!("obj" / "prop" / "key");
        let p2 = path!("obj" / "prop" / "key");
        let p3 = path!("obj" / "prop" / "other");
        let p4 = path!("other" / "prop" / "key");

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
        assert_ne!(p1, p4);

        let p5 = StorePath::new(store_key!("obj"));
        let p6 = StorePath::new(store_key!("obj"));
        assert_eq!(p5, p6);

        // Reference comparisons
        assert_eq!(p1, &p2);
        assert_eq!(&p1, p2);
        assert_eq!(&p1, &p2);
    }
}
