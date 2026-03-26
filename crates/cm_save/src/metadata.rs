//! Save metadata and listing utilities.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::errors::SaveError;
use crate::format::SAVE_EXTENSION;
use crate::snapshot::SaveSnapshot;

/// Metadata describing a save file without loading the full world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveMetadata {
    /// Name of the save (file stem without extension).
    pub save_name: String,
    /// When the save was created (real-world time).
    pub created_at: DateTime<Utc>,
    /// In-game date string (e.g. "2026-08-15").
    pub game_date: String,
    /// Manager name.
    pub manager_name: String,
    /// Club name / ID.
    pub club_name: String,
    /// Full file path on disk.
    #[serde(default)]
    pub file_path: String,
    /// File size in bytes.
    #[serde(default)]
    pub file_size: u64,
}

/// Scan a directory for `.cmsave` files and return metadata for each.
///
/// Files that cannot be read or parsed are silently skipped.
pub fn list_saves(save_dir: &Path) -> Vec<SaveMetadata> {
    let mut results = Vec::new();

    let entries = match std::fs::read_dir(save_dir) {
        Ok(e) => e,
        Err(_) => return results,
    };

    for entry in entries.flatten() {
        let path = entry.path();

        // Only consider files with the right extension
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext != SAVE_EXTENSION {
            continue;
        }

        // Try to read the snapshot
        let path_str = match path.to_str() {
            Some(s) => s,
            None => continue,
        };

        // Get file size before reading (cheaper than loading full snapshot)
        let file_size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);

        if let Ok(snapshot) = SaveSnapshot::read_from_file(path_str) {
            let save_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();

            results.push(SaveMetadata {
                save_name,
                created_at: snapshot.created_at,
                game_date: snapshot.payload.game_state.date.clone(),
                manager_name: snapshot.payload.game_state.manager_name.clone(),
                club_name: snapshot.payload.game_state.club_id.clone(),
                file_path: path_str.to_string(),
                file_size,
            });
        }
    }

    // Sort by created_at descending (most recent first)
    results.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    results
}

/// Delete a save file at the given path.
///
/// Returns `Ok(())` if the file was deleted, or an error if removal failed.
pub fn delete_save(path: &str) -> Result<(), SaveError> {
    let p = Path::new(path);
    if !p.exists() {
        return Err(SaveError::NotFound(path.to_string()));
    }
    std::fs::remove_file(p)?;
    Ok(())
}

/// Returns true if the game should auto-save based on days played.
///
/// Auto-saves every `interval` game days. Returns `true` when
/// `days_played` is a non-zero multiple of `interval`.
pub fn should_auto_save(days_played: u32, interval: u32) -> bool {
    if interval == 0 || days_played == 0 {
        return false;
    }
    days_played % interval == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// Create a unique temporary directory for testing.
    fn temp_test_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("cm_save_test_{name}_{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        dir
    }

    #[test]
    fn test_should_auto_save_basic() {
        // Every 7 days
        assert!(!should_auto_save(0, 7));
        assert!(!should_auto_save(1, 7));
        assert!(!should_auto_save(6, 7));
        assert!(should_auto_save(7, 7));
        assert!(!should_auto_save(8, 7));
        assert!(should_auto_save(14, 7));
        assert!(should_auto_save(21, 7));
    }

    #[test]
    fn test_should_auto_save_interval_one() {
        // Every day
        assert!(should_auto_save(1, 1));
        assert!(should_auto_save(5, 1));
    }

    #[test]
    fn test_should_auto_save_zero_interval() {
        // Disabled
        assert!(!should_auto_save(7, 0));
        assert!(!should_auto_save(100, 0));
    }

    #[test]
    fn test_list_saves_empty_dir() {
        let tmp = temp_test_dir("empty");
        let saves = list_saves(&tmp);
        assert!(saves.is_empty());
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_list_saves_ignores_non_save_files() {
        let tmp = temp_test_dir("nonsave");
        fs::write(tmp.join("notes.txt"), "hello").unwrap();
        let saves = list_saves(&tmp);
        assert!(saves.is_empty());
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_list_saves_nonexistent_dir() {
        let saves = list_saves(Path::new("/nonexistent/path/12345"));
        assert!(saves.is_empty());
    }

    #[test]
    fn test_save_metadata_struct() {
        let meta = SaveMetadata {
            save_name: "slot1".to_string(),
            created_at: Utc::now(),
            game_date: "2026-08-15".to_string(),
            manager_name: "Jose".to_string(),
            club_name: "Porto".to_string(),
            file_path: "saves/slot1.cmsave".to_string(),
            file_size: 12345,
        };
        assert_eq!(meta.save_name, "slot1");
        assert_eq!(meta.manager_name, "Jose");
        assert_eq!(meta.club_name, "Porto");
        assert_eq!(meta.game_date, "2026-08-15");
        assert_eq!(meta.file_path, "saves/slot1.cmsave");
        assert_eq!(meta.file_size, 12345);
    }

    #[test]
    fn test_delete_save_nonexistent() {
        let result = delete_save("/nonexistent/path/fake.cmsave");
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_save_real_file() {
        let tmp = temp_test_dir("delete");
        let file_path = tmp.join("test.cmsave");
        fs::write(&file_path, "fake save data").unwrap();
        assert!(file_path.exists());

        let result = delete_save(file_path.to_str().unwrap());
        assert!(result.is_ok());
        assert!(!file_path.exists());

        let _ = fs::remove_dir_all(&tmp);
    }
}
