//! Scouting system.

use crate::ids::PlayerId;
use serde::{Deserialize, Serialize};

/// Scout report on a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoutReport {
    pub player_id: PlayerId,
    pub scout_accuracy: u8, // percentage
    pub estimated_ability: u8,
    pub estimated_potential: u8,
    pub recommended: bool,
    pub notes: String,
}

impl ScoutReport {
    /// Create a new scout report.
    pub fn new(player_id: PlayerId, scout_accuracy: u8) -> Self {
        Self {
            player_id,
            scout_accuracy,
            estimated_ability: 50,
            estimated_potential: 60,
            recommended: false,
            notes: String::new(),
        }
    }
}
