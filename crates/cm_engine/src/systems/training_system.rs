//! Training system - handles player development and fitness.

use crate::config::GameConfig;
use crate::state::GameState;
use cm_core::world::{World, TrainingFocus};
use cm_core::ids::ClubId;

/// Training intensity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrainingIntensity {
    Light,    // Recovery, low injury risk
    Normal,   // Balanced
    High,     // Fast development, higher injury risk
    Extreme,  // Maximum development, very high injury risk
}

impl TrainingIntensity {
    /// Get fitness recovery/drain factor.
    pub fn fitness_factor(&self) -> i8 {
        match self {
            Self::Light => 5,    // Recover fitness
            Self::Normal => 0,   // Maintain
            Self::High => -3,    // Slight drain
            Self::Extreme => -8, // Heavy drain
        }
    }

    /// Get attribute improvement chance multiplier.
    pub fn development_multiplier(&self) -> f32 {
        match self {
            Self::Light => 0.5,
            Self::Normal => 1.0,
            Self::High => 1.5,
            Self::Extreme => 2.0,
        }
    }

    /// Get injury risk multiplier.
    pub fn injury_risk(&self) -> f32 {
        match self {
            Self::Light => 0.25,
            Self::Normal => 1.0,
            Self::High => 2.0,
            Self::Extreme => 4.0,
        }
    }
}

/// Training system.
pub struct TrainingSystem;

impl TrainingSystem {
    /// Run daily training updates.
    pub fn run_daily(&self, _cfg: &GameConfig, world: &mut World, _state: &mut GameState) {
        // Process training for all players
        for player in world.players.values_mut() {
            // Skip injured players
            if player.is_injured() {
                continue;
            }

            // Default intensity for now
            let intensity = TrainingIntensity::Normal;
            
            // Update fitness
            let fitness_change = intensity.fitness_factor();
            player.fitness = (player.fitness as i16 + fitness_change as i16)
                .clamp(0, 100) as u8;

            // Young players have better development potential
            let age = 25; // Would calculate from birth_date
            let age_factor = if age < 21 {
                1.5
            } else if age < 25 {
                1.2
            } else if age < 30 {
                1.0
            } else {
                0.5
            };

            // Development chance based on potential gap
            let current_rating = player.overall_rating();
            let potential_gap = player.potential.saturating_sub(current_rating);
            
            // Small chance of attribute improvement
            let base_improvement_chance = 0.01 * potential_gap as f32 * age_factor
                * intensity.development_multiplier();
            
            // In real implementation, use RNG here
            // For now, very slow steady improvement for players under potential
            if potential_gap > 0 && player.fitness > 70 {
                // Would randomly improve attributes based on training focus
            }
        }
    }

    /// Apply focused training for a club.
    pub fn apply_club_training(
        &self,
        world: &mut World,
        club_id: &ClubId,
        focus: TrainingFocus,
        intensity: TrainingIntensity,
    ) {
        let player_ids: Vec<_> = world.players
            .values()
            .filter(|p| p.club_id.as_ref() == Some(club_id) && !p.is_injured())
            .map(|p| p.id.clone())
            .collect();

        for player_id in player_ids {
            if let Some(player) = world.players.get_mut(&player_id) {
                // Apply fitness change
                let fitness_change = intensity.fitness_factor();
                player.fitness = (player.fitness as i16 + fitness_change as i16)
                    .clamp(0, 100) as u8;
            }
        }
    }

    /// Rest players (light training/recovery).
    pub fn rest_squad(&self, world: &mut World, club_id: &ClubId) {
        self.apply_club_training(world, club_id, TrainingFocus::Fitness, TrainingIntensity::Light);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cm_core::world::Player;
    use cm_core::ids::{NationId, PlayerId};
    use chrono::NaiveDate;
    use cm_core::world::Position;

    fn setup_test() -> (World, GameState, TrainingSystem) {
        let mut world = World::new();
        
        let mut player = Player::new(
            "P001",
            "Test",
            "Player",
            NationId::new("ENG"),
            NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            Position::MidfielderCenter,
        );
        player.fitness = 80;
        player.club_id = Some(ClubId::new("LIV"));
        world.players.insert(player.id.clone(), player);
        
        let state = GameState::default();
        let system = TrainingSystem;
        
        (world, state, system)
    }

    #[test]
    fn test_intensity_fitness_factors() {
        assert_eq!(TrainingIntensity::Light.fitness_factor(), 5);
        assert_eq!(TrainingIntensity::Normal.fitness_factor(), 0);
        assert!(TrainingIntensity::Extreme.fitness_factor() < 0);
    }

    #[test]
    fn test_intensity_development() {
        assert!(TrainingIntensity::High.development_multiplier() > TrainingIntensity::Normal.development_multiplier());
    }

    #[test]
    fn test_intensity_injury_risk() {
        assert!(TrainingIntensity::Extreme.injury_risk() > TrainingIntensity::Light.injury_risk());
    }

    #[test]
    fn test_daily_training() {
        let (mut world, mut state, system) = setup_test();
        let config = GameConfig::default();
        
        let _initial_fitness = world.players.get(&PlayerId::new("P001")).unwrap().fitness;
        system.run_daily(&config, &mut world, &mut state);
        
        // Fitness should be within bounds
        let player = world.players.get(&PlayerId::new("P001")).unwrap();
        assert!(player.fitness <= 100);
    }

    #[test]
    fn test_rest_squad() {
        let (mut world, _, system) = setup_test();
        let club_id = ClubId::new("LIV");
        
        // Reduce fitness first
        if let Some(player) = world.players.get_mut(&PlayerId::new("P001")) {
            player.fitness = 70;
        }
        
        system.rest_squad(&mut world, &club_id);
        
        let player = world.players.get(&PlayerId::new("P001")).unwrap();
        assert!(player.fitness >= 70);
    }
}
