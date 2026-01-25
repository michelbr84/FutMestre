//! Utility error types.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = UtilError::InvalidPath("test/path".to_string());
        assert!(err.to_string().contains("test/path"));
    }

    #[test]
    fn test_hash_mismatch_display() {
        let err = UtilError::HashMismatch {
            expected: "abc123".to_string(),
            actual: "def456".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("abc123"));
        assert!(msg.contains("def456"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let util_err: UtilError = io_err.into();
        assert!(matches!(util_err, UtilError::Io(_)));
    }
}
