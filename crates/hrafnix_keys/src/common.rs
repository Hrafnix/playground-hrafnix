/// Reserved keyword strings that are not valid as bare `StoreKey` values.
pub(crate) const KEY_WORDS: [&str; 2] = ["true", "false"];

/// Returns `true` if `s` is a non-empty key that starts with `prefix` and whose
/// remaining characters satisfy the standard key rules (first char `a-z`, rest
/// `a-z | 0-9 | _`). When `prefix` is empty, the key must also not match any
/// reserved keyword in [`KEY_WORDS`].
#[allow(
    clippy::indexing_slicing,
    reason = "All indexed access is guarded by explicit length and loop-bound checks."
)]
pub(crate) const fn is_valid_key_with_prefix(s: &str, prefix: &str) -> bool {
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
    if prefix_bytes.is_empty() && first_after_prefix == b'_' {
        return false;
    }
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
