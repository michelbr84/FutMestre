//! Hashing utilities.

use sha2::{Digest, Sha256};

use crate::errors::UtilError;

/// Hash bytes with SHA256 and return hex string.
pub fn hash_bytes_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

/// Verify SHA256 hash matches expected hex string.
pub fn verify_sha256_hex(data: &[u8], expected: &str) -> Result<(), UtilError> {
    let actual = hash_bytes_sha256(data);
    if actual == expected {
        Ok(())
    } else {
        Err(UtilError::HashMismatch {
            expected: expected.to_string(),
            actual,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_empty_bytes() {
        let hash = hash_bytes_sha256(&[]);
        // SHA256 of empty string is well-known
        assert_eq!(
            hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_hash_hello_world() {
        let hash = hash_bytes_sha256(b"hello world");
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_hash_consistency() {
        let data = b"test data for hashing";
        let hash1 = hash_bytes_sha256(data);
        let hash2 = hash_bytes_sha256(data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_different_data() {
        let hash1 = hash_bytes_sha256(b"data1");
        let hash2 = hash_bytes_sha256(b"data2");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_correct_hash() {
        let data = b"hello world";
        let hash = hash_bytes_sha256(data);
        assert!(verify_sha256_hex(data, &hash).is_ok());
    }

    #[test]
    fn test_verify_incorrect_hash() {
        let data = b"hello world";
        let result = verify_sha256_hex(data, "invalid_hash");
        assert!(result.is_err());
        if let Err(UtilError::HashMismatch { expected, actual }) = result {
            assert_eq!(expected, "invalid_hash");
            assert!(!actual.is_empty());
        } else {
            panic!("Expected HashMismatch error");
        }
    }

    #[test]
    fn test_hash_binary_data() {
        let data: Vec<u8> = (0..=255).collect();
        let hash = hash_bytes_sha256(&data);
        assert_eq!(hash.len(), 64); // SHA256 produces 64 hex chars
    }
}
