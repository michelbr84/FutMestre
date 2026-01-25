//! Tactics AI - Formation and mentality decision making.

use cm_core::world::{Formation, Mentality, Tactics, Tempo};

/// Select formation based on squad strength distribution.
/// 
/// Attack strength and defense strength should be 0-100 representing
/// the average ability in those areas of the squad.
pub fn recommend_formation(attack_strength: u8, defense_strength: u8) -> Formation {
    let diff = attack_strength as i16 - defense_strength as i16;
    
    // Strong attack bias -> attacking formations
    if diff > 15 {
        if attack_strength > 75 {
            Formation::F433  // 3 forwards for very strong attack
        } else {
            Formation::F4231 // Balanced with attacking midfielder
        }
    }
    // Strong defense bias -> defensive formations  
    else if diff < -15 {
        if defense_strength > 75 {
            Formation::F532  // 5 defenders for very strong defense
        } else {
            Formation::F451 // Compact midfield protection
        }
    }
    // Balanced squad
    else {
        if attack_strength > 70 && defense_strength > 70 {
            Formation::F4231 // Flexible formation for quality squad
        } else if attack_strength < 50 && defense_strength < 50 {
            Formation::F4141 // Defensive shield for weaker squad
        } else {
            Formation::F442  // Classic balanced formation
        }
    }
}

/// Adjust mentality based on current game situation.
/// 
/// `score_diff` is own_goals - opponent_goals (positive = winning)
/// `minutes_remaining` is minutes left in the match (0-90+ for extra time)
pub fn adjust_mentality(score_diff: i8, minutes_remaining: u8) -> Mentality {
    // Winning scenarios
    if score_diff > 0 {
        if score_diff >= 3 {
            // Comfortable lead - protect it
            Mentality::Defensive
        } else if minutes_remaining <= 15 {
            // Close lead near end - protect
            Mentality::Cautious
        } else if minutes_remaining <= 30 {
            // Lead with some time left
            Mentality::Balanced
        } else {
            // Early lead - can still push
            Mentality::Balanced
        }
    }
    // Drawing scenarios
    else if score_diff == 0 {
        if minutes_remaining <= 10 {
            // Need a goal urgently
            Mentality::AllOutAttack
        } else if minutes_remaining <= 20 {
            Mentality::Attacking
        } else {
            Mentality::Balanced
        }
    }
    // Losing scenarios
    else {
        let goals_behind = (-score_diff) as u8;
        
        if goals_behind >= 3 && minutes_remaining <= 30 {
            // Far behind late - desperate
            Mentality::AllOutAttack
        } else if goals_behind >= 2 && minutes_remaining <= 20 {
            Mentality::AllOutAttack
        } else if goals_behind >= 1 && minutes_remaining <= 15 {
            Mentality::AllOutAttack
        } else if minutes_remaining <= 30 {
            Mentality::Attacking
        } else {
            // Still time to come back
            Mentality::Attacking
        }
    }
}

/// Adjust tempo based on game situation and squad fitness.
pub fn adjust_tempo(
    score_diff: i8,
    minutes_remaining: u8,
    average_fitness: u8,
) -> Tempo {
    // Low fitness - slow down
    if average_fitness < 60 {
        return Tempo::Slow;
    }
    
    // Desperate situations need fast tempo
    if score_diff < 0 && minutes_remaining <= 20 {
        return Tempo::Fast;
    }
    
    // Protecting a lead - slow down
    if score_diff > 0 && minutes_remaining <= 20 {
        return Tempo::Slow;
    }
    
    // High fitness allows faster tempo
    if average_fitness > 80 {
        Tempo::Fast
    } else {
        Tempo::Normal
    }
}

/// Recommend pressing intensity based on fitness and game state.
/// Returns a value 0-100.
pub fn recommend_pressing(
    score_diff: i8,
    minutes_remaining: u8,
    average_fitness: u8,
) -> u8 {
    // Base pressing on fitness
    let base_pressing = (average_fitness as i16 - 20).max(30).min(80) as u8;
    
    // Adjust based on game state
    if score_diff < 0 && minutes_remaining <= 30 {
        // Losing late - press high
        (base_pressing + 20).min(100)
    } else if score_diff > 1 && minutes_remaining <= 30 {
        // Comfortable lead - reduce pressing to conserve energy
        (base_pressing as i16 - 20).max(20) as u8
    } else {
        base_pressing
    }
}

/// Recommend defensive line height based on opponent and formation.
/// Returns a value 0-100 (0 = deep, 100 = high).
pub fn recommend_defensive_line(
    formation: Formation,
    opponent_pace: u8,
    own_pace: u8,
) -> u8 {
    // Start with formation-based defaults
    let base_line = match formation {
        Formation::F532 | Formation::F451 | Formation::F4141 => 35, // Defensive formations = deeper
        Formation::F433 | Formation::F343 => 65, // Attacking formations = higher
        _ => 50, // Balanced
    };
    
    // Adjust based on pace differential
    let pace_diff = own_pace as i16 - opponent_pace as i16;
    let adjustment = (pace_diff / 4).clamp(-15, 15) as i8;
    
    ((base_line as i16) + (adjustment as i16)).clamp(20, 80) as u8
}

/// Generate complete tactics recommendation.
pub fn recommend_tactics(
    attack_strength: u8,
    defense_strength: u8,
    average_fitness: u8,
    squad_pace: u8,
    opponent_pace: u8,
) -> Tactics {
    let formation = recommend_formation(attack_strength, defense_strength);
    let defensive_line = recommend_defensive_line(formation, opponent_pace, squad_pace);
    
    // More attacking squads use wider play
    let width = if attack_strength > defense_strength {
        60 + (attack_strength.saturating_sub(defense_strength) / 3).min(20)
    } else {
        50
    };
    
    // Direct passing based on squad quality
    let avg_quality = (attack_strength as u16 + defense_strength as u16) / 2;
    let direct_passing = if avg_quality > 70 {
        40 // Better players = more patient build-up
    } else if avg_quality < 50 {
        70 // Weaker players = more direct
    } else {
        50
    };
    
    Tactics {
        formation,
        mentality: Mentality::Balanced, // Will be adjusted during match
        tempo: if average_fitness > 70 { Tempo::Fast } else { Tempo::Normal },
        pressing: recommend_pressing(0, 90, average_fitness),
        defensive_line,
        width,
        direct_passing,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recommend_formation_attack_bias() {
        // Strong attack should get attacking formation
        let formation = recommend_formation(85, 60);
        assert_eq!(formation, Formation::F433);
        
        // Moderate attack bias
        let formation = recommend_formation(75, 55);
        assert_eq!(formation, Formation::F4231);
    }

    #[test]
    fn test_recommend_formation_defense_bias() {
        // Strong defense should get defensive formation
        let formation = recommend_formation(60, 85);
        assert_eq!(formation, Formation::F532);
        
        // Moderate defense bias
        let formation = recommend_formation(55, 75);
        assert_eq!(formation, Formation::F451);
    }

    #[test]
    fn test_recommend_formation_balanced() {
        // High quality balanced squad
        let formation = recommend_formation(75, 75);
        assert_eq!(formation, Formation::F4231);
        
        // Average quality balanced squad
        let formation = recommend_formation(60, 65);
        assert_eq!(formation, Formation::F442);
        
        // Low quality squad
        let formation = recommend_formation(45, 45);
        assert_eq!(formation, Formation::F4141);
    }

    #[test]
    fn test_adjust_mentality_winning() {
        // Big lead - defensive
        assert_eq!(adjust_mentality(3, 30), Mentality::Defensive);
        assert_eq!(adjust_mentality(4, 60), Mentality::Defensive);
        
        // Close lead late - cautious
        assert_eq!(adjust_mentality(1, 10), Mentality::Cautious);
        
        // Early lead - balanced
        assert_eq!(adjust_mentality(1, 60), Mentality::Balanced);
    }

    #[test]
    fn test_adjust_mentality_drawing() {
        // Drawing late - all out attack
        assert_eq!(adjust_mentality(0, 5), Mentality::AllOutAttack);
        assert_eq!(adjust_mentality(0, 10), Mentality::AllOutAttack);
        
        // Drawing with time - attacking then balanced
        assert_eq!(adjust_mentality(0, 15), Mentality::Attacking);
        assert_eq!(adjust_mentality(0, 45), Mentality::Balanced);
    }

    #[test]
    fn test_adjust_mentality_losing() {
        // Far behind late - desperate
        assert_eq!(adjust_mentality(-3, 20), Mentality::AllOutAttack);
        assert_eq!(adjust_mentality(-2, 15), Mentality::AllOutAttack);
        assert_eq!(adjust_mentality(-1, 10), Mentality::AllOutAttack);
        
        // Behind with time - attacking
        assert_eq!(adjust_mentality(-1, 60), Mentality::Attacking);
    }

    #[test]
    fn test_adjust_tempo() {
        // Low fitness
        assert_eq!(adjust_tempo(0, 45, 50), Tempo::Slow);
        
        // Losing late - fast
        assert_eq!(adjust_tempo(-1, 15, 70), Tempo::Fast);
        
        // Winning late - slow
        assert_eq!(adjust_tempo(2, 10, 80), Tempo::Slow);
        
        // High fitness normal situation
        assert_eq!(adjust_tempo(0, 45, 85), Tempo::Fast);
        
        // Normal fitness
        assert_eq!(adjust_tempo(0, 45, 70), Tempo::Normal);
    }

    #[test]
    fn test_recommend_pressing() {
        // Normal conditions
        let pressing = recommend_pressing(0, 60, 75);
        assert!(pressing >= 40 && pressing <= 70);
        
        // Losing late - high pressing
        let pressing = recommend_pressing(-1, 20, 75);
        assert!(pressing >= 60);
        
        // Comfortable lead - low pressing
        let pressing = recommend_pressing(2, 20, 75);
        assert!(pressing <= 60);
    }

    #[test]
    fn test_recommend_defensive_line() {
        // Defensive formation
        let line = recommend_defensive_line(Formation::F532, 70, 70);
        assert!(line < 50);
        
        // Attacking formation
        let line = recommend_defensive_line(Formation::F433, 70, 70);
        assert!(line > 50);
        
        // Fast opponent - drop deeper
        let line = recommend_defensive_line(Formation::F442, 90, 60);
        assert!(line < 50);
        
        // Faster than opponent - higher line
        let line = recommend_defensive_line(Formation::F442, 60, 90);
        assert!(line > 50);
    }

    #[test]
    fn test_recommend_tactics() {
        // Attacking squad
        let tactics = recommend_tactics(80, 60, 75, 70, 70);
        assert!(matches!(tactics.formation, Formation::F433 | Formation::F4231));
        assert!(tactics.width >= 60);
        
        // Defensive squad with low fitness
        let tactics = recommend_tactics(55, 80, 55, 60, 80);
        assert!(matches!(tactics.formation, Formation::F532 | Formation::F451));
        assert_eq!(tactics.tempo, Tempo::Normal);
        
        // Balanced high quality squad
        let tactics = recommend_tactics(75, 75, 80, 75, 70);
        assert_eq!(tactics.formation, Formation::F4231);
        assert!(tactics.direct_passing < 50); // Patient build-up
    }

    #[test]
    fn test_edge_cases() {
        // Zero values
        assert_eq!(recommend_formation(0, 0), Formation::F4141);
        
        // Maximum values
        let formation = recommend_formation(100, 100);
        assert_eq!(formation, Formation::F4231);
        
        // Extreme score differences
        assert_eq!(adjust_mentality(10, 5), Mentality::Defensive);
        assert_eq!(adjust_mentality(-10, 5), Mentality::AllOutAttack);
    }
}
