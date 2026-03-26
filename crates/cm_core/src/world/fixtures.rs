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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_fixtures() -> Fixtures {
        let mut fixtures = Fixtures::new();
        let comp = CompetitionId::new("BRA1");
        let date1 = NaiveDate::from_ymd_opt(2024, 4, 13).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 4, 20).unwrap();

        fixtures.add(Fixture::new(
            comp.clone(),
            1,
            date1,
            ClubId::new("FLA"),
            ClubId::new("PAL"),
        ));
        fixtures.add(Fixture::new(
            comp.clone(),
            1,
            date1,
            ClubId::new("SAO"),
            ClubId::new("COR"),
        ));
        fixtures.add(Fixture::new(
            comp.clone(),
            2,
            date2,
            ClubId::new("PAL"),
            ClubId::new("SAO"),
        ));
        fixtures.add(Fixture::new(
            comp.clone(),
            2,
            date2,
            ClubId::new("COR"),
            ClubId::new("FLA"),
        ));

        fixtures
    }

    #[test]
    fn test_fixtures_generation() {
        let fixtures = make_fixtures();
        assert_eq!(fixtures.matches.len(), 4);
    }

    #[test]
    fn test_fixtures_on_date() {
        let fixtures = make_fixtures();
        let date1 = NaiveDate::from_ymd_opt(2024, 4, 13).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 4, 20).unwrap();
        let date_none = NaiveDate::from_ymd_opt(2024, 5, 1).unwrap();

        assert_eq!(fixtures.on_date(date1).len(), 2);
        assert_eq!(fixtures.on_date(date2).len(), 2);
        assert_eq!(fixtures.on_date(date_none).len(), 0);
    }

    #[test]
    fn test_fixtures_for_team() {
        let fixtures = make_fixtures();
        let fla = ClubId::new("FLA");

        let fla_fixtures = fixtures.for_team(&fla);
        // FLA plays home in round 1 and away in round 2
        assert_eq!(fla_fixtures.len(), 2);
    }

    #[test]
    fn test_fixtures_upcoming() {
        let mut fixtures = make_fixtures();

        // All should be upcoming initially
        assert_eq!(fixtures.upcoming().len(), 4);

        // Play one match
        fixtures.matches[0].set_result(2, 1, 50000);
        assert_eq!(fixtures.upcoming().len(), 3);
    }

    #[test]
    fn test_fixture_set_result() {
        let comp = CompetitionId::new("BRA1");
        let date = NaiveDate::from_ymd_opt(2024, 4, 13).unwrap();
        let mut fixture = Fixture::new(comp, 1, date, ClubId::new("FLA"), ClubId::new("PAL"));

        assert!(!fixture.is_played());
        fixture.set_result(3, 1, 60000);
        assert!(fixture.is_played());

        let result = fixture.result.unwrap();
        assert!(result.is_home_win());
        assert!(!result.is_away_win());
        assert!(!result.is_draw());
        assert_eq!(result.home_goals, 3);
        assert_eq!(result.away_goals, 1);
        assert_eq!(result.attendance, 60000);
    }

    #[test]
    fn test_fixtures_next_for_team() {
        let mut fixtures = make_fixtures();
        let fla = ClubId::new("FLA");

        // First unplayed fixture for FLA
        let next = fixtures.next_for_team(&fla);
        assert!(next.is_some());

        // Play FLA's first game
        fixtures.matches[0].set_result(1, 0, 50000);

        // Next fixture should be the round 2 game
        let next = fixtures.next_for_team(&fla);
        assert!(next.is_some());
        assert_eq!(next.unwrap().round, 2);
    }

    #[test]
    fn test_match_result_draw() {
        let result = MatchResult::new(1, 1, 45000);
        assert!(result.is_draw());
        assert!(!result.is_home_win());
        assert!(!result.is_away_win());
    }

    #[test]
    fn test_fixture_id_format() {
        let comp = CompetitionId::new("BRA1");
        let date = NaiveDate::from_ymd_opt(2024, 4, 13).unwrap();
        let fixture = Fixture::new(comp, 1, date, ClubId::new("FLA"), ClubId::new("PAL"));

        assert!(fixture.id.as_str().contains("BRA1"));
        assert!(fixture.id.as_str().contains("FLA"));
        assert!(fixture.id.as_str().contains("PAL"));
    }
}
