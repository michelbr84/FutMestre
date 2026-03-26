//! Tactics and formations.

use serde::{Deserialize, Serialize};

/// Formation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Formation {
    F442,
    F433,
    F352,
    F451,
    F4231,
    F3412,
    F532,
    F4141,
    F4411,
    F343,
}

impl Formation {
    /// Get display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::F442 => "4-4-2",
            Self::F433 => "4-3-3",
            Self::F352 => "3-5-2",
            Self::F451 => "4-5-1",
            Self::F4231 => "4-2-3-1",
            Self::F3412 => "3-4-1-2",
            Self::F532 => "5-3-2",
            Self::F4141 => "4-1-4-1",
            Self::F4411 => "4-4-1-1",
            Self::F343 => "3-4-3",
        }
    }

    /// Get defenders count.
    pub fn defenders(&self) -> u8 {
        match self {
            Self::F442 | Self::F433 | Self::F451 | Self::F4231 | Self::F4141 | Self::F4411 => 4,
            Self::F352 | Self::F3412 | Self::F343 => 3,
            Self::F532 => 5,
        }
    }

    /// Get midfielders count.
    pub fn midfielders(&self) -> u8 {
        match self {
            Self::F442 | Self::F4411 => 4,
            Self::F433 | Self::F532 => 3,
            Self::F352 | Self::F451 | Self::F4141 => 5,
            Self::F4231 => 5,
            Self::F3412 => 5,
            Self::F343 => 4,
        }
    }

    /// Get forwards count.
    pub fn forwards(&self) -> u8 {
        match self {
            Self::F442 | Self::F352 | Self::F532 | Self::F3412 => 2,
            Self::F433 | Self::F343 => 3,
            Self::F451 | Self::F4231 | Self::F4141 | Self::F4411 => 1,
        }
    }
}

impl Default for Formation {
    fn default() -> Self {
        Self::F442
    }
}

/// Mentality setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mentality {
    Defensive,
    Cautious,
    Balanced,
    Attacking,
    AllOutAttack,
}

impl Default for Mentality {
    fn default() -> Self {
        Self::Balanced
    }
}

/// Tempo setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tempo {
    Slow,
    Normal,
    Fast,
}

impl Default for Tempo {
    fn default() -> Self {
        Self::Normal
    }
}

/// Team tactics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tactics {
    pub formation: Formation,
    pub mentality: Mentality,
    pub tempo: Tempo,
    pub pressing: u8,       // 0-100
    pub defensive_line: u8, // 0-100 (low to high)
    pub width: u8,          // 0-100 (narrow to wide)
    pub direct_passing: u8, // 0-100 (short to long)
}

impl Tactics {
    /// Create default tactics.
    pub fn new() -> Self {
        Self {
            formation: Formation::F442,
            mentality: Mentality::Balanced,
            tempo: Tempo::Normal,
            pressing: 50,
            defensive_line: 50,
            width: 50,
            direct_passing: 50,
        }
    }
}

/// Tactics preset for loading.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticsPreset {
    pub name: String,
    pub tactics: Tactics,
}
