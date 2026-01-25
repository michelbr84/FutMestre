//! Competition system - manages fixtures, tables, and results.

use crate::config::GameConfig;
use crate::state::GameState;
use cm_core::world::{World, Fixture, Table, TableRow};
use cm_core::ids::{ClubId, CompetitionId};
use chrono::NaiveDate;

/// Competition system.
pub struct CompetitionSystem;

impl CompetitionSystem {
    /// Run daily competition logic.
    pub fn run_daily(&self, _cfg: &GameConfig, world: &mut World, state: &mut GameState) {
        let today = state.date.date();
        
        // Check for fixtures today
        let todays_fixtures = self.get_fixtures_for_date(world, today);
        
        if !todays_fixtures.is_empty() {
            state.flags.match_day = true;
            state.add_message(format!(
                "{} fixture(s) scheduled for today",
                todays_fixtures.len()
            ));
        }
    }

    /// Get fixtures for a specific date.
    pub fn get_fixtures_for_date<'a>(&self, world: &'a World, date: NaiveDate) -> Vec<&'a Fixture> {
        world.competitions.values()
            .flat_map(|comp| &comp.fixtures.matches)
            .filter(|f| f.date == date && !f.is_played())
            .collect()
    }

    /// Generate league fixtures for a competition.
    pub fn generate_league_fixtures(
        &self,
        competition_id: &CompetitionId,
        clubs: &[ClubId],
        start_date: NaiveDate,
    ) -> Vec<Fixture> {
        let mut fixtures = Vec::new();
        let num_clubs = clubs.len();
        
        if num_clubs < 2 {
            return fixtures;
        }

        // Generate round-robin schedule
        let mut round: u8 = 1;
        let mut current_date = start_date;
        
        // First half of season (home matches)
        for i in 0..num_clubs {
            for j in (i + 1)..num_clubs {
                fixtures.push(Fixture::new(
                    competition_id.clone(),
                    round,
                    current_date,
                    clubs[i].clone(),
                    clubs[j].clone(),
                ));
                
                // Next weekend
                if fixtures.len() % (num_clubs / 2) == 0 {
                    round = round.saturating_add(1);
                    current_date = current_date + chrono::Duration::days(7);
                }
            }
        }

        // Second half of season (reversed home/away)
        let midpoint = fixtures.len();
        for i in 0..midpoint {
            let original = &fixtures[i];
            fixtures.push(Fixture::new(
                competition_id.clone(),
                round,
                current_date,
                original.away_id.clone(),
                original.home_id.clone(),
            ));
            
            if (fixtures.len() - midpoint) % (num_clubs / 2) == 0 {
                round = round.saturating_add(1);
                current_date = current_date + chrono::Duration::days(7);
            }
        }

        fixtures
    }

    /// Update table after a match result.
    pub fn update_table_result(
        &self,
        table: &mut Table,
        home_club: &ClubId,
        away_club: &ClubId,
        home_goals: u8,
        away_goals: u8,
    ) {
        // Ensure teams are in the table
        table.add_team(home_club.clone());
        table.add_team(away_club.clone());
        
        // Record result (3 for win, 1 for draw)
        table.record_result(home_club, away_club, home_goals, away_goals, 3, 1);
    }

    /// Get sorted table standings.
    pub fn get_standings<'a>(&self, table: &'a Table) -> Vec<&'a TableRow> {
        let mut standings: Vec<_> = table.rows.iter().collect();
        
        // Sort by: points, goal difference, goals for
        standings.sort_by(|a, b| {
            b.points.cmp(&a.points)
                .then_with(|| b.goal_difference().cmp(&a.goal_difference()))
                .then_with(|| b.goals_for.cmp(&a.goals_for))
        });

        standings
    }

    /// Get club's position in table (1-indexed).
    pub fn get_position(&self, table: &Table, club_id: &ClubId) -> Option<usize> {
        table.position(club_id)
    }

    /// Check for user club's upcoming fixture.
    pub fn get_next_fixture<'a>(&self, world: &'a World, club_id: &ClubId, after_date: NaiveDate) -> Option<&'a Fixture> {
        world.competitions.values()
            .flat_map(|comp| &comp.fixtures.matches)
            .filter(|f| {
                !f.is_played() && 
                f.date > after_date &&
                (f.home_id == *club_id || f.away_id == *club_id)
            })
            .min_by_key(|f| f.date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 8, 1).unwrap()
    }

    #[test]
    fn test_generate_league_fixtures() {
        let system = CompetitionSystem;
        let comp_id = CompetitionId::new("PL");
        let clubs = vec![
            ClubId::new("LIV"),
            ClubId::new("MAN"),
            ClubId::new("CHE"),
            ClubId::new("ARS"),
        ];
        
        let fixtures = system.generate_league_fixtures(&comp_id, &clubs, test_date());
        
        // 4 teams = 6 matches per half = 12 total
        assert_eq!(fixtures.len(), 12);
    }

    #[test]
    fn test_generate_fixtures_too_few_clubs() {
        let system = CompetitionSystem;
        let comp_id = CompetitionId::new("PL");
        let clubs = vec![ClubId::new("LIV")];
        
        let fixtures = system.generate_league_fixtures(&comp_id, &clubs, test_date());
        assert!(fixtures.is_empty());
    }

    #[test]
    fn test_update_table_home_win() {
        let system = CompetitionSystem;
        let mut table = Table::new();
        
        let home = ClubId::new("LIV");
        let away = ClubId::new("MAN");
        
        system.update_table_result(&mut table, &home, &away, 3, 1);
        
        let home_row = table.get_team(&home).unwrap();
        assert_eq!(home_row.won, 1);
        assert_eq!(home_row.points, 3);
        assert_eq!(home_row.goals_for, 3);
        
        let away_row = table.get_team(&away).unwrap();
        assert_eq!(away_row.lost, 1);
        assert_eq!(away_row.points, 0);
    }

    #[test]
    fn test_update_table_draw() {
        let system = CompetitionSystem;
        let mut table = Table::new();
        
        let home = ClubId::new("LIV");
        let away = ClubId::new("MAN");
        
        system.update_table_result(&mut table, &home, &away, 2, 2);
        
        let home_row = table.get_team(&home).unwrap();
        assert_eq!(home_row.drawn, 1);
        assert_eq!(home_row.points, 1);
        
        let away_row = table.get_team(&away).unwrap();
        assert_eq!(away_row.drawn, 1);
        assert_eq!(away_row.points, 1);
    }

    #[test]
    fn test_get_standings_sorted() {
        let system = CompetitionSystem;
        let mut table = Table::new();
        
        let liv = ClubId::new("LIV");
        let man = ClubId::new("MAN");
        let che = ClubId::new("CHE");
        
        // LIV wins twice
        system.update_table_result(&mut table, &liv, &man, 2, 0);
        system.update_table_result(&mut table, &liv, &che, 3, 1);
        
        // CHE wins once
        system.update_table_result(&mut table, &che, &man, 1, 0);
        
        let standings = system.get_standings(&table);
        
        assert_eq!(standings[0].club_id, liv);
        assert_eq!(standings[0].points, 6);
        
        assert_eq!(standings[1].club_id, che);
        assert_eq!(standings[2].club_id, man);
    }

    #[test]
    fn test_get_position() {
        let system = CompetitionSystem;
        let mut table = Table::new();
        
        let liv = ClubId::new("LIV");
        let man = ClubId::new("MAN");
        
        system.update_table_result(&mut table, &liv, &man, 2, 0);
        
        assert_eq!(system.get_position(&table, &liv), Some(1));
        assert_eq!(system.get_position(&table, &man), Some(2));
    }
}
