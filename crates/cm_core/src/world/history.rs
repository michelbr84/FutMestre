//! Historical records.

use crate::ids::{ClubId, CompetitionId};
use serde::{Deserialize, Serialize};

/// Season record for a club.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonRecord {
    pub season: String,
    pub competition_id: CompetitionId,
    pub position: u8,
    pub played: u16,
    pub won: u16,
    pub drawn: u16,
    pub lost: u16,
    pub goals_for: u16,
    pub goals_against: u16,
    pub points: u16,
}

/// Club history.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClubHistory {
    pub seasons: Vec<SeasonRecord>,
    pub league_titles: u16,
    pub cup_wins: u16,
    pub european_wins: u16,
}

impl ClubHistory {
    /// Create empty history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a season record.
    pub fn add_season(&mut self, record: SeasonRecord) {
        self.seasons.push(record);
    }
}

/// Player history entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSeasonStats {
    pub season: String,
    pub club_id: ClubId,
    pub appearances: u16,
    pub goals: u16,
    pub assists: u16,
    pub yellow_cards: u16,
    pub red_cards: u16,
    pub average_rating: f32,
}

/// Player career history.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayerHistory {
    pub seasons: Vec<PlayerSeasonStats>,
}

impl PlayerHistory {
    /// Create empty history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a season.
    pub fn add_season(&mut self, stats: PlayerSeasonStats) {
        self.seasons.push(stats);
    }

    /// Total career goals.
    pub fn total_goals(&self) -> u16 {
        self.seasons.iter().map(|s| s.goals).sum()
    }

    /// Total career appearances.
    pub fn total_appearances(&self) -> u16 {
        self.seasons.iter().map(|s| s.appearances).sum()
    }
}
