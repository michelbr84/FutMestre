//! Core error types.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Entity not found: {entity_type} with id {id}")]
    NotFound { entity_type: String, id: String },

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Date parse error: {0}")]
    DateParse(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_error() {
        let err = CoreError::NotFound {
            entity_type: "Player".to_string(),
            id: "P001".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("Player"));
        assert!(msg.contains("P001"));
    }

    #[test]
    fn test_invalid_operation_error() {
        let err = CoreError::InvalidOperation("Cannot transfer during closed window".to_string());
        assert!(err.to_string().contains("Cannot transfer"));
    }

    #[test]
    fn test_validation_error() {
        let err = CoreError::Validation("Age must be positive".to_string());
        assert!(err.to_string().contains("Age must be positive"));
    }

    #[test]
    fn test_date_parse_error() {
        let err = CoreError::DateParse("invalid date format".to_string());
        assert!(err.to_string().contains("invalid date format"));
    }

    #[test]
    fn test_serialization_error() {
        let err = CoreError::Serialization("JSON parse failed".to_string());
        assert!(err.to_string().contains("JSON parse failed"));
    }

    #[test]
    fn test_error_debug() {
        let err = CoreError::NotFound {
            entity_type: "Club".to_string(),
            id: "LIV".to_string(),
        };
        let debug = format!("{:?}", err);
        assert!(debug.contains("NotFound"));
    }
}
