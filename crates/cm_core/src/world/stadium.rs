//! Stadium entity.

use crate::ids::StadiumId;
use serde::{Deserialize, Serialize};

/// A stadium.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stadium {
    pub id: StadiumId,
    pub name: String,
    pub capacity: u32,
    pub seating_capacity: u32,
    pub pitch_quality: u8,
}

impl Stadium {
    /// Create a new stadium.
    pub fn new(id: impl Into<StadiumId>, name: impl Into<String>, capacity: u32) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            capacity,
            seating_capacity: capacity,
            pitch_quality: 70,
        }
    }

    /// Get average attendance based on capacity.
    pub fn average_attendance(&self, fill_rate: f32) -> u32 {
        (self.capacity as f32 * fill_rate) as u32
    }
}
