//! Game configuration.

use cm_core::sim::GameRules;
use serde::{Deserialize, Serialize};

/// Game mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    /// Sandbox: choose any club, no restrictions.
    Sandbox,
    /// Career from Serie D: start in the lowest division, goal is to reach Serie A and win.
    CareerSerieD,
}

impl Default for GameMode {
    fn default() -> Self {
        Self::Sandbox
    }
}

/// Wage display format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WageDisplay {
    Weekly,
    Monthly,
    Yearly,
}

impl Default for WageDisplay {
    fn default() -> Self {
        Self::Weekly
    }
}

/// Game configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub rules: GameRules,
    pub difficulty: u8,
    pub auto_save: bool,
    pub auto_save_interval: u16, // days
    pub game_mode: GameMode,
    pub wage_display: WageDisplay,
    pub match_speed: u8,         // 1-5, commentary speed
    pub background_matches: bool, // show other match scores
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            rules: GameRules::default(),
            difficulty: 50,
            auto_save: true,
            auto_save_interval: 7,
            game_mode: GameMode::Sandbox,
            wage_display: WageDisplay::Weekly,
            match_speed: 3,
            background_matches: true,
        }
    }
}
