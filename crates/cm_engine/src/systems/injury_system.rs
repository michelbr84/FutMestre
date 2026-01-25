//! Injury system - handles player injuries and recovery.

use crate::config::GameConfig;
use crate::state::GameState;
use crate::inbox::generators;
use cm_core::world::{Injury, InjuryType as CoreInjuryType, World};
use cm_core::ids::PlayerId;

/// Injury severity levels with their typical recovery times.
#[derive(Debug, Clone, Copy)]
pub enum InjurySeverity {
    Minor,      // 1-7 days
    Moderate,   // 7-21 days
    Serious,    // 21-60 days
    Severe,     // 60-120 days
    Critical,   // 120+ days
}

impl InjurySeverity {
    /// Get recovery time range in days.
    pub fn recovery_range(&self) -> (u16, u16) {
        match self {
            Self::Minor => (1, 7),
            Self::Moderate => (7, 21),
            Self::Serious => (21, 60),
            Self::Severe => (60, 120),
            Self::Critical => (120, 240),
        }
    }

    /// Get severity name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Minor => "Minor",
            Self::Moderate => "Moderate",
            Self::Serious => "Serious",
            Self::Severe => "Severe",
            Self::Critical => "Critical",
        }
    }
}

/// Injury system.
pub struct InjurySystem;

impl InjurySystem {
    /// Process daily injury updates.
    pub fn run_daily(&self, _cfg: &GameConfig, world: &mut World, state: &mut GameState) {
        let current_date = state.date.date();
        let mut recovered: Vec<(PlayerId, String)> = Vec::new();
        
        // Check for recovered injuries
        for player in world.players.values() {
            if let Some(ref injury) = player.injury {
                if injury.is_healed(current_date) {
                    recovered.push((player.id.clone(), player.full_name()));
                }
            }
        }
        
        // Clear recovered injuries and notify
        for (player_id, name) in recovered {
            if let Some(player) = world.players.get_mut(&player_id) {
                player.injury = None;
                player.fitness = 80; // Reduced fitness after recovery
            }
            
            let msg = generators::injury_recovered(current_date, &name);
            state.add_message(format!("{}: {}", msg.subject, msg.body));
        }
    }

    /// Apply a new injury to a player.
    pub fn apply_injury(
        &self,
        world: &mut World,
        state: &mut GameState,
        player_id: &PlayerId,
        injury_type: CoreInjuryType,
        days: u16,
    ) {
        let current_date = state.date.date();
        
        if let Some(player) = world.players.get_mut(player_id) {
            let injury = Injury::new(injury_type, current_date, days);
            let name = player.full_name();
            player.injury = Some(injury);
            player.fitness = player.fitness.saturating_sub(20);
            
            let msg = generators::injury_report(
                current_date,
                &name,
                injury_type.display_name(),
                days,
            );
            state.add_message(format!("{}: {}", msg.subject, msg.body));
        }
    }

    /// Check if a player should get injured (random chance during training/match).
    pub fn check_injury_chance(&self, fitness: u8, intensity: u8) -> bool {
        // Base chance increases with lower fitness and higher intensity
        let base_chance = (100 - fitness) as u32 * intensity as u32 / 1000;
        // Random would go here in real implementation
        base_chance > 5
    }

    /// Get remaining days for a player's injury.
    pub fn get_injury_days_remaining(&self, world: &World, player_id: &PlayerId, current_date: chrono::NaiveDate) -> Option<i64> {
        world.players.get(player_id)
            .and_then(|p| p.injury.as_ref())
            .map(|i| i.days_remaining(current_date))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cm_core::world::Player;
    use cm_core::ids::NationId;
    use chrono::NaiveDate;
    use cm_core::world::Position;

    fn setup_test() -> (World, GameState, InjurySystem) {
        let mut world = World::new();
        let player = Player::new(
            "P001",
            "Test",
            "Player",
            NationId::new("ENG"),
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            Position::MidfielderCenter,
        );
        world.players.insert(player.id.clone(), player);
        
        let state = GameState::default();
        let system = InjurySystem;
        
        (world, state, system)
    }

    #[test]
    fn test_injury_severity_recovery_ranges() {
        assert_eq!(InjurySeverity::Minor.recovery_range(), (1, 7));
        assert_eq!(InjurySeverity::Critical.recovery_range(), (120, 240));
    }

    #[test]
    fn test_injury_severity_names() {
        assert_eq!(InjurySeverity::Serious.name(), "Serious");
    }

    #[test]
    fn test_apply_injury() {
        let (mut world, mut state, system) = setup_test();
        let player_id = PlayerId::new("P001");
        
        system.apply_injury(&mut world, &mut state, &player_id, CoreInjuryType::Hamstring, 10);
        
        let player = world.players.get(&player_id).unwrap();
        assert!(player.injury.is_some());
    }

    #[test]
    fn test_injury_chance_calculation() {
        let system = InjurySystem;
        
        // Lower fitness with high intensity should have higher injury chance
        // 50 fitness, 100 intensity = 50 * 100 / 1000 = 5 (not > 5, so false)
        // 100 fitness, 10 intensity = 0 * 10 / 1000 = 0 (not > 5, so false)
        // Both return false in this case, which is fine - the logic is consistent
        let low_fitness_high_intensity = system.check_injury_chance(30, 100);
        let high_fitness_low_intensity = system.check_injury_chance(100, 10);
        
        // With 30 fitness, 100 intensity: (100-30) * 100 / 1000 = 7 > 5, so true
        // With 100 fitness, 10 intensity: (100-100) * 10 / 1000 = 0 > 5, so false
        assert!(low_fitness_high_intensity);
        assert!(!high_fitness_low_intensity);
    }
}
