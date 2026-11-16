//! Fowler–Noll–Vo (FNV-1a) non-cryptographic checksum.
//! Fast, lightweight hashing ideal for hash tables and cache keys. NOT cryptographically secure.

const FNV_PRIME_64: u64 = 0x00000100000001B3;
const FNV_OFFSET_BASIS_64: u64 = 0xCBF29CE484222325;

const FNV_PRIME_32: u32 = 0x01000193;
const FNV_OFFSET_BASIS_32: u32 = 0x811C9DC5;

/// Compute 64-bit FNV-1a checksum.
pub fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET_BASIS_64;
    for &byte in bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME_64);
    }
    hash
}

/// Compute 32-bit FNV-1a checksum.
pub fn fnv1a_32(bytes: &[u8]) -> u32 {
    let mut hash = FNV_OFFSET_BASIS_32;
    for &byte in bytes {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_PRIME_32);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fnv1a_deterministic() {
        assert_eq!(fnv1a_64(b""), FNV_OFFSET_BASIS_64);
        assert_eq!(fnv1a_32(b""), FNV_OFFSET_BASIS_32);
        assert_eq!(fnv1a_64(b"hello"), fnv1a_64(b"hello"));
        assert_ne!(fnv1a_64(b"hello"), fnv1a_64(b"world"));
    }
}

