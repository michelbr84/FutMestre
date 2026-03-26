//! Data validators.

use crate::errors::DataError;
use cm_core::world::World;

/// Validate world data.
pub fn validate_world(world: &World) -> Result<(), DataError> {
    // Check that all player club_ids reference valid clubs
    for (player_id, player) in &world.players {
        if let Some(club_id) = &player.club_id {
            if !world.clubs.contains_key(club_id) {
                return Err(DataError::Validation(format!(
                    "Player {} references non-existent club {}",
                    player_id, club_id
                )));
            }
        }
    }

    // Check that all competition teams exist
    for (comp_id, comp) in &world.competitions {
        for team_id in &comp.teams {
            if !world.clubs.contains_key(team_id) {
                return Err(DataError::Validation(format!(
                    "Competition {} references non-existent team {}",
                    comp_id, team_id
                )));
            }
        }
    }

    Ok(())
}

/// Validate that a club has enough players.
pub fn validate_squad_size(world: &World, min_size: usize) -> Vec<String> {
    let mut issues = Vec::new();

    for (club_id, club) in &world.clubs {
        if club.player_ids.len() < min_size {
            issues.push(format!(
                "Club {} has only {} players (minimum: {})",
                club_id,
                club.player_ids.len(),
                min_size
            ));
        }
    }

    issues
}
