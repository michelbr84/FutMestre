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
