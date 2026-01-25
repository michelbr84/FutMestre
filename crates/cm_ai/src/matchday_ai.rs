//! Matchday AI - Lineup selection and match tactics.

use cm_core::ids::{ClubId, PlayerId};
use cm_core::world::{Formation, Mentality, Player, Position, Tactics, Tempo, World};

/// Starting lineup with formation positions.
#[derive(Debug, Clone)]
pub struct Lineup {
    pub formation: Formation,
    pub players: Vec<PlayerId>,
    pub captain: Option<PlayerId>,
}

impl Lineup {
    pub fn new(formation: Formation) -> Self {
        Self {
            formation,
            players: Vec::new(),
            captain: None,
        }
    }
    
    /// Check if lineup is complete (11 players).
    pub fn is_complete(&self) -> bool {
        self.players.len() == 11
    }
}

/// Select starting lineup for a club.
/// Returns list of player IDs for the best available 11.
pub fn select_lineup(world: &World, club_id: &ClubId) -> Vec<PlayerId> {
    let lineup = select_lineup_with_formation(world, club_id, Formation::default());
    lineup.players
}

/// Select lineup with specific formation.
pub fn select_lineup_with_formation(
    world: &World,
    club_id: &ClubId,
    formation: Formation,
) -> Lineup {
    let players = world.club_players(club_id);
    let available: Vec<&Player> = players.into_iter()
        .filter(|p| p.is_available())
        .collect();
    
    let mut lineup = Lineup::new(formation);
    let mut selected: Vec<PlayerId> = Vec::new();
    
    // Required positions based on formation
    let positions_needed = get_positions_for_formation(formation);
    
    // Select best player for each position
    for position in positions_needed {
        if let Some(player) = find_best_available_for_position(&available, &selected, position) {
            selected.push(player.id.clone());
        }
    }
    
    // If we don't have 11, fill with best remaining available
    while selected.len() < 11 {
        if let Some(best) = available.iter()
            .filter(|p| !selected.contains(&p.id))
            .max_by_key(|p| p.overall_rating())
        {
            selected.push(best.id.clone());
        } else {
            break;
        }
    }
    
    // Select captain (highest leadership among selected)
    let captain = selected.iter()
        .filter_map(|id| {
            world.players.get(id)
                .map(|p| (id.clone(), p.attributes.mental.leadership))
        })
        .max_by_key(|(_, leadership)| *leadership)
        .map(|(id, _)| id);
    
    lineup.players = selected;
    lineup.captain = captain;
    lineup
}

/// Get required positions for a formation.
fn get_positions_for_formation(formation: Formation) -> Vec<Position> {
    let mut positions = vec![Position::Goalkeeper];
    
    match formation {
        Formation::F442 => {
            positions.extend([
                Position::DefenderLeft, Position::DefenderCenter, 
                Position::DefenderCenter, Position::DefenderRight,
                Position::MidfielderLeft, Position::MidfielderCenter,
                Position::MidfielderCenter, Position::MidfielderRight,
                Position::ForwardCenter, Position::ForwardCenter,
            ]);
        }
        Formation::F433 => {
            positions.extend([
                Position::DefenderLeft, Position::DefenderCenter,
                Position::DefenderCenter, Position::DefenderRight,
                Position::MidfielderCenter, Position::MidfielderCenter,
                Position::MidfielderCenter,
                Position::ForwardLeft, Position::ForwardCenter, Position::ForwardRight,
            ]);
        }
        Formation::F352 => {
            positions.extend([
                Position::DefenderCenter, Position::DefenderCenter, Position::DefenderCenter,
                Position::MidfielderLeft, Position::MidfielderDefensive,
                Position::MidfielderCenter, Position::MidfielderCenter, Position::MidfielderRight,
                Position::ForwardCenter, Position::ForwardCenter,
            ]);
        }
        Formation::F451 => {
            positions.extend([
                Position::DefenderLeft, Position::DefenderCenter,
                Position::DefenderCenter, Position::DefenderRight,
                Position::MidfielderLeft, Position::MidfielderCenter,
                Position::MidfielderDefensive, Position::MidfielderCenter, Position::MidfielderRight,
                Position::ForwardCenter,
            ]);
        }
        Formation::F4231 => {
            positions.extend([
                Position::DefenderLeft, Position::DefenderCenter,
                Position::DefenderCenter, Position::DefenderRight,
                Position::MidfielderDefensive, Position::MidfielderDefensive,
                Position::MidfielderLeft, Position::MidfielderAttacking, Position::MidfielderRight,
                Position::ForwardCenter,
            ]);
        }
        Formation::F532 => {
            positions.extend([
                Position::DefenderLeft, Position::DefenderCenter, Position::DefenderCenter,
                Position::DefenderCenter, Position::DefenderRight,
                Position::MidfielderCenter, Position::MidfielderDefensive, Position::MidfielderCenter,
                Position::ForwardCenter, Position::ForwardCenter,
            ]);
        }
        Formation::F4141 => {
            positions.extend([
                Position::DefenderLeft, Position::DefenderCenter,
                Position::DefenderCenter, Position::DefenderRight,
                Position::MidfielderDefensive,
                Position::MidfielderLeft, Position::MidfielderCenter,
                Position::MidfielderCenter, Position::MidfielderRight,
                Position::ForwardCenter,
            ]);
        }
        Formation::F4411 => {
            positions.extend([
                Position::DefenderLeft, Position::DefenderCenter,
                Position::DefenderCenter, Position::DefenderRight,
                Position::MidfielderLeft, Position::MidfielderCenter,
                Position::MidfielderCenter, Position::MidfielderRight,
                Position::MidfielderAttacking,
                Position::ForwardCenter,
            ]);
        }
        Formation::F3412 => {
            positions.extend([
                Position::DefenderCenter, Position::DefenderCenter, Position::DefenderCenter,
                Position::MidfielderLeft, Position::MidfielderCenter,
                Position::MidfielderCenter, Position::MidfielderRight,
                Position::MidfielderAttacking,
                Position::ForwardCenter, Position::ForwardCenter,
            ]);
        }
        Formation::F343 => {
            positions.extend([
                Position::DefenderCenter, Position::DefenderCenter, Position::DefenderCenter,
                Position::MidfielderLeft, Position::MidfielderCenter,
                Position::MidfielderCenter, Position::MidfielderRight,
                Position::ForwardLeft, Position::ForwardCenter, Position::ForwardRight,
            ]);
        }
    }
    
    positions
}

/// Find best available player for a position.
fn find_best_available_for_position<'a>(
    available: &[&'a Player],
    already_selected: &[PlayerId],
    position: Position,
) -> Option<&'a Player> {
    // First try exact position match
    let mut candidates: Vec<_> = available.iter()
        .filter(|p| !already_selected.contains(&p.id))
        .filter(|p| p.position == position)
        .copied()
        .collect();
    
    // If no exact match, try players with this as secondary position
    if candidates.is_empty() {
        candidates = available.iter()
            .filter(|p| !already_selected.contains(&p.id))
            .filter(|p| p.secondary_positions.contains(&position))
            .copied()
            .collect();
    }
    
    // If still none, try similar positions
    if candidates.is_empty() {
        let similar = get_similar_positions(position);
        candidates = available.iter()
            .filter(|p| !already_selected.contains(&p.id))
            .filter(|p| similar.contains(&p.position))
            .copied()
            .collect();
    }
    
    // Sort by rating and form, pick best
    candidates.sort_by(|a, b| {
        let a_score = (a.overall_rating() as u16 * 2) + (a.form as u16);
        let b_score = (b.overall_rating() as u16 * 2) + (b.form as u16);
        b_score.cmp(&a_score)
    });
    
    candidates.first().copied()
}

/// Get similar positions that could fill in.
fn get_similar_positions(position: Position) -> Vec<Position> {
    match position {
        Position::Goalkeeper => vec![],
        Position::DefenderCenter => vec![Position::MidfielderDefensive],
        Position::DefenderLeft => vec![Position::MidfielderLeft, Position::DefenderCenter],
        Position::DefenderRight => vec![Position::MidfielderRight, Position::DefenderCenter],
        Position::MidfielderCenter => vec![Position::MidfielderDefensive, Position::MidfielderAttacking],
        Position::MidfielderLeft => vec![Position::ForwardLeft, Position::DefenderLeft],
        Position::MidfielderRight => vec![Position::ForwardRight, Position::DefenderRight],
        Position::MidfielderDefensive => vec![Position::MidfielderCenter, Position::DefenderCenter],
        Position::MidfielderAttacking => vec![Position::MidfielderCenter, Position::ForwardCenter],
        Position::ForwardCenter => vec![Position::MidfielderAttacking, Position::ForwardLeft, Position::ForwardRight],
        Position::ForwardLeft => vec![Position::MidfielderLeft, Position::ForwardCenter],
        Position::ForwardRight => vec![Position::MidfielderRight, Position::ForwardCenter],
    }
}

/// Select tactics for a match against an opponent.
pub fn select_tactics(world: &World, club_id: &ClubId, opponent_id: &ClubId) -> Tactics {
    let own_players = world.club_players(club_id);
    let opponent_players = world.club_players(opponent_id);
    
    // Analyze own squad
    let own_strength = calculate_squad_strength(&own_players);
    let own_fitness = calculate_average_fitness(&own_players);
    
    // Analyze opponent
    let opponent_strength = calculate_squad_strength(&opponent_players);
    
    // Get reputations
    let own_rep = world.clubs.get(club_id)
        .map(|c| c.reputation)
        .unwrap_or(50);
    let opponent_rep = world.clubs.get(opponent_id)
        .map(|c| c.reputation)
        .unwrap_or(50);
    
    // Determine approach based on relative strength
    let strength_diff = own_strength.0 as i16 - opponent_strength.0 as i16;
    let rep_diff = own_rep as i16 - opponent_rep as i16;
    
    // Choose formation based on relative strength
    let formation = if strength_diff > 10 || rep_diff > 20 {
        // Stronger - attack
        Formation::F433
    } else if strength_diff < -10 || rep_diff < -20 {
        // Weaker - defend
        Formation::F451
    } else {
        // Balanced
        Formation::F442
    };
    
    // Choose mentality
    let mentality = if rep_diff > 20 {
        Mentality::Attacking
    } else if rep_diff < -20 {
        Mentality::Cautious
    } else {
        Mentality::Balanced
    };
    
    // Choose tempo based on fitness
    let tempo = if own_fitness > 80 {
        Tempo::Fast
    } else if own_fitness < 60 {
        Tempo::Slow
    } else {
        Tempo::Normal
    };
    
    // Pressing based on fitness and approach
    let pressing = match (own_fitness > 70, strength_diff > 0) {
        (true, true) => 70,   // Fit and strong - press high
        (true, false) => 55,  // Fit but weaker - moderate press
        (false, true) => 45,  // Less fit but strong - conserve energy
        (false, false) => 35, // Less fit and weaker - stay compact
    };
    
    // Defensive line
    let defensive_line = if strength_diff > 10 {
        65 // Higher line when dominant
    } else if strength_diff < -10 {
        35 // Deeper when outmatched
    } else {
        50
    };
    
    Tactics {
        formation,
        mentality,
        tempo,
        pressing,
        defensive_line,
        width: 50,
        direct_passing: if rep_diff < -15 { 65 } else { 45 },
    }
}

/// Calculate squad strength (attack, defense).
fn calculate_squad_strength(players: &[&Player]) -> (u8, u8) {
    if players.is_empty() {
        return (50, 50);
    }
    
    let attackers: Vec<_> = players.iter()
        .filter(|p| p.position.is_forward() || p.position == Position::MidfielderAttacking)
        .collect();
    
    let defenders: Vec<_> = players.iter()
        .filter(|p| p.position.is_defender() || p.position == Position::MidfielderDefensive)
        .collect();
    
    let attack = if attackers.is_empty() {
        50
    } else {
        let sum: u32 = attackers.iter().map(|p| p.overall_rating() as u32).sum();
        (sum / attackers.len() as u32) as u8
    };
    
    let defense = if defenders.is_empty() {
        50
    } else {
        let sum: u32 = defenders.iter().map(|p| p.overall_rating() as u32).sum();
        (sum / defenders.len() as u32) as u8
    };
    
    (attack, defense)
}

/// Calculate average fitness of available players.
fn calculate_average_fitness(players: &[&Player]) -> u8 {
    let available: Vec<_> = players.iter()
        .filter(|p| p.is_available())
        .collect();
    
    if available.is_empty() {
        return 70;
    }
    
    let sum: u32 = available.iter().map(|p| p.fitness as u32).sum();
    (sum / available.len() as u32) as u8
}

/// Select substitutes for the bench.
pub fn select_substitutes(
    world: &World,
    club_id: &ClubId,
    starters: &[PlayerId],
    max_subs: usize,
) -> Vec<PlayerId> {
    let players = world.club_players(club_id);
    
    let mut available_subs: Vec<_> = players.iter()
        .filter(|p| p.is_available())
        .filter(|p| !starters.contains(&p.id))
        .collect();
    
    // Sort by overall rating
    available_subs.sort_by(|a, b| b.overall_rating().cmp(&a.overall_rating()));
    
    // Ensure we have a goalkeeper on bench if possible
    let mut subs = Vec::new();
    
    let gk_on_bench = available_subs.iter()
        .find(|p| p.position == Position::Goalkeeper);
    
    if let Some(gk) = gk_on_bench {
        subs.push(gk.id.clone());
    }
    
    // Fill remaining spots with best available
    for player in available_subs.iter() {
        if subs.len() >= max_subs {
            break;
        }
        if !subs.contains(&player.id) {
            subs.push(player.id.clone());
        }
    }
    
    subs
}

/// Recommend a substitution based on current match state.
pub fn recommend_substitution(
    world: &World,
    club_id: &ClubId,
    starters: &[PlayerId],
    minute: u8,
    score_diff: i8,
) -> Option<(PlayerId, PlayerId)> {
    let players = world.club_players(club_id);
    
    // Find tired or poor-form players in starting lineup
    let mut candidates_out: Vec<_> = starters.iter()
        .filter_map(|id| world.players.get(id))
        .filter(|p| {
            p.fitness < 60 || // Tired
            p.form < 40 // Poor form
        })
        .collect();
    
    // Sort by fatigue/form (worst first)
    candidates_out.sort_by_key(|p| (p.fitness, p.form));
    
    let player_out = candidates_out.first()?;
    
    // Find best available replacement
    let player_in = players.iter()
        .filter(|p| p.is_available())
        .filter(|p| !starters.contains(&p.id))
        .filter(|p| {
            p.position == player_out.position ||
            p.secondary_positions.contains(&player_out.position) ||
            get_similar_positions(player_out.position).contains(&p.position)
        })
        .max_by_key(|p| p.overall_rating())?;
    
    Some((player_out.id.clone(), player_in.id.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use cm_core::ids::NationId;
    use cm_core::world::Club;
    use std::collections::HashSet;

    fn create_test_player(
        id: &str,
        position: Position,
        quality: u8,
        fitness: u8,
    ) -> Player {
        let birth_date = NaiveDate::from_ymd_opt(1995, 6, 15).unwrap();
        let mut player = Player::new(
            id,
            "Test",
            id,
            NationId::new("test"),
            birth_date,
            position,
        );
        
        player.fitness = fitness;
        player.form = 60;
        
        // Set attributes
        player.attributes.technical.finishing = quality;
        player.attributes.technical.dribbling = quality;
        player.attributes.technical.passing = quality;
        player.attributes.technical.tackling = quality;
        player.attributes.technical.marking = quality;
        player.attributes.mental.off_the_ball = quality;
        player.attributes.mental.positioning = quality;
        player.attributes.mental.vision = quality;
        player.attributes.mental.leadership = quality;
        player.attributes.physical.strength = quality;
        player.attributes.physical.stamina = quality;
        player.attributes.technical.first_touch = quality;
        player.attributes.goalkeeper.handling = quality;
        player.attributes.goalkeeper.reflexes = quality;
        player.attributes.goalkeeper.positioning = quality;
        player.attributes.goalkeeper.one_on_ones = quality;
        
        player
    }

    fn setup_test_world() -> (World, ClubId, ClubId) {
        let mut world = World::new();
        
        let club_id = ClubId::new("home_club");
        let opponent_id = ClubId::new("away_club");
        
        let mut club = Club::new("home_club", "Home FC", NationId::new("test"));
        club.reputation = 70;
        
        let mut opponent = Club::new("away_club", "Away FC", NationId::new("test"));
        opponent.reputation = 60;
        
        // Create full squad for home team
        let home_players = vec![
            create_test_player("h_gk1", Position::Goalkeeper, 75, 90),
            create_test_player("h_gk2", Position::Goalkeeper, 65, 95),
            create_test_player("h_dc1", Position::DefenderCenter, 78, 85),
            create_test_player("h_dc2", Position::DefenderCenter, 76, 88),
            create_test_player("h_dc3", Position::DefenderCenter, 72, 90),
            create_test_player("h_dl1", Position::DefenderLeft, 74, 82),
            create_test_player("h_dr1", Position::DefenderRight, 75, 86),
            create_test_player("h_dm1", Position::MidfielderDefensive, 77, 80),
            create_test_player("h_mc1", Position::MidfielderCenter, 80, 84),
            create_test_player("h_mc2", Position::MidfielderCenter, 78, 88),
            create_test_player("h_ml1", Position::MidfielderLeft, 76, 90),
            create_test_player("h_mr1", Position::MidfielderRight, 74, 85),
            create_test_player("h_am1", Position::MidfielderAttacking, 82, 87),
            create_test_player("h_fc1", Position::ForwardCenter, 85, 92),
            create_test_player("h_fc2", Position::ForwardCenter, 78, 86),
            create_test_player("h_fl1", Position::ForwardLeft, 80, 89),
            create_test_player("h_fr1", Position::ForwardRight, 79, 83),
        ];
        
        for mut player in home_players {
            player.club_id = Some(club_id.clone());
            club.add_player(player.id.clone());
            world.players.insert(player.id.clone(), player);
        }
        
        // Create squad for away team
        let away_players = vec![
            create_test_player("a_gk1", Position::Goalkeeper, 68, 90),
            create_test_player("a_dc1", Position::DefenderCenter, 70, 88),
            create_test_player("a_dc2", Position::DefenderCenter, 68, 90),
            create_test_player("a_dl1", Position::DefenderLeft, 66, 85),
            create_test_player("a_dr1", Position::DefenderRight, 67, 87),
            create_test_player("a_mc1", Position::MidfielderCenter, 72, 86),
            create_test_player("a_mc2", Position::MidfielderCenter, 70, 84),
            create_test_player("a_ml1", Position::MidfielderLeft, 69, 88),
            create_test_player("a_mr1", Position::MidfielderRight, 68, 85),
            create_test_player("a_fc1", Position::ForwardCenter, 74, 90),
            create_test_player("a_fc2", Position::ForwardCenter, 71, 88),
        ];
        
        for mut player in away_players {
            player.club_id = Some(opponent_id.clone());
            opponent.add_player(player.id.clone());
            world.players.insert(player.id.clone(), player);
        }
        
        world.clubs.insert(club_id.clone(), club);
        world.clubs.insert(opponent_id.clone(), opponent);
        
        (world, club_id, opponent_id)
    }

    #[test]
    fn test_select_lineup_returns_11_players() {
        let (world, club_id, _) = setup_test_world();
        
        let lineup = select_lineup(&world, &club_id);
        
        assert_eq!(lineup.len(), 11, "Lineup should have 11 players");
    }

    #[test]
    fn test_select_lineup_with_formation() {
        let (world, club_id, _) = setup_test_world();
        
        let lineup = select_lineup_with_formation(&world, &club_id, Formation::F433);
        
        assert!(lineup.is_complete());
        assert_eq!(lineup.formation, Formation::F433);
        assert!(lineup.captain.is_some());
    }

    #[test]
    fn test_lineup_includes_goalkeeper() {
        let (world, club_id, _) = setup_test_world();
        
        let lineup = select_lineup(&world, &club_id);
        
        let has_goalkeeper = lineup.iter()
            .any(|id| {
                world.players.get(id)
                    .map(|p| p.position == Position::Goalkeeper)
                    .unwrap_or(false)
            });
        
        assert!(has_goalkeeper, "Lineup should include a goalkeeper");
    }

    #[test]
    fn test_select_tactics_against_weaker_opponent() {
        let (world, club_id, opponent_id) = setup_test_world();
        
        let tactics = select_tactics(&world, &club_id, &opponent_id);
        
        // Home team is stronger, should be more attacking
        assert!(
            matches!(tactics.mentality, Mentality::Attacking | Mentality::Balanced),
            "Should be attacking or balanced against weaker opponent"
        );
    }

    #[test]
    fn test_select_tactics_against_stronger_opponent() {
        let (world, club_id, opponent_id) = setup_test_world();
        
        // Reverse roles - select tactics as weaker team
        let tactics = select_tactics(&world, &opponent_id, &club_id);
        
        // Away team is weaker
        assert!(
            matches!(tactics.mentality, Mentality::Cautious | Mentality::Balanced),
            "Should be cautious or balanced against stronger opponent"
        );
    }

    #[test]
    fn test_select_substitutes() {
        let (world, club_id, _) = setup_test_world();
        
        let starters = select_lineup(&world, &club_id);
        let subs = select_substitutes(&world, &club_id, &starters, 7);
        
        assert!(subs.len() <= 7);
        assert!(subs.iter().all(|id| !starters.contains(id)));
    }

    #[test]
    fn test_substitutes_include_goalkeeper() {
        let (world, club_id, _) = setup_test_world();
        
        let starters = select_lineup(&world, &club_id);
        let subs = select_substitutes(&world, &club_id, &starters, 7);
        
        let has_gk = subs.iter()
            .any(|id| {
                world.players.get(id)
                    .map(|p| p.position == Position::Goalkeeper)
                    .unwrap_or(false)
            });
        
        assert!(has_gk, "Bench should include a goalkeeper");
    }

    #[test]
    fn test_get_positions_for_formation() {
        let positions = get_positions_for_formation(Formation::F442);
        assert_eq!(positions.len(), 11);
        
        let gk_count = positions.iter()
            .filter(|p| **p == Position::Goalkeeper)
            .count();
        assert_eq!(gk_count, 1, "Should have exactly one goalkeeper");
        
        let positions_433 = get_positions_for_formation(Formation::F433);
        let forward_count = positions_433.iter()
            .filter(|p| p.is_forward())
            .count();
        assert_eq!(forward_count, 3, "4-3-3 should have 3 forwards");
    }

    #[test]
    fn test_lineup_no_duplicates() {
        let (world, club_id, _) = setup_test_world();
        
        let lineup = select_lineup(&world, &club_id);
        
        let mut seen = std::collections::HashSet::new();
        for id in &lineup {
            assert!(seen.insert(id), "Player should not appear twice");
        }
    }

    #[test]
    fn test_select_lineup_with_unavailable_players() {
        let (mut world, club_id, _) = setup_test_world();
        
        // Make some players unavailable (low fitness)
        for (i, player) in world.players.values_mut().enumerate() {
            if player.club_id.as_ref() == Some(&club_id) && i % 3 == 0 {
                player.fitness = 30; // Below availability threshold
            }
        }
        
        let lineup = select_lineup(&world, &club_id);
        
        // Should still get 11 players if enough available
        assert_eq!(lineup.len(), 11);
        
        // None should have low fitness
        for id in &lineup {
            let player = world.players.get(id).unwrap();
            assert!(player.is_available(), "Selected player should be available");
        }
    }

    #[test]
    fn test_calculate_squad_strength() {
        let (world, club_id, _) = setup_test_world();
        let players = world.club_players(&club_id);
        
        let (attack, defense) = calculate_squad_strength(&players);
        
        assert!(attack > 50, "Home team should have decent attack");
        assert!(defense > 50, "Home team should have decent defense");
    }

    #[test]
    fn test_similar_positions() {
        let similar = get_similar_positions(Position::DefenderLeft);
        assert!(similar.contains(&Position::MidfielderLeft));
        
        let similar_fc = get_similar_positions(Position::ForwardCenter);
        assert!(similar_fc.contains(&Position::MidfielderAttacking));
    }
}
