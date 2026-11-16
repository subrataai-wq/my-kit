//! Unique ids and monotonic ULID generation.
//!
//! Provides both lightweight pseudo-random IDs for UI/DOM use, and cryptographically
//! random, time-ordered 128-bit ULIDs for domain entity identifiers.

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};

pub use ulid::Ulid;

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generates a new 128-bit time-ordered ULID with millisecond precision.
pub fn new_ulid() -> Ulid {
    Ulid::new()
}

/// Generates a 26-character Crockford Base32 ULID string (e.g. "01ARZ3NDEKTSV4RRFFQ69G5FAV").
pub fn new_ulid_string() -> String {
    Ulid::new().to_string()
}

/// Generates a monotonically ascending ULID string, guaranteeing strictly increasing sort order
/// even when multiple identifiers are created within the exact same millisecond.
pub fn new_monotonic_ulid_string() -> String {
    use std::cell::RefCell;
    thread_local! {
        static MONOTONIC_GEN: RefCell<ulid::Generator> = const { RefCell::new(ulid::Generator::new()) };
    }
    MONOTONIC_GEN.with(|generator| {
        generator
            .borrow_mut()
            .generate()
            .unwrap_or_else(|_| Ulid::new())
            .to_string()
    })
}

/// A random-looking 64 bit number. Works on all targets, including web.
pub fn random_u64() -> u64 {

    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u64(COUNTER.fetch_add(1, Ordering::Relaxed));
    hasher.finish()
}

/// 32 letters and numbers (hex), like `"9f3a...c1"`.
pub fn unique_id() -> String {
    format!("{:016x}{:016x}", random_u64(), random_u64())
}

/// Short id of `len` characters (a-z, 0-9). Good for UI keys.
pub fn short_id(len: usize) -> String {
    const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut out = String::with_capacity(len);
    let mut bits = 0u64;
    let mut left = 0;
    for _ in 0..len {
        if left == 0 {
            bits = random_u64();
            left = 10;
        }
        out.push(ALPHABET[(bits % ALPHABET.len() as u64) as usize] as char);
        bits /= ALPHABET.len() as u64;
        left -= 1;
    }
    out
}

/// Random number from `lo` to `hi` (both included). Slight bias is fine for UI use.
pub fn random_range(lo: u64, hi: u64) -> u64 {
    let (lo, hi) = if lo <= hi { (lo, hi) } else { (hi, lo) };
    let span = hi - lo;
    if span == u64::MAX {
        return random_u64();
    }
    lo + random_u64() % (span + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn ids_are_unique_and_sized() {
        let set: HashSet<String> = (0..2000).map(|_| unique_id()).collect();
        assert_eq!(set.len(), 2000);
        assert!(set.iter().all(|s| s.len() == 32));
        assert_eq!(short_id(8).len(), 8);
        assert!(
            short_id(12)
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        );
        assert_eq!(short_id(0), "");
    }

    #[test]
    fn range_stays_inside() {
        for _ in 0..500 {
            let n = random_range(5, 9);
            assert!((5..=9).contains(&n));
        }
        assert_eq!(random_range(3, 3), 3);
        assert!((1..=2).contains(&random_range(2, 1)));
    }

    #[test]
    fn ulid_is_26_chars_and_sortable() {
        let u1 = new_ulid();
        let s = new_ulid_string();
        assert_eq!(s.len(), 26);
        assert_eq!(u1.to_string().len(), 26);

        let mut monotonic_ids = Vec::new();
        for _ in 0..100 {
            monotonic_ids.push(new_monotonic_ulid_string());
        }
        for window in monotonic_ids.windows(2) {
            assert!(
                window[0] < window[1],
                "{} was not strictly less than {}",
                window[0],
                window[1]
            );
        }
    }
}
