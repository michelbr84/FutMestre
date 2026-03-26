//! Compression utilities.

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::errors::SaveError;

/// Write gzip compressed data to file.
pub fn write_gzip<P: AsRef<Path>>(path: P, data: &[u8]) -> Result<(), SaveError> {
    // Create parent directories if needed
    if let Some(parent) = path.as_ref().parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;
    let mut encoder = GzEncoder::new(file, Compression::default());
    encoder.write_all(data)?;
    encoder.finish()?;
    Ok(())
}

/// Read gzip compressed data from file.
pub fn read_gzip<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, SaveError> {
    let file = File::open(path)?;
    let mut decoder = GzDecoder::new(file);
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;
    Ok(data)
}

/// Compress data in memory.
pub fn compress(data: &[u8]) -> Result<Vec<u8>, SaveError> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}

/// Decompress data in memory.
pub fn decompress(data: &[u8]) -> Result<Vec<u8>, SaveError> {
    let mut decoder = GzDecoder::new(data);
    let mut output = Vec::new();
    decoder.read_to_end(&mut output)?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_decompression_roundtrip() {
        let original = b"Hello, this is test data for compression. ".repeat(100);
        let compressed = compress(&original).expect("compression should succeed");
        let decompressed = decompress(&compressed).expect("decompression should succeed");
        assert_eq!(original.to_vec(), decompressed);
    }

    #[test]
    fn test_compression_reduces_size() {
        // Repetitive data compresses well
        let original = b"AAAAAAAAAA".repeat(1000);
        let compressed = compress(&original).expect("compression should succeed");
        assert!(
            compressed.len() < original.len(),
            "Compressed ({}) should be smaller than original ({})",
            compressed.len(),
            original.len()
        );
    }

    #[test]
    fn test_compression_empty_data() {
        let original = b"";
        let compressed = compress(original).expect("compression should succeed");
        let decompressed = decompress(&compressed).expect("decompression should succeed");
        assert!(decompressed.is_empty());
    }

    #[test]
    fn test_decompression_invalid_data() {
        let result = decompress(b"this is not gzip data");
        assert!(result.is_err());
    }

    #[test]
    fn test_compression_json_data() {
        let json = serde_json::json!({
            "name": "Test World",
            "players": [{"id": "P001", "name": "Test Player"}],
            "clubs": [{"id": "FLA", "name": "Flamengo"}]
        });
        let original = serde_json::to_vec(&json).unwrap();
        let compressed = compress(&original).expect("compression should succeed");
        let decompressed = decompress(&compressed).expect("decompression should succeed");
        assert_eq!(original, decompressed);
    }

    #[test]
    fn test_write_read_gzip_file() {
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("cm_test_save.gz");
        let path_str = path.to_str().unwrap();

        let original = b"Test save data for FutMestre";
        write_gzip(path_str, original).expect("write should succeed");
        let read_back = read_gzip(path_str).expect("read should succeed");
        assert_eq!(original.to_vec(), read_back);

        // Cleanup
        let _ = std::fs::remove_file(&path);
    }
}
