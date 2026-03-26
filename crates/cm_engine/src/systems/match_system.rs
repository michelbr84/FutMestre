//! Match system - runs actual match simulations using cm_match engine.

use rand::Rng;

use crate::config::GameConfig;
use crate::state::GameState;
use cm_core::ids::{ClubId, CompetitionId, PlayerId};
use cm_core::world::{TopScorer, World};
use cm_match::{simulate_match, MatchEventType, MatchInput, MatchResult, TeamStrength};

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
                    // Skip user's fixtures — the GUI handles those manually
                    if fixture.home_id == state.club_id || fixture.away_id == state.club_id {
                        continue;
                    }
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
            let home_strength = world
                .clubs
                .get(home_id)
                .map(|c| TeamStrength::from_club(c))
                .unwrap_or_default();
            let away_strength = world
                .clubs
                .get(away_id)
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

            let home_name = world
                .clubs
                .get(home_id)
                .map(|c| c.short_name.clone())
                .unwrap_or_else(|| home_id.to_string());
            let away_name = world
                .clubs
                .get(away_id)
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

                // Atualizar artilharia da competicao
                update_top_scorers_from_result(comp_id, &result, &mut comp.top_scorers);
            }

            // Generate match report for user's team
            if home_id == &state.club_id || away_id == &state.club_id {
                let msg = format!(
                    "Resultado: {} {} x {} {} ({})",
                    home_name, result.home_goals, result.away_goals, away_name, comp_id
                );
                state.add_message(msg);

                // Add key events to inbox
                for event in &result.events {
                    if !event.description.is_empty() {
                        state.add_message(format!("  {}' - {}", event.minute, event.description));
                    }
                }

                // Preservar resultado completo para exibicao na TUI
                state.last_match_result = Some(result.clone());
            }
        }

        // Coletar resultados de outros jogos para mensagem consolidada
        let mut other_results: Vec<String> = Vec::new();

        for (comp_id, fix_idx, home_id, away_id) in &to_simulate {
            if home_id == &state.club_id || away_id == &state.club_id {
                continue;
            }

            if let Some(comp) = world.competitions.get(comp_id) {
                if let Some(fixture) = comp.fixtures.matches.get(*fix_idx) {
                    if let Some(ref res) = fixture.result {
                        let h_name = world
                            .clubs
                            .get(home_id)
                            .map(|c| c.short_name.clone())
                            .unwrap_or_else(|| home_id.to_string());
                        let a_name = world
                            .clubs
                            .get(away_id)
                            .map(|c| c.short_name.clone())
                            .unwrap_or_else(|| away_id.to_string());
                        other_results.push(format!(
                            "{} {} x {} {}",
                            h_name, res.home_goals, res.away_goals, a_name
                        ));
                    }
                }
            }
        }

        if !other_results.is_empty() {
            let summary = format!("Resultados da rodada: {}", other_results.join(" | "));
            state.add_message(summary);
        }

        state.flags.match_day = false;
    }
}

/// Atualizar artilharia da competicao com base no resultado de uma partida.
///
/// Como o motor de partida atual usa "Jogador" generico sem PlayerId,
/// criamos entradas por clube com um ID sintetico baseado no clube.
/// Quando o motor evoluir para rastrear jogadores individuais, esta funcao
/// sera atualizada para usar os IDs reais.
fn update_top_scorers_from_result(
    _comp_id: &CompetitionId,
    result: &MatchResult,
    top_scorers: &mut Vec<TopScorer>,
) {
    // Contar gols de cada time pelos eventos
    let mut home_goals_from_events: u16 = 0;
    let mut away_goals_from_events: u16 = 0;

    for event in &result.events {
        if matches!(event.event_type, MatchEventType::Goal) {
            if event.description.contains("mandante") {
                home_goals_from_events += 1;
            } else if event.description.contains("visitante") {
                away_goals_from_events += 1;
            }
        }
    }

    // Se nao conseguimos extrair dos eventos, usar o placar
    if home_goals_from_events == 0 && away_goals_from_events == 0 {
        home_goals_from_events = result.home_goals as u16;
        away_goals_from_events = result.away_goals as u16;
    }

    // Atualizar ou criar entrada para o "artilheiro" do clube mandante
    if home_goals_from_events > 0 {
        let synthetic_id = PlayerId::new(format!("SCORER-{}", result.home_id));
        if let Some(entry) = top_scorers
            .iter_mut()
            .find(|s| s.club_id == result.home_id && s.player_id == synthetic_id)
        {
            entry.goals += home_goals_from_events;
        } else {
            top_scorers.push(TopScorer {
                player_id: synthetic_id,
                club_id: result.home_id.clone(),
                goals: home_goals_from_events,
                assists: 0,
            });
        }
    }

    // Atualizar ou criar entrada para o "artilheiro" do clube visitante
    if away_goals_from_events > 0 {
        let synthetic_id = PlayerId::new(format!("SCORER-{}", result.away_id));
        if let Some(entry) = top_scorers
            .iter_mut()
            .find(|s| s.club_id == result.away_id && s.player_id == synthetic_id)
        {
            entry.goals += away_goals_from_events;
        } else {
            top_scorers.push(TopScorer {
                player_id: synthetic_id,
                club_id: result.away_id.clone(),
                goals: away_goals_from_events,
                assists: 0,
            });
        }
    }

    // Ordenar por gols (decrescente)
    top_scorers.sort_by(|a, b| b.goals.cmp(&a.goals));
}
