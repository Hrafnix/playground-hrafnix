//! Integration tests for the [`StoreError`] type.
//!
//! Covers the `Display`, `Debug`, `Clone`, and `PartialEq` implementations for
//! every [`StoreError`] variant, ensuring that error messages are formatted
//! correctly and that equality comparisons behave as expected.
use datastore::StoreError;
use shareable_string::ShareableString;

#[test]
fn test_store_error_display() {
    assert_eq!(
        format!("{}", StoreError::KeyEmpty),
        "Invalid key: Key cannot be empty"
    );
    assert_eq!(
        format!("{}", StoreError::KeyInvalidCharacter("abc!".to_string())),
        "Invalid key: 'abc!'. Keys must only contain a-z, 0-9 and _"
    );
    assert_eq!(
        format!("{}", StoreError::ObjectNotFound),
        "Object not found"
    );
    assert_eq!(
        format!("{}", StoreError::ObjectKeyAlreadyExists),
        "Object key already exists"
    );
    assert_eq!(
        format!("{}", StoreError::ParameterNotFound),
        "Parameter not found"
    );
    assert_eq!(format!("{}", StoreError::ExpiredProxy), "Proxy is invalid");
    assert_eq!(format!("{}", StoreError::KeyNotFound), "Key not found");
    assert_eq!(format!("{}", StoreError::InvalidPath), "Invalid path");
    assert_eq!(
        format!("{}", StoreError::InvalidPathSegment("segment".to_string())),
        "Invalid path segment: segment"
    );
    assert_eq!(format!("{}", StoreError::IndexNotFound), "Index not found");
    assert_eq!(
        format!("{}", StoreError::UndoNotAvailable),
        "Undo not available"
    );
    assert_eq!(
        format!("{}", StoreError::RedoNotAvailable),
        "Redo not available"
    );
    assert_eq!(
        format!("{}", StoreError::SerializationError("failed".to_string())),
        "Serialization error: failed"
    );
    assert_eq!(
        format!(
            "{}",
            StoreError::ParameterConflict(ShareableString::from("conflict"))
        ),
        "Parameter conflict: conflict"
    );
    assert_eq!(
        format!("{}", StoreError::MissingSchema("schema".to_string())),
        "Missing schema: schema"
    );
    assert_eq!(
        format!("{}", StoreError::SchemaMismatch("mismatch".to_string())),
        "Schema mismatch: mismatch"
    );
    assert_eq!(
        format!("{}", StoreError::NestedContainerNotSupported),
        "Nested containers are not supported in this context"
    );
}

#[test]
fn test_store_error_partial_eq() {
    assert_eq!(StoreError::KeyEmpty, StoreError::KeyEmpty);
    assert_eq!(
        StoreError::KeyInvalidCharacter("a".to_string()),
        StoreError::KeyInvalidCharacter("a".to_string())
    );
    assert_ne!(
        StoreError::KeyInvalidCharacter("a".to_string()),
        StoreError::KeyInvalidCharacter("b".to_string())
    );
    assert_eq!(
        StoreError::InvalidPathSegment("seg".to_string()),
        StoreError::InvalidPathSegment("seg".to_string())
    );
    assert_ne!(
        StoreError::InvalidPathSegment("a".to_string()),
        StoreError::InvalidPathSegment("b".to_string())
    );
    assert_eq!(
        StoreError::SerializationError("e".to_string()),
        StoreError::SerializationError("e".to_string())
    );
    assert_ne!(
        StoreError::SerializationError("a".to_string()),
        StoreError::SerializationError("b".to_string())
    );
    assert_eq!(
        StoreError::ParameterConflict(ShareableString::from("c")),
        StoreError::ParameterConflict(ShareableString::from("c"))
    );
    assert_ne!(
        StoreError::ParameterConflict(ShareableString::from("a")),
        StoreError::ParameterConflict(ShareableString::from("b"))
    );
    assert_eq!(
        StoreError::SchemaMismatch("x".to_string()),
        StoreError::SchemaMismatch("x".to_string())
    );
    assert_ne!(
        StoreError::SchemaMismatch("x".to_string()),
        StoreError::SchemaMismatch("y".to_string())
    );
    assert_eq!(
        StoreError::MissingSchema("s".to_string()),
        StoreError::MissingSchema("s".to_string())
    );
    assert_ne!(
        StoreError::MissingSchema("a".to_string()),
        StoreError::MissingSchema("b".to_string())
    );
    assert_eq!(
        StoreError::NestedContainerNotSupported,
        StoreError::NestedContainerNotSupported
    );
    assert_ne!(StoreError::KeyEmpty, StoreError::ObjectNotFound);
    assert_ne!(
        StoreError::NestedContainerNotSupported,
        StoreError::InvalidPath
    );
}

#[test]
fn test_store_error_clone() {
    let err = StoreError::KeyInvalidCharacter("abc".to_string());
    assert_eq!(err.clone(), err);

    let err = StoreError::SerializationError("oops".to_string());
    assert_eq!(err.clone(), err);

    let err = StoreError::ParameterConflict(ShareableString::from("field"));
    assert_eq!(err.clone(), err);

    let err = StoreError::SchemaMismatch("bad".to_string());
    assert_eq!(err.clone(), err);

    let err = StoreError::NestedContainerNotSupported;
    assert_eq!(err.clone(), err);

    let err = StoreError::KeyEmpty;
    assert_eq!(err.clone(), err);
}

#[test]
fn test_store_error_debug() {
    assert_eq!(format!("{:?}", StoreError::KeyEmpty), "KeyEmpty");
    assert_eq!(
        format!("{:?}", StoreError::KeyInvalidCharacter("x!".to_string())),
        r#"KeyInvalidCharacter("x!")"#
    );
    assert_eq!(
        format!("{:?}", StoreError::ObjectNotFound),
        "ObjectNotFound"
    );
    assert_eq!(
        format!("{:?}", StoreError::ObjectKeyAlreadyExists),
        "ObjectKeyAlreadyExists"
    );
    assert_eq!(
        format!("{:?}", StoreError::ParameterNotFound),
        "ParameterNotFound"
    );
    assert_eq!(format!("{:?}", StoreError::ExpiredProxy), "ExpiredProxy");
    assert_eq!(format!("{:?}", StoreError::KeyNotFound), "KeyNotFound");
    assert_eq!(format!("{:?}", StoreError::InvalidPath), "InvalidPath");
    assert_eq!(
        format!("{:?}", StoreError::InvalidPathSegment("seg".to_string())),
        r#"InvalidPathSegment("seg")"#
    );
    assert_eq!(format!("{:?}", StoreError::IndexNotFound), "IndexNotFound");
    assert_eq!(
        format!("{:?}", StoreError::UndoNotAvailable),
        "UndoNotAvailable"
    );
    assert_eq!(
        format!("{:?}", StoreError::RedoNotAvailable),
        "RedoNotAvailable"
    );
    assert_eq!(
        format!("{:?}", StoreError::SerializationError("err".to_string())),
        r#"SerializationError("err")"#
    );
    assert_eq!(
        format!(
            "{:?}",
            StoreError::ParameterConflict(ShareableString::from("conflict"))
        ),
        format!("ParameterConflict({:?})", ShareableString::from("conflict"))
    );
    assert_eq!(
        format!("{:?}", StoreError::MissingSchema("s".to_string())),
        r#"MissingSchema("s")"#
    );
    assert_eq!(
        format!("{:?}", StoreError::SchemaMismatch("m".to_string())),
        r#"SchemaMismatch("m")"#
    );
    assert_eq!(
        format!("{:?}", StoreError::NestedContainerNotSupported),
        "NestedContainerNotSupported"
    );
}
