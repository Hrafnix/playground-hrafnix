//! Shared support for file integration tests.

use camino::{Utf8Path, Utf8PathBuf};
use hrafnix_files::common::MAX_FILE_SIZE;
use hrafnix_files::metadata::data::Metadata;
use hrafnix_message::message::Message;
use std::fs;
use std::io::ErrorKind;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Valid version 1 metadata entry.
pub(super) const METADATA: &str = "{\"display-name\": \"\", \"description\": \"\", \"author\": \"\", \"minimum-version-major\": 0, \"minimum-version-minor\": 0}";

/// Valid version 1 contents of a model or component root entry.
pub(super) const CONTENTS: &str = "{\"metadata\": {\"display-name\": \"\", \"description\": \"\", \"author\": \"\", \"minimum-version-major\": 0, \"minimum-version-minor\": 0}}";

/// Returns the metadata stored in the `minimal` test data files for `kind`.
pub(super) fn metadata(kind: &str) -> Metadata {
    Metadata {
        display_name: "Example".to_owned(),
        description: format!("An example {kind}."),
        author: "Hrafnix".to_owned(),
        minimum_version_major: 1,
        minimum_version_minor: 2,
    }
}

/// An isolated directory removed when its test finishes.
#[derive(Debug)]
pub(super) struct TestDirectory {
    /// The directory owned by this test.
    pub(super) path: Utf8PathBuf,
}

impl TestDirectory {
    /// Creates a unique directory without replacing any existing path.
    pub(super) fn new() -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

        loop {
            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            let path = Utf8PathBuf::try_from(std::env::temp_dir())
                .unwrap()
                .join(format!("hrafnix-files-test-{}-{id}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Self { path },
                Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
                Err(error) => panic!("Failed to create test directory '{path}': {error}"),
            }
        }
    }

    /// Returns the sorted names of every entry in this directory.
    pub(super) fn entries(&self) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(&self.path)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }

    /// Returns the names of leftover temporary files written beside this directory.
    pub(super) fn sibling_temporary_files(&self) -> Vec<String> {
        let prefix = format!(".{}.", self.path.file_name().unwrap());
        let mut names: Vec<String> = fs::read_dir(self.path.parent().unwrap())
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|name| name.starts_with(&prefix))
            .collect();
        names.sort();
        names
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        // Never panic during cleanup: a failing test is already unwinding.
        drop(fs::remove_dir_all(&self.path));
    }
}

/// Checks creation and replacement at the common 255-byte filename limit.
#[cfg(unix)]
pub(super) fn assert_long_filenames(
    extension: &str,
    write: fn(&Utf8Path) -> Result<(), Message>,
    read: fn(&Utf8Path) -> Result<(), Message>,
) {
    for stem in ["a".repeat(251), format!("{}ab", "界".repeat(83))] {
        let directory = TestDirectory::new();
        let name = format!("{stem}.{extension}");
        assert_eq!(name.len(), 255);
        let path = directory.path.join(&name);

        write(&path).unwrap();
        read(&path).unwrap();
        fs::write(&path, "original contents").unwrap();
        write(&path).unwrap();
        read(&path).unwrap();
        assert_eq!(directory.entries(), [name]);
    }
}

/// Checks that root entries must be objects rather than arrays or scalar values.
pub(super) fn assert_object_root(root: &str, read: fn(&Utf8Path) -> Result<(), Message>) {
    let directory = TestDirectory::new();
    let path = directory.path.join("root.json");
    fs::write(
        &path,
        format!("{{\"file-version\":1,\"{root}\":{CONTENTS}}}"),
    )
    .unwrap();
    read(&path).unwrap();

    for body in ["[]", "[{}]", "[1]", "[[]]", "null", "true", "1", "\"text\""] {
        let document = format!("{{\"file-version\":1,\"{root}\":{body}}}");
        fs::write(&path, &document).unwrap();
        let error = read(&path).unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_json_data",
            "{document}"
        );
        assert_eq!(
            error
                .translate_data()
                .message_params()
                .get("path")
                .map(hrafnix_shareable_string::ShareableString::as_str),
            Some(path.as_str()),
            "{document}"
        );
        assert!(error.extra_detail().is_some(), "{document}");
    }
}

/// Checks rejection of anything after the top-level object while allowing JSON whitespace.
pub(super) fn assert_json_trailing_content(root: &str, read: fn(&Utf8Path) -> Result<(), Message>) {
    let directory = TestDirectory::new();
    let path = directory.path.join("trailing.json");
    for document in [
        format!("{{\"file-version\": 1, \"{root}\": {CONTENTS}}}"),
        format!("{{\n  \"file-version\": 1,\n  \"{root}\": {CONTENTS}\n}}"),
    ] {
        for suffix in ["", " \t\r\n", "\n\n"] {
            fs::write(&path, format!("{document}{suffix}")).unwrap();
            read(&path).unwrap();
        }
        for (suffix, key) in [
            ("{}", "files_json_syntax"),
            ("garbage", "files_json_syntax"),
            (",", "files_json_syntax"),
            ("}", "files_json_syntax"),
            ("null", "files_json_syntax"),
            ("// comment", "files_json_syntax"),
            ("\u{a0}", "files_json_syntax"),
            ("\u{feff}", "files_json_syntax"),
            ("\"", "files_json_syntax"),
            ("{\"file-version\": 1}", "files_json_syntax"),
        ] {
            fs::write(&path, format!("{document} \n{suffix}")).unwrap();
            let error = read(&path).unwrap_err();
            assert_eq!(
                error.translate_data().message_key().as_str(),
                key,
                "{suffix}"
            );
            assert_eq!(
                error
                    .translate_data()
                    .message_params()
                    .get("path")
                    .unwrap()
                    .as_str(),
                path.as_str()
            );
            assert_eq!(
                error
                    .translate_data()
                    .message_params()
                    .get("line")
                    .unwrap()
                    .as_str(),
                document.lines().count().checked_add(1).unwrap().to_string(),
                "{suffix}"
            );
        }
    }
}

/// Checks that a UTF-8 BOM is accepted while invalid UTF-8 is rejected with its byte
/// position before any JSON is parsed.
pub(super) fn assert_utf8_handling(root: &str, read: fn(&Utf8Path) -> Result<(), Message>) {
    let directory = TestDirectory::new();
    let path = directory.path.join("encoding.json");
    let document = format!("{{\"file-version\": 1, \"{root}\": {CONTENTS}}}");

    fs::write(&path, format!("\u{feff}{document}")).unwrap();
    read(&path).unwrap();

    let invalid = [
        b"{\"\xF0\x90\x80\": 1,".as_slice(),
        document.strip_prefix('{').unwrap().as_bytes(),
    ]
    .concat();
    let truncated = [document.as_bytes(), b"\n\xE2\x82".as_slice()].concat();
    let truncated_at = document.len().checked_add(1).unwrap().to_string();
    for (bytes, key, params) in [
        (
            invalid,
            "files_invalid_utf8",
            vec![("position", "2".to_owned()), ("length", "3".to_owned())],
        ),
        (
            truncated,
            "files_incomplete_utf8",
            vec![("position", truncated_at)],
        ),
    ] {
        fs::write(&path, bytes).unwrap();
        let error = read(&path).unwrap_err();
        assert_eq!(error.translate_data().message_key().as_str(), key);
        assert_eq!(
            error
                .translate_data()
                .message_params()
                .get("path")
                .unwrap()
                .as_str(),
            path.as_str()
        );
        for (name, value) in params {
            assert_eq!(
                error
                    .translate_data()
                    .message_params()
                    .get(name)
                    .unwrap()
                    .as_str(),
                value,
                "{key}: {name}"
            );
        }
        assert!(error.extra_detail().is_some());
    }
}

/// Checks that a FIFO is rejected without waiting for its write end to be opened.
#[cfg(unix)]
pub(super) fn assert_fifo_rejected(read: fn(&Utf8Path) -> Result<(), Message>) {
    assert_fifo_operation_rejected(read, None, "files_io_not_a_file");
}

/// Checks that saving below a FIFO reports a non-directory parent without blocking.
#[cfg(unix)]
pub(super) fn assert_fifo_parent_rejected(write: fn(&Utf8Path) -> Result<(), Message>) {
    assert_fifo_operation_rejected(write, Some("document.json"), "files_io_not_a_directory");
}

/// Runs a FIFO operation in an isolated subprocess with a timeout.
#[cfg(unix)]
fn assert_fifo_operation_rejected(
    operation: fn(&Utf8Path) -> Result<(), Message>,
    file_name: Option<&str>,
    expected_key: &str,
) {
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};

    /// Passes the operation's path to the isolated subprocess.
    const FIFO_PATH_ENV: &str = "HRAFNIX_FILES_TEST_FIFO_PATH";

    if let Some(path) = std::env::var_os(FIFO_PATH_ENV) {
        let path = Utf8PathBuf::from_path_buf(path.into()).unwrap();
        let error = operation(&path).unwrap_err();
        assert_eq!(error.translate_data().message_key().as_str(), expected_key);
        assert_eq!(
            error
                .translate_data()
                .message_params()
                .get("path")
                .map(hrafnix_shareable_string::ShareableString::as_str),
            Some(path.as_str())
        );
        return;
    }

    let directory = TestDirectory::new();
    let path = directory.path.join("fifo");
    assert!(
        Command::new("mkfifo")
            .arg(&path)
            .status()
            .unwrap()
            .success()
    );
    let path = match file_name {
        Some(name) => path.join(name),
        None => path,
    };
    // Isolate a potentially blocking regression without needing nonblocking OS flags.
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            std::thread::current().name().unwrap(),
            "--nocapture",
        ])
        .env(FIFO_PATH_ENV, &path)
        .stdout(Stdio::null())
        .spawn()
        .unwrap();
    let started = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if started.elapsed() >= Duration::from_secs(2) {
            child.kill().unwrap();
            break child.wait().unwrap();
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    assert!(
        status.success(),
        "FIFO operation failed or timed out: {status}"
    );
}

/// Checks that an existing read-only file is neither replaced nor accompanied by a
/// leftover temporary file.
pub(super) fn assert_read_only_file_preserved(write: fn(&Utf8Path) -> Result<(), Message>) {
    let directory = TestDirectory::new();
    let path = directory.path.join("read-only.json");
    fs::write(&path, "original contents").unwrap();
    let mut permissions = fs::metadata(&path).unwrap().permissions();
    permissions.set_readonly(true);
    fs::set_permissions(&path, permissions).unwrap();

    let error = write(&path).unwrap_err();
    let still_read_only = fs::metadata(&path).unwrap().permissions().readonly();
    // Windows refuses to delete read-only files when the directory is cleaned up.
    #[cfg(windows)]
    #[allow(
        clippy::permissions_set_readonly_false,
        reason = "Clearing the attribute is intended and only happens on Windows."
    )]
    {
        let mut permissions = fs::metadata(&path).unwrap().permissions();
        permissions.set_readonly(false);
        fs::set_permissions(&path, permissions).unwrap();
    }

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_permission_denied"
    );
    assert_eq!(
        error
            .translate_data()
            .message_params()
            .get("path")
            .unwrap()
            .as_str(),
        path.as_str()
    );
    assert!(
        error
            .extra_detail()
            .is_some_and(|detail| detail.as_str().contains("read-only"))
    );
    assert!(still_read_only);
    assert_eq!(fs::read_to_string(&path).unwrap(), "original contents");
    assert_eq!(directory.entries(), ["read-only.json"]);
}

/// Checks that a file just over [`MAX_FILE_SIZE`] is rejected without being read.
///
/// The file is extended with `set_len`, which is sparse on common file systems, so the
/// test neither writes nor reads 256 MiB.
pub(super) fn assert_oversized_file_rejected(read: fn(&Utf8Path) -> Result<(), Message>) {
    let directory = TestDirectory::new();
    let path = directory.path.join("oversized.json");
    fs::File::create(&path)
        .unwrap()
        .set_len(MAX_FILE_SIZE.checked_add(1).unwrap())
        .unwrap();

    let error = read(&path).unwrap_err();

    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_file_too_large"
    );
    assert_eq!(
        error
            .translate_data()
            .message_params()
            .get("path")
            .unwrap()
            .as_str(),
        path.as_str()
    );
    assert!(
        error
            .extra_detail()
            .is_some_and(|detail| detail.as_str().contains("268435456-byte"))
    );
}

/// Checks that a directory-open failure occurs before replacing an existing file.
#[cfg(unix)]
pub(super) fn assert_save_error_preserves_file(write: fn(&Utf8Path) -> Result<(), Message>) {
    use std::os::unix::fs::PermissionsExt;

    let directory = TestDirectory::new();
    let path = directory.path.join("existing.json");
    fs::write(&path, "original contents").unwrap();
    let permissions = fs::metadata(&directory.path).unwrap().permissions();
    fs::set_permissions(&directory.path, fs::Permissions::from_mode(0o300)).unwrap();
    let can_read_directory = fs::File::open(&directory.path).is_ok();
    let result = write(&path);
    fs::set_permissions(&directory.path, permissions).unwrap();
    if can_read_directory {
        eprintln!(
            "Skipping permission assertions: this process can read a directory with mode 0300"
        );
        return;
    }
    let error = result.unwrap_err();
    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_permission_denied"
    );
    assert_eq!(fs::read_to_string(&path).unwrap(), "original contents");
    assert_eq!(directory.entries(), ["existing.json"]);
    assert_eq!(directory.sibling_temporary_files(), Vec::<String>::new());
}
