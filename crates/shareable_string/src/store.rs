use crate::string::ShareableString;
use parking_lot::RwLock;
use rustc_hash::FxHashSet;
use std::sync::Arc;

/// A store for interning `ShareableString`s.
/// This store ensures that duplicate strings are only stored once in memory.
#[derive(Debug, Clone)]
pub struct SharedStringStore {
    /// The shared, lock-protected set of interned strings.
    string_store: Arc<RwLock<FxHashSet<ShareableString>>>,
}

impl Default for SharedStringStore {
    #[hotpath::measure]
    fn default() -> Self {
        Self::new()
    }
}

impl SharedStringStore {
    /// Creates a new, empty `SharedStringStore`.
    #[must_use]
    #[hotpath::measure]
    pub fn new() -> Self {
        Self {
            string_store: Arc::new(RwLock::new(FxHashSet::default())),
        }
    }

    /// Returns a `ShareableString` for the given key, interning it if it's not already in the store.
    #[hotpath::measure]
    pub fn get<S>(&self, key: S) -> ShareableString
    where
        S: Into<ShareableString> + AsRef<str>,
    {
        self.launder(key)
    }

    /// Returns the number of unique strings in the store.
    #[must_use]
    #[hotpath::measure]
    pub fn len(&self) -> usize {
        self.string_store.read().len()
    }

    /// Checks if the internal string store is empty.
    #[must_use]
    #[hotpath::measure]
    pub fn is_empty(&self) -> bool {
        self.string_store.read().is_empty()
    }

    /// Returns true if the store contains the specified string.
    #[must_use]
    #[hotpath::measure]
    pub fn contains(&self, key: &str) -> bool {
        self.string_store.read().contains(key)
    }

    /// Copies all strings from another store into this one.
    #[hotpath::measure]
    pub fn copy_from(&self, other: &SharedStringStore) {
        let other_store = other.string_store.read();
        let mut self_store = self.string_store.write();

        for value in other_store.iter() {
            // We only add the string if it doesn't already exist in the store.
            if !self_store.contains(value.as_str()) {
                self_store.insert(value.clone());
            }
        }
    }

    /// Adds a `ShareableString` to the store if it's not already present.
    #[hotpath::measure]
    pub fn add(&self, string: &ShareableString) {
        // If the string is already in the store, we don't need to do anything.
        // If not, we add it to enable interning for this string in the future.
        // We are taking the assumption that the string is not in the store.
        let mut store = self.string_store.write();

        if !store.contains(string.as_str()) {
            store.insert(string.clone());
        }
    }

    /// Interns the given key and returns the shared instance.
    #[hotpath::measure]
    pub fn launder<S>(&self, key: S) -> ShareableString
    where
        S: Into<ShareableString> + AsRef<str>,
    {
        // Fast path for laundering, check if it exists in the store with a read lock.
        // Preventing a wait time for acquiring the write lock.
        if let Some(existing) = self.string_store.read().get(key.as_ref()) {
            return existing.clone();
        }

        // In tests, fire a hook here to allow deterministic simulation of another writer
        // inserting the same key between our read-lock check and the write-lock acquisition.
        #[cfg(test)]
        AFTER_READ_HOOK.with(|h| {
            if let Some(f) = h.borrow().as_ref() {
                f();
            }
        });

        // Long path for laundering, check if it exists in the store with a write lock, if not, insert the string into the store.
        let mut store = self.string_store.write();

        // We are checking if someone acquired the write lock and inserted a new key after our read.
        // There is a slim chance of this happening, but it's possible.
        if let Some(existing) = store.get(key.as_ref()) {
            return existing.clone();
        }

        let key = key.into();
        store.insert(key.clone());
        key
    }
}

#[cfg(test)]
thread_local! {
    /// Fires between the read-lock check and the write-lock acquisition inside `launder`.
    /// Set this in a test to deterministically exercise the double-check branch.
    static AFTER_READ_HOOK: std::cell::RefCell<Option<Box<dyn Fn()>>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_interning() {
        let store = SharedStringStore::new();
        let s1 = store.get("hello");
        let s2 = store.get("hello");
        let s3 = store.get("world");

        // Check underline data.
        assert_eq!(s1.as_ref(), "hello");
        assert_eq!(s2.as_ref(), "hello");
        assert_eq!(s3.as_ref(), "world");

        // Check equality.
        assert_eq!(s1, s2);
        assert_ne!(s1, s3);

        // Check underlying Arc sharing.
        assert!(Arc::ptr_eq(s1.as_arc(), s2.as_arc()));
        assert!(!Arc::ptr_eq(s1.as_arc(), s3.as_arc()));

        // Check hash.
        assert_eq!(s1.current_blake3_hash(), s2.current_blake3_hash());
        assert_ne!(s1.current_blake3_hash(), s3.current_blake3_hash());

        // Check store count.
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn test_various_input_types() {
        let store = SharedStringStore::new();

        let a = store.get("hi");
        let b = store.get(String::from("hi"));

        // Check underline data.
        assert_eq!(a.as_ref(), "hi");
        assert_eq!(b.as_ref(), "hi");

        // Check equality.
        assert_eq!(a, b);

        // Check underlying Arc sharing.
        assert!(Arc::ptr_eq(a.as_arc(), b.as_arc()));

        // Check hash.
        assert_eq!(a.current_blake3_hash(), b.current_blake3_hash());

        // Check store count.
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_copy_from_preserves_memory_address() {
        let store_a = SharedStringStore::new();
        let a_hello = store_a.get("hello");
        let a_world = store_a.get("world");

        assert_eq!(store_a.len(), 2);

        let store_b = SharedStringStore::new();
        assert_eq!(store_b.len(), 0);

        store_b.copy_from(&store_a);
        assert_eq!(store_b.len(), 2);

        // After copying, store_b should hold clones of the same ShareableString values (same Arc)
        let b_hello = store_b.get("hello");
        let b_world = store_b.get("world");

        assert_eq!(a_hello, b_hello);
        assert_eq!(a_world, b_world);

        assert!(Arc::ptr_eq(a_hello.as_arc(), b_hello.as_arc()));
        assert!(Arc::ptr_eq(a_world.as_arc(), b_world.as_arc()));
    }

    #[test]
    fn test_store_add() {
        let store_a = SharedStringStore::new();
        let a_x = store_a.get("x");

        let store_b = SharedStringStore::new();
        let b_x = store_b.get("x");
        let b_y = store_b.get("y");

        // Check underling arc pointers.
        assert!(!Arc::ptr_eq(a_x.as_arc(), b_x.as_arc()));

        // If the key exists, add() inserts it.
        let len_before_x = store_a.len();
        store_a.add(&b_x);

        let a_x_after = store_a.get("x");
        assert_eq!(store_a.len(), len_before_x);
        assert!(Arc::ptr_eq(a_x_after.as_arc(), a_x.as_arc()));
        assert!(!Arc::ptr_eq(a_x_after.as_arc(), b_x.as_arc()));

        // If the key is missing, add() inserts it.
        let len_before_y = store_a.len();
        store_a.add(&b_y);
        assert_eq!(store_a.len(), len_before_y + 1);

        let a_y_after = store_a.get("y");
        assert!(Arc::ptr_eq(a_y_after.as_arc(), b_y.as_arc()));
        assert_eq!(store_a.len(), 2);
    }

    #[test]
    fn test_store_len() {
        let store = SharedStringStore::new();
        assert_eq!(store.len(), 0);
        assert!(store.is_empty());

        let _a = store.get("a");
        assert_eq!(store.len(), 1);
        assert!(!store.is_empty());

        let _a = store.get("a");
        assert_eq!(store.len(), 1);
        assert!(!store.is_empty());

        let _b = store.get("b");
        assert_eq!(store.len(), 2);
        assert!(!store.is_empty());
    }

    #[test]
    fn test_shared_string_fxhashset() {
        use rustc_hash::FxHashSet;

        let store = SharedStringStore::new();
        let a1 = store.get("same");
        let a2 = store.get("same");
        let b = store.get("different");

        let mut set = FxHashSet::default();
        assert!(set.insert(a1.clone()));
        assert!(!set.insert(a2.clone())); // equal => should not insert
        assert!(set.insert(b));

        assert_eq!(set.len(), 2);
        assert!(set.contains(&a1));
        assert!(set.contains(&a2));
        assert!(!set.contains(&store.get("not-present")));
    }

    #[test]
    fn test_store_is_thread_safe() {
        use std::hint::spin_loop;
        use std::sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        };
        use std::thread;
        use std::thread::sleep;
        use std::time::Duration;

        let count = 10;

        let store = SharedStringStore::new();
        let ready = Arc::new(AtomicBool::new(false));
        let mut handles = Vec::new();

        // Spawn threads looking for "shared"
        for _ in 0..count {
            let store_clone = store.clone();
            let ready_clone = Arc::clone(&ready);
            handles.push(thread::spawn(move || {
                while !ready_clone.load(Ordering::Acquire) {
                    spin_loop();
                }
                store_clone.get("shared")
            }));
        }

        // Spawn 1 thread looking for "not-shared"
        let store_clone = store.clone();
        let ready_clone = Arc::clone(&ready);
        handles.push(thread::spawn(move || {
            while !ready_clone.load(Ordering::Acquire) {
                spin_loop();
            }
            store_clone.get("not-shared")
        }));

        sleep(Duration::new(0, 1));

        ready.store(true, Ordering::Release);

        let mut shared_results = Vec::new();
        let mut not_shared_result = None;

        for (i, handle) in handles.into_iter().enumerate() {
            let res = handle.join().unwrap();
            if i < count {
                shared_results.push(res);
            } else {
                not_shared_result = Some(res);
            }
        }

        let not_shared = not_shared_result.unwrap();

        // Verify all "shared" results are the same and share the same Arc
        let first_shared = &shared_results[0];
        for s in &shared_results[1..] {
            assert_eq!(first_shared, s);
            assert!(Arc::ptr_eq(first_shared.as_arc(), s.as_arc()));
        }

        // Verify the "not-shared" result is different
        assert_ne!(first_shared, &not_shared);
        assert!(!Arc::ptr_eq(first_shared.as_arc(), not_shared.as_arc()));

        // Check store count.
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn test_copy_from_does_not_override_existing() {
        let dst = SharedStringStore::new();
        let dst_v = dst.get("k");
        assert_eq!(dst.len(), 1);

        let src = SharedStringStore::new();
        let src_v = src.get("k");
        assert_eq!(src.len(), 1);

        // Sanity: different stores => different allocations initially.
        assert!(!Arc::ptr_eq(dst_v.as_arc(), src_v.as_arc()));

        dst.copy_from(&src);

        // After copy, dst should still point at its original value for the same key.
        let dst_after = dst.get("k");
        assert!(Arc::ptr_eq(dst_after.as_arc(), dst_v.as_arc()));
        assert!(!Arc::ptr_eq(dst_after.as_arc(), src_v.as_arc()));
    }

    #[test]
    fn test_store_launder() {
        let store = SharedStringStore::new();
        let s1 = store.launder("test");
        let s2 = store.launder(String::from("test"));
        let s3 = ShareableString::new("test");
        let s4 = store.launder(s3.clone());

        assert_eq!(s1, "test");
        assert_eq!(s2, "test");
        assert_eq!(s4, "test");

        assert!(Arc::ptr_eq(s1.as_arc(), s2.as_arc()));
        assert!(Arc::ptr_eq(s1.as_arc(), s4.as_arc()));
        // s3 was created outside, but launder should return the one in store.
        assert!(!Arc::ptr_eq(s1.as_arc(), s3.as_arc()));
    }

    #[test]
    fn test_store_contains() {
        let store = SharedStringStore::new();
        store.get("present");
        assert!(store.contains("present"));
        assert!(!store.contains("absent"));
    }

    /// Deterministically exercises the double-check branch inside `launder`:
    /// the `if let Some(existing) = store.get(key.as_ref())` guard that runs
    /// after the write lock is acquired.
    ///
    /// The hook fires between the read-lock check and the write-lock acquisition,
    /// simulating a concurrent writer inserting the same key in that window.
    #[test]
    fn test_launder_double_check_path() {
        let store = SharedStringStore::new();
        let store_clone = store.clone();

        AFTER_READ_HOOK.with(|h| {
            *h.borrow_mut() = Some(Box::new(move || {
                // Simulate a concurrent writer inserting the key after our read-lock check
                // failed but before we acquire the write lock.
                store_clone.add(&ShareableString::new("race"));
            }));
        });

        let result = store.launder("race");

        // Clean up the hook before any assertion that might panic.
        AFTER_READ_HOOK.with(|h| *h.borrow_mut() = None);

        // The double-check branch must have returned the already-inserted instance.
        assert_eq!(result.as_ref(), "race");
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_store_default() {
        let store = SharedStringStore::default();
        assert_eq!(store.len(), 0);
        assert!(store.is_empty());
    }
}
