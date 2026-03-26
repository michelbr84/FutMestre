//! Save snapshot for persistence.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::compression::{read_gzip, write_gzip};
use crate::errors::SaveError;
use crate::integrity::{hash_bytes_sha256, verify_sha256_hex};
use crate::versioning::SAVE_VERSION;
use cm_core::world::World;

/// Save payload containing game state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavePayload {
    pub world: World,
    pub game_config: GameConfigData,
    pub game_state: GameStateData,
}

/// Game configuration data for save.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameConfigData {
    pub difficulty: u8,
    pub auto_save: bool,
}

/// Game state data for save.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameStateData {
    pub date: String,
    pub manager_name: String,
    pub club_id: String,
    pub inbox: Vec<String>,
}

/// Complete save snapshot.
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveSnapshot {
    pub version: u32,
    pub created_at: DateTime<Utc>,
    pub sha256: String,
    pub payload: SavePayload,
}

impl SaveSnapshot {
    /// Create a new snapshot.
    pub fn new(
        world: World,
        config: GameConfigData,
        state: GameStateData,
    ) -> Result<Self, SaveError> {
        let payload = SavePayload {
            world,
            game_config: config,
            game_state: state,
        };

        let bytes = serde_json::to_vec(&payload)?;
        let sha256 = hash_bytes_sha256(&bytes);

        Ok(Self {
            version: SAVE_VERSION,
            created_at: Utc::now(),
            sha256,
            payload,
        })
    }

    /// Write to file with compression.
    pub fn write_to_file(&self, path: &str) -> Result<(), SaveError> {
        let bytes = serde_json::to_vec(self)?;
        write_gzip(path, &bytes)
    }

    /// Read from file with decompression.
    pub fn read_from_file(path: &str) -> Result<Self, SaveError> {
        let bytes = read_gzip(path)?;
        let snap: SaveSnapshot = serde_json::from_slice(&bytes)?;

        // Verify integrity
        let payload_bytes = serde_json::to_vec(&snap.payload)?;
        verify_sha256_hex(&payload_bytes, &snap.sha256)?;

        Ok(snap)
    }

    /// Get the world.
    pub fn world(&self) -> &World {
        &self.payload.world
    }

    /// Get mutable world.
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.payload.world
    }

    /// Get game state.
    pub fn state(&self) -> &GameStateData {
        &self.payload.game_state
    }

    /// Get mutable game state.
    pub fn state_mut(&mut self) -> &mut GameStateData {
        &mut self.payload.game_state
    }
}
