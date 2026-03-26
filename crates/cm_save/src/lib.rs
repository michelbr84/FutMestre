//! # CM Save
//!
//! Save game system with compression and integrity verification.

pub mod compression;
pub mod errors;
pub mod format;
pub mod integrity;
pub mod metadata;
pub mod snapshot;
pub mod versioning;

pub use errors::SaveError;
pub mod export;

pub use metadata::{delete_save, list_saves, should_auto_save, SaveMetadata};
pub use snapshot::SaveSnapshot;
