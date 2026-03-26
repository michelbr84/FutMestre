//! Morale system - handles player and team morale.

use crate::config::GameConfig;
use crate::state::GameState;
use cm_core::ids::ClubId;
use cm_core::world::{Morale, MoraleLevel, World};

/// Morale change events.
#[derive(Debug, Clone, Copy)]
pub enum MoraleEvent {
    MatchWin,
    MatchDraw,
    MatchLoss,
    BigWin,    // 3+ goal margin
    HeavyLoss, // 3+ goal margin
    GoalScored,
    CleanSheet, // For defenders/GK
    ContractRenewal,
    TransferRejected, // Wanted to leave
    DroppedFromSquad,
    CupProgress,
    CupElimination,
    PromotionAchieved,
    RelegationThreat,
}

impl MoraleEvent {
    /// Get morale change value (-100 to +100).
    pub fn effect(&self) -> i8 {
        match self {
            Self::MatchWin => 10,
            Self::MatchDraw => 0,
            Self::MatchLoss => -10,
            Self::BigWin => 20,
            Self::HeavyLoss => -25,
            Self::GoalScored => 5,
            Self::CleanSheet => 5,
            Self::ContractRenewal => 15,
            Self::TransferRejected => -20,
            Self::DroppedFromSquad => -15,
            Self::CupProgress => 15,
            Self::CupElimination => -15,
            Self::PromotionAchieved => 50,
            Self::RelegationThreat => -30,
        }
    }
}

/// Morale system.
pub struct MoraleSystem;

impl MoraleSystem {
    /// Run daily morale updates.
    pub fn run_daily(&self, _cfg: &GameConfig, world: &mut World, _state: &mut GameState) {
        // Natural morale drift toward neutral (50)
        for player in world.players.values_mut() {
            let current = player.morale.value;

            // Drift toward 50
            let drift = if current > 50 {
                -1
            } else if current < 50 {
                1
            } else {
                0
            };

            player.morale.adjust(drift);

            // Form affected by morale
            let form_change = match player.morale.level() {
                MoraleLevel::Superb => 2,
                MoraleLevel::Good => 1,
                MoraleLevel::Okay => 0,
                MoraleLevel::Poor => -1,
                MoraleLevel::VeryPoor => -2,
            };

            player.form = (player.form as i16 + form_change).clamp(1, 100) as u8;
        }
    }

    /// Apply a morale event to a player.
    pub fn apply_player_event(
        &self,
        world: &mut World,
        player_id: &cm_core::ids::PlayerId,
        event: MoraleEvent,
    ) {
        if let Some(player) = world.players.get_mut(player_id) {
            player.morale.adjust(event.effect());
        }
    }

    /// Apply a morale event to entire squad.
    pub fn apply_squad_event(&self, world: &mut World, club_id: &ClubId, event: MoraleEvent) {
        let player_ids: Vec<_> = world
            .players
            .values()
            .filter(|p| p.club_id.as_ref() == Some(club_id))
            .map(|p| p.id.clone())
            .collect();

        for player_id in player_ids {
            self.apply_player_event(world, &player_id, event);
        }
    }

    /// Get average squad morale for a club.
    pub fn squad_morale(&self, world: &World, club_id: &ClubId) -> u8 {
        let players: Vec<_> = world
            .players
            .values()
            .filter(|p| p.club_id.as_ref() == Some(club_id))
            .collect();

        if players.is_empty() {
            return 50;
        }

        let total: u32 = players.iter().map(|p| p.morale.value as u32).sum();
        (total / players.len() as u32) as u8
    }

    /// Check for morale-related concerns.
    pub fn check_morale_concerns(&self, world: &World, club_id: &ClubId) -> Vec<String> {
        let mut concerns = Vec::new();

        for player in world.players.values() {
            if player.club_id.as_ref() != Some(club_id) {
                continue;
            }

            match player.morale.level() {
                MoraleLevel::VeryPoor => {
                    concerns.push(format!(
                        "{} has very poor morale and may want to leave",
                        player.full_name()
                    ));
                }
                MoraleLevel::Poor => {
                    concerns.push(format!("{} has poor morale", player.full_name()));
                }
                _ => {}
            }
        }

        concerns
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use cm_core::ids::{NationId, PlayerId};
    use cm_core::world::Player;
    use cm_core::world::Position;

    fn setup_test() -> (World, GameState, MoraleSystem) {
        let mut world = World::new();

        let mut player = Player::new(
            "P001",
            "Test",
            "Player",
            NationId::new("ENG"),
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            Position::MidfielderCenter,
        );
        player.club_id = Some(ClubId::new("LIV"));
        player.morale = Morale::new(50);
        world.players.insert(player.id.clone(), player);

        let state = GameState::default();
        let system = MoraleSystem;

        (world, state, system)
    }

    #[test]
    fn test_morale_event_effects() {
        assert!(MoraleEvent::MatchWin.effect() > 0);
        assert!(MoraleEvent::MatchLoss.effect() < 0);
        assert_eq!(MoraleEvent::MatchDraw.effect(), 0);
        assert!(MoraleEvent::BigWin.effect() > MoraleEvent::MatchWin.effect());
    }

    #[test]
    fn test_apply_player_event() {
        let (mut world, _, system) = setup_test();
        let player_id = PlayerId::new("P001");

        let initial = world.players.get(&player_id).unwrap().morale.value;
        system.apply_player_event(&mut world, &player_id, MoraleEvent::MatchWin);

        let after = world.players.get(&player_id).unwrap().morale.value;
        assert!(after > initial);
    }

    #[test]
    fn test_apply_squad_event() {
        let (mut world, _, system) = setup_test();
        let club_id = ClubId::new("LIV");

        system.apply_squad_event(&mut world, &club_id, MoraleEvent::BigWin);

        let player = world.players.get(&PlayerId::new("P001")).unwrap();
        assert!(player.morale.value > 50);
    }

    #[test]
    fn test_squad_morale() {
        let (world, _, system) = setup_test();
        let morale = system.squad_morale(&world, &ClubId::new("LIV"));
        assert_eq!(morale, 50);
    }

    #[test]
    fn test_morale_concerns() {
        let (mut world, _, system) = setup_test();
        let club_id = ClubId::new("LIV");

        // Set player to very poor morale
        if let Some(player) = world.players.get_mut(&PlayerId::new("P001")) {
            player.morale = Morale::new(10);
        }

        let concerns = system.check_morale_concerns(&world, &club_id);
        assert!(!concerns.is_empty());
        assert!(concerns[0].contains("very poor morale"));
    }

    #[test]
    fn test_daily_morale_drift() {
        let (mut world, mut state, system) = setup_test();
        let config = GameConfig::default();

        // Set morale high
        if let Some(player) = world.players.get_mut(&PlayerId::new("P001")) {
            player.morale = Morale::new(80);
        }

        system.run_daily(&config, &mut world, &mut state);

        let player = world.players.get(&PlayerId::new("P001")).unwrap();
        assert!(player.morale.value < 80); // Should drift down toward 50
    }
}
