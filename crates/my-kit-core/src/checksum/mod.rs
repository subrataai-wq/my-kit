//! Data integrity checksums (FNV-1a, CRC-32).
//!
//! Note: Algorithms in this module are non-cryptographic and intended strictly
//! for integrity verification and hash table keys. For passwords, use the `argon2` crate;
//! for cryptographic security, use `sha2`; for secret memory wiping, use `zeroize`.

pub mod crc32;
pub mod fnv;

pub mod prelude {
    pub use super::crc32::crc32;
    pub use super::fnv::{fnv1a_32, fnv1a_64};
}

