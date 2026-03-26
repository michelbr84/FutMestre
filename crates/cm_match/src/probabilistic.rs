//! Probabilistic match simulation.
//!
//! Integrates tactics, discipline, injuries, fatigue, set pieces,
//! commentary, ratings, and referee modules for a realistic
//! minute-by-minute simulation.

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::commentary;
use crate::discipline::DisciplineTracker;
use crate::fatigue;
use crate::injuries;
use crate::model::{MatchEvent, MatchEventType, MatchInput, MatchResult, MatchStats, TeamStrength};
use crate::ratings;
use crate::referee;
use crate::set_pieces::{self, SetPieceType};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Home advantage bonuses.
const HOME_ATTACK_BONUS: u8 = 3;
const HOME_MIDFIELD_BONUS: u8 = 2;
const HOME_MORALE_BONUS: u8 = 2;

/// Referee strictness (neutral default).
const REFEREE_STRICTNESS: u8 = 50;

/// Base foul probability per minute per team.
const FOUL_CHANCE_PER_MINUTE: f32 = 0.14;

/// Base corner chance per minute per team.
const CORNER_CHANCE_PER_MINUTE: f32 = 0.06;

/// Penalty chance when a foul is committed near the box.
const PENALTY_FROM_FOUL_CHANCE: f32 = 0.04;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Apply home advantage to the home team strength (returns a modified copy).
fn apply_home_advantage(home: &TeamStrength) -> TeamStrength {
    TeamStrength {
        attack: home.attack.saturating_add(HOME_ATTACK_BONUS).min(100),
        midfield: home.midfield.saturating_add(HOME_MIDFIELD_BONUS).min(100),
        defense: home.defense,
        finishing: home.finishing,
        morale: home.morale.saturating_add(HOME_MORALE_BONUS).min(100),
        fitness: home.fitness,
    }
}

/// Compute possession percentage for home team (0.0 - 1.0).
fn possession_share(home_mid: u8, away_mid: u8, rng: &mut ChaCha8Rng) -> f64 {
    let h = home_mid as f64;
    let a = away_mid as f64;
    let base = h / (h + a + 1.0);
    // Add small random variance (+/- 5%)
    let noise: f64 = rng.gen_range(-0.05..0.05);
    (base + noise).clamp(0.25, 0.75)
}

/// Calculate attack chance for a team in a given minute.
fn attack_chance(
    attack: u8,
    finishing: u8,
    opponent_defense: u8,
    morale: u8,
    fitness_factor: f32,
    possession_pct: f64,
) -> f32 {
    let atk = attack as f32;
    let fin = finishing as f32;
    let def = opponent_defense as f32;
    let mor = morale as f32;

    let edge = (atk - def * 0.7).max(0.0);
    let base = 0.008 + edge * 0.0006;

    // Finishing quality
    let finish_mod = 1.0 + (fin - 50.0) / 200.0;
    // Morale
    let morale_mod = 1.0 + (mor - 50.0) / 300.0;
    // Possession gives slight boost
    let poss_mod = 0.7 + (possession_pct as f32) * 0.6;

    (base * finish_mod * morale_mod * poss_mod * fitness_factor).clamp(0.003, 0.045)
}

/// Determine if a shot is on target (probability based on finishing).
fn shot_on_target(finishing: u8, rng: &mut ChaCha8Rng) -> bool {
    let prob = 0.25 + (finishing as f32 - 30.0) / 200.0;
    rng.gen::<f32>() < prob.clamp(0.20, 0.65)
}

/// Determine if an on-target shot results in a goal.
fn shot_is_goal(finishing: u8, opponent_defense: u8, rng: &mut ChaCha8Rng) -> bool {
    let base = 0.30 + (finishing as f32 - opponent_defense as f32) / 200.0;
    rng.gen::<f32>() < base.clamp(0.15, 0.55)
}

// ---------------------------------------------------------------------------
// Main simulation
// ---------------------------------------------------------------------------

/// Simulate a match tick-by-tick.
pub fn simulate_match(input: &MatchInput) -> MatchResult {
    let mut rng = match input.seed {
        Some(s) => ChaCha8Rng::seed_from_u64(s),
        None => ChaCha8Rng::from_entropy(),
    };

    // -- Apply home advantage --
    let home_str = apply_home_advantage(&input.home);
    let away_str = input.away.clone();

    // -- State accumulators --
    let mut home_goals: u8 = 0;
    let mut away_goals: u8 = 0;
    let mut highlights: Vec<String> = Vec::new();
    let mut events: Vec<MatchEvent> = Vec::new();

    let mut stats = MatchStats::default();
    let mut home_discipline = DisciplineTracker::new();
    let mut away_discipline = DisciplineTracker::new();

    // Track red cards per side for strength reduction
    let mut home_reds: u8 = 0;
    let mut away_reds: u8 = 0;

    // Cumulative possession ticks
    let mut home_poss_ticks: u32 = 0;
    let total_minutes = input.minutes as u32;

    // Fitness factor degrades over the match
    let home_fitness_start = home_str.fitness;
    let away_fitness_start = away_str.fitness;

    // -- Minute-by-minute loop --
    for minute in 1..=input.minutes {
        let m = minute as u32;

        // Fatigue: fitness degrades linearly; use fatigue module for reference
        let home_current_fitness =
            fatigue::calculate_match_fatigue(home_fitness_start, minute, 65, 50);
        let away_current_fitness =
            fatigue::calculate_match_fatigue(away_fitness_start, minute, 65, 50);

        let home_fitness_factor = home_current_fitness as f32 / home_fitness_start.max(1) as f32;
        let away_fitness_factor = away_current_fitness as f32 / away_fitness_start.max(1) as f32;

        // Red-card strength penalty (each red loses ~8% effectiveness)
        let home_red_mod = 1.0 - (home_reds as f32 * 0.08);
        let away_red_mod = 1.0 - (away_reds as f32 * 0.08);

        let h_fit = home_fitness_factor * home_red_mod;
        let a_fit = away_fitness_factor * away_red_mod;

        // --- Possession ---
        let poss = possession_share(home_str.midfield, away_str.midfield, &mut rng);
        if rng.gen::<f64>() < poss {
            home_poss_ticks += 1;
        }

        // --- Fouls ---
        // Away team fouling
        if rng.gen::<f32>() < FOUL_CHANCE_PER_MINUTE {
            stats.away_fouls += 1;
            let foul_severity: u8 = rng.gen_range(10..80);
            let card_roll: f32 = rng.gen();
            let player_id =
                cm_core::ids::PlayerId::new(&format!("AWAY_{}", rng.gen_range(1..=11u8)));

            if let Some(card) = referee::check_card(REFEREE_STRICTNESS, foul_severity, card_roll) {
                match card {
                    referee::CardType::Yellow => {
                        stats.away_yellow_cards += 1;
                        let second = away_discipline.yellow_card(player_id.clone());
                        let desc =
                            commentary::card_commentary(minute, &player_id.to_string(), true);
                        highlights.push(desc.clone());
                        events.push(MatchEvent {
                            minute: m,
                            event_type: MatchEventType::YellowCard,
                            description: desc,
                        });
                        if second {
                            stats.away_red_cards += 1;
                            away_reds += 1;
                            let desc2 =
                                commentary::card_commentary(minute, &player_id.to_string(), false);
                            highlights.push(desc2.clone());
                            events.push(MatchEvent {
                                minute: m,
                                event_type: MatchEventType::RedCard,
                                description: desc2,
                            });
                        }
                    }
                    referee::CardType::Red => {
                        stats.away_red_cards += 1;
                        away_reds += 1;
                        away_discipline.red_card(player_id.clone());
                        let desc =
                            commentary::card_commentary(minute, &player_id.to_string(), false);
                        highlights.push(desc.clone());
                        events.push(MatchEvent {
                            minute: m,
                            event_type: MatchEventType::RedCard,
                            description: desc,
                        });
                    }
                }
            }

            // Set piece after foul: could be penalty or free kick
            if rng.gen::<f32>() < PENALTY_FROM_FOUL_CHANCE {
                // Penalty for home
                let pen_chance =
                    set_pieces::set_piece_goal_chance(SetPieceType::Penalty, home_str.finishing);
                if rng.gen::<f32>() < pen_chance {
                    home_goals += 1;
                    stats.home_shots += 1;
                    stats.home_shots_on_target += 1;
                    let desc = commentary::penalty_commentary(minute, true);
                    highlights.push(desc.clone());
                    events.push(MatchEvent {
                        minute: m,
                        event_type: MatchEventType::Penalty,
                        description: desc,
                    });
                } else {
                    stats.home_shots += 1;
                    let desc = commentary::penalty_commentary(minute, false);
                    highlights.push(desc.clone());
                    events.push(MatchEvent {
                        minute: m,
                        event_type: MatchEventType::PenaltyMiss,
                        description: desc,
                    });
                }
            } else {
                // Free kick
                let fk_chance =
                    set_pieces::set_piece_goal_chance(SetPieceType::FreeKick, home_str.finishing);
                if rng.gen::<f32>() < fk_chance {
                    home_goals += 1;
                    stats.home_shots += 1;
                    stats.home_shots_on_target += 1;
                    let desc = commentary::freekick_goal_commentary(minute, "mandante");
                    highlights.push(desc.clone());
                    events.push(MatchEvent {
                        minute: m,
                        event_type: MatchEventType::FreeKick,
                        description: desc,
                    });
                }
            }
        }

        // Home team fouling
        if rng.gen::<f32>() < FOUL_CHANCE_PER_MINUTE {
            stats.home_fouls += 1;
            let foul_severity: u8 = rng.gen_range(10..80);
            let card_roll: f32 = rng.gen();
            let player_id =
                cm_core::ids::PlayerId::new(&format!("HOME_{}", rng.gen_range(1..=11u8)));

            if let Some(card) = referee::check_card(REFEREE_STRICTNESS, foul_severity, card_roll) {
                match card {
                    referee::CardType::Yellow => {
                        stats.home_yellow_cards += 1;
                        let second = home_discipline.yellow_card(player_id.clone());
                        let desc =
                            commentary::card_commentary(minute, &player_id.to_string(), true);
                        highlights.push(desc.clone());
                        events.push(MatchEvent {
                            minute: m,
                            event_type: MatchEventType::YellowCard,
                            description: desc,
                        });
                        if second {
                            stats.home_red_cards += 1;
                            home_reds += 1;
                            let desc2 =
                                commentary::card_commentary(minute, &player_id.to_string(), false);
                            highlights.push(desc2.clone());
                            events.push(MatchEvent {
                                minute: m,
                                event_type: MatchEventType::RedCard,
                                description: desc2,
                            });
                        }
                    }
                    referee::CardType::Red => {
                        stats.home_red_cards += 1;
                        home_reds += 1;
                        home_discipline.red_card(player_id.clone());
                        let desc =
                            commentary::card_commentary(minute, &player_id.to_string(), false);
                        highlights.push(desc.clone());
                        events.push(MatchEvent {
                            minute: m,
                            event_type: MatchEventType::RedCard,
                            description: desc,
                        });
                    }
                }
            }

            // Set piece after foul (for away team)
            if rng.gen::<f32>() < PENALTY_FROM_FOUL_CHANCE {
                let pen_chance =
                    set_pieces::set_piece_goal_chance(SetPieceType::Penalty, away_str.finishing);
                if rng.gen::<f32>() < pen_chance {
                    away_goals += 1;
                    stats.away_shots += 1;
                    stats.away_shots_on_target += 1;
                    let desc = commentary::penalty_commentary(minute, true);
                    highlights.push(desc.clone());
                    events.push(MatchEvent {
                        minute: m,
                        event_type: MatchEventType::Penalty,
                        description: desc,
                    });
                } else {
                    stats.away_shots += 1;
                    let desc = commentary::penalty_commentary(minute, false);
                    highlights.push(desc.clone());
                    events.push(MatchEvent {
                        minute: m,
                        event_type: MatchEventType::PenaltyMiss,
                        description: desc,
                    });
                }
            } else {
                let fk_chance =
                    set_pieces::set_piece_goal_chance(SetPieceType::FreeKick, away_str.finishing);
                if rng.gen::<f32>() < fk_chance {
                    away_goals += 1;
                    stats.away_shots += 1;
                    stats.away_shots_on_target += 1;
                    let desc = commentary::freekick_goal_commentary(minute, "visitante");
                    highlights.push(desc.clone());
                    events.push(MatchEvent {
                        minute: m,
                        event_type: MatchEventType::FreeKick,
                        description: desc,
                    });
                }
            }
        }

        // --- Corners ---
        if rng.gen::<f32>() < CORNER_CHANCE_PER_MINUTE {
            stats.home_corners += 1;
            let corner_chance =
                set_pieces::set_piece_goal_chance(SetPieceType::Corner, home_str.attack);
            if rng.gen::<f32>() < corner_chance {
                home_goals += 1;
                stats.home_shots += 1;
                stats.home_shots_on_target += 1;
                let desc = commentary::corner_goal_commentary(minute, "mandante");
                highlights.push(desc.clone());
                events.push(MatchEvent {
                    minute: m,
                    event_type: MatchEventType::Corner,
                    description: desc,
                });
            } else {
                events.push(MatchEvent {
                    minute: m,
                    event_type: MatchEventType::Corner,
                    description: commentary::corner_commentary(minute, "mandante"),
                });
            }
        }
        if rng.gen::<f32>() < CORNER_CHANCE_PER_MINUTE {
            stats.away_corners += 1;
            let corner_chance =
                set_pieces::set_piece_goal_chance(SetPieceType::Corner, away_str.attack);
            if rng.gen::<f32>() < corner_chance {
                away_goals += 1;
                stats.away_shots += 1;
                stats.away_shots_on_target += 1;
                let desc = commentary::corner_goal_commentary(minute, "visitante");
                highlights.push(desc.clone());
                events.push(MatchEvent {
                    minute: m,
                    event_type: MatchEventType::Corner,
                    description: desc,
                });
            } else {
                events.push(MatchEvent {
                    minute: m,
                    event_type: MatchEventType::Corner,
                    description: commentary::corner_commentary(minute, "visitante"),
                });
            }
        }

        // --- Open-play attack opportunities ---
        let home_chance = attack_chance(
            home_str.attack,
            home_str.finishing,
            away_str.defense,
            home_str.morale,
            h_fit,
            poss,
        );
        let away_chance = attack_chance(
            away_str.attack,
            away_str.finishing,
            home_str.defense,
            away_str.morale,
            a_fit,
            1.0 - poss,
        );

        // Home attack
        if rng.gen::<f32>() < home_chance {
            stats.home_shots += 1;
            if shot_on_target(home_str.finishing, &mut rng) {
                stats.home_shots_on_target += 1;
                if shot_is_goal(home_str.finishing, away_str.defense, &mut rng) {
                    home_goals += 1;
                    let desc = commentary::goal_commentary(minute, "Jogador", "mandante");
                    highlights.push(desc.clone());
                    events.push(MatchEvent {
                        minute: m,
                        event_type: MatchEventType::Goal,
                        description: desc,
                    });
                } else {
                    let desc = commentary::save_commentary(minute, "Goleiro visitante");
                    highlights.push(desc);
                }
            }
        }

        // Away attack
        if rng.gen::<f32>() < away_chance {
            stats.away_shots += 1;
            if shot_on_target(away_str.finishing, &mut rng) {
                stats.away_shots_on_target += 1;
                if shot_is_goal(away_str.finishing, home_str.defense, &mut rng) {
                    away_goals += 1;
                    let desc = commentary::goal_commentary(minute, "Jogador", "visitante");
                    highlights.push(desc.clone());
                    events.push(MatchEvent {
                        minute: m,
                        event_type: MatchEventType::Goal,
                        description: desc,
                    });
                } else {
                    let desc = commentary::save_commentary(minute, "Goleiro mandante");
                    highlights.push(desc);
                }
            }
        }

        // --- Injuries ---
        let home_injury_chance = injuries::injury_chance_per_minute(home_current_fitness, 70, 50);
        if injuries::check_injury(&mut rng, home_injury_chance) {
            let _severity = injuries::injury_severity(&mut rng);
            let desc = commentary::injury_commentary(minute, "Jogador mandante");
            highlights.push(desc.clone());
            events.push(MatchEvent {
                minute: m,
                event_type: MatchEventType::Injury,
                description: desc,
            });
        }

        let away_injury_chance = injuries::injury_chance_per_minute(away_current_fitness, 70, 50);
        if injuries::check_injury(&mut rng, away_injury_chance) {
            let _severity = injuries::injury_severity(&mut rng);
            let desc = commentary::injury_commentary(minute, "Jogador visitante");
            highlights.push(desc.clone());
            events.push(MatchEvent {
                minute: m,
                event_type: MatchEventType::Injury,
                description: desc,
            });
        }

        // --- Half-time ---
        if minute == 45 && input.minutes >= 90 {
            let hn = input.home_id.to_string();
            let an = input.away_id.to_string();
            let desc = commentary::halftime_commentary(home_goals, away_goals, &hn, &an);
            highlights.push(desc.clone());
            events.push(MatchEvent {
                minute: m,
                event_type: MatchEventType::HalfTime,
                description: desc,
            });
        }
    }

    // --- Full-time ---
    let home_name = input.home_id.to_string();
    let away_name = input.away_id.to_string();
    let ft_desc = commentary::fulltime_commentary(home_goals, away_goals, &home_name, &away_name);
    highlights.push(ft_desc.clone());
    events.push(MatchEvent {
        minute: input.minutes as u32,
        event_type: MatchEventType::FullTime,
        description: ft_desc,
    });

    // --- Compute final possession % ---
    let total_ticks = total_minutes.max(1);
    stats.home_possession = (home_poss_ticks as f64 / total_ticks as f64 * 100.0).round();
    stats.away_possession = 100.0 - stats.home_possession;

    let mut result = MatchResult {
        home_id: input.home_id.clone(),
        away_id: input.away_id.clone(),
        home_goals,
        away_goals,
        highlights,
        stats,
        events,
        player_ratings: vec![],
    };

    // Gerar ratings individuais dos jogadores
    result.player_ratings = ratings::generate_player_ratings(&result, &mut rng);

    result
}

/// Simulate with extra time (and penalties if still drawn).
pub fn simulate_with_extra_time(input: &MatchInput) -> MatchResult {
    let mut result = simulate_match(input);

    // If draw after regular time, play extra time (30 minutes)
    if result.is_draw() {
        result.highlights.push(commentary::extra_time_commentary());
        result.events.push(MatchEvent {
            minute: 90,
            event_type: MatchEventType::ExtraTime,
            description: commentary::extra_time_commentary(),
        });

        let mut extra_input = input.clone();
        extra_input.minutes = 30;
        extra_input.seed = input.seed.map(|s| s.wrapping_add(1));

        let extra = simulate_match(&extra_input);

        result.home_goals += extra.home_goals;
        result.away_goals += extra.away_goals;

        // Merge stats
        result.stats.home_shots += extra.stats.home_shots;
        result.stats.away_shots += extra.stats.away_shots;
        result.stats.home_shots_on_target += extra.stats.home_shots_on_target;
        result.stats.away_shots_on_target += extra.stats.away_shots_on_target;
        result.stats.home_fouls += extra.stats.home_fouls;
        result.stats.away_fouls += extra.stats.away_fouls;
        result.stats.home_corners += extra.stats.home_corners;
        result.stats.away_corners += extra.stats.away_corners;
        result.stats.home_yellow_cards += extra.stats.home_yellow_cards;
        result.stats.away_yellow_cards += extra.stats.away_yellow_cards;
        result.stats.home_red_cards += extra.stats.home_red_cards;
        result.stats.away_red_cards += extra.stats.away_red_cards;

        // Offset extra-time events to 91+
        for mut ev in extra.events {
            ev.minute += 90;
            result.events.push(ev);
        }
        for hl in extra.highlights {
            result.highlights.push(hl);
        }
    }

    // If still draw, go to penalties
    if result.is_draw() {
        result
            .highlights
            .push(commentary::penalty_shootout_commentary());

        let mut rng = match input.seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s.wrapping_add(100)),
            None => ChaCha8Rng::from_entropy(),
        };

        let (h_pen, a_pen) = simulate_penalty_shootout(&mut rng);
        result.home_goals += h_pen;
        result.away_goals += a_pen;

        let desc = format!(
            "Penaltis: {} - {} (total: {} - {})",
            h_pen, a_pen, result.home_goals, result.away_goals
        );
        result.highlights.push(desc.clone());
        result.events.push(MatchEvent {
            minute: 120,
            event_type: MatchEventType::Penalty,
            description: desc,
        });
    }

    result
}

/// Simulate a penalty shootout. Returns (home_goals, away_goals).
fn simulate_penalty_shootout(rng: &mut ChaCha8Rng) -> (u8, u8) {
    let mut home_scored: u8 = 0;
    let mut away_scored: u8 = 0;
    let mut home_taken: u8 = 0;
    let mut away_taken: u8 = 0;

    // Standard 5 rounds
    for _ in 0..5 {
        home_taken += 1;
        if rng.gen::<f32>() < 0.75 {
            home_scored += 1;
        }

        away_taken += 1;
        if rng.gen::<f32>() < 0.75 {
            away_scored += 1;
        }

        // Check if winner is decided early
        let home_remaining = 5 - home_taken;
        let away_remaining = 5 - away_taken;

        if home_scored > away_scored + away_remaining {
            break;
        }
        if away_scored > home_scored + home_remaining {
            break;
        }
    }

    // Sudden death if tied after 5
    while home_scored == away_scored {
        if rng.gen::<f32>() < 0.75 {
            home_scored += 1;
        }
        if rng.gen::<f32>() < 0.75 {
            away_scored += 1;
        }
        // If both score or both miss, continue
        if home_scored != away_scored {
            break;
        }
    }

    (home_scored, away_scored)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::TeamStrength;
    use cm_core::ids::ClubId;

    #[test]
    fn test_simulate_match_deterministic() {
        let input = MatchInput {
            home_id: ClubId::new("LIV"),
            away_id: ClubId::new("ARS"),
            home: TeamStrength {
                attack: 80,
                midfield: 75,
                defense: 78,
                finishing: 82,
                morale: 70,
                fitness: 85,
            },
            away: TeamStrength {
                attack: 75,
                midfield: 72,
                defense: 76,
                finishing: 78,
                morale: 65,
                fitness: 80,
            },
            minutes: 90,
            seed: Some(42),
        };

        let result1 = simulate_match(&input);
        let result2 = simulate_match(&input);

        // Same seed should give same result
        assert_eq!(result1.home_goals, result2.home_goals);
        assert_eq!(result1.away_goals, result2.away_goals);
        assert_eq!(result1.stats.home_shots, result2.stats.home_shots);
        assert_eq!(result1.stats.away_fouls, result2.stats.away_fouls);
        assert_eq!(result1.events.len(), result2.events.len());
    }

    #[test]
    fn test_match_has_stats() {
        let input = MatchInput {
            home_id: ClubId::new("LIV"),
            away_id: ClubId::new("ARS"),
            home: TeamStrength {
                attack: 80,
                midfield: 75,
                defense: 78,
                finishing: 82,
                morale: 70,
                fitness: 85,
            },
            away: TeamStrength {
                attack: 75,
                midfield: 72,
                defense: 76,
                finishing: 78,
                morale: 65,
                fitness: 80,
            },
            minutes: 90,
            seed: Some(99),
        };

        let result = simulate_match(&input);
        // Possession should sum to 100
        assert!((result.stats.home_possession + result.stats.away_possession - 100.0).abs() < 0.01);
        // Should have events (at minimum full-time)
        assert!(!result.events.is_empty());
        assert!(!result.highlights.is_empty());
    }

    #[test]
    fn test_penalty_shootout() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let (h, a) = simulate_penalty_shootout(&mut rng);
        // At least one side should have scored
        assert!(h > 0 || a > 0);
        // They should not be equal (shootout must produce a winner)
        assert_ne!(h, a);
    }

    #[test]
    fn test_extra_time_simulation() {
        let input = MatchInput {
            home_id: ClubId::new("HOME"),
            away_id: ClubId::new("AWAY"),
            home: TeamStrength {
                attack: 50,
                midfield: 50,
                defense: 50,
                finishing: 50,
                morale: 50,
                fitness: 50,
            },
            away: TeamStrength {
                attack: 50,
                midfield: 50,
                defense: 50,
                finishing: 50,
                morale: 50,
                fitness: 50,
            },
            minutes: 90,
            seed: Some(42),
        };

        // Just verify it doesn't panic and produces a result
        let result = simulate_with_extra_time(&input);
        assert!(!result.highlights.is_empty());
    }
}
