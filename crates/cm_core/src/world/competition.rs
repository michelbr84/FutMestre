//! Competition entity.

use serde::{Deserialize, Serialize};
use crate::ids::{CompetitionId, NationId, ClubId};
use super::{Fixtures, Table};

/// Competition type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompetitionType {
    League,
    Cup,
    SuperCup,
    International,
}

impl Default for CompetitionType {
    fn default() -> Self {
        Self::League
    }
}

/// Division level in the league pyramid.
/// Serie A is the top division (level 1), Serie D is the lowest (level 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DivisionLevel {
    /// Top division (level 1).
    SerieA = 1,
    /// Second division (level 2).
    SerieB = 2,
    /// Third division (level 3).
    SerieC = 3,
    /// Fourth division (level 4).
    SerieD = 4,
}

impl DivisionLevel {
    /// Get the numeric level (1-4).
    pub fn level(&self) -> u8 {
        *self as u8
    }

    /// Check if this is the top division.
    pub fn is_top(&self) -> bool {
        *self == DivisionLevel::SerieA
    }

    /// Check if this is the bottom division.
    pub fn is_bottom(&self) -> bool {
        *self == DivisionLevel::SerieD
    }

    /// Get the division above (for promotion). Returns None for Serie A.
    pub fn division_above(&self) -> Option<DivisionLevel> {
        match self {
            DivisionLevel::SerieA => None,
            DivisionLevel::SerieB => Some(DivisionLevel::SerieA),
            DivisionLevel::SerieC => Some(DivisionLevel::SerieB),
            DivisionLevel::SerieD => Some(DivisionLevel::SerieC),
        }
    }

    /// Get the division below (for relegation). Returns None for Serie D.
    pub fn division_below(&self) -> Option<DivisionLevel> {
        match self {
            DivisionLevel::SerieA => Some(DivisionLevel::SerieB),
            DivisionLevel::SerieB => Some(DivisionLevel::SerieC),
            DivisionLevel::SerieC => Some(DivisionLevel::SerieD),
            DivisionLevel::SerieD => None,
        }
    }

    /// Get the display name for this division.
    pub fn name(&self) -> &'static str {
        match self {
            DivisionLevel::SerieA => "Série A",
            DivisionLevel::SerieB => "Série B",
            DivisionLevel::SerieC => "Série C",
            DivisionLevel::SerieD => "Série D",
        }
    }

    /// Get all division levels in order from top to bottom.
    pub fn all() -> &'static [DivisionLevel] {
        &[
            DivisionLevel::SerieA,
            DivisionLevel::SerieB,
            DivisionLevel::SerieC,
            DivisionLevel::SerieD,
        ]
    }

    /// Minimum number of teams allowed in a division.
    pub fn min_teams() -> usize {
        12
    }

    /// Maximum number of teams allowed in a division.
    pub fn max_teams() -> usize {
        20
    }
}

/// A football competition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competition {
    pub id: CompetitionId,
    pub name: String,
    pub short_name: String,
    pub nation_id: Option<NationId>,
    pub competition_type: CompetitionType,
    pub division_level: Option<DivisionLevel>,
    pub reputation: u8,
    pub teams: Vec<ClubId>,
    pub fixtures: Fixtures,
    pub table: Table,
    pub current_round: u8,
    pub total_rounds: u8,
}

impl Competition {
    /// Create a new competition.
    pub fn new(
        id: impl Into<CompetitionId>,
        name: impl Into<String>,
        competition_type: CompetitionType,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            short_name: String::new(),
            nation_id: None,
            competition_type,
            division_level: None,
            reputation: 50,
            teams: Vec::new(),
            fixtures: Fixtures::new(),
            table: Table::new(),
            current_round: 0,
            total_rounds: 0,
        }
    }

    /// Create a new league competition with a division level.
    pub fn new_league(
        id: impl Into<CompetitionId>,
        name: impl Into<String>,
        division_level: DivisionLevel,
    ) -> Self {
        let mut comp = Self::new(id, name, CompetitionType::League);
        comp.division_level = Some(division_level);
        comp
    }

    /// Check if league.
    pub fn is_league(&self) -> bool {
        self.competition_type == CompetitionType::League
    }

    /// Check if cup.
    pub fn is_cup(&self) -> bool {
        self.competition_type == CompetitionType::Cup
    }

    /// Add a team.
    pub fn add_team(&mut self, club_id: ClubId) {
        if !self.teams.contains(&club_id) {
            self.teams.push(club_id.clone());
            self.table.add_team(club_id);
        }
    }

    /// Get number of teams.
    pub fn team_count(&self) -> usize {
        self.teams.len()
    }
}
