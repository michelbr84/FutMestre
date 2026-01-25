//! Serde extension utilities.

use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

use crate::errors::UtilError;
use crate::fs;

/// Load JSON from file.
pub fn load_json<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T, UtilError> {
    let content = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}

/// Save as JSON to file.
pub fn save_json<T: Serialize, P: AsRef<Path>>(path: P, value: &T) -> Result<(), UtilError> {
    let content = serde_json::to_string_pretty(value)?;
    fs::write_string(path, &content)
}

/// Serialize to JSON string.
pub fn to_json<T: Serialize>(value: &T) -> Result<String, UtilError> {
    Ok(serde_json::to_string_pretty(value)?)
}

/// Deserialize from JSON string.
pub fn from_json<T: DeserializeOwned>(json: &str) -> Result<T, UtilError> {
    Ok(serde_json::from_str(json)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::fs as std_fs;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestStruct {
        name: String,
        value: i32,
        items: Vec<String>,
    }

    fn temp_dir() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("cm_utils_serde_test_{}", std::process::id()));
        std_fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn cleanup(dir: &std::path::Path) {
        let _ = std_fs::remove_dir_all(dir);
    }

    #[test]
    fn test_to_json_struct() {
        let data = TestStruct {
            name: "test".to_string(),
            value: 42,
            items: vec!["a".to_string(), "b".to_string()],
        };
        let json = to_json(&data).unwrap();
        assert!(json.contains("\"name\": \"test\""));
        assert!(json.contains("\"value\": 42"));
    }

    #[test]
    fn test_from_json_struct() {
        let json = r#"{"name": "test", "value": 42, "items": ["a", "b"]}"#;
        let data: TestStruct = from_json(json).unwrap();
        assert_eq!(data.name, "test");
        assert_eq!(data.value, 42);
        assert_eq!(data.items, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_roundtrip_json_string() {
        let original = TestStruct {
            name: "roundtrip".to_string(),
            value: 123,
            items: vec!["x".to_string()],
        };
        let json = to_json(&original).unwrap();
        let parsed: TestStruct = from_json(&json).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_save_and_load_json() {
        let dir = temp_dir();
        let path = dir.join("test.json");

        let data = TestStruct {
            name: "file_test".to_string(),
            value: 999,
            items: vec!["item1".to_string(), "item2".to_string()],
        };

        save_json(&path, &data).unwrap();
        let loaded: TestStruct = load_json(&path).unwrap();
        assert_eq!(data, loaded);

        cleanup(&dir);
    }

    #[test]
    fn test_from_json_invalid() {
        let result: Result<TestStruct, _> = from_json("not valid json");
        assert!(result.is_err());
    }

    #[test]
    fn test_from_json_wrong_type() {
        let json = r#"{"name": 123, "value": "wrong", "items": []}"#;
        let result: Result<TestStruct, _> = from_json(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_json_nonexistent() {
        let result: Result<TestStruct, _> = load_json("/nonexistent/path/file.json");
        assert!(result.is_err());
    }

    #[test]
    fn test_to_json_primitives() {
        assert!(to_json(&42).unwrap().contains("42"));
        assert!(to_json(&"hello").unwrap().contains("hello"));
        assert!(to_json(&true).unwrap().contains("true"));
    }

    #[test]
    fn test_json_vec() {
        let items = vec![1, 2, 3, 4, 5];
        let json = to_json(&items).unwrap();
        let parsed: Vec<i32> = from_json(&json).unwrap();
        assert_eq!(items, parsed);
    }

    #[test]
    fn test_json_nested() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("key1".to_string(), vec![1, 2, 3]);
        map.insert("key2".to_string(), vec![4, 5]);

        let json = to_json(&map).unwrap();
        let parsed: HashMap<String, Vec<i32>> = from_json(&json).unwrap();
        assert_eq!(map, parsed);
    }
}
