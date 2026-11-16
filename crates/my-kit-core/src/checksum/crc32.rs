//! IEEE 802.3 CRC-32 non-cryptographic checksum.
//! Useful for verifying data integrity in network packets and files.

const CRC32_POLYNOMIAL: u32 = 0xEDB88320;

/// Compute IEEE 802.3 32-bit Cyclic Redundancy Check (CRC-32).
pub fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFFu32;
    for &byte in bytes {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ CRC32_POLYNOMIAL;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32_standard_test_vector() {
        // Standard check: "123456789" produces 0xCBF43926
        assert_eq!(crc32(b"123456789"), 0xCBF43926);
        assert_eq!(crc32(b""), 0x00000000);
    }
}

