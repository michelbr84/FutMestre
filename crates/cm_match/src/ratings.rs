//! Player match ratings.

use rand::Rng;
use rand_chacha::ChaCha8Rng;

use crate::model::{MatchEventType, MatchResult, PlayerMatchRating, TeamSide};

/// Calculate player match rating from individual stats.
pub fn calculate_rating(
    goals: u8,
    assists: u8,
    passes_completed: u16,
    tackles_won: u8,
    saves: u8,
    mistakes: u8,
) -> f32 {
    let mut rating = 6.0; // Base rating

    rating += goals as f32 * 0.8;
    rating += assists as f32 * 0.5;
    rating += (passes_completed as f32 / 20.0).min(1.0);
    rating += tackles_won as f32 * 0.2;
    rating += saves as f32 * 0.3;
    rating -= mistakes as f32 * 0.5;

    rating.clamp(1.0, 10.0)
}

/// Calculate man of the match.
pub fn determine_motm(ratings: &[(String, f32)]) -> Option<String> {
    ratings
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(name, _)| name.clone())
}

/// Gera ratings de desempenho para todos os jogadores apos a simulacao.
///
/// Cria 11 jogadores por time com IDs sinteticos (HOME_1..HOME_11, AWAY_1..AWAY_11),
/// distribui gols/assists entre eles com base nos eventos, e calcula ratings individuais.
pub fn generate_player_ratings(
    result: &MatchResult,
    rng: &mut ChaCha8Rng,
) -> Vec<PlayerMatchRating> {
    let mut ratings: Vec<PlayerMatchRating> = Vec::with_capacity(22);

    // Contar cartoes por time a partir dos eventos
    let mut home_yellows: u8 = 0;
    let mut home_reds: u8 = 0;
    let mut away_yellows: u8 = 0;
    let mut away_reds: u8 = 0;

    for event in &result.events {
        match event.event_type {
            MatchEventType::YellowCard => {
                if event.description.contains("HOME_") {
                    home_yellows += 1;
                } else {
                    away_yellows += 1;
                }
            }
            MatchEventType::RedCard => {
                if event.description.contains("HOME_") {
                    home_reds += 1;
                } else {
                    away_reds += 1;
                }
            }
            _ => {}
        }
    }

    let home_won = result.home_goals > result.away_goals;
    let away_won = result.away_goals > result.home_goals;
    let home_clean_sheet = result.away_goals == 0;
    let away_clean_sheet = result.home_goals == 0;

    // Gerar 11 jogadores por time
    for side_idx in 0..2 {
        let (
            team_side,
            team_goals,
            team_assists,
            team_shots,
            team_won,
            team_lost,
            clean_sheet,
            yellows,
            reds,
        ) = if side_idx == 0 {
            (
                TeamSide::Home,
                result.home_goals,
                result.home_goals.saturating_sub(0), // assists <= goals
                result.stats.home_shots as u8,
                home_won,
                away_won,
                home_clean_sheet,
                home_yellows,
                home_reds,
            )
        } else {
            (
                TeamSide::Away,
                result.away_goals,
                result.away_goals.saturating_sub(0),
                result.stats.away_shots as u8,
                away_won,
                home_won,
                away_clean_sheet,
                away_yellows,
                away_reds,
            )
        };

        let prefix = if side_idx == 0 { "HOME" } else { "AWAY" };

        // Distribuir gols e assists entre jogadores aleatoriamente
        let mut player_goals = [0u8; 11];
        let mut player_assists = [0u8; 11];
        let mut player_shots = [0u8; 11];
        let mut player_tackles = [0u8; 11];
        let mut player_passes = [0u8; 11];
        let mut player_saves = [0u8; 11];
        let mut player_yellows = [false; 11];
        let mut player_reds = [false; 11];

        // Distribuir gols (nao para goleiro idx 0)
        for _ in 0..team_goals {
            let idx = rng.gen_range(1..11usize);
            player_goals[idx] += 1;
        }

        // Distribuir assists (diferente do goleador quando possivel)
        for _ in 0..team_goals.min(team_assists) {
            let idx = rng.gen_range(1..11usize);
            player_assists[idx] += 1;
        }

        // Distribuir chutes
        for _ in 0..team_shots {
            let idx = rng.gen_range(1..11usize);
            player_shots[idx] += 1;
        }

        // Distribuir desarmes (mais para defensores e meio-campistas)
        let total_tackles: u8 = rng.gen_range(10..25);
        for _ in 0..total_tackles {
            let idx = rng.gen_range(1..8usize); // defensores e meias
            player_tackles[idx] += 1;
        }

        // Distribuir passes
        for passes in &mut player_passes {
            *passes = rng.gen_range(15..50);
        }

        // Goleiro: defesas
        let saves_count = if side_idx == 0 {
            result
                .stats
                .away_shots_on_target
                .saturating_sub(result.away_goals as u32) as u8
        } else {
            result
                .stats
                .home_shots_on_target
                .saturating_sub(result.home_goals as u32) as u8
        };
        player_saves[0] = saves_count;

        // Distribuir cartoes amarelos
        for _ in 0..yellows {
            let idx = rng.gen_range(0..11usize);
            player_yellows[idx] = true;
        }

        // Distribuir cartoes vermelhos
        for _ in 0..reds {
            let idx = rng.gen_range(0..11usize);
            player_reds[idx] = true;
        }

        // Calcular rating para cada jogador
        for i in 0..11 {
            let mut rating: f32 = 6.0;

            // Gols marcados: +1.0 cada
            rating += player_goals[i] as f32 * 1.0;

            // Assistencias: +0.5 cada
            rating += player_assists[i] as f32 * 0.5;

            // Clean sheet (goleiro e defensores idx 0..5): +0.5
            if clean_sheet && i < 5 {
                rating += 0.5;
            }

            // Cartao vermelho: -2.0
            if player_reds[i] {
                rating -= 2.0;
            }

            // Cartao amarelo: -0.3
            if player_yellows[i] {
                rating -= 0.3;
            }

            // Time venceu: +0.3
            if team_won {
                rating += 0.3;
            }

            // Time perdeu: -0.3
            if team_lost {
                rating -= 0.3;
            }

            // Variancia aleatoria: +/- 0.5
            let variance: f32 = rng.gen_range(-0.5..0.5);
            rating += variance;

            // Clampar entre 1.0 e 10.0
            rating = rating.clamp(1.0, 10.0);

            ratings.push(PlayerMatchRating {
                player_id: format!("{}_{}", prefix, i + 1),
                team: team_side,
                rating,
                goals: player_goals[i],
                assists: player_assists[i],
                shots: player_shots[i],
                tackles: player_tackles[i],
                passes_completed: player_passes[i],
                saves: player_saves[i],
                man_of_the_match: false, // sera definido abaixo
            });
        }
    }

    // Determinar man of the match (maior rating geral)
    if let Some(best_idx) = ratings
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.rating.partial_cmp(&b.rating).unwrap())
        .map(|(idx, _)| idx)
    {
        ratings[best_idx].man_of_the_match = true;
    }

    ratings
}
