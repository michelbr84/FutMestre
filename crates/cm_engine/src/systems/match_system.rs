//! Match system - runs actual match simulations using cm_match engine.

use rand::Rng;

use crate::config::GameConfig;
use crate::state::GameState;
use cm_core::ids::{ClubId, CompetitionId};
use cm_core::world::World;
use cm_match::{MatchInput, TeamStrength, simulate_match};

/// Match system.
pub struct MatchSystem;

impl MatchSystem {
    /// Run match day - simulate all matches scheduled for today.
    pub fn run_match_day(&self, _cfg: &GameConfig, world: &mut World, state: &mut GameState) {
        let today = state.date.date();
        let mut rng = rand::thread_rng();

        // Collect fixtures to simulate
        let mut to_simulate: Vec<(CompetitionId, usize, ClubId, ClubId)> = Vec::new();

        for (comp_id, comp) in &world.competitions {
            for (idx, fixture) in comp.fixtures.matches.iter().enumerate() {
                if fixture.date == today && !fixture.is_played() {
                    to_simulate.push((
                        comp_id.clone(),
                        idx,
                        fixture.home_id.clone(),
                        fixture.away_id.clone(),
                    ));
                }
            }
        }

        if to_simulate.is_empty() {
            state.flags.match_day = false;
            return;
        }

        // Simulate each match
        for (comp_id, fix_idx, home_id, away_id) in &to_simulate {
            let home_strength = world.clubs.get(home_id)
                .map(|c| TeamStrength::from_club(c))
                .unwrap_or_default();
            let away_strength = world.clubs.get(away_id)
                .map(|c| TeamStrength::from_club(c))
                .unwrap_or_default();

            let input = MatchInput {
                home_id: home_id.clone(),
                away_id: away_id.clone(),
                home: home_strength,
                away: away_strength,
                minutes: 90,
                seed: Some(rng.gen()),
            };

            let result = simulate_match(&input);

            let home_name = world.clubs.get(home_id)
                .map(|c| c.short_name.clone())
                .unwrap_or_else(|| home_id.to_string());
            let away_name = world.clubs.get(away_id)
                .map(|c| c.short_name.clone())
                .unwrap_or_else(|| away_id.to_string());

            // Update fixture result
            if let Some(comp) = world.competitions.get_mut(comp_id) {
                if let Some(fixture) = comp.fixtures.matches.get_mut(*fix_idx) {
                    fixture.set_result(result.home_goals, result.away_goals, 0);
                }

                // Update league table
                if comp.is_league() {
                    comp.table.record_result(
                        home_id,
                        away_id,
                        result.home_goals,
                        result.away_goals,
                        3, // win points
                        1, // draw points
                    );
                }
            }

            // Generate match report for user's team
            if home_id == &state.club_id || away_id == &state.club_id {
                let msg = format!(
                    "Resultado: {} {} x {} {} ({})",
                    home_name, result.home_goals,
                    result.away_goals, away_name,
                    comp_id
                );
                state.add_message(msg);

                // Add key events to inbox
                for event in &result.events {
                    if !event.description.is_empty() {
                        state.add_message(format!("  {}' - {}", event.minute, event.description));
                    }
                }
            }
        }

        let user_played = to_simulate.iter()
            .any(|(_, _, h, a)| h == &state.club_id || a == &state.club_id);
        if !user_played {
            state.add_message(format!("{} jogo(s) simulado(s) hoje.", to_simulate.len()));
        }

        state.flags.match_day = false;
    }
}
