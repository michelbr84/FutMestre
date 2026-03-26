//! Match model types.

use cm_core::ids::ClubId;
use cm_core::world::Club;
use serde::{Deserialize, Serialize};

/// Match input for simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchInput {
    pub home_id: ClubId,
    pub away_id: ClubId,
    pub home: TeamStrength,
    pub away: TeamStrength,
    pub minutes: u8,
    pub seed: Option<u64>,
}

/// Team strength for match calculation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TeamStrength {
    pub attack: u8,
    pub midfield: u8,
    pub defense: u8,
    pub finishing: u8,
    pub morale: u8,
    pub fitness: u8,
}

impl TeamStrength {
    /// Create from club (simplified calculation).
    pub fn from_club(club: &Club) -> Self {
        // Use reputation as a proxy for overall strength
        let base = club.reputation;
        Self {
            attack: base.saturating_sub(5),
            midfield: base,
            defense: base.saturating_add(5).min(100),
            finishing: base.saturating_sub(10),
            morale: 70,
            fitness: 80,
        }
    }

    /// Overall strength.
    pub fn overall(&self) -> u8 {
        ((self.attack as u16 + self.midfield as u16 + self.defense as u16) / 3) as u8
    }
}

/// Match statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MatchStats {
    pub home_possession: f64,
    pub away_possession: f64,
    pub home_shots: u32,
    pub away_shots: u32,
    pub home_shots_on_target: u32,
    pub away_shots_on_target: u32,
    pub home_fouls: u32,
    pub away_fouls: u32,
    pub home_corners: u32,
    pub away_corners: u32,
    pub home_yellow_cards: u32,
    pub away_yellow_cards: u32,
    pub home_red_cards: u32,
    pub away_red_cards: u32,
}

/// Type of match event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchEventType {
    Goal,
    YellowCard,
    RedCard,
    Injury,
    Substitution,
    Corner,
    FreeKick,
    Penalty,
    PenaltyMiss,
    HalfTime,
    FullTime,
    ExtraTime,
}

/// A single match event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchEvent {
    pub minute: u32,
    pub event_type: MatchEventType,
    pub description: String,
}

/// Lado do time na partida.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeamSide {
    Home,
    Away,
}

/// Rating de desempenho individual de um jogador na partida.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMatchRating {
    pub player_id: String,
    pub team: TeamSide,
    pub rating: f32, // 1.0 - 10.0
    pub goals: u8,
    pub assists: u8,
    pub shots: u8,
    pub tackles: u8,
    pub passes_completed: u8,
    pub saves: u8, // for goalkeepers
    pub man_of_the_match: bool,
}

/// Match result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub home_id: ClubId,
    pub away_id: ClubId,
    pub home_goals: u8,
    pub away_goals: u8,
    pub highlights: Vec<String>,
    #[serde(default)]
    pub stats: MatchStats,
    #[serde(default)]
    pub events: Vec<MatchEvent>,
    #[serde(default)]
    pub player_ratings: Vec<PlayerMatchRating>,
}

impl MatchResult {
    /// Check if home win.
    pub fn is_home_win(&self) -> bool {
        self.home_goals > self.away_goals
    }

    /// Check if away win.
    pub fn is_away_win(&self) -> bool {
        self.away_goals > self.home_goals
    }

    /// Check if draw.
    pub fn is_draw(&self) -> bool {
        self.home_goals == self.away_goals
    }

    /// Get result string.
    pub fn result_string(&self) -> String {
        format!("{} - {}", self.home_goals, self.away_goals)
    }
}
