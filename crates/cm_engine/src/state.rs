//! Game state.

use chrono::NaiveDate;
use cm_core::ids::ClubId;
use cm_core::sim::GameDate;
use cm_match::MatchResult;
use serde::{Deserialize, Serialize};

/// Objetivo de carreira (modo Serie D).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareerObjective {
    pub description: String,
    pub completed: bool,
    pub season: String,
}

/// Game state flags.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameFlags {
    pub match_day: bool,
    pub transfer_window_open: bool,
    pub season_end: bool,
    pub dirty: bool,
}

/// Game state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub date: GameDate,
    pub manager_name: String,
    pub club_id: ClubId,
    pub inbox: Vec<String>,
    pub flags: GameFlags,
    pub days_played: u32,
    /// Resultado da ultima partida do usuario (preservado para exibicao na TUI).
    #[serde(skip)]
    pub last_match_result: Option<MatchResult>,
    /// Objetivos de carreira (modo Serie D).
    #[serde(default)]
    pub career_objectives: Vec<CareerObjective>,
}

impl GameState {
    /// Create a new game state.
    pub fn new(start_date: NaiveDate, manager_name: String, club_id: ClubId) -> Self {
        Self {
            date: GameDate::from(start_date),
            manager_name,
            club_id,
            inbox: Vec::new(),
            flags: GameFlags::default(),
            days_played: 0,
            last_match_result: None,
            career_objectives: Vec::new(),
        }
    }

    /// Add inbox message.
    pub fn add_message(&mut self, message: impl Into<String>) {
        self.inbox.push(message.into());
    }

    /// Get season string.
    pub fn season(&self) -> String {
        self.date.season_string()
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new(
            NaiveDate::from_ymd_opt(2001, 7, 1).unwrap(),
            "Manager".to_string(),
            ClubId::new("LIV"),
        )
    }
}
