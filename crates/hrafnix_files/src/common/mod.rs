use camino::{Utf8Path, Utf8PathBuf};
use hrafnix_message::message::{Message, MessageCategory, MessageLevel};
use hrafnix_shareable_string::ShareableString;
use serde::de::value::MapAccessDeserializer;
use serde::de::{DeserializeOwned, DeserializeSeed, IgnoredAny, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use serde_json::error::Category;
use std::collections::HashMap;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Read, Write};
use std::marker::PhantomData;
use std::str::Utf8Error;
use std::sync::atomic::{AtomicU64, Ordering};

/// The file version used by default when writing components and models.
pub const FILE_VERSION: u32 = 1;

/// The largest file, in bytes, that is read into memory: 256 MiB.
pub const MAX_FILE_SIZE: u64 = 256 * 1024 * 1024;

/// The top-level key that carries the file format version, as in
/// `{"file-version": 1, "model": {}}`.
const FILE_VERSION_KEY: &str = "file-version";

/// The byte order mark some editors prepend to UTF-8 text.
const BYTE_ORDER_MARK: char = '\u{feff}';

/// Darwin's `ENOTSUP`, which differs from `EOPNOTSUPP` there and is therefore left as
/// [`ErrorKind::Uncategorized`](ErrorKind) by `std` rather than mapped to
/// [`ErrorKind::Unsupported`].
#[cfg(target_vendor = "apple")]
const ENOTSUP: i32 = 45;

/// Reads `path` into memory as UTF-8, rejecting anything that is not a regular file.
///
/// Validating the encoding once up front lets every later stage work on `str`, report
/// invalid UTF-8 with its byte position, and keep I/O failures separate from parsing.
/// Files are small, so holding them whole is cheaper than a second pass over a seekable
/// reader. Files larger than [`MAX_FILE_SIZE`] are rejected.
pub(crate) fn read_file_to_string(path: &Utf8Path) -> Result<String, Message> {
    read_file_to_string_with_limit(path, MAX_FILE_SIZE)
}

/// Reads `path` as UTF-8, rejecting regular files larger than `limit` bytes.
///
/// The size is checked before reading so that oversized files fail without being
/// loaded, and again while reading in case the file grows in between.
fn read_file_to_string_with_limit(path: &Utf8Path, limit: u64) -> Result<String, Message> {
    let (file, length) = open_file(path)?;
    if length > limit {
        return Err(file_too_large_error(path, limit));
    }

    let mut bytes = Vec::with_capacity(usize::try_from(length).unwrap_or(0));
    file.take(limit.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|error| file_io_error(path, &error))?;
    if u64::try_from(bytes.len()).map_or(true, |read| read > limit) {
        return Err(file_too_large_error(path, limit));
    }

    String::from_utf8(bytes).map_err(|error| utf8_error(path, &error.utf8_error()))
}

/// Reports that `path` exceeds the `limit` bytes that may be read.
fn file_too_large_error(path: &Utf8Path, limit: u64) -> Message {
    file_io_error(
        path,
        &io::Error::new(
            ErrorKind::FileTooLarge,
            format!("file exceeds the {limit}-byte size limit"),
        ),
    )
}

/// Opens `path` for reading, rejecting anything that is not a regular file, and returns
/// it with its length in bytes.
///
/// I/O failures keep the OS diagnostic as extra detail via [`file_io_error`], matching
/// the writer. Metadata checks are not atomic with opening: concurrent replacement with
/// a special file may still block the open.
fn open_file(path: &Utf8Path) -> Result<(File, u64), Message> {
    let metadata = fs::metadata(path).map_err(|error| file_io_error(path, &error))?;
    if !metadata.is_file() {
        return Err(file_error(path, "files_io_not_a_file", []));
    }

    let file = File::open(path).map_err(|error| file_io_error(path, &error))?;

    // Validate the opened handle too, since the path may have changed.
    let metadata = file
        .metadata()
        .map_err(|error| file_io_error(path, &error))?;
    if !metadata.is_file() {
        return Err(file_error(path, "files_io_not_a_file", []));
    }

    Ok((file, metadata.len()))
}

/// Returns the JSON text of `source`, without a leading byte order mark.
///
/// RFC 8259 lets parsers ignore a byte order mark, and files saved by some editors
/// carry one, so it is not treated as an error.
fn json_text(source: &str) -> &str {
    source.strip_prefix(BYTE_ORDER_MARK).unwrap_or(source)
}

/// Reads the `file-version` entry of the top-level object.
///
/// The whole text is parsed so that malformed JSON is reported first, but only the
/// version is judged. The root entry and any other keys are left to [`read_document`],
/// so an unsupported version is reported before structural differences that a newer
/// version may have introduced.
///
/// # Errors
///
/// Returns a localized error if the text is not valid JSON, the top level is not an
/// object, or the `file-version` entry is missing, repeated, or not an integer within
/// `u32`.
pub(crate) fn read_file_version(source: &str, path: &Utf8Path, root: &str) -> Result<u32, Message> {
    let top_level = parse_top_level::<IgnoredAny>(source, root).map_err(|error| {
        // The lenient pass accepts any entry values, so the only data error left is a
        // top level that is not an object.
        if error.classify() == Category::Data {
            file_error(path, "files_expected_root", [("root", root.to_owned())])
        } else {
            json_error(path, error)
        }
    })?;
    if top_level.duplicate_version {
        return Err(file_error(path, "files_duplicate_file_version", []));
    }
    let version = top_level
        .version
        .ok_or_else(|| file_error(path, "files_missing_file_version", []))?;

    parse_file_version(&version).ok_or_else(|| {
        file_error(
            path,
            "files_invalid_file_version",
            [("version", version.to_string())],
        )
    })
}

/// Deserializes the root entry of `source` as an object of type `T` after the version
/// has been checked.
///
/// The full text is parsed again rather than a saved slice of the root entry, so that
/// positions in diagnostics refer to the file.
///
/// # Errors
///
/// Returns a localized error if the root entry is missing or repeated, a key other
/// than `file-version` and the root is present, or the root entry is not an object
/// matching `T`.
pub(crate) fn read_document<T: DeserializeOwned>(
    source: &str,
    path: &Utf8Path,
    root: &str,
) -> Result<T, Message> {
    let top_level =
        parse_top_level::<RootObject<T>>(source, root).map_err(|error| json_error(path, error))?;
    let document = top_level
        .root
        .ok_or_else(|| file_error(path, "files_expected_root", [("root", root.to_owned())]))?;
    if top_level.duplicate_root {
        return Err(file_error(
            path,
            "files_duplicate_root",
            [("root", root.to_owned())],
        ));
    }
    if let Some(key) = top_level.unknown_key {
        return Err(file_error(
            path,
            "files_unknown_top_level_key",
            [("key", key)],
        ));
    }

    Ok(document.document)
}

/// A root entry that must be an object even when `T` also accepts sequences.
struct RootObject<T> {
    /// The deserialized root entry.
    document: T,
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for RootObject<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(RootObjectVisitor {
            document: PhantomData,
        })
    }
}

/// Passes an object's map entries to `T` without buffering away parser positions.
struct RootObjectVisitor<T> {
    /// The type of the root entry.
    document: PhantomData<T>,
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for RootObjectVisitor<T> {
    type Value = RootObject<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object")
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        T::deserialize(MapAccessDeserializer::new(map)).map(|document| RootObject { document })
    }
}

/// Parses the top-level object of `source`, deserializing the entry named `root` as `T`.
///
/// Every value is parsed, so the whole text is checked for valid JSON syntax, including
/// anything after the closing brace.
fn parse_top_level<T: DeserializeOwned>(
    source: &str,
    root: &str,
) -> Result<TopLevel<T>, serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_str(json_text(source));
    let top_level = TopLevelSeed {
        root,
        document: PhantomData,
    }
    .deserialize(&mut deserializer)?;
    deserializer.end()?;
    Ok(top_level)
}

/// Converts a `file-version` value to `u32`, accepting only JSON integers in range.
///
/// Fractions, exponents, negatives, and strings are rejected even when they denote a
/// whole number, so a version always round-trips through the writer unchanged.
fn parse_file_version(version: &Value) -> Option<u32> {
    version
        .as_u64()
        .and_then(|version| u32::try_from(version).ok())
}

/// The entries of a file's top-level object, collected without judging them so that
/// callers decide which problem to report first.
struct TopLevel<T> {
    /// The first `file-version` value, if any.
    version: Option<Value>,
    /// Whether `file-version` appeared more than once.
    duplicate_version: bool,
    /// The first root entry, if any.
    root: Option<T>,
    /// Whether the root entry appeared more than once.
    duplicate_root: bool,
    /// The first key that is neither `file-version` nor the root.
    unknown_key: Option<String>,
}

/// Deserializes a [`TopLevel`] whose root entry is named at run time.
struct TopLevelSeed<'a, T> {
    /// The key of the root entry.
    root: &'a str,
    /// The type the root entry is deserialized as.
    document: PhantomData<T>,
}

impl<'de, T: Deserialize<'de>> DeserializeSeed<'de> for TopLevelSeed<'_, T> {
    type Value = TopLevel<T>;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_map(self)
    }
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for TopLevelSeed<'_, T> {
    type Value = TopLevel<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "an object with `{FILE_VERSION_KEY}` and `{}` entries",
            self.root
        )
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut top_level = TopLevel {
            version: None,
            duplicate_version: false,
            root: None,
            duplicate_root: false,
            unknown_key: None,
        };
        while let Some(key) = map.next_key::<String>()? {
            if key == FILE_VERSION_KEY {
                if top_level.version.is_some() {
                    top_level.duplicate_version = true;
                    map.next_value::<IgnoredAny>()?;
                } else {
                    top_level.version = Some(map.next_value()?);
                }
            } else if key == self.root {
                if top_level.root.is_some() {
                    top_level.duplicate_root = true;
                    map.next_value::<IgnoredAny>()?;
                } else {
                    top_level.root = Some(map.next_value()?);
                }
            } else {
                if top_level.unknown_key.is_none() {
                    top_level.unknown_key = Some(key);
                }
                map.next_value::<IgnoredAny>()?;
            }
        }

        Ok(top_level)
    }
}

/// Creates a localized file error that always identifies `path` alongside `params`.
pub(crate) fn file_error<const N: usize>(
    path: &Utf8Path,
    key: &str,
    params: [(&str, String); N],
) -> Message {
    let mut params: HashMap<_, _> = params
        .into_iter()
        .map(|(name, value)| (name.into(), value.into()))
        .collect();
    params.insert("path".into(), path.as_str().into());

    Message::new_with_params(
        MessageLevel::Error,
        MessageCategory::File,
        key.into(),
        params,
        None,
    )
}

/// The on-disk shape of every file: the current [`FILE_VERSION`] first, then the root
/// entry.
struct Document<'a, T> {
    /// The key of the root entry.
    root: &'a str,
    /// The root entry.
    body: &'a T,
}

impl<T: Serialize> Serialize for Document<'_, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry(FILE_VERSION_KEY, &FILE_VERSION)?;
        map.serialize_entry(self.root, self.body)?;
        map.end()
    }
}

/// Serializes `document` as the `root` entry of a pretty-printed UTF-8 JSON object
/// carrying the current [`FILE_VERSION`], then atomically replaces its destination
/// file.
///
/// The document is written to a uniquely named temporary file in the destination's
/// directory and renamed over `path`. Failures before the rename leave any existing
/// file untouched. A subsequent directory-sync failure explicitly reports that the
/// destination has been replaced but crash durability is uncertain.
/// Because the destination is replaced rather than rewritten, an existing file's
/// ownership and extended attributes are not preserved, and a symbolic link at `path`
/// is replaced by a regular file while its target is left unchanged. On Unix, the
/// permission bits of an existing regular file (or symbolic link target) are copied to
/// the replacement; set-user-ID, set-group-ID, and sticky bits are not.
/// An existing regular file (or symbolic link target) that is marked read-only is
/// never replaced; this is checked before writing, so it is not atomic with the rename.
/// On file systems that cannot sync at all (for example network, exFAT, and FUSE
/// volumes on macOS), syncing is skipped and durability is best-effort.
/// All errors report the destination path.
pub(crate) fn write_json_file(
    path: &Utf8Path,
    root: &str,
    document: &impl Serialize,
) -> Result<(), Message> {
    let mut json = serde_json::to_string_pretty(&Document {
        root,
        body: document,
    })
    .map_err(|error| json_serialization_error(path, error))?;
    json.push('\n');

    reject_read_only_destination(path).map_err(|error| file_io_error(path, &error))?;

    #[cfg(unix)]
    let parent_directory =
        open_parent_directory(path).map_err(|error| file_io_error(path, &error))?;

    let (temporary_path, file) = create_temporary_file(path)?;
    write_and_replace(file, &temporary_path, path, &json).map_err(|error| {
        // The destination is untouched, so only the temporary file needs cleaning up.
        drop(fs::remove_file(&temporary_path));
        file_io_error(path, &error)
    })?;

    #[cfg(unix)]
    sync_parent_directory(path, &parent_directory)?;

    Ok(())
}

/// Refuses to replace an existing regular file that is marked read-only.
///
/// Replacing a file only needs write access to its directory, so without this check a
/// read-only file would be silently overwritten on Unix, while Windows refuses. A
/// missing destination or one that is not a regular file is left for later steps.
fn reject_read_only_destination(path: &Utf8Path) -> io::Result<()> {
    match fs::metadata(path) {
        Ok(metadata) if metadata.is_file() && metadata.permissions().readonly() => Err(
            io::Error::new(ErrorKind::PermissionDenied, "destination file is read-only"),
        ),
        _ => Ok(()),
    }
}

/// Writes and flushes `json` to `file`, then renames the temporary file over `path`.
///
/// On Unix, the permission bits of an existing regular file at `path` are first copied
/// to `file`, since replacing a file resets its mode. A missing destination keeps the
/// default mode, and one that is not a regular file keeps the private mode chosen by
/// `temporary_file_mode`; any other problem with the destination is left for the
/// rename to report.
fn write_and_replace(
    mut file: File,
    temporary_path: &Utf8Path,
    path: &Utf8Path,
    json: &str,
) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        if let Ok(metadata) = fs::metadata(path)
            && metadata.is_file()
        {
            file.set_permissions(fs::Permissions::from_mode(
                metadata.permissions().mode() & 0o777,
            ))?;
        }
    }

    file.write_all(json.as_bytes())?;
    sync_best_effort(&file)?;
    drop(file);
    fs::rename(temporary_path, path)
}

/// Syncs `file`, treating file systems that cannot sync at all as best-effort.
///
/// On Apple targets [`File::sync_all`] issues `F_FULLFSYNC`, which SMB, NFS, exFAT, and
/// FUSE volumes reject outright. Saving there must still succeed, so only genuine sync
/// failures are reported.
fn sync_best_effort(file: &File) -> io::Result<()> {
    match file.sync_all() {
        Err(error) if is_sync_unsupported(&error) => Ok(()),
        result => result,
    }
}

/// Reports whether `error` means the file system does not support syncing, rather than
/// that a sync was attempted and failed.
fn is_sync_unsupported(error: &io::Error) -> bool {
    #[cfg(target_vendor = "apple")]
    if error.raw_os_error() == Some(ENOTSUP) {
        return true;
    }
    error.kind() == ErrorKind::Unsupported
}

/// Opens the parent directory before replacement, so access errors cannot occur
/// after the original file has already been overwritten.
///
/// Metadata checks are not atomic with opening: concurrent replacement with a
/// special file may still block the open.
#[cfg(unix)]
fn open_parent_directory(path: &Utf8Path) -> io::Result<File> {
    let parent = match path.parent() {
        Some(parent) if !parent.as_str().is_empty() => parent,
        _ => Utf8Path::new("."),
    };
    if !fs::metadata(parent)?.is_dir() {
        return Err(ErrorKind::NotADirectory.into());
    }
    let directory = File::open(parent)?;
    if !directory.metadata()?.is_dir() {
        return Err(ErrorKind::NotADirectory.into());
    }
    Ok(directory)
}

/// Flushes the committed rename, distinguishing durability errors from failures
/// that leave the original file untouched.
#[cfg(unix)]
fn sync_parent_directory(path: &Utf8Path, directory: &File) -> Result<(), Message> {
    sync_best_effort(directory).map_err(|error| {
        Message::new_with_params(
            MessageLevel::Error,
            MessageCategory::File,
            "files_io_replaced_sync_failed".into(),
            HashMap::from([("path".into(), path.as_str().into())]),
            Some(error.to_string().into()),
        )
    })
}

/// How many unique temporary names are tried before saving gives up.
const TEMPORARY_FILE_ATTEMPTS: u32 = 100;

/// Creates a new hidden temporary file beside `path`, retrying with fresh names up to
/// [`TEMPORARY_FILE_ATTEMPTS`] times.
fn create_temporary_file(path: &Utf8Path) -> Result<(Utf8PathBuf, File), Message> {
    static NEXT_ID: AtomicU64 = AtomicU64::new(0);

    create_temporary_file_with(path, || NEXT_ID.fetch_add(1, Ordering::Relaxed))
}

/// Creates a temporary file beside `path`, naming each attempt with `next_id`.
///
/// # Errors
///
/// Returns a localized error if creation fails for a reason other than a name
/// collision, or if every attempted name already exists.
fn create_temporary_file_with(
    path: &Utf8Path,
    mut next_id: impl FnMut() -> u64,
) -> Result<(Utf8PathBuf, File), Message> {
    let Some(file_name) = path.file_name() else {
        return Err(file_io_error(
            path,
            &io::Error::new(ErrorKind::InvalidInput, "path has no file name"),
        ));
    };
    // At most 128 UTF-8 bytes, leaving room for the unique suffix within NAME_MAX.
    let prefix: String = file_name.chars().take(32).collect();
    let process = std::process::id();

    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;

        options.mode(temporary_file_mode(path));
    }

    for _ in 0..TEMPORARY_FILE_ATTEMPTS {
        let id = next_id();
        let temporary_path = path.with_file_name(format!(".{prefix}.{process}-{id}.tmp"));
        match options.open(&temporary_path) {
            Ok(file) => return Ok((temporary_path, file)),
            Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
            Err(error) => return Err(file_io_error(path, &error)),
        }
    }

    // `files_io_already_exists` would wrongly suggest the destination is the problem.
    Err(diagnostic_message(
        path,
        "files_io_error",
        HashMap::new(),
        format!(
            "could not create a unique temporary file after {TEMPORARY_FILE_ATTEMPTS} attempts"
        ),
    ))
}

/// Chooses the mode the temporary file for `path` is atomically created with.
///
/// When replacing an existing file, the temporary file starts owner-only (`0600`) so
/// no other user can open it before [`write_and_replace`] copies the destination's
/// permission bits; `chmod` would not revoke a descriptor opened in that window. Only
/// a destination that is definitely missing gets the usual `0666` (less the umask),
/// matching any other newly created file.
#[cfg(unix)]
fn temporary_file_mode(path: &Utf8Path) -> u32 {
    match fs::metadata(path) {
        Err(error) if error.kind() == ErrorKind::NotFound => 0o666,
        _ => 0o600,
    }
}

/// Creates a localized file I/O error, retaining the OS diagnostic as extra detail.
///
/// Handles every stable error kind, with a fallback for the non-exhaustive enum.
pub(crate) fn file_io_error(path: &Utf8Path, error: &io::Error) -> Message {
    #[allow(
        clippy::wildcard_enum_match_arm,
        reason = "The remaining known variants are unstable and cannot be named on stable Rust."
    )]
    let key = match error.kind() {
        ErrorKind::NotFound => "files_io_not_found",
        ErrorKind::PermissionDenied => "files_io_permission_denied",
        ErrorKind::ConnectionRefused => "files_io_connection_refused",
        ErrorKind::ConnectionReset => "files_io_connection_reset",
        ErrorKind::HostUnreachable => "files_io_host_unreachable",
        ErrorKind::NetworkUnreachable => "files_io_network_unreachable",
        ErrorKind::ConnectionAborted => "files_io_connection_aborted",
        ErrorKind::NotConnected => "files_io_not_connected",
        ErrorKind::AddrInUse => "files_io_address_in_use",
        ErrorKind::AddrNotAvailable => "files_io_address_not_available",
        ErrorKind::NetworkDown => "files_io_network_down",
        ErrorKind::BrokenPipe => "files_io_broken_pipe",
        ErrorKind::AlreadyExists => "files_io_already_exists",
        ErrorKind::WouldBlock => "files_io_would_block",
        ErrorKind::NotADirectory => "files_io_not_a_directory",
        ErrorKind::IsADirectory => "files_io_not_a_file",
        ErrorKind::DirectoryNotEmpty => "files_io_directory_not_empty",
        ErrorKind::ReadOnlyFilesystem => "files_io_read_only_filesystem",
        ErrorKind::StaleNetworkFileHandle => "files_io_stale_network_file_handle",
        ErrorKind::InvalidInput => "files_io_invalid_input",
        ErrorKind::InvalidData => "files_io_invalid_data",
        ErrorKind::TimedOut => "files_io_timed_out",
        ErrorKind::WriteZero => "files_io_write_zero",
        ErrorKind::StorageFull => "files_io_storage_full",
        ErrorKind::NotSeekable => "files_io_not_seekable",
        ErrorKind::QuotaExceeded => "files_io_quota_exceeded",
        ErrorKind::FileTooLarge => "files_io_file_too_large",
        ErrorKind::ResourceBusy => "files_io_resource_busy",
        ErrorKind::ExecutableFileBusy => "files_io_executable_file_busy",
        ErrorKind::Deadlock => "files_io_deadlock",
        ErrorKind::CrossesDevices => "files_io_crosses_devices",
        ErrorKind::TooManyLinks => "files_io_too_many_links",
        ErrorKind::InvalidFilename => "files_io_invalid_filename",
        ErrorKind::ArgumentListTooLong => "files_io_argument_list_too_long",
        ErrorKind::Interrupted => "files_io_interrupted",
        ErrorKind::Unsupported => "files_io_unsupported",
        ErrorKind::UnexpectedEof => "files_io_unexpected_eof",
        ErrorKind::OutOfMemory => "files_io_out_of_memory",
        ErrorKind::Other => "files_io_other",
        _ => "files_io_error",
    };

    Message::new_with_params(
        MessageLevel::Error,
        MessageCategory::File,
        key.into(),
        HashMap::from([("path".into(), path.as_str().into())]),
        Some(error.to_string().into()),
    )
}

/// Maps a JSON parse error to a localized diagnostic by category, retaining the
/// parser's own message with its position as extra detail.
///
/// `serde_json` does not expose its individual error codes, so its categories are the
/// finest stable distinction available. The match is deliberately exhaustive without
/// a wildcard arm: a `serde_json` upgrade that adds a category must fail to compile
/// here until it has its own diagnostic key and translations.
pub(crate) fn json_error(path: &Utf8Path, error: serde_json::Error) -> Message {
    let key = match error.classify() {
        Category::Io => return file_io_error(path, &io::Error::from(error)),
        Category::Syntax => "files_json_syntax",
        Category::Eof => "files_json_unexpected_eof",
        Category::Data => "files_json_data",
    };
    let params = HashMap::from([
        ("line".into(), error.line().to_string().into()),
        ("column".into(), error.column().to_string().into()),
    ]);

    diagnostic_message(path, key, params, error.to_string())
}

/// Maps a JSON serialization error to a localized diagnostic.
///
/// Serializing to a string has no positions, so apart from I/O every failure comes
/// from a `Serialize` implementation and is retained as extra detail.
pub(crate) fn json_serialization_error(path: &Utf8Path, error: serde_json::Error) -> Message {
    if error.classify() == Category::Io {
        return file_io_error(path, &io::Error::from(error));
    }

    diagnostic_message(
        path,
        "files_json_serialization",
        HashMap::new(),
        error.to_string(),
    )
}

/// Maps invalid UTF-8 found while reading a file to a localized diagnostic naming the
/// byte position.
pub(crate) fn utf8_error(path: &Utf8Path, error: &Utf8Error) -> Message {
    let position: ShareableString = error.valid_up_to().to_string().into();
    let (key, params) = match error.error_len() {
        Some(length) => (
            "files_invalid_utf8",
            HashMap::from([
                ("position".into(), position),
                ("length".into(), length.to_string().into()),
            ]),
        ),
        None => (
            "files_incomplete_utf8",
            HashMap::from([("position".into(), position)]),
        ),
    };

    diagnostic_message(path, key, params, error.to_string())
}

/// Builds a localized diagnostic that identifies the file alongside `params`, keeping
/// the untranslated parser or OS text as extra detail.
fn diagnostic_message(
    path: &Utf8Path,
    key: &str,
    mut params: HashMap<ShareableString, ShareableString>,
    diagnostic: String,
) -> Message {
    params.insert("path".into(), path.as_str().into());

    Message::new_with_params(
        MessageLevel::Error,
        MessageCategory::File,
        key.into(),
        params,
        Some(diagnostic.into()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_shareable_string::{ShareableString, SharedStringStore};
    use hrafnix_translation::generate_translation_map;

    /// A root entry with one field, so that data errors inside the root can be provoked.
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Body {
        /// Any value.
        value: u32,
    }

    /// Returns the named parameter of `message` as text.
    fn param<'a>(message: &'a Message, name: &str) -> Option<&'a str> {
        message
            .translate_data()
            .message_params()
            .get(name)
            .map(ShareableString::as_str)
    }

    /// Checks that `message` translates in every language, substituting every
    /// parameter and differing from the English text.
    fn assert_translates_everywhere(message: &Message, english: &str) {
        let map = generate_translation_map(&SharedStringStore::new());
        let key = message.translate_data().message_key().as_str();
        assert_eq!(
            message
                .translated_message(&map, "en")
                .as_ref()
                .map(ShareableString::as_str),
            Some(english),
            "{key}"
        );
        for language in ["en", "zh", "de", "es", "fr", "ja", "ko"] {
            let translated = message.translated_message(&map, language).unwrap();
            assert!(!translated.as_str().contains("%{"), "{key}: {language}");
            for value in message.translate_data().message_params().values() {
                assert!(
                    translated.as_str().contains(value.as_str()),
                    "{key}: {language}: {value}"
                );
            }
            if language != "en" {
                assert_ne!(translated.as_str(), english, "{key}: {language}");
            }
        }
    }

    #[test]
    fn version_is_read_without_judging_the_rest_of_the_document() {
        let path = Utf8Path::new("model.hxm");
        for json in [
            "{\"file-version\": 1, \"model\": {}}",
            "{\"file-version\":1}",
            " \t\r\n{ \"file-version\" : 1 }\r\n\t ",
            "{\"model\": {}, \"file-version\": 1}",
            "{\"file-version\": 1, \"model\": {\"unknown\": [1, {\"a\": null}]}, \"extra\": 2}",
            "{\"file-version\": 1, \"component\": {}}",
            "{\"file-version\": 1, \"model\": {}, \"model\": {}}",
            "\u{feff}{\"file-version\": 1, \"model\": {}}",
        ] {
            assert_eq!(read_file_version(json, path, "model").unwrap(), 1, "{json}");
        }
        assert_eq!(
            read_file_version("{\"file-version\": 4294967295}", path, "model").unwrap(),
            u32::MAX
        );
    }

    #[test]
    fn top_level_must_be_an_object() {
        let path = Utf8Path::new("model.hxm");
        for json in [
            "[]",
            "[{\"file-version\": 1, \"model\": {}}]",
            "\"text\"",
            "1",
            "true",
            "null",
            "\u{feff}[]",
        ] {
            let error = read_file_version(json, path, "model").unwrap_err();
            assert_eq!(
                error.translate_data().message_key().as_str(),
                "files_expected_root",
                "{json}"
            );
            assert_eq!(param(&error, "root"), Some("model"), "{json}");
            assert_eq!(param(&error, "path"), Some("model.hxm"), "{json}");
            assert!(error.extra_detail().is_none(), "{json}");
        }
    }

    #[test]
    fn malformed_json_reports_its_category_and_position() {
        let path = Utf8Path::new("model.hxm");
        for (json, key, line, column) in [
            ("", "files_json_unexpected_eof", "1", "0"),
            (" \n", "files_json_unexpected_eof", "2", "0"),
            ("{", "files_json_unexpected_eof", "1", "1"),
            (
                "{\"file-version\": 1",
                "files_json_unexpected_eof",
                "1",
                "18",
            ),
            (
                "{\"file-version\": 1,\n\"model\": {",
                "files_json_unexpected_eof",
                "2",
                "10",
            ),
            ("not JSON", "files_json_syntax", "1", "2"),
            ("{\"file-version\": 1,}", "files_json_syntax", "1", "20"),
            ("{\"file-version\": 01}", "files_json_syntax", "1", "19"),
            ("{\"file-version\": +1}", "files_json_syntax", "1", "18"),
            ("{'file-version': 1}", "files_json_syntax", "1", "2"),
            (
                "{\"file-version\": 1} extra",
                "files_json_syntax",
                "1",
                "21",
            ),
            ("{\"file-version\": 1}\n{}", "files_json_syntax", "2", "1"),
            ("\u{a0}{\"file-version\": 1}", "files_json_syntax", "1", "1"),
            (
                "{\"file-version\": 1}\u{a0}",
                "files_json_syntax",
                "1",
                "20",
            ),
            (
                "\u{feff}\u{feff}{\"file-version\": 1}",
                "files_json_syntax",
                "1",
                "1",
            ),
        ] {
            let error = read_file_version(json, path, "model").unwrap_err();
            assert_eq!(error.translate_data().message_key().as_str(), key, "{json}");
            assert_eq!(param(&error, "path"), Some("model.hxm"), "{json}");
            assert_eq!(param(&error, "line"), Some(line), "{json}");
            assert_eq!(param(&error, "column"), Some(column), "{json}");
            assert!(
                error.extra_detail().is_some_and(|detail| detail
                    .as_str()
                    .ends_with(&format!("at line {line} column {column}"))),
                "{json}: {:?}",
                error.extra_detail()
            );
        }
    }

    #[test]
    fn file_version_entry_is_required_unique_and_an_integer() {
        let path = Utf8Path::new("model.hxm");
        for (json, key, version) in [
            ("{}", "files_missing_file_version", None),
            ("{\"model\": {}}", "files_missing_file_version", None),
            ("{\"File-Version\": 1}", "files_missing_file_version", None),
            ("{\"file_version\": 1}", "files_missing_file_version", None),
            (
                "{\"model\": {\"file-version\": 1}}",
                "files_missing_file_version",
                None,
            ),
            (
                "{\"file-version\": 1, \"file-version\": 1}",
                "files_duplicate_file_version",
                None,
            ),
            (
                "{\"file-version\": \"x\", \"model\": {}, \"file-version\": 2}",
                "files_duplicate_file_version",
                None,
            ),
            (
                "{\"file-version\": \"1\"}",
                "files_invalid_file_version",
                Some("\"1\""),
            ),
            (
                "{\"file-version\": \"\"}",
                "files_invalid_file_version",
                Some("\"\""),
            ),
            (
                "{\"file-version\": 1.0}",
                "files_invalid_file_version",
                Some("1.0"),
            ),
            (
                "{\"file-version\": 1e0}",
                "files_invalid_file_version",
                Some("1.0"),
            ),
            (
                "{\"file-version\": -1}",
                "files_invalid_file_version",
                Some("-1"),
            ),
            (
                "{\"file-version\": -0}",
                "files_invalid_file_version",
                Some("-0.0"),
            ),
            (
                "{\"file-version\": null}",
                "files_invalid_file_version",
                Some("null"),
            ),
            (
                "{\"file-version\": true}",
                "files_invalid_file_version",
                Some("true"),
            ),
            (
                "{\"file-version\": [1]}",
                "files_invalid_file_version",
                Some("[1]"),
            ),
            (
                "{\"file-version\": {}}",
                "files_invalid_file_version",
                Some("{}"),
            ),
            (
                "{\"file-version\": 4294967296}",
                "files_invalid_file_version",
                Some("4294967296"),
            ),
            (
                "{\"file-version\": 18446744073709551616}",
                "files_invalid_file_version",
                Some("1.8446744073709552e+19"),
            ),
        ] {
            let error = read_file_version(json, path, "model").unwrap_err();
            assert_eq!(error.translate_data().message_key().as_str(), key, "{json}");
            assert_eq!(param(&error, "path"), Some("model.hxm"), "{json}");
            assert_eq!(param(&error, "version"), version, "{json}");
        }
    }

    #[test]
    fn documents_require_exactly_the_root_entry() {
        let path = Utf8Path::new("model.hxm");
        assert_eq!(
            read_document::<Body>(
                "{\"file-version\": 1, \"model\": {\"value\": 7}}",
                path,
                "model"
            )
            .unwrap(),
            Body { value: 7 }
        );
        assert_eq!(
            read_document::<Body>(
                "\u{feff}{\"model\": {\"value\": 7}, \"file-version\": 1}",
                path,
                "model"
            )
            .unwrap(),
            Body { value: 7 }
        );

        for (json, key, parameter) in [
            (
                "{\"file-version\": 1}",
                "files_expected_root",
                ("root", "model"),
            ),
            (
                "{\"file-version\": 1, \"component\": {\"value\": 7}}",
                "files_expected_root",
                ("root", "model"),
            ),
            (
                "{\"file-version\": 1, \"Model\": {\"value\": 7}}",
                "files_expected_root",
                ("root", "model"),
            ),
            (
                "{\"file-version\": 1, \"model\": {\"value\": 7}, \"model\": {\"value\": 7}}",
                "files_duplicate_root",
                ("root", "model"),
            ),
            (
                "{\"file-version\": 1, \"model\": {\"value\": 7}, \"extra\": null}",
                "files_unknown_top_level_key",
                ("key", "extra"),
            ),
            (
                "{\"\": 1, \"file-version\": 1, \"model\": {\"value\": 7}}",
                "files_unknown_top_level_key",
                ("key", ""),
            ),
        ] {
            let error = read_document::<Body>(json, path, "model").unwrap_err();
            assert_eq!(error.translate_data().message_key().as_str(), key, "{json}");
            assert_eq!(param(&error, "path"), Some("model.hxm"), "{json}");
            assert_eq!(param(&error, parameter.0), Some(parameter.1), "{json}");
            assert!(error.extra_detail().is_none(), "{json}");
        }
    }

    #[test]
    fn root_entries_that_do_not_match_report_data_errors_with_positions() {
        let path = Utf8Path::new("model.hxm");
        for (json, column) in [
            ("{\"file-version\": 1, \"model\": {}}", "31"),
            ("{\"file-version\": 1, \"model\": {\"value\": \"7\"}}", "42"),
            ("{\"file-version\": 1, \"model\": {\"value\": -7}}", "41"),
            (
                "{\"file-version\": 1, \"model\": {\"value\": 7, \"other\": 1}}",
                "49",
            ),
            ("{\"file-version\": 1, \"model\": []}", "29"),
            ("{\"file-version\": 1, \"model\": [7]}", "29"),
            ("{\"file-version\": 1, \"model\": null}", "33"),
        ] {
            let error = read_document::<Body>(json, path, "model").unwrap_err();
            assert_eq!(
                error.translate_data().message_key().as_str(),
                "files_json_data",
                "{json}"
            );
            assert_eq!(param(&error, "line"), Some("1"), "{json}");
            assert_eq!(param(&error, "column"), Some(column), "{json}");
            assert!(error.extra_detail().is_some(), "{json}");
        }
    }

    #[test]
    fn documents_are_written_with_the_version_first() {
        #[derive(Serialize)]
        struct Component {
            name: &'static str,
        }

        assert_eq!(
            serde_json::to_string_pretty(&Document {
                root: "component",
                body: &Component { name: "pump" },
            })
            .unwrap(),
            "{\n  \"file-version\": 1,\n  \"component\": {\n    \"name\": \"pump\"\n  }\n}"
        );
    }

    #[test]
    fn every_json_error_category_has_a_localized_diagnostic() {
        let path = Utf8Path::new("component.hxc");
        let cases = [
            (
                serde_json::from_str::<Value>("}").unwrap_err(),
                "files_json_syntax",
                "'component.hxc': Invalid JSON syntax at line 1, column 1",
            ),
            (
                serde_json::from_str::<Value>("{\n  \"a\": [1,").unwrap_err(),
                "files_json_unexpected_eof",
                "'component.hxc': Unexpected end of JSON data at line 2, column 10",
            ),
            (
                serde_json::from_str::<u32>("\"text\"").unwrap_err(),
                "files_json_data",
                "'component.hxc': JSON data does not match the expected structure at line 1, column 6",
            ),
        ];
        let mut keys = std::collections::HashSet::new();

        for (error, key, english) in cases {
            let diagnostic = error.to_string();
            let message = json_error(path, error);

            assert!(keys.insert(key), "duplicate JSON diagnostic: {key}");
            assert_eq!(message.level(), MessageLevel::Error);
            assert_eq!(message.category(), MessageCategory::File);
            assert_eq!(message.translate_data().message_key().as_str(), key);
            assert_eq!(message.translate_data().message_params().len(), 3);
            assert_eq!(param(&message, "path"), Some(path.as_str()));
            assert_eq!(
                message.extra_detail().map(ShareableString::as_str),
                Some(diagnostic.as_str())
            );
            assert_translates_everywhere(&message, english);
        }
    }

    #[test]
    fn json_io_errors_preserve_file_io_diagnostics() {
        let path = Utf8Path::new("component.hxc");
        let error = io::Error::new(ErrorKind::PermissionDenied, "read denied");
        let expected = file_io_error(path, &error);

        assert_eq!(json_error(path, serde_json::Error::io(error)), expected);
    }

    #[test]
    fn serialization_errors_have_a_localized_diagnostic() {
        /// Fails to serialize, like a map with keys that are not strings would.
        struct Failing;

        impl Serialize for Failing {
            fn serialize<S: Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
                Err(serde::ser::Error::custom("unrepresentable value"))
            }
        }

        let path = Utf8Path::new("model.hxm");
        let error = serde_json::to_string(&Failing).unwrap_err();
        let diagnostic = error.to_string();
        let message = json_serialization_error(path, error);

        assert_eq!(message.level(), MessageLevel::Error);
        assert_eq!(message.category(), MessageCategory::File);
        assert_eq!(
            message.translate_data().message_key().as_str(),
            "files_json_serialization"
        );
        assert_eq!(message.translate_data().message_params().len(), 1);
        assert_eq!(param(&message, "path"), Some(path.as_str()));
        assert_eq!(
            message.extra_detail().map(ShareableString::as_str),
            Some(diagnostic.as_str())
        );
        assert_eq!(diagnostic, "unrepresentable value");
        assert_translates_everywhere(
            &message,
            "'model.hxm': Data could not be serialized as JSON",
        );
    }

    #[test]
    fn serialization_preserves_io_errors() {
        let path = Utf8Path::new("model.hxm");
        let error = io::Error::new(ErrorKind::PermissionDenied, "write denied");
        let expected = file_io_error(path, &error);

        assert_eq!(
            json_serialization_error(path, serde_json::Error::io(error)),
            expected
        );
    }

    #[test]
    fn invalid_utf8_reports_its_byte_position() {
        let path = Utf8Path::new("model.hxm");
        let invalid = String::from_utf8(vec![b'a', b'b', 0xF0, 0x90, 0x80, b'z'])
            .unwrap_err()
            .utf8_error();
        let incomplete = String::from_utf8(vec![b'a', b'b', 0xF0, 0x90])
            .unwrap_err()
            .utf8_error();
        let cases = [
            (
                invalid,
                "files_invalid_utf8",
                "'model.hxm': Invalid UTF-8 at byte 2 (invalid sequence length: 3)",
                HashMap::from([("position", "2"), ("length", "3")]),
            ),
            (
                incomplete,
                "files_incomplete_utf8",
                "'model.hxm': Incomplete UTF-8 sequence at byte 2",
                HashMap::from([("position", "2")]),
            ),
        ];

        for (error, key, english, parameters) in cases {
            let message = utf8_error(path, &error);

            assert_eq!(message.level(), MessageLevel::Error);
            assert_eq!(message.category(), MessageCategory::File);
            assert_eq!(message.translate_data().message_key().as_str(), key);
            assert_eq!(
                message.translate_data().message_params().len(),
                parameters.len().checked_add(1).unwrap()
            );
            assert_eq!(param(&message, "path"), Some(path.as_str()));
            for (name, value) in parameters {
                assert_eq!(param(&message, name), Some(value), "{key}: {name}");
            }
            assert_eq!(
                message.extra_detail().map(ShareableString::as_str),
                Some(error.to_string().as_str())
            );
            assert_translates_everywhere(&message, english);
        }
    }

    #[cfg(unix)]
    #[test]
    fn sync_failures_explicitly_report_that_replacement_has_committed() {
        let path = Utf8Path::new("model.hxm");
        // A device cannot be fsynced, providing a deterministic sync failure.
        let error = sync_parent_directory(path, &File::open("/dev/null").unwrap()).unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_io_replaced_sync_failed"
        );
        assert!(error.extra_detail().is_some());
        let map = generate_translation_map(&SharedStringStore::new());
        assert_eq!(
            error.translated_message(&map, "en").unwrap().as_str(),
            "File 'model.hxm' has been replaced, but syncing its parent directory failed; persistence across a crash is uncertain"
        );
        for language in ["en", "zh", "de", "es", "fr", "ja", "ko"] {
            let translated = error.translated_message(&map, language).unwrap();
            assert!(translated.as_str().contains(path.as_str()));
            assert!(!translated.as_str().contains("%{"));
        }
    }

    #[test]
    fn unsupported_sync_is_best_effort_but_other_sync_failures_are_not() {
        assert!(is_sync_unsupported(&io::Error::from(
            ErrorKind::Unsupported
        )));
        #[cfg(target_vendor = "apple")]
        assert!(is_sync_unsupported(&io::Error::from_raw_os_error(ENOTSUP)));

        for kind in [
            ErrorKind::PermissionDenied,
            ErrorKind::InvalidInput,
            ErrorKind::Other,
        ] {
            assert!(!is_sync_unsupported(&io::Error::from(kind)), "{kind:?}");
        }
        #[cfg(unix)]
        {
            // The deterministic device failure used above must stay fatal.
            let error = File::open("/dev/null").unwrap().sync_all().unwrap_err();
            assert!(!is_sync_unsupported(&error), "{error}");
        }
    }

    #[test]
    fn file_io_errors_use_localized_messages_and_separate_os_details() {
        let map = generate_translation_map(&SharedStringStore::new());
        let path = Utf8Path::new("components/example.hxc");
        let cases: [(ErrorKind, &str); 39] = [
            (ErrorKind::NotFound, "files_io_not_found"),
            (ErrorKind::PermissionDenied, "files_io_permission_denied"),
            (ErrorKind::ConnectionRefused, "files_io_connection_refused"),
            (ErrorKind::ConnectionReset, "files_io_connection_reset"),
            (ErrorKind::HostUnreachable, "files_io_host_unreachable"),
            (
                ErrorKind::NetworkUnreachable,
                "files_io_network_unreachable",
            ),
            (ErrorKind::ConnectionAborted, "files_io_connection_aborted"),
            (ErrorKind::NotConnected, "files_io_not_connected"),
            (ErrorKind::AddrInUse, "files_io_address_in_use"),
            (
                ErrorKind::AddrNotAvailable,
                "files_io_address_not_available",
            ),
            (ErrorKind::NetworkDown, "files_io_network_down"),
            (ErrorKind::BrokenPipe, "files_io_broken_pipe"),
            (ErrorKind::AlreadyExists, "files_io_already_exists"),
            (ErrorKind::WouldBlock, "files_io_would_block"),
            (ErrorKind::NotADirectory, "files_io_not_a_directory"),
            (ErrorKind::IsADirectory, "files_io_not_a_file"),
            (ErrorKind::DirectoryNotEmpty, "files_io_directory_not_empty"),
            (
                ErrorKind::ReadOnlyFilesystem,
                "files_io_read_only_filesystem",
            ),
            (
                ErrorKind::StaleNetworkFileHandle,
                "files_io_stale_network_file_handle",
            ),
            (ErrorKind::InvalidInput, "files_io_invalid_input"),
            (ErrorKind::InvalidData, "files_io_invalid_data"),
            (ErrorKind::TimedOut, "files_io_timed_out"),
            (ErrorKind::WriteZero, "files_io_write_zero"),
            (ErrorKind::StorageFull, "files_io_storage_full"),
            (ErrorKind::NotSeekable, "files_io_not_seekable"),
            (ErrorKind::QuotaExceeded, "files_io_quota_exceeded"),
            (ErrorKind::FileTooLarge, "files_io_file_too_large"),
            (ErrorKind::ResourceBusy, "files_io_resource_busy"),
            (
                ErrorKind::ExecutableFileBusy,
                "files_io_executable_file_busy",
            ),
            (ErrorKind::Deadlock, "files_io_deadlock"),
            (ErrorKind::CrossesDevices, "files_io_crosses_devices"),
            (ErrorKind::TooManyLinks, "files_io_too_many_links"),
            (ErrorKind::InvalidFilename, "files_io_invalid_filename"),
            (
                ErrorKind::ArgumentListTooLong,
                "files_io_argument_list_too_long",
            ),
            (ErrorKind::Interrupted, "files_io_interrupted"),
            (ErrorKind::Unsupported, "files_io_unsupported"),
            (ErrorKind::UnexpectedEof, "files_io_unexpected_eof"),
            (ErrorKind::OutOfMemory, "files_io_out_of_memory"),
            (ErrorKind::Other, "files_io_other"),
        ];

        for (kind, key) in cases {
            let error = io::Error::new(kind, "untranslated OS diagnostic");
            let message = file_io_error(path, &error);

            assert_eq!(message.level(), MessageLevel::Error);
            assert_eq!(message.category(), MessageCategory::File);
            assert_eq!(message.translate_data().message_key().as_str(), key);
            assert_eq!(message.translate_data().message_params().len(), 1);
            assert_eq!(
                message
                    .translate_data()
                    .message_params()
                    .get("path")
                    .map(ShareableString::as_str),
                Some(path.as_str())
            );
            assert_eq!(
                message.extra_detail().map(ShareableString::as_str),
                Some("untranslated OS diagnostic")
            );
            let english = message.translated_message(&map, "en").unwrap();
            for language in ["en", "zh", "de", "es", "fr", "ja", "ko"] {
                let translated = message.translated_message(&map, language).unwrap();
                assert!(translated.as_str().contains(path.as_str()));
                assert!(!translated.as_str().contains("untranslated OS diagnostic"));
                assert!(!translated.as_str().contains("%{"));
                if language != "en" {
                    assert_ne!(translated, english, "{key}: {language}");
                }
            }
        }
    }

    #[test]
    fn unrecognized_os_error_uses_localized_fallback() {
        let error = io::Error::from_raw_os_error(i32::MAX);
        let message = file_io_error(Utf8Path::new("example.hxc"), &error);
        let map = generate_translation_map(&SharedStringStore::new());

        assert_eq!(
            message.translate_data().message_key().as_str(),
            "files_io_error"
        );
        assert_eq!(
            message.extra_detail().map(ShareableString::as_str),
            Some(error.to_string().as_str())
        );
        assert_eq!(
            message
                .translated_message(&map, "en")
                .as_ref()
                .map(ShareableString::as_str),
            Some("File I/O error for 'example.hxc'")
        );
    }

    /// Creates an empty, uniquely named directory under the system temporary directory.
    fn temporary_directory(name: &str) -> Utf8PathBuf {
        let directory = Utf8PathBuf::try_from(std::env::temp_dir())
            .unwrap()
            .join(format!("hrafnix-files-{name}-{}", std::process::id()));
        drop(fs::remove_dir_all(&directory));
        fs::create_dir(&directory).unwrap();
        directory
    }

    #[test]
    fn temporary_file_creation_gives_up_after_repeated_collisions() {
        let directory = temporary_directory("collisions");
        let path = directory.join("model.hxm");
        let colliding = path.with_file_name(format!(".model.hxm.{}-0.tmp", std::process::id()));
        fs::write(&colliding, "").unwrap();
        let mut attempts = 0_u32;

        let error = create_temporary_file_with(&path, || {
            attempts = attempts.saturating_add(1);
            0
        })
        .unwrap_err();
        let entries = fs::read_dir(&directory).unwrap().count();
        drop(fs::remove_dir_all(&directory));

        assert_eq!(attempts, TEMPORARY_FILE_ATTEMPTS);
        assert_eq!(entries, 1);
        assert_eq!(param(&error, "path"), Some(path.as_str()));
        assert!(
            error
                .extra_detail()
                .is_some_and(|detail| detail.as_str().contains("unique temporary file"))
        );
        assert_translates_everywhere(&error, &format!("File I/O error for '{path}'"));
    }

    #[cfg(unix)]
    #[test]
    fn replacing_a_file_preserves_its_permission_bits() {
        use std::os::unix::fs::PermissionsExt;

        let directory = temporary_directory("permissions");
        let path = directory.join("model.hxm");
        let mut modes = Vec::new();
        for mode in [0o600, 0o640, 0o604] {
            fs::write(&path, "").unwrap();
            fs::set_permissions(&path, fs::Permissions::from_mode(mode)).unwrap();
            let result = write_json_file(&path, "model", &HashMap::<String, u32>::new());
            modes.push((
                mode,
                result.is_ok(),
                fs::metadata(&path).unwrap().permissions().mode() & 0o7777,
            ));
            fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
        }
        drop(fs::remove_dir_all(&directory));

        for (mode, saved, actual) in modes {
            assert!(saved, "{mode:o}");
            assert_eq!(actual, mode, "{mode:o}");
        }
    }

    #[cfg(unix)]
    #[test]
    fn temporary_file_is_private_until_permissions_are_copied() {
        use std::os::unix::fs::PermissionsExt;

        let directory = temporary_directory("temporary-mode");
        let path = directory.join("model.hxm");
        let mode_of = |file: &File| file.metadata().unwrap().permissions().mode() & 0o7777;

        let (_, missing) = create_temporary_file(&path).unwrap();
        let default_mode = mode_of(&File::create(directory.join("default")).unwrap());
        fs::write(&path, "").unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
        let (_, replacing) = create_temporary_file(&path).unwrap();
        let modes = (mode_of(&missing), mode_of(&replacing));
        drop(fs::remove_dir_all(&directory));

        assert_eq!(modes, (default_mode, 0o600));
    }

    #[test]
    fn files_up_to_the_size_limit_are_read() {
        let directory = temporary_directory("size-limit");
        let path = directory.join("model.hxm");
        let mut results = Vec::new();
        for (contents, limit) in [("", 0), ("{}", 2), ("{}", 3), ("{}", 1), ("{\"a\": 1}", 0)] {
            fs::write(&path, contents).unwrap();
            results.push((
                contents,
                limit,
                read_file_to_string_with_limit(&path, limit),
            ));
        }
        drop(fs::remove_dir_all(&directory));

        for (contents, limit, result) in results {
            if u64::try_from(contents.len()).unwrap() <= limit {
                assert_eq!(result.unwrap(), contents, "{contents}: {limit}");
            } else {
                let error = result.unwrap_err();
                assert_eq!(
                    error.translate_data().message_key().as_str(),
                    "files_io_file_too_large",
                    "{contents}: {limit}"
                );
                assert_eq!(param(&error, "path"), Some(path.as_str()));
                assert!(
                    error
                        .extra_detail()
                        .is_some_and(|detail| detail.as_str().contains(&format!("{limit}-byte"))),
                    "{contents}: {limit}"
                );
            }
        }
    }
}
