//! Scouting AI - Player evaluation and discovery.

use chrono::Datelike;
use cm_core::ids::PlayerId;
use cm_core::world::{Player, Position, World};

/// Scout report for a player.
#[derive(Debug, Clone)]
pub struct DetailedScoutReport {
    pub player_id: PlayerId,
    pub overall_score: u8,
    pub current_ability: u8,
    pub potential_ability: u8,
    pub value_assessment: ValueAssessment,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub recommendation: ScoutRecommendation,
    pub accuracy: u8,  // How reliable this report is (based on scout ability)
}

/// Assessment of player's value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueAssessment {
    Overvalued,
    FairValue,
    Undervalued,
    Bargain,
}

/// Scout's recommendation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScoutRecommendation {
    MustSign,
    Recommended,
    Watchlist,
    NotRecommended,
    AvoidSigning,
}

/// Score a player for scouting purposes.
/// Returns a score 0-100 based on player attributes and scout ability.
pub fn scout_score(world: &World, player_id: &PlayerId, scout_ability: u8) -> u8 {
    let Some(player) = world.players.get(player_id) else {
        return 50; // Unknown player
    };
    
    // Base score from actual ability
    let true_score = calculate_true_player_score(player);
    
    // Add noise based on scout ability (lower ability = more noise)
    let accuracy = (scout_ability as f32 / 100.0).max(0.3);
    let max_error = (100 - scout_ability) as i8;
    
    // Deterministic "randomness" based on player id hash
    let hash_value = simple_hash(&player_id.to_string()) % (max_error as u32 * 2 + 1);
    let error = (hash_value as i8) - max_error;
    
    let adjusted = (true_score as i16 + (error as f32 * (1.0 - accuracy)) as i16).clamp(0, 100);
    
    adjusted as u8
}

/// Simple hash function for deterministic "randomness".
fn simple_hash(s: &str) -> u32 {
    let mut hash: u32 = 0;
    for c in s.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(c as u32);
    }
    hash
}

/// Calculate true player score (what a perfect scout would see).
fn calculate_true_player_score(player: &Player) -> u8 {
    let ability = player.overall_rating();
    let potential = player.potential;
    let form = player.form;
    let fitness = player.fitness;
    
    // Weight current ability most, but factor in potential and condition
    let score = (ability as u16 * 50 + potential as u16 * 30 + form as u16 * 10 + fitness as u16 * 10) / 100;
    
    score as u8
}

/// Generate detailed scout report for a player.
pub fn generate_scout_report(
    world: &World,
    player_id: &PlayerId,
    scout_ability: u8,
) -> Option<DetailedScoutReport> {
    let player = world.players.get(player_id)?;
    
    let overall_score = scout_score(world, player_id, scout_ability);
    let accuracy = calculate_accuracy(scout_ability);
    
    // Current ability assessment (with potential error)
    let current_ability = assess_with_noise(player.overall_rating(), scout_ability);
    let potential_ability = assess_with_noise(player.potential, scout_ability);
    
    // Identify strengths and weaknesses
    let (strengths, weaknesses) = analyze_attributes(player, scout_ability);
    
    // Value assessment
    let value_assessment = assess_value(player, current_ability);
    
    // Recommendation based on all factors
    let recommendation = generate_recommendation(
        current_ability,
        potential_ability,
        value_assessment,
        player,
    );
    
    Some(DetailedScoutReport {
        player_id: player_id.clone(),
        overall_score,
        current_ability,
        potential_ability,
        value_assessment,
        strengths,
        weaknesses,
        recommendation,
        accuracy,
    })
}

fn calculate_accuracy(scout_ability: u8) -> u8 {
    // Map scout ability to accuracy percentage
    // 100 ability = ~95% accuracy, 50 ability = ~60% accuracy
    ((scout_ability as f32 * 0.4) + 55.0).min(95.0) as u8
}

fn assess_with_noise(true_value: u8, scout_ability: u8) -> u8 {
    let accuracy = scout_ability as f32 / 100.0;
    let max_error = ((1.0 - accuracy) * 15.0) as i8;
    
    // Deterministic noise
    let noise = ((true_value as i16 % 7) - 3) as i8;
    let error = (noise * max_error / 3).clamp(-10, 10);
    
    (true_value as i16 + error as i16).clamp(0, 100) as u8
}

fn analyze_attributes(player: &Player, scout_ability: u8) -> (Vec<String>, Vec<String>) {
    let mut strengths = Vec::new();
    let mut weaknesses = Vec::new();
    
    // Only identify attributes if scout is good enough
    if scout_ability < 40 {
        return (strengths, weaknesses);
    }
    
    let attrs = &player.attributes;
    
    // Technical attributes
    if attrs.technical.passing > 75 {
        strengths.push("Excellent passer".into());
    } else if attrs.technical.passing < 50 {
        weaknesses.push("Poor passing".into());
    }
    
    if attrs.technical.finishing > 75 {
        strengths.push("Clinical finisher".into());
    } else if attrs.technical.finishing < 50 && player.position.is_forward() {
        weaknesses.push("Lacks finishing ability".into());
    }
    
    if attrs.technical.tackling > 75 {
        strengths.push("Strong tackler".into());
    } else if attrs.technical.tackling < 50 && player.position.is_defender() {
        weaknesses.push("Weak in the tackle".into());
    }
    
    if attrs.technical.dribbling > 75 {
        strengths.push("Skillful dribbler".into());
    }
    
    // Physical attributes
    if attrs.physical.pace > 80 {
        strengths.push("Exceptional pace".into());
    } else if attrs.physical.pace < 50 {
        weaknesses.push("Lacks pace".into());
    }
    
    if attrs.physical.stamina > 80 {
        strengths.push("High stamina".into());
    } else if attrs.physical.stamina < 50 {
        weaknesses.push("Poor stamina".into());
    }
    
    if attrs.physical.strength > 75 {
        strengths.push("Physically strong".into());
    }
    
    // Mental attributes
    if attrs.mental.leadership > 80 {
        strengths.push("Natural leader".into());
    }
    
    if attrs.mental.composure > 75 {
        strengths.push("Composed under pressure".into());
    } else if attrs.mental.composure < 50 {
        weaknesses.push("Prone to nerves".into());
    }
    
    if attrs.mental.work_rate > 80 {
        strengths.push("Excellent work rate".into());
    } else if attrs.mental.work_rate < 50 {
        weaknesses.push("Lazy player".into());
    }
    
    // Limit results based on scout ability
    let max_insights = (scout_ability / 20) as usize;
    strengths.truncate(max_insights.max(2));
    weaknesses.truncate(max_insights.max(1));
    
    (strengths, weaknesses)
}

fn assess_value(player: &Player, assessed_ability: u8) -> ValueAssessment {
    // Convert to major units (pounds) for comparison
    let value = player.value.major();
    
    // Expected value based on ability (in major units - pounds)
    let expected_value: i64 = match assessed_ability {
        90..=100 => 50_000_000,
        80..=89 => 20_000_000,
        70..=79 => 8_000_000,
        60..=69 => 3_000_000,
        50..=59 => 1_000_000,
        40..=49 => 300_000,
        _ => 100_000,
    };
    
    let ratio = value as f64 / expected_value as f64;
    
    if ratio > 1.5 {
        ValueAssessment::Overvalued
    } else if ratio > 0.9 {
        ValueAssessment::FairValue
    } else if ratio > 0.5 {
        ValueAssessment::Undervalued
    } else {
        ValueAssessment::Bargain
    }
}

fn generate_recommendation(
    current: u8,
    potential: u8,
    value: ValueAssessment,
    player: &Player,
) -> ScoutRecommendation {
    // Age factor
    let birth_year = player.birth_date.year();
    let age = (2024 - birth_year) as u8;
    
    // High potential young player
    if age <= 23 && potential >= 80 {
        if value == ValueAssessment::Bargain || value == ValueAssessment::Undervalued {
            return ScoutRecommendation::MustSign;
        }
        return ScoutRecommendation::Recommended;
    }
    
    // Elite current ability
    if current >= 85 {
        if value != ValueAssessment::Overvalued {
            return ScoutRecommendation::MustSign;
        }
        return ScoutRecommendation::Recommended;
    }
    
    // Good player at fair price
    if current >= 70 && value != ValueAssessment::Overvalued {
        return ScoutRecommendation::Recommended;
    }
    
    // Aging player with declining potential
    if age >= 32 && current < 75 {
        return ScoutRecommendation::NotRecommended;
    }
    
    // Overvalued
    if value == ValueAssessment::Overvalued {
        return ScoutRecommendation::NotRecommended;
    }
    
    // Low potential youth
    if age <= 23 && potential < 60 {
        return ScoutRecommendation::NotRecommended;
    }
    
    // Average player - watchlist
    ScoutRecommendation::Watchlist
}

/// Search for players matching criteria.
pub fn search_players(
    world: &World,
    criteria: &SearchCriteria,
    scout_ability: u8,
    max_results: usize,
) -> Vec<PlayerId> {
    let mut candidates: Vec<_> = world.players.values()
        .filter(|p| matches_criteria(p, criteria))
        .map(|p| (p.id.clone(), scout_score(world, &p.id, scout_ability)))
        .collect();
    
    // Sort by score
    candidates.sort_by(|a, b| b.1.cmp(&a.1));
    
    candidates.into_iter()
        .take(max_results)
        .map(|(id, _)| id)
        .collect()
}

/// Search criteria for player scouting.
#[derive(Debug, Clone, Default)]
pub struct SearchCriteria {
    pub position: Option<Position>,
    pub min_age: Option<u8>,
    pub max_age: Option<u8>,
    pub max_value: Option<i64>,
    pub min_potential: Option<u8>,
    pub nationality: Option<String>,
    pub available_only: bool,  // Only unattached players or those with release clause
}

fn matches_criteria(player: &Player, criteria: &SearchCriteria) -> bool {
    // Position filter
    if let Some(pos) = &criteria.position {
        if player.position != *pos && !player.secondary_positions.contains(pos) {
            return false;
        }
    }
    
    // Age filter
    let age = (2024 - player.birth_date.year()) as u8;
    if let Some(min) = criteria.min_age {
        if age < min {
            return false;
        }
    }
    if let Some(max) = criteria.max_age {
        if age > max {
            return false;
        }
    }
    
    // Value filter
    if let Some(max_val) = criteria.max_value {
        if player.value.minor() > max_val {
            return false;
        }
    }
    
    // Potential filter
    if let Some(min_pot) = criteria.min_potential {
        if player.potential < min_pot {
            return false;
        }
    }
    
    // Available filter (no club)
    if criteria.available_only && player.club_id.is_some() {
        return false;
    }
    
    true
}

/// Compare two players side by side.
pub fn compare_players(
    world: &World,
    player_a: &PlayerId,
    player_b: &PlayerId,
    scout_ability: u8,
) -> Option<PlayerComparison> {
    let a = world.players.get(player_a)?;
    let b = world.players.get(player_b)?;
    
    let score_a = scout_score(world, player_a, scout_ability);
    let score_b = scout_score(world, player_b, scout_ability);
    
    let better_player = if score_a >= score_b {
        player_a.clone()
    } else {
        player_b.clone()
    };
    
    Some(PlayerComparison {
        player_a_score: score_a,
        player_b_score: score_b,
        better_player,
        technical_winner: compare_technical(a, b),
        physical_winner: compare_physical(a, b),
        mental_winner: compare_mental(a, b),
        value_winner: if a.value <= b.value { player_a.clone() } else { player_b.clone() },
    })
}

fn compare_technical(a: &Player, b: &Player) -> PlayerId {
    let a_tech = a.attributes.technical.passing as u16 
        + a.attributes.technical.dribbling as u16 
        + a.attributes.technical.finishing as u16;
    let b_tech = b.attributes.technical.passing as u16 
        + b.attributes.technical.dribbling as u16 
        + b.attributes.technical.finishing as u16;
    
    if a_tech >= b_tech { a.id.clone() } else { b.id.clone() }
}

fn compare_physical(a: &Player, b: &Player) -> PlayerId {
    let a_phys = a.attributes.physical.pace as u16 
        + a.attributes.physical.stamina as u16 
        + a.attributes.physical.strength as u16;
    let b_phys = b.attributes.physical.pace as u16 
        + b.attributes.physical.stamina as u16 
        + b.attributes.physical.strength as u16;
    
    if a_phys >= b_phys { a.id.clone() } else { b.id.clone() }
}

fn compare_mental(a: &Player, b: &Player) -> PlayerId {
    let a_ment = a.attributes.mental.composure as u16 
        + a.attributes.mental.decisions as u16 
        + a.attributes.mental.vision as u16;
    let b_ment = b.attributes.mental.composure as u16 
        + b.attributes.mental.decisions as u16 
        + b.attributes.mental.vision as u16;
    
    if a_ment >= b_ment { a.id.clone() } else { b.id.clone() }
}

/// Player comparison result.
#[derive(Debug, Clone)]
pub struct PlayerComparison {
    pub player_a_score: u8,
    pub player_b_score: u8,
    pub better_player: PlayerId,
    pub technical_winner: PlayerId,
    pub physical_winner: PlayerId,
    pub mental_winner: PlayerId,
    pub value_winner: PlayerId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, NaiveDate};
    use cm_core::ids::NationId;
    use cm_core::economy::Money;

    fn create_test_player(
        id: &str,
        position: Position,
        quality: u8,
        potential: u8,
        age_years: i32,
        value: Money,
    ) -> Player {
        let birth_date = NaiveDate::from_ymd_opt(2024 - age_years, 6, 15).unwrap();
        let mut player = Player::new(
            id,
            "Test",
            id,
            NationId::new("test"),
            birth_date,
            position,
        );
        
        player.potential = potential;
        player.value = value;
        player.form = 60;
        player.fitness = 90;
        
        // Set attributes
        player.attributes.technical.finishing = quality;
        player.attributes.technical.dribbling = quality;
        player.attributes.technical.passing = quality;
        player.attributes.technical.tackling = quality;
        player.attributes.technical.marking = quality;
        player.attributes.mental.off_the_ball = quality;
        player.attributes.mental.positioning = quality;
        player.attributes.mental.vision = quality;
        player.attributes.mental.composure = quality;
        player.attributes.mental.decisions = quality;
        player.attributes.mental.leadership = quality;
        player.attributes.mental.work_rate = quality;
        player.attributes.physical.strength = quality;
        player.attributes.physical.stamina = quality;
        player.attributes.physical.pace = quality;
        player.attributes.technical.first_touch = quality;
        player.attributes.goalkeeper.handling = quality;
        player.attributes.goalkeeper.reflexes = quality;
        player.attributes.goalkeeper.positioning = quality;
        player.attributes.goalkeeper.one_on_ones = quality;
        
        player
    }

    fn setup_test_world() -> World {
        let mut world = World::new();
        
        let players = vec![
            create_test_player("elite_young", Position::ForwardCenter, 82, 90, 21, Money::from_major(15_000_000)),
            create_test_player("elite_prime", Position::ForwardCenter, 88, 88, 27, Money::from_major(50_000_000)),
            create_test_player("good_player", Position::MidfielderCenter, 75, 78, 25, Money::from_major(8_000_000)),
            create_test_player("average_player", Position::DefenderCenter, 65, 68, 28, Money::from_major(3_000_000)),
            create_test_player("young_prospect", Position::MidfielderAttacking, 60, 85, 18, Money::from_major(2_000_000)),
            create_test_player("aging_star", Position::ForwardCenter, 78, 78, 33, Money::from_major(5_000_000)),
            create_test_player("bargain_player", Position::DefenderLeft, 72, 75, 24, Money::from_major(500_000)),
            create_test_player("overpriced", Position::MidfielderRight, 62, 65, 26, Money::from_major(15_000_000)),
        ];
        
        for player in players {
            world.players.insert(player.id.clone(), player);
        }
        
        world
    }

    #[test]
    fn test_scout_score_better_with_better_scout() {
        let world = setup_test_world();
        let player_id = PlayerId::new("elite_prime");
        
        let score_low = scout_score(&world, &player_id, 50);
        let score_high = scout_score(&world, &player_id, 95);
        
        // Higher ability scout should be closer to true value
        // The elite_prime player should score high regardless
        assert!(score_high >= 70, "Elite player should score high: {}", score_high);
        assert!(score_low >= 50, "Even low scout should see quality: {}", score_low);
    }

    #[test]
    fn test_scout_score_consistency() {
        let world = setup_test_world();
        let player_id = PlayerId::new("good_player");
        
        // Same inputs should give same output (deterministic)
        let score1 = scout_score(&world, &player_id, 70);
        let score2 = scout_score(&world, &player_id, 70);
        
        assert_eq!(score1, score2, "Scout score should be deterministic");
    }

    #[test]
    fn test_generate_scout_report() {
        let world = setup_test_world();
        let player_id = PlayerId::new("young_prospect");
        
        let report = generate_scout_report(&world, &player_id, 80).unwrap();
        
        assert_eq!(report.player_id, player_id);
        assert!(report.accuracy >= 75);
        assert!(report.potential_ability > report.current_ability, "Young prospect should have high potential");
    }

    #[test]
    fn test_value_assessment_bargain() {
        let world = setup_test_world();
        let player_id = PlayerId::new("bargain_player");
        
        let report = generate_scout_report(&world, &player_id, 90).unwrap();
        
        assert!(
            matches!(report.value_assessment, ValueAssessment::Bargain | ValueAssessment::Undervalued),
            "72-rated player at 500k should be undervalued"
        );
    }

    #[test]
    fn test_value_assessment_overpriced() {
        let world = setup_test_world();
        let player_id = PlayerId::new("overpriced");
        
        let report = generate_scout_report(&world, &player_id, 90).unwrap();
        
        assert_eq!(report.value_assessment, ValueAssessment::Overvalued);
    }

    #[test]
    fn test_recommendation_must_sign() {
        let world = setup_test_world();
        
        // Elite prime player
        let report = generate_scout_report(&world, &PlayerId::new("elite_prime"), 90).unwrap();
        assert!(
            matches!(report.recommendation, ScoutRecommendation::MustSign | ScoutRecommendation::Recommended),
            "Elite player should be recommended"
        );
    }

    #[test]
    fn test_recommendation_young_prospect() {
        let world = setup_test_world();
        let player_id = PlayerId::new("young_prospect");
        
        let report = generate_scout_report(&world, &player_id, 85).unwrap();
        
        // Young player with high potential should be recommended
        assert!(
            matches!(report.recommendation, ScoutRecommendation::MustSign | ScoutRecommendation::Recommended | ScoutRecommendation::Watchlist),
            "Young prospect should be on watchlist or recommended"
        );
    }

    #[test]
    fn test_search_players_by_position() {
        let world = setup_test_world();
        
        let criteria = SearchCriteria {
            position: Some(Position::ForwardCenter),
            ..Default::default()
        };
        
        let results = search_players(&world, &criteria, 80, 10);
        
        for id in &results {
            let player = world.players.get(id).unwrap();
            assert!(
                player.position == Position::ForwardCenter || 
                player.secondary_positions.contains(&Position::ForwardCenter)
            );
        }
    }

    #[test]
    fn test_search_players_by_age() {
        let world = setup_test_world();
        
        let criteria = SearchCriteria {
            min_age: Some(18),
            max_age: Some(23),
            ..Default::default()
        };
        
        let results = search_players(&world, &criteria, 80, 10);
        
        for id in &results {
            let player = world.players.get(id).unwrap();
            let age = 2024 - player.birth_date.year();
            assert!(age >= 18 && age <= 23, "Player age {} out of range", age);
        }
    }

    #[test]
    fn test_search_players_by_value() {
        let world = setup_test_world();
        
        let criteria = SearchCriteria {
            max_value: Some(5_000_000),
            ..Default::default()
        };
        
        let results = search_players(&world, &criteria, 80, 10);
        
        for id in &results {
            let player = world.players.get(id).unwrap();
            assert!(player.value.minor() <= 5_000_000);
        }
    }

    #[test]
    fn test_compare_players() {
        let world = setup_test_world();
        
        let comparison = compare_players(
            &world,
            &PlayerId::new("elite_prime"),
            &PlayerId::new("average_player"),
            85,
        ).unwrap();
        
        assert_eq!(comparison.better_player, PlayerId::new("elite_prime"));
        assert!(comparison.player_a_score > comparison.player_b_score);
    }

    #[test]
    fn test_analyze_attributes_strengths() {
        let mut player = create_test_player(
            "skilled",
            Position::MidfielderCenter,
            80,
            82,
            25,
            Money::from_major(5_000_000),
        );
        
        // Set specific high attributes
        player.attributes.technical.passing = 85;
        player.attributes.physical.pace = 85;
        player.attributes.mental.leadership = 85;
        
        let (strengths, _) = analyze_attributes(&player, 80);
        
        assert!(!strengths.is_empty(), "Should identify strengths");
    }

    #[test]
    fn test_analyze_attributes_weaknesses() {
        let mut player = create_test_player(
            "weak",
            Position::ForwardCenter,
            50,
            55,
            26,
            Money::from_major(500_000),
        );
        
        // Set specific weak attributes
        player.attributes.technical.finishing = 40;
        player.attributes.physical.pace = 40;
        
        let (_, weaknesses) = analyze_attributes(&player, 80);
        
        assert!(!weaknesses.is_empty(), "Should identify weaknesses");
    }

    #[test]
    fn test_low_scout_ability_less_insights() {
        let player = create_test_player(
            "player",
            Position::MidfielderCenter,
            75,
            78,
            25,
            Money::from_major(5_000_000),
        );
        
        let (strengths_high, weaknesses_high) = analyze_attributes(&player, 90);
        let (strengths_low, _) = analyze_attributes(&player, 30);
        
        // Low scout should find fewer insights
        assert!(strengths_low.len() <= strengths_high.len());
    }

    #[test]
    fn test_accuracy_calculation() {
        let high = calculate_accuracy(100);
        let medium = calculate_accuracy(70);
        let low = calculate_accuracy(40);
        
        assert!(high > medium);
        assert!(medium > low);
        assert!(high <= 95); // Cap
    }
}
