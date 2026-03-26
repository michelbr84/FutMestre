//! Integrity verification.

use crate::errors::SaveError;
use sha2::{Digest, Sha256};

/// Hash bytes with SHA256 and return hex string.
pub fn hash_bytes_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

/// Verify SHA256 hash matches expected hex string.
pub fn verify_sha256_hex(data: &[u8], expected: &str) -> Result<(), SaveError> {
    let actual = hash_bytes_sha256(data);
    if actual == expected {
        Ok(())
    } else {
        Err(SaveError::IntegrityError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_deterministic() {
        let data = b"hello world";
        let hash1 = hash_bytes_sha256(data);
        let hash2 = hash_bytes_sha256(data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_different_data() {
        let hash1 = hash_bytes_sha256(b"hello");
        let hash2 = hash_bytes_sha256(b"world");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_correct_hash() {
        let data = b"test data for integrity check";
        let hash = hash_bytes_sha256(data);
        assert!(verify_sha256_hex(data, &hash).is_ok());
    }

    #[test]
    fn test_verify_wrong_hash() {
        let data = b"test data";
        let result = verify_sha256_hex(data, "wrong_hash");
        assert!(result.is_err());
    }

    #[test]
    fn test_hash_is_hex_string() {
        let hash = hash_bytes_sha256(b"data");
        // SHA256 produces 64 hex characters
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
