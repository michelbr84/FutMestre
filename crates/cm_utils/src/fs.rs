//! Filesystem utilities.

use std::fs;
use std::path::Path;

use crate::errors::UtilError;

/// Read a file to string.
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String, UtilError> {
    Ok(fs::read_to_string(path)?)
}

/// Read a file to bytes.
pub fn read_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, UtilError> {
    Ok(fs::read(path)?)
}

/// Write string to file.
pub fn write_string<P: AsRef<Path>>(path: P, content: &str) -> Result<(), UtilError> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(fs::write(path, content)?)
}

/// Write bytes to file.
pub fn write_bytes<P: AsRef<Path>>(path: P, content: &[u8]) -> Result<(), UtilError> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(fs::write(path, content)?)
}

/// Check if a path exists.
pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

/// Ensure directory exists.
pub fn ensure_dir<P: AsRef<Path>>(path: P) -> Result<(), UtilError> {
    Ok(fs::create_dir_all(path)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs as std_fs;

    fn temp_dir() -> std::path::PathBuf {
        static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!("cm_utils_test_{}_{}", std::process::id(), id));
        std_fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn cleanup(dir: &std::path::Path) {
        let _ = std_fs::remove_dir_all(dir);
    }

    #[test]
    fn test_write_and_read_string() {
        let dir = temp_dir();
        let path = dir.join("test.txt");

        write_string(&path, "hello world").unwrap();
        let content = read_to_string(&path).unwrap();
        assert_eq!(content, "hello world");

        cleanup(&dir);
    }

    #[test]
    fn test_write_and_read_bytes() {
        let dir = temp_dir();
        let path = dir.join("test.bin");

        let data = vec![0u8, 1, 2, 3, 255];
        write_bytes(&path, &data).unwrap();
        let content = read_bytes(&path).unwrap();
        assert_eq!(content, data);

        cleanup(&dir);
    }

    #[test]
    fn test_exists() {
        let dir = temp_dir();
        let path = dir.join("exists.txt");

        assert!(!exists(&path));
        write_string(&path, "test").unwrap();
        assert!(exists(&path));

        cleanup(&dir);
    }

    #[test]
    fn test_ensure_dir() {
        let dir = temp_dir();
        let nested = dir.join("a").join("b").join("c");

        assert!(!exists(&nested));
        ensure_dir(&nested).unwrap();
        assert!(exists(&nested));

        cleanup(&dir);
    }

    #[test]
    fn test_write_creates_parent_dirs() {
        let dir = temp_dir();
        let path = dir.join("nested").join("deep").join("file.txt");

        write_string(&path, "content").unwrap();
        assert!(exists(&path));
        let content = read_to_string(&path).unwrap();
        assert_eq!(content, "content");

        cleanup(&dir);
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_to_string("/nonexistent/path/file.txt");
        assert!(result.is_err());
    }
}
