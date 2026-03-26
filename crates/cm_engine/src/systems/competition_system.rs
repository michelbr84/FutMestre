//! Competition system - manages fixtures, tables, and results.

use crate::config::GameConfig;
use crate::state::GameState;
use chrono::NaiveDate;
use cm_core::ids::{ClubId, CompetitionId};
use cm_core::world::{
    Competition, CompetitionType, DivisionLevel, Fixture, Fixtures, Table, TableRow, World,
};

/// Number of teams promoted/relegated between divisions.
const PROMOTION_RELEGATION_SLOTS: usize = 3;

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
                "{} jogo(s) agendado(s) para hoje",
                todays_fixtures.len()
            ));
        }
    }

    /// Get fixtures for a specific date.
    pub fn get_fixtures_for_date<'a>(&self, world: &'a World, date: NaiveDate) -> Vec<&'a Fixture> {
        world
            .competitions
            .values()
            .flat_map(|comp| &comp.fixtures.matches)
            .filter(|f| f.date == date && !f.is_played())
            .collect()
    }

    /// Generate league fixtures using proper round-robin (circle method).
    ///
    /// For N teams (even), generates (N-1) rounds in the first half,
    /// each with exactly N/2 matches. Then mirrors for the second half
    /// with home/away swapped. Each round is 7 days apart.
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

        // Ensure even number of teams (add dummy if needed, then filter)
        let mut teams: Vec<ClubId> = clubs.to_vec();
        let has_dummy = num_clubs % 2 != 0;
        if has_dummy {
            teams.push(ClubId::new("__BYE__"));
        }
        let n = teams.len();
        let total_rounds = n - 1;
        let matches_per_round = n / 2;

        // Circle method: fix teams[0], rotate the rest
        let mut rotating: Vec<ClubId> = teams[1..].to_vec();

        let mut round: u8 = 1;
        let mut current_date = start_date;

        // First half of season
        for _ in 0..total_rounds {
            for m in 0..matches_per_round {
                let home = if m == 0 {
                    teams[0].clone()
                } else {
                    rotating[m - 1].clone()
                };
                let away = rotating[rotating.len() - 1 - m].clone();

                // Skip bye matches
                if home.to_string() == "__BYE__" || away.to_string() == "__BYE__" {
                    continue;
                }

                fixtures.push(Fixture::new(
                    competition_id.clone(),
                    round,
                    current_date,
                    home,
                    away,
                ));
            }

            // Rotate: last element goes to front
            rotating.rotate_right(1);
            round = round.saturating_add(1);
            current_date = current_date + chrono::Duration::days(7);
        }

        // Second half of season (home/away swapped)
        let first_half: Vec<(u8, NaiveDate, ClubId, ClubId)> = fixtures
            .iter()
            .map(|f| (f.round, f.date, f.home_id.clone(), f.away_id.clone()))
            .collect();

        for (_, _, home, away) in &first_half {
            fixtures.push(Fixture::new(
                competition_id.clone(),
                round,
                current_date,
                away.clone(),
                home.clone(),
            ));

            // Count fixtures added in this round
            let added_this_round = fixtures.len() - first_half.len();
            if added_this_round > 0 && added_this_round % matches_per_round == 0 {
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
            b.points
                .cmp(&a.points)
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
    pub fn get_next_fixture<'a>(
        &self,
        world: &'a World,
        club_id: &ClubId,
        after_date: NaiveDate,
    ) -> Option<&'a Fixture> {
        world
            .competitions
            .values()
            .flat_map(|comp| &comp.fixtures.matches)
            .filter(|f| {
                !f.is_played()
                    && f.date > after_date
                    && (f.home_id == *club_id || f.away_id == *club_id)
            })
            .min_by_key(|f| f.date)
    }

    /// Check if all fixtures in a competition have been played.
    pub fn is_season_complete(competition: &Competition) -> bool {
        if competition.fixtures.matches.is_empty() {
            return false;
        }
        competition.fixtures.matches.iter().all(|f| f.is_played())
    }

    /// Get competition IDs for all completed league seasons.
    pub fn get_completed_leagues(world: &World) -> Vec<CompetitionId> {
        world
            .competitions
            .values()
            .filter(|c| c.is_league() && Self::is_season_complete(c))
            .map(|c| c.id.clone())
            .collect()
    }
}

// === Free functions for season management ===

/// Detect and process end of season for all divisions.
/// Returns the IDs of competitions whose seasons just ended.
pub fn process_end_of_season(world: &mut World) -> Vec<CompetitionId> {
    let completed: Vec<CompetitionId> = world
        .competitions
        .values()
        .filter(|c| c.is_league() && c.division_level.is_some())
        .filter(|c| CompetitionSystem::is_season_complete(c))
        .map(|c| c.id.clone())
        .collect();

    completed
}

/// Apply promotion and relegation between divisions.
///
/// Top 3 of each division promote (except Serie A, the top division).
/// Bottom 3 of each division relegate (except Serie D, the bottom division).
///
/// Returns a list of (ClubId, from_division, to_division) moves applied.
pub fn apply_promotion_relegation(
    world: &mut World,
) -> Vec<(ClubId, DivisionLevel, DivisionLevel)> {
    // First, collect the sorted standings for each division
    let system = CompetitionSystem;
    let mut division_standings: Vec<(DivisionLevel, CompetitionId, Vec<ClubId>)> = Vec::new();

    for comp in world.competitions.values() {
        if let Some(div_level) = comp.division_level {
            if comp.is_league() {
                let standings = system.get_standings(&comp.table);
                let ordered_clubs: Vec<ClubId> =
                    standings.iter().map(|row| row.club_id.clone()).collect();
                division_standings.push((div_level, comp.id.clone(), ordered_clubs));
            }
        }
    }

    // Sort by division level so we process in order
    division_standings.sort_by_key(|(level, _, _)| *level);

    // Determine promotions and relegations
    let mut moves: Vec<(ClubId, DivisionLevel, DivisionLevel)> = Vec::new();

    for (div_level, _comp_id, standings) in &division_standings {
        let num_teams = standings.len();
        if num_teams == 0 {
            continue;
        }

        // Promotion: top 3 go up (not from Serie A)
        if let Some(above) = div_level.division_above() {
            let promote_count = PROMOTION_RELEGATION_SLOTS.min(num_teams);
            for i in 0..promote_count {
                moves.push((standings[i].clone(), *div_level, above));
            }
        }

        // Relegation: bottom 3 go down (not from Serie D)
        if let Some(below) = div_level.division_below() {
            let relegate_count = PROMOTION_RELEGATION_SLOTS.min(num_teams);
            for i in 0..relegate_count {
                let idx = num_teams - 1 - i;
                moves.push((standings[idx].clone(), *div_level, below));
            }
        }
    }

    // Now apply the moves: remove clubs from old competitions, add to new ones
    // Build a map from division level to competition id
    let div_to_comp: std::collections::HashMap<DivisionLevel, CompetitionId> = world
        .competitions
        .values()
        .filter(|c| c.is_league() && c.division_level.is_some())
        .map(|c| (c.division_level.unwrap(), c.id.clone()))
        .collect();

    for (club_id, from_div, to_div) in &moves {
        // Remove from old competition
        if let Some(from_comp_id) = div_to_comp.get(from_div) {
            if let Some(comp) = world.competitions.get_mut(from_comp_id) {
                comp.teams.retain(|id| id != club_id);
            }
        }
        // Add to new competition
        if let Some(to_comp_id) = div_to_comp.get(to_div) {
            if let Some(comp) = world.competitions.get_mut(to_comp_id) {
                if !comp.teams.contains(club_id) {
                    comp.teams.push(club_id.clone());
                }
            }
        }
    }

    moves
}

/// Generate new season fixtures for all league divisions and reset tables.
pub fn generate_new_season(world: &mut World, start_date: NaiveDate) {
    let system = CompetitionSystem;

    // Collect competition IDs and their team lists
    let league_info: Vec<(CompetitionId, Vec<ClubId>)> = world
        .competitions
        .values()
        .filter(|c| c.is_league() && c.division_level.is_some())
        .map(|c| (c.id.clone(), c.teams.clone()))
        .collect();

    for (comp_id, clubs) in league_info {
        // Generate new fixtures
        let fixtures = system.generate_league_fixtures(&comp_id, &clubs, start_date);

        if let Some(comp) = world.competitions.get_mut(&comp_id) {
            // Reset table
            comp.table = Table::new();
            for club_id in &comp.teams {
                comp.table.add_team(club_id.clone());
            }

            // Set new fixtures
            comp.fixtures = Fixtures::new();
            for fixture in fixtures {
                comp.fixtures.add(fixture);
            }

            // Reset round counters
            comp.current_round = 0;
            if !comp.fixtures.matches.is_empty() {
                comp.total_rounds = comp
                    .fixtures
                    .matches
                    .iter()
                    .map(|f| f.round)
                    .max()
                    .unwrap_or(0);
            }
        }
    }
}

/// Generate a knockout cup draw with all Serie A and Serie B teams.
/// Returns the CompetitionId of the created cup.
pub fn generate_cup_draw(
    world: &mut World,
    cup_name: &str,
    start_date: NaiveDate,
) -> CompetitionId {
    // Collect all Serie A and Serie B teams
    let mut cup_teams: Vec<ClubId> = Vec::new();

    for comp in world.competitions.values() {
        if let Some(div) = comp.division_level {
            if div == DivisionLevel::SerieA || div == DivisionLevel::SerieB {
                cup_teams.extend(comp.teams.iter().cloned());
            }
        }
    }

    let cup_id = CompetitionId::new(format!("CUP-{}", cup_name.replace(' ', "-")));

    // Create the cup competition
    let mut cup = Competition::new(cup_id.clone(), cup_name, CompetitionType::Cup);
    for team in &cup_teams {
        cup.teams.push(team.clone());
    }

    // Generate knockout fixtures
    // Pair teams for first round; if odd number, last team gets a bye (not paired)
    let num_teams = cup_teams.len();
    let round: u8 = 1;
    let mut fixtures = Vec::new();

    // Simple knockout: pair teams sequentially
    let pairs = num_teams / 2;
    for i in 0..pairs {
        let home = cup_teams[i * 2].clone();
        let away = cup_teams[i * 2 + 1].clone();
        fixtures.push(Fixture::new(cup_id.clone(), round, start_date, home, away));
    }

    cup.total_rounds = calculate_knockout_rounds(num_teams);
    cup.current_round = 1;

    for fixture in fixtures {
        cup.fixtures.add(fixture);
    }

    world.competitions.insert(cup_id.clone(), cup);

    cup_id
}

/// Calculate how many rounds are needed for a knockout tournament.
fn calculate_knockout_rounds(num_teams: usize) -> u8 {
    if num_teams <= 1 {
        return 0;
    }
    let mut rounds = 0u8;
    let mut remaining = num_teams;
    while remaining > 1 {
        remaining = (remaining + 1) / 2; // ceiling division for byes
        rounds += 1;
    }
    rounds
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 8, 1).unwrap()
    }

    /// Helper: create a world with 4 divisions, each having `teams_per_div` teams.
    fn create_world_with_divisions(teams_per_div: usize) -> World {
        let mut world = World::new();
        let divisions = [
            ("SERIE-A", "Série A", DivisionLevel::SerieA),
            ("SERIE-B", "Série B", DivisionLevel::SerieB),
            ("SERIE-C", "Série C", DivisionLevel::SerieC),
            ("SERIE-D", "Série D", DivisionLevel::SerieD),
        ];

        for (id, name, level) in &divisions {
            let mut comp = Competition::new_league(*id, *name, *level);
            for i in 0..teams_per_div {
                let club_id = ClubId::new(format!("{}-T{:02}", id, i + 1));
                comp.add_team(club_id);
            }
            world.competitions.insert(comp.id.clone(), comp);
        }

        world
    }

    /// Helper: generate fixtures and mark all as played with deterministic results.
    /// Teams are ordered so that lower-index teams win more (team 0 beats everyone, etc.)
    fn simulate_full_season(world: &mut World) {
        let system = CompetitionSystem;
        let start = test_date();

        // Collect info first to avoid borrow issues
        let comp_info: Vec<(CompetitionId, Vec<ClubId>)> = world
            .competitions
            .values()
            .filter(|c| c.is_league() && c.division_level.is_some())
            .map(|c| (c.id.clone(), c.teams.clone()))
            .collect();

        for (comp_id, clubs) in comp_info {
            let fixtures = system.generate_league_fixtures(&comp_id, &clubs, start);

            if let Some(comp) = world.competitions.get_mut(&comp_id) {
                comp.fixtures = Fixtures::new();
                for mut fixture in fixtures {
                    // Home team always wins 2-1 for deterministic standings
                    fixture.set_result(2, 1, 1000);

                    // Update table
                    comp.table
                        .record_result(&fixture.home_id, &fixture.away_id, 2, 1, 3, 1);

                    comp.fixtures.add(fixture);
                }
            }
        }
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

    // === New tests for division system ===

    #[test]
    fn test_division_level_basics() {
        assert_eq!(DivisionLevel::SerieA.level(), 1);
        assert_eq!(DivisionLevel::SerieD.level(), 4);
        assert!(DivisionLevel::SerieA.is_top());
        assert!(!DivisionLevel::SerieA.is_bottom());
        assert!(DivisionLevel::SerieD.is_bottom());
        assert!(!DivisionLevel::SerieD.is_top());

        assert_eq!(
            DivisionLevel::SerieB.division_above(),
            Some(DivisionLevel::SerieA)
        );
        assert_eq!(DivisionLevel::SerieA.division_above(), None);
        assert_eq!(
            DivisionLevel::SerieA.division_below(),
            Some(DivisionLevel::SerieB)
        );
        assert_eq!(DivisionLevel::SerieD.division_below(), None);
    }

    #[test]
    fn test_season_end_detection_not_complete() {
        let mut world = create_world_with_divisions(4);
        // No fixtures generated yet -- season cannot be complete
        let completed = process_end_of_season(&mut world);
        assert!(
            completed.is_empty(),
            "No fixtures means season is not complete"
        );
    }

    #[test]
    fn test_season_end_detection_complete() {
        let mut world = create_world_with_divisions(4);
        simulate_full_season(&mut world);

        let completed = process_end_of_season(&mut world);
        assert_eq!(completed.len(), 4, "All 4 divisions should be complete");
    }

    #[test]
    fn test_season_end_detection_partial() {
        let mut world = create_world_with_divisions(4);
        let system = CompetitionSystem;
        let start = test_date();

        // Only generate and play fixtures for Serie A
        let serie_a_id = CompetitionId::new("SERIE-A");
        let clubs: Vec<ClubId> = world.competitions.get(&serie_a_id).unwrap().teams.clone();
        let fixtures = system.generate_league_fixtures(&serie_a_id, &clubs, start);

        if let Some(comp) = world.competitions.get_mut(&serie_a_id) {
            comp.fixtures = Fixtures::new();
            for mut fixture in fixtures {
                fixture.set_result(1, 0, 500);
                comp.fixtures.add(fixture);
            }
        }

        let completed = process_end_of_season(&mut world);
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0], serie_a_id);
    }

    #[test]
    fn test_promotion_relegation_moves() {
        let mut world = create_world_with_divisions(6);
        simulate_full_season(&mut world);

        let moves = apply_promotion_relegation(&mut world);

        // Serie A: no promotion (top), 3 relegated to B
        // Serie B: 3 promoted to A, 3 relegated to C
        // Serie C: 3 promoted to B, 3 relegated to D
        // Serie D: 3 promoted to C, no relegation (bottom)
        // Total: 3 + 3+3 + 3+3 + 3 = 18 moves
        assert_eq!(moves.len(), 18);

        // Check no promotions from Serie A
        let serie_a_promotions: Vec<_> = moves
            .iter()
            .filter(|(_, from, to)| *from == DivisionLevel::SerieA && *to < *from)
            .collect();
        assert!(
            serie_a_promotions.is_empty(),
            "No team should promote from Serie A"
        );

        // Check no relegations from Serie D
        let serie_d_relegations: Vec<_> = moves
            .iter()
            .filter(|(_, from, to)| *from == DivisionLevel::SerieD && *to > *from)
            .collect();
        assert!(
            serie_d_relegations.is_empty(),
            "No team should relegate from Serie D"
        );

        // Check that promotions go to the correct division
        for (_, from, to) in &moves {
            if to < from {
                // Promotion: to should be exactly one level above from
                assert_eq!(from.division_above(), Some(*to));
            } else {
                // Relegation: to should be exactly one level below from
                assert_eq!(from.division_below(), Some(*to));
            }
        }
    }

    #[test]
    fn test_promotion_relegation_team_counts() {
        let mut world = create_world_with_divisions(6);
        simulate_full_season(&mut world);

        // Record original team counts
        let original_counts: std::collections::HashMap<CompetitionId, usize> = world
            .competitions
            .values()
            .filter(|c| c.is_league() && c.division_level.is_some())
            .map(|c| (c.id.clone(), c.teams.len()))
            .collect();

        apply_promotion_relegation(&mut world);

        // After promotion/relegation, each division should still have the same number of teams
        // because for each team that leaves, one arrives
        for comp in world.competitions.values() {
            if comp.is_league() && comp.division_level.is_some() {
                let original = original_counts.get(&comp.id).unwrap();
                assert_eq!(
                    comp.teams.len(),
                    *original,
                    "Division {:?} should have same team count after promo/rel",
                    comp.division_level
                );
            }
        }
    }

    #[test]
    fn test_generate_new_season() {
        let mut world = create_world_with_divisions(4);
        simulate_full_season(&mut world);

        // Verify season is complete
        let completed = process_end_of_season(&mut world);
        assert_eq!(completed.len(), 4);

        // Generate new season
        let new_start = NaiveDate::from_ymd_opt(2025, 8, 1).unwrap();
        generate_new_season(&mut world, new_start);

        // All competitions should have fresh fixtures (none played)
        for comp in world.competitions.values() {
            if comp.is_league() && comp.division_level.is_some() {
                assert!(
                    !comp.fixtures.matches.is_empty(),
                    "Division {:?} should have new fixtures",
                    comp.division_level
                );
                assert!(
                    comp.fixtures.matches.iter().all(|f| !f.is_played()),
                    "All new fixtures should be unplayed"
                );
                // Table should be reset (all zeros)
                for row in &comp.table.rows {
                    assert_eq!(row.played, 0, "Table should be reset");
                    assert_eq!(row.points, 0, "Points should be reset");
                }
            }
        }
    }

    #[test]
    fn test_cup_draw_generation() {
        let mut world = create_world_with_divisions(6);

        let cup_start = NaiveDate::from_ymd_opt(2025, 3, 1).unwrap();
        let cup_id = generate_cup_draw(&mut world, "Copa FutMestre", cup_start);

        let cup = world.competitions.get(&cup_id).unwrap();

        // Should include all Serie A + Serie B teams = 6 + 6 = 12
        assert_eq!(cup.teams.len(), 12);
        assert!(cup.is_cup());

        // Should have first round fixtures: 12 teams -> 6 matches
        assert_eq!(cup.fixtures.matches.len(), 6);

        // No Serie C or Serie D teams in the cup
        let serie_c_id = CompetitionId::new("SERIE-C");
        let serie_c_teams: Vec<ClubId> = world.competitions.get(&serie_c_id).unwrap().teams.clone();
        for team in &serie_c_teams {
            assert!(
                !cup.teams.contains(team),
                "Serie C teams should not be in cup"
            );
        }
    }

    #[test]
    fn test_cup_draw_knockout_rounds() {
        assert_eq!(calculate_knockout_rounds(2), 1);
        assert_eq!(calculate_knockout_rounds(4), 2);
        assert_eq!(calculate_knockout_rounds(8), 3);
        assert_eq!(calculate_knockout_rounds(16), 4);
        assert_eq!(calculate_knockout_rounds(12), 4); // 12 -> 6 -> 3 -> 2 -> 1
        assert_eq!(calculate_knockout_rounds(1), 0);
    }

    #[test]
    fn test_cup_draw_all_fixtures_unplayed() {
        let mut world = create_world_with_divisions(6);
        let cup_start = NaiveDate::from_ymd_opt(2025, 3, 1).unwrap();
        let cup_id = generate_cup_draw(&mut world, "Test Cup", cup_start);

        let cup = world.competitions.get(&cup_id).unwrap();
        for fixture in &cup.fixtures.matches {
            assert!(!fixture.is_played());
            assert_eq!(fixture.date, cup_start);
            assert_eq!(fixture.round, 1);
        }
    }
}
