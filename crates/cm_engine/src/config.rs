//! Game configuration.

use cm_core::economy::Money;
use cm_core::sim::GameRules;
use serde::{Deserialize, Serialize};

/// Game mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    /// Sandbox: choose any club, no restrictions.
    Sandbox,
    /// Career from Serie D: start in the lowest division, goal is to reach Serie A and win.
    CareerSerieD,
    /// Challenge: modo desafio com restricoes especificas.
    Challenge,
}

impl Default for GameMode {
    fn default() -> Self {
        Self::Sandbox
    }
}

/// Restricoes do modo desafio.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChallengeRestrictions {
    /// Orcamento maximo de transferencias.
    #[serde(default)]
    pub max_transfer_budget: Option<Money>,
    /// Tamanho maximo do elenco.
    #[serde(default)]
    pub max_squad_size: Option<usize>,
    /// Proibido comprar jogadores.
    #[serde(default)]
    pub no_buying: bool,
    /// Apenas jogadores da base (sub-23).
    #[serde(default)]
    pub youth_only: bool,
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
    pub match_speed: u8,          // 1-5, commentary speed
    pub background_matches: bool, // show other match scores
    #[serde(default = "default_save_compressed")]
    pub save_compressed: bool, // compress save files
    /// Texto piscante ligado/desligado (estilo Elifoot).
    #[serde(default = "default_true")]
    pub flashing_text: bool,
    /// Restricoes do modo desafio.
    #[serde(default)]
    pub challenge_restrictions: ChallengeRestrictions,
}

fn default_true() -> bool {
    true
}

fn default_save_compressed() -> bool {
    true
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
            save_compressed: true,
            flashing_text: true,
            challenge_restrictions: ChallengeRestrictions::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = GameConfig::default();
        assert_eq!(config.difficulty, 50);
        assert!(config.auto_save);
        assert_eq!(config.auto_save_interval, 7);
        assert_eq!(config.game_mode, GameMode::Sandbox);
        assert_eq!(config.wage_display, WageDisplay::Weekly);
        assert_eq!(config.match_speed, 3);
        assert!(config.background_matches);
        assert!(config.save_compressed);
        assert!(config.flashing_text);
        assert!(!config.challenge_restrictions.no_buying);
        assert!(!config.challenge_restrictions.youth_only);
    }

    #[test]
    fn test_config_serialization() {
        let config = GameConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let parsed: GameConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.difficulty, config.difficulty);
        assert_eq!(parsed.auto_save, config.auto_save);
        assert_eq!(parsed.game_mode, config.game_mode);
        assert_eq!(parsed.wage_display, config.wage_display);
        assert_eq!(parsed.match_speed, config.match_speed);
        assert_eq!(parsed.save_compressed, config.save_compressed);
    }

    #[test]
    fn test_config_custom_values() {
        let mut config = GameConfig::default();
        config.difficulty = 80;
        config.auto_save = false;
        config.game_mode = GameMode::CareerSerieD;
        config.wage_display = WageDisplay::Monthly;

        let json = serde_json::to_string(&config).unwrap();
        let parsed: GameConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.difficulty, 80);
        assert!(!parsed.auto_save);
        assert_eq!(parsed.game_mode, GameMode::CareerSerieD);
        assert_eq!(parsed.wage_display, WageDisplay::Monthly);
    }

    #[test]
    fn test_game_mode_default() {
        assert_eq!(GameMode::default(), GameMode::Sandbox);
    }

    #[test]
    fn test_wage_display_default() {
        assert_eq!(WageDisplay::default(), WageDisplay::Weekly);
    }

    #[test]
    fn test_challenge_restrictions_default() {
        let r = ChallengeRestrictions::default();
        assert!(r.max_transfer_budget.is_none());
        assert!(r.max_squad_size.is_none());
        assert!(!r.no_buying);
        assert!(!r.youth_only);
    }
}
