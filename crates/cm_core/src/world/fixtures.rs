//! Fixtures and matches.

use crate::ids::{ClubId, CompetitionId, MatchId, StadiumId};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A scheduled match.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fixture {
    pub id: MatchId,
    pub competition_id: CompetitionId,
    pub round: u8,
    pub date: NaiveDate,
    pub home_id: ClubId,
    pub away_id: ClubId,
    pub stadium_id: Option<StadiumId>,
    pub result: Option<MatchResult>,
}

/// Match result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub home_goals: u8,
    pub away_goals: u8,
    pub attendance: u32,
    pub played: bool,
}

impl MatchResult {
    /// Create a new result.
    pub fn new(home_goals: u8, away_goals: u8, attendance: u32) -> Self {
        Self {
            home_goals,
            away_goals,
            attendance,
            played: true,
        }
    }

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
}

impl Fixture {
    /// Create a new fixture.
    pub fn new(
        competition_id: CompetitionId,
        round: u8,
        date: NaiveDate,
        home_id: ClubId,
        away_id: ClubId,
    ) -> Self {
        let id = format!("M-{}-{}-{}-{}", competition_id, round, home_id, away_id);
        Self {
            id: MatchId::new(id),
            competition_id,
            round,
            date,
            home_id,
            away_id,
            stadium_id: None,
            result: None,
        }
    }

    /// Check if match has been played.
    pub fn is_played(&self) -> bool {
        self.result.is_some()
    }

    /// Set result.
    pub fn set_result(&mut self, home_goals: u8, away_goals: u8, attendance: u32) {
        self.result = Some(MatchResult::new(home_goals, away_goals, attendance));
    }
}

/// Collection of fixtures.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Fixtures {
    pub matches: Vec<Fixture>,
}

impl Fixtures {
    /// Create empty fixtures.
    pub fn new() -> Self {
        Self {
            matches: Vec::new(),
        }
    }

    /// Add a fixture.
    pub fn add(&mut self, fixture: Fixture) {
        self.matches.push(fixture);
    }

    /// Get fixtures for a date.
    pub fn on_date(&self, date: NaiveDate) -> Vec<&Fixture> {
        self.matches.iter().filter(|f| f.date == date).collect()
    }

    /// Get fixtures for a team.
    pub fn for_team(&self, club_id: &ClubId) -> Vec<&Fixture> {
        self.matches
            .iter()
            .filter(|f| &f.home_id == club_id || &f.away_id == club_id)
            .collect()
    }

    /// Get upcoming unplayed fixtures.
    pub fn upcoming(&self) -> Vec<&Fixture> {
        self.matches.iter().filter(|f| !f.is_played()).collect()
    }

    /// Get next fixture for a team.
    pub fn next_for_team(&self, club_id: &ClubId) -> Option<&Fixture> {
        self.for_team(club_id).into_iter().find(|f| !f.is_played())
    }
}
