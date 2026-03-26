//! Match engine tests.

#[cfg(test)]
mod match_tests {
    use crate::model::{MatchInput, MatchResult, TeamSide, TeamStrength};
    use crate::probabilistic::{simulate_match, simulate_with_extra_time};
    use cm_core::ids::ClubId;

    #[test]
    fn test_basic_match() {
        let input = MatchInput {
            home_id: ClubId::new("HOME"),
            away_id: ClubId::new("AWAY"),
            home: TeamStrength::default(),
            away: TeamStrength::default(),
            minutes: 90,
            seed: Some(123),
        };

        let result = simulate_match(&input);
        assert!(!result.highlights.is_empty());
    }

    #[test]
    fn test_stronger_team_advantage() {
        let mut home_wins = 0;
        let mut away_wins = 0;

        for seed in 0..100 {
            let input = MatchInput {
                home_id: ClubId::new("STRONG"),
                away_id: ClubId::new("WEAK"),
                home: TeamStrength {
                    attack: 90,
                    midfield: 85,
                    defense: 88,
                    finishing: 90,
                    morale: 80,
                    fitness: 90,
                },
                away: TeamStrength {
                    attack: 50,
                    midfield: 45,
                    defense: 48,
                    finishing: 50,
                    morale: 50,
                    fitness: 60,
                },
                minutes: 90,
                seed: Some(seed),
            };

            let result = simulate_match(&input);
            if result.is_home_win() {
                home_wins += 1;
            } else if result.is_away_win() {
                away_wins += 1;
            }
        }

        // Strong team should win more often
        assert!(home_wins > away_wins);
    }

    #[test]
    fn test_match_result_methods() {
        let result = MatchResult {
            home_id: ClubId::new("HOME"),
            away_id: ClubId::new("AWAY"),
            home_goals: 2,
            away_goals: 1,
            highlights: vec![],
            stats: Default::default(),
            events: vec![],
            player_ratings: vec![],
        };

        assert!(result.is_home_win());
        assert!(!result.is_away_win());
        assert!(!result.is_draw());
        assert_eq!(result.result_string(), "2 - 1");
    }

    #[test]
    fn test_match_result_draw() {
        let result = MatchResult {
            home_id: ClubId::new("HOME"),
            away_id: ClubId::new("AWAY"),
            home_goals: 1,
            away_goals: 1,
            highlights: vec![],
            stats: Default::default(),
            events: vec![],
            player_ratings: vec![],
        };

        assert!(!result.is_home_win());
        assert!(!result.is_away_win());
        assert!(result.is_draw());
    }

    #[test]
    fn test_team_strength_overall() {
        let strength = TeamStrength {
            attack: 60,
            midfield: 70,
            defense: 80,
            finishing: 65,
            morale: 70,
            fitness: 85,
        };

        assert_eq!(strength.overall(), 70); // (60 + 70 + 80) / 3
    }

    #[test]
    fn test_extra_time_simulation() {
        // Use a seed that produces a draw in regular time
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

        // Just verify it doesn't panic
        let _ = simulate_with_extra_time(&input);
    }

    #[test]
    fn test_player_ratings_generated() {
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

        let result = simulate_match(&input);

        // Deve ter 22 ratings (11 por time)
        assert_eq!(result.player_ratings.len(), 22);

        // 11 home, 11 away
        let home_count = result
            .player_ratings
            .iter()
            .filter(|r| r.team == TeamSide::Home)
            .count();
        let away_count = result
            .player_ratings
            .iter()
            .filter(|r| r.team == TeamSide::Away)
            .count();
        assert_eq!(home_count, 11);
        assert_eq!(away_count, 11);

        // Todos os ratings devem estar entre 1.0 e 10.0
        for r in &result.player_ratings {
            assert!(r.rating >= 1.0, "Rating {} abaixo de 1.0", r.rating);
            assert!(r.rating <= 10.0, "Rating {} acima de 10.0", r.rating);
        }

        // Exatamente um man_of_the_match
        let motm_count = result
            .player_ratings
            .iter()
            .filter(|r| r.man_of_the_match)
            .count();
        assert_eq!(motm_count, 1);

        // Total de gols nos ratings deve bater com o resultado
        let home_goals_in_ratings: u8 = result
            .player_ratings
            .iter()
            .filter(|r| r.team == TeamSide::Home)
            .map(|r| r.goals)
            .sum();
        let away_goals_in_ratings: u8 = result
            .player_ratings
            .iter()
            .filter(|r| r.team == TeamSide::Away)
            .map(|r| r.goals)
            .sum();
        assert_eq!(home_goals_in_ratings, result.home_goals);
        assert_eq!(away_goals_in_ratings, result.away_goals);
    }

    #[test]
    fn test_player_ratings_deterministic() {
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

        let result1 = simulate_match(&input);
        let result2 = simulate_match(&input);

        assert_eq!(result1.player_ratings.len(), result2.player_ratings.len());
        for (r1, r2) in result1
            .player_ratings
            .iter()
            .zip(result2.player_ratings.iter())
        {
            assert_eq!(r1.player_id, r2.player_id);
            assert!((r1.rating - r2.rating).abs() < 0.001);
            assert_eq!(r1.goals, r2.goals);
            assert_eq!(r1.man_of_the_match, r2.man_of_the_match);
        }
    }
}

#[cfg(test)]
mod ratings_tests {
    use crate::ratings::{calculate_rating, determine_motm};

    #[test]
    fn test_base_rating() {
        let rating = calculate_rating(0, 0, 0, 0, 0, 0);
        assert!((rating - 6.0).abs() < 0.01);
    }

    #[test]
    fn test_rating_with_goals() {
        let rating = calculate_rating(2, 0, 0, 0, 0, 0);
        assert!(rating > 6.0);
    }

    #[test]
    fn test_rating_with_assists() {
        let rating = calculate_rating(0, 2, 0, 0, 0, 0);
        assert!(rating > 6.0);
    }

    #[test]
    fn test_rating_with_mistakes() {
        let rating = calculate_rating(0, 0, 0, 0, 0, 3);
        assert!(rating < 6.0);
    }

    #[test]
    fn test_rating_clamped() {
        // Very good performance
        let high_rating = calculate_rating(5, 3, 80, 10, 5, 0);
        assert!(high_rating <= 10.0);

        // Very poor performance
        let low_rating = calculate_rating(0, 0, 0, 0, 0, 10);
        assert!(low_rating >= 1.0);
    }

    #[test]
    fn test_determine_motm() {
        let ratings = vec![
            ("Player A".to_string(), 7.5),
            ("Player B".to_string(), 8.2),
            ("Player C".to_string(), 6.8),
        ];

        let motm = determine_motm(&ratings);
        assert_eq!(motm, Some("Player B".to_string()));
    }

    #[test]
    fn test_determine_motm_empty() {
        let ratings: Vec<(String, f32)> = vec![];
        assert!(determine_motm(&ratings).is_none());
    }
}

#[cfg(test)]
mod tactics_tests {
    use crate::model::TeamStrength;
    use crate::tactics::{
        apply_tactics_modifiers, formation_attack_bonus, formation_defense_bonus,
    };
    use cm_core::world::{Formation, Mentality, Tactics, Tempo};

    fn default_tactics() -> Tactics {
        Tactics {
            formation: Formation::F442,
            mentality: Mentality::Balanced,
            tempo: Tempo::Normal,
            pressing: 50,
            width: 50,
            defensive_line: 50,
            direct_passing: 50,
        }
    }

    #[test]
    fn test_balanced_tactics_no_change() {
        let base = TeamStrength {
            attack: 70,
            midfield: 70,
            defense: 70,
            finishing: 70,
            morale: 70,
            fitness: 80,
        };
        let tactics = default_tactics();

        let modified = apply_tactics_modifiers(&base, &tactics);
        assert_eq!(modified.attack, 70);
        assert_eq!(modified.defense, 70);
    }

    #[test]
    fn test_defensive_tactics() {
        let base = TeamStrength {
            attack: 70,
            midfield: 70,
            defense: 70,
            finishing: 70,
            morale: 70,
            fitness: 80,
        };
        let mut tactics = default_tactics();
        tactics.mentality = Mentality::Defensive;

        let modified = apply_tactics_modifiers(&base, &tactics);
        assert!(modified.attack < 70);
        assert!(modified.defense > 70);
    }

    #[test]
    fn test_attacking_tactics() {
        let base = TeamStrength {
            attack: 70,
            midfield: 70,
            defense: 70,
            finishing: 70,
            morale: 70,
            fitness: 80,
        };
        let mut tactics = default_tactics();
        tactics.mentality = Mentality::Attacking;

        let modified = apply_tactics_modifiers(&base, &tactics);
        assert!(modified.attack > 70);
        assert!(modified.defense < 70);
    }

    #[test]
    fn test_high_pressing_midfield_boost() {
        let base = TeamStrength {
            attack: 70,
            midfield: 70,
            defense: 70,
            finishing: 70,
            morale: 70,
            fitness: 80,
        };
        let mut tactics = default_tactics();
        tactics.pressing = 80;

        let modified = apply_tactics_modifiers(&base, &tactics);
        assert!(modified.midfield > 70);
    }

    #[test]
    fn test_formation_attack_bonuses() {
        assert!(formation_attack_bonus(Formation::F433) > 0);
        assert!(formation_attack_bonus(Formation::F532) < 0);
        assert_eq!(formation_attack_bonus(Formation::F442), 0);
    }

    #[test]
    fn test_formation_defense_bonuses() {
        assert!(formation_defense_bonus(Formation::F532) > 0);
        assert!(formation_defense_bonus(Formation::F343) < 0);
        assert_eq!(formation_defense_bonus(Formation::F442), 0);
    }
}

#[cfg(test)]
mod fatigue_tests {
    use crate::fatigue::{calculate_match_fatigue, calculate_recovery};

    #[test]
    fn test_match_fatigue_90_minutes() {
        let fitness = calculate_match_fatigue(100, 90, 70, 50);
        assert!(fitness < 100);
    }

    #[test]
    fn test_fatigue_no_minutes() {
        let fitness = calculate_match_fatigue(100, 0, 70, 50);
        assert_eq!(fitness, 100);
    }

    #[test]
    fn test_stamina_affects_fatigue() {
        let low_stamina = calculate_match_fatigue(100, 90, 30, 50);
        let high_stamina = calculate_match_fatigue(100, 90, 90, 50);
        assert!(low_stamina < high_stamina);
    }

    #[test]
    fn test_recovery() {
        let recovered = calculate_recovery(70, 3, 70);
        assert!(recovered > 70);
    }

    #[test]
    fn test_recovery_capped_at_100() {
        let recovered = calculate_recovery(95, 10, 100);
        assert_eq!(recovered, 100);
    }
}

#[cfg(test)]
mod commentary_tests {
    use crate::commentary::*;

    #[test]
    fn test_goal_commentary() {
        let home_goal = goal_commentary(45, "Gerrard", "Liverpool");
        assert!(home_goal.contains("45"));
        assert!(home_goal.contains("Gerrard"));
        assert!(home_goal.contains("Liverpool"));

        let away_goal = goal_commentary(67, "Henry", "Arsenal");
        assert!(away_goal.contains("Arsenal"));
    }

    #[test]
    fn test_save_commentary() {
        let save = save_commentary(33, "Casillas");
        assert!(save.contains("33"));
        assert!(save.contains("Casillas"));
    }

    #[test]
    fn test_card_commentary() {
        let yellow = card_commentary(55, "Keane", true);
        assert!(yellow.contains("AMARELO"));

        let red = card_commentary(88, "Vieira", false);
        assert!(red.contains("VERMELHO"));
    }

    #[test]
    fn test_halftime_commentary() {
        let ht = halftime_commentary(2, 1, "Liverpool", "Arsenal");
        assert!(ht.contains("2"));
        assert!(ht.contains("1"));
        assert!(ht.contains("Liverpool"));
    }

    #[test]
    fn test_fulltime_commentary_home_win() {
        let ft = fulltime_commentary(3, 1, "Liverpool", "Arsenal");
        assert!(ft.contains("VITORIA do Liverpool"));
    }

    #[test]
    fn test_fulltime_commentary_away_win() {
        let ft = fulltime_commentary(1, 2, "Liverpool", "Arsenal");
        assert!(ft.contains("VITORIA do Arsenal"));
    }

    #[test]
    fn test_fulltime_commentary_draw() {
        let ft = fulltime_commentary(1, 1, "Liverpool", "Arsenal");
        assert!(ft.contains("EMPATE"));
    }
}
