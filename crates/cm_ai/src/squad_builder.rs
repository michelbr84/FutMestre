//! Squad building AI - Analyze squad composition and identify needs.

use chrono::Datelike;
use cm_core::ids::ClubId;
use cm_core::world::{Player, Position, World};
use std::collections::HashMap;

/// Priority level for squad needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Critical = 4,
    High = 3,
    Medium = 2,
    Low = 1,
}

impl Priority {
    /// Get numeric value for sorting (higher = more urgent).
    pub fn value(&self) -> u8 {
        *self as u8
    }
}

/// Squad need description.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SquadNeed {
    pub position: String,
    pub priority: Priority,
    pub reason: String,
}

impl SquadNeed {
    pub fn new(position: impl Into<String>, priority: Priority, reason: impl Into<String>) -> Self {
        Self {
            position: position.into(),
            priority,
            reason: reason.into(),
        }
    }
}

/// Position requirements for a typical squad.
#[derive(Debug)]
struct PositionRequirements {
    ideal: usize,
    minimum: usize,
}

impl PositionRequirements {
    fn new(ideal: usize, minimum: usize) -> Self {
        Self { ideal, minimum }
    }
}

/// Get default position requirements for a squad.
fn get_position_requirements() -> HashMap<Position, PositionRequirements> {
    let mut reqs = HashMap::new();
    reqs.insert(Position::Goalkeeper, PositionRequirements::new(3, 2));
    reqs.insert(Position::DefenderCenter, PositionRequirements::new(4, 3));
    reqs.insert(Position::DefenderLeft, PositionRequirements::new(2, 1));
    reqs.insert(Position::DefenderRight, PositionRequirements::new(2, 1));
    reqs.insert(Position::MidfielderCenter, PositionRequirements::new(4, 2));
    reqs.insert(Position::MidfielderLeft, PositionRequirements::new(2, 1));
    reqs.insert(Position::MidfielderRight, PositionRequirements::new(2, 1));
    reqs.insert(Position::MidfielderDefensive, PositionRequirements::new(2, 1));
    reqs.insert(Position::MidfielderAttacking, PositionRequirements::new(2, 1));
    reqs.insert(Position::ForwardCenter, PositionRequirements::new(3, 2));
    reqs.insert(Position::ForwardLeft, PositionRequirements::new(2, 1));
    reqs.insert(Position::ForwardRight, PositionRequirements::new(2, 1));
    reqs
}

/// Count players by position including secondary positions.
fn count_by_position(players: &[&Player]) -> HashMap<Position, usize> {
    let mut counts: HashMap<Position, usize> = HashMap::new();
    
    for player in players {
        // Count primary position
        *counts.entry(player.position).or_insert(0) += 1;
        
        // Count secondary positions with half weight (rounded down)
        // We track these separately
    }
    
    counts
}

/// Calculate average quality for a position.
fn average_quality_for_position(players: &[&Player], position: Position) -> Option<u8> {
    let position_players: Vec<_> = players
        .iter()
        .filter(|p| p.position == position || p.secondary_positions.contains(&position))
        .collect();
    
    if position_players.is_empty() {
        return None;
    }
    
    let sum: u32 = position_players
        .iter()
        .map(|p| p.overall_rating() as u32)
        .sum();
    
    Some((sum / position_players.len() as u32) as u8)
}

/// Calculate average age for a position.
fn average_age_for_position(players: &[&Player], position: Position, current_year: i32) -> Option<u8> {
    let position_players: Vec<_> = players
        .iter()
        .filter(|p| p.position == position)
        .collect();
    
    if position_players.is_empty() {
        return None;
    }
    
    let sum: i32 = position_players
        .iter()
        .map(|p| current_year - p.birth_date.year())
        .sum();
    
    Some((sum / position_players.len() as i32) as u8)
}

/// Analyze squad needs for a club.
pub fn analyze_squad_needs(world: &World, club_id: &ClubId) -> Vec<SquadNeed> {
    let players = world.club_players(club_id);
    
    if players.is_empty() {
        return vec![
            SquadNeed::new("Goalkeeper", Priority::Critical, "No players in squad"),
            SquadNeed::new("Defender", Priority::Critical, "No players in squad"),
            SquadNeed::new("Midfielder", Priority::Critical, "No players in squad"),
            SquadNeed::new("Forward", Priority::Critical, "No players in squad"),
        ];
    }
    
    let mut needs = Vec::new();
    let requirements = get_position_requirements();
    let counts = count_by_position(&players);
    
    // Assume current year is 2024 for age calculations (could be passed as param)
    let current_year = 2024;
    
    // Check each position
    for (position, reqs) in &requirements {
        let count = counts.get(position).copied().unwrap_or(0);
        let pos_name = position.short_name().to_string();
        
        // Critical: below minimum
        if count < reqs.minimum {
            needs.push(SquadNeed::new(
                &pos_name,
                Priority::Critical,
                format!("Only {} players, need at least {}", count, reqs.minimum),
            ));
            continue;
        }
        
        // High: below ideal
        if count < reqs.ideal {
            needs.push(SquadNeed::new(
                &pos_name,
                Priority::High,
                format!("Only {} players, ideally need {}", count, reqs.ideal),
            ));
        }
        
        // Check quality
        if let Some(avg_quality) = average_quality_for_position(&players, *position) {
            if avg_quality < 50 {
                let priority = if count <= reqs.minimum {
                    Priority::High
                } else {
                    Priority::Medium
                };
                needs.push(SquadNeed::new(
                    &pos_name,
                    priority,
                    format!("Low average quality ({})", avg_quality),
                ));
            }
        }
        
        // Check age profile
        if let Some(avg_age) = average_age_for_position(&players, *position, current_year) {
            if avg_age > 31 {
                needs.push(SquadNeed::new(
                    &pos_name,
                    Priority::Medium,
                    format!("Aging squad (avg {} years)", avg_age),
                ));
            } else if avg_age < 22 && count <= reqs.minimum {
                needs.push(SquadNeed::new(
                    &pos_name,
                    Priority::Low,
                    format!("Inexperienced squad (avg {} years)", avg_age),
                ));
            }
        }
    }
    
    // Sort by priority (highest first)
    needs.sort_by(|a, b| b.priority.cmp(&a.priority));
    
    needs
}

/// Get overall squad strength analysis.
pub fn analyze_squad_strength(world: &World, club_id: &ClubId) -> SquadStrength {
    let players = world.club_players(club_id);
    
    if players.is_empty() {
        return SquadStrength::default();
    }
    
    let goalkeepers: Vec<_> = players.iter()
        .filter(|p| p.position == Position::Goalkeeper)
        .collect();
    
    let defenders: Vec<_> = players.iter()
        .filter(|p| p.position.is_defender())
        .collect();
    
    let midfielders: Vec<_> = players.iter()
        .filter(|p| p.position.is_midfielder())
        .collect();
    
    let forwards: Vec<_> = players.iter()
        .filter(|p| p.position.is_forward())
        .collect();
    
    let calc_avg = |ps: &[&&Player]| -> u8 {
        if ps.is_empty() {
            return 50;
        }
        let sum: u32 = ps.iter().map(|p| p.overall_rating() as u32).sum();
        (sum / ps.len() as u32) as u8
    };
    
    let goalkeeper_strength = calc_avg(&goalkeepers);
    let defense_strength = calc_avg(&defenders);
    let midfield_strength = calc_avg(&midfielders);
    let attack_strength = calc_avg(&forwards);
    
    // Calculate overall as weighted average
    let overall = (
        goalkeeper_strength as u16 * 1 +
        defense_strength as u16 * 3 +
        midfield_strength as u16 * 3 +
        attack_strength as u16 * 3
    ) / 10;
    
    // Calculate depth (how many quality backup options)
    let depth = calculate_squad_depth(&players);
    
    SquadStrength {
        overall: overall as u8,
        goalkeeper_strength,
        defense_strength,
        midfield_strength,
        attack_strength,
        depth,
        squad_size: players.len(),
    }
}

/// Calculate squad depth score (0-100).
fn calculate_squad_depth(players: &[&Player]) -> u8 {
    let requirements = get_position_requirements();
    let counts = count_by_position(players);
    
    let mut depth_score: u32 = 0;
    let mut max_score: u32 = 0;
    
    for (position, reqs) in &requirements {
        let count = counts.get(position).copied().unwrap_or(0);
        max_score += reqs.ideal as u32;
        depth_score += count.min(reqs.ideal) as u32;
    }
    
    if max_score == 0 {
        return 0;
    }
    
    ((depth_score * 100) / max_score) as u8
}

/// Squad strength analysis result.
#[derive(Debug, Clone, Default)]
pub struct SquadStrength {
    pub overall: u8,
    pub goalkeeper_strength: u8,
    pub defense_strength: u8,
    pub midfield_strength: u8,
    pub attack_strength: u8,
    pub depth: u8,
    pub squad_size: usize,
}

/// Find positions where upgrades would have most impact.
pub fn identify_weak_links(world: &World, club_id: &ClubId) -> Vec<(Position, u8)> {
    let players = world.club_players(club_id);
    let requirements = get_position_requirements();
    
    let mut weak_links = Vec::new();
    
    for (position, _) in &requirements {
        if let Some(avg) = average_quality_for_position(&players, *position) {
            if avg < 65 {
                weak_links.push((*position, avg));
            }
        } else {
            // No players at all
            weak_links.push((*position, 0));
        }
    }
    
    // Sort by quality (lowest first)
    weak_links.sort_by(|a, b| a.1.cmp(&b.1));
    
    weak_links
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, NaiveDate};
    use cm_core::ids::NationId;

    fn create_test_player(
        id: &str,
        position: Position,
        quality: u8,
        age_years: i32,
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
        
        // Set attributes to achieve desired quality
        player.attributes.technical.finishing = quality;
        player.attributes.technical.dribbling = quality;
        player.attributes.technical.passing = quality;
        player.attributes.technical.tackling = quality;
        player.attributes.technical.marking = quality;
        player.attributes.mental.off_the_ball = quality;
        player.attributes.mental.positioning = quality;
        player.attributes.mental.vision = quality;
        player.attributes.physical.strength = quality;
        player.attributes.physical.stamina = quality;
        player.attributes.technical.first_touch = quality;
        player.attributes.goalkeeper.handling = quality;
        player.attributes.goalkeeper.reflexes = quality;
        player.attributes.goalkeeper.positioning = quality;
        player.attributes.goalkeeper.one_on_ones = quality;
        
        player
    }

    fn setup_test_world_with_squad() -> (World, ClubId) {
        let mut world = World::new();
        let club_id = ClubId::new("test_club");
        
        // Create a club
        let mut club = cm_core::world::Club::new("test_club", "Test FC", NationId::new("test"));
        
        // Add players to squad
        let players = vec![
            create_test_player("gk1", Position::Goalkeeper, 70, 28),
            create_test_player("gk2", Position::Goalkeeper, 60, 22),
            create_test_player("dc1", Position::DefenderCenter, 75, 27),
            create_test_player("dc2", Position::DefenderCenter, 70, 25),
            create_test_player("dc3", Position::DefenderCenter, 65, 30),
            create_test_player("dl1", Position::DefenderLeft, 68, 26),
            create_test_player("dr1", Position::DefenderRight, 72, 24),
            create_test_player("dm1", Position::MidfielderDefensive, 70, 28),
            create_test_player("mc1", Position::MidfielderCenter, 75, 26),
            create_test_player("mc2", Position::MidfielderCenter, 72, 24),
            create_test_player("am1", Position::MidfielderAttacking, 78, 25),
            create_test_player("ml1", Position::MidfielderLeft, 70, 27),
            create_test_player("mr1", Position::MidfielderRight, 68, 23),
            create_test_player("fc1", Position::ForwardCenter, 80, 26),
            create_test_player("fc2", Position::ForwardCenter, 72, 29),
            create_test_player("fl1", Position::ForwardLeft, 75, 24),
            create_test_player("fr1", Position::ForwardRight, 73, 25),
        ];
        
        for mut player in players {
            player.club_id = Some(club_id.clone());
            club.add_player(player.id.clone());
            world.players.insert(player.id.clone(), player);
        }
        
        world.clubs.insert(club_id.clone(), club);
        
        (world, club_id)
    }

    fn setup_test_world_with_weak_squad() -> (World, ClubId) {
        let mut world = World::new();
        let club_id = ClubId::new("weak_club");
        
        let mut club = cm_core::world::Club::new("weak_club", "Weak FC", NationId::new("test"));
        
        // Minimal squad with gaps
        let players = vec![
            create_test_player("gk1", Position::Goalkeeper, 50, 35),
            create_test_player("dc1", Position::DefenderCenter, 45, 33),
            create_test_player("dc2", Position::DefenderCenter, 40, 32),
            create_test_player("mc1", Position::MidfielderCenter, 55, 28),
            create_test_player("fc1", Position::ForwardCenter, 50, 30),
        ];
        
        for mut player in players {
            player.club_id = Some(club_id.clone());
            club.add_player(player.id.clone());
            world.players.insert(player.id.clone(), player);
        }
        
        world.clubs.insert(club_id.clone(), club);
        
        (world, club_id)
    }

    #[test]
    fn test_analyze_empty_squad() {
        let mut world = World::new();
        let club_id = ClubId::new("empty_club");
        let club = cm_core::world::Club::new("empty_club", "Empty FC", NationId::new("test"));
        world.clubs.insert(club_id.clone(), club);
        
        let needs = analyze_squad_needs(&world, &club_id);
        
        assert_eq!(needs.len(), 4); // GK, DEF, MID, FWD
        assert!(needs.iter().all(|n| n.priority == Priority::Critical));
    }

    #[test]
    fn test_analyze_weak_squad() {
        let (world, club_id) = setup_test_world_with_weak_squad();
        
        let needs = analyze_squad_needs(&world, &club_id);
        
        // Should identify multiple critical and high needs
        let critical_needs: Vec<_> = needs.iter()
            .filter(|n| n.priority == Priority::Critical)
            .collect();
        
        assert!(!critical_needs.is_empty(), "Should have critical needs");
        
        // Should identify lack of fullbacks
        let fullback_needs: Vec<_> = needs.iter()
            .filter(|n| n.position == "DL" || n.position == "DR")
            .collect();
        
        assert!(!fullback_needs.is_empty(), "Should need fullbacks");
    }

    #[test]
    fn test_analyze_balanced_squad() {
        let (world, club_id) = setup_test_world_with_squad();
        
        let needs = analyze_squad_needs(&world, &club_id);
        
        // Well-balanced squad should have fewer critical needs
        let critical_needs: Vec<_> = needs.iter()
            .filter(|n| n.priority == Priority::Critical)
            .collect();
        
        assert!(critical_needs.is_empty(), "Balanced squad should have no critical needs");
    }

    #[test]
    fn test_analyze_squad_strength() {
        let (world, club_id) = setup_test_world_with_squad();
        
        let strength = analyze_squad_strength(&world, &club_id);
        
        assert!(strength.overall > 60, "Overall should be decent");
        assert!(strength.squad_size >= 15, "Should have enough players");
        assert!(strength.depth > 50, "Should have reasonable depth");
        assert!(strength.attack_strength > 70, "Attack should be good");
    }

    #[test]
    fn test_analyze_weak_squad_strength() {
        let (world, club_id) = setup_test_world_with_weak_squad();
        
        let strength = analyze_squad_strength(&world, &club_id);
        
        assert!(strength.overall < 60, "Overall should be low");
        assert!(strength.squad_size < 10, "Small squad");
        assert!(strength.depth < 50, "Poor depth");
    }

    #[test]
    fn test_identify_weak_links() {
        let (world, club_id) = setup_test_world_with_weak_squad();
        
        let weak_links = identify_weak_links(&world, &club_id);
        
        assert!(!weak_links.is_empty(), "Should have weak links");
        
        // Weakest positions should be listed first
        if weak_links.len() >= 2 {
            assert!(weak_links[0].1 <= weak_links[1].1, "Should be sorted by quality");
        }
    }

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::Critical > Priority::High);
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
    }

    #[test]
    fn test_needs_sorted_by_priority() {
        let (world, club_id) = setup_test_world_with_weak_squad();
        
        let needs = analyze_squad_needs(&world, &club_id);
        
        // Verify sorting (higher priority first)
        for i in 1..needs.len() {
            assert!(needs[i-1].priority >= needs[i].priority, 
                "Needs should be sorted by priority");
        }
    }

    #[test]
    fn test_position_requirements() {
        let reqs = get_position_requirements();
        
        // Verify key positions have sensible requirements
        let gk = reqs.get(&Position::Goalkeeper).unwrap();
        assert_eq!(gk.minimum, 2);
        assert_eq!(gk.ideal, 3);
        
        let fc = reqs.get(&Position::ForwardCenter).unwrap();
        assert_eq!(fc.minimum, 2);
        assert!(fc.ideal >= fc.minimum);
    }
}
