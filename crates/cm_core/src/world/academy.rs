//! Youth academy.

use serde::{Deserialize, Serialize};

/// Youth academy.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Academy {
    /// Academy level (1-10).
    pub level: u8,
    /// Youth recruitment rating (1-20).
    pub youth_recruitment: u8,
    /// Junior coaching rating (1-20).
    pub junior_coaching: u8,
    /// Youth facilities rating (1-20).
    pub facilities: u8,
}

impl Academy {
    /// Create a new academy.
    pub fn new(level: u8) -> Self {
        Self {
            level: level.min(10),
            youth_recruitment: 10,
            junior_coaching: 10,
            facilities: 10,
        }
    }

    /// Get overall academy rating.
    pub fn overall_rating(&self) -> u8 {
        ((self.youth_recruitment as u16 + self.junior_coaching as u16 + self.facilities as u16) / 3)
            as u8
    }
}
