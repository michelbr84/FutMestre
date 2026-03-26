//! World module - all game entities.

pub mod academy;
pub mod attributes;
pub mod board;
pub mod calendar;
pub mod club;
pub mod competition;
pub mod contract;
pub mod fixtures;
pub mod history;
pub mod injury;
pub mod morale;
pub mod nation;
pub mod player;
pub mod referee;
pub mod reputation;
pub mod scouting;
pub mod stadium;
pub mod staff;
pub mod table;
pub mod tactics;
pub mod training;

pub use academy::Academy;
pub use attributes::{
    Attributes, GoalkeeperAttributes, MentalAttributes, PhysicalAttributes, TechnicalAttributes,
};
pub use board::Board;
pub use calendar::{Calendar, CalendarEntry};
pub use club::Club;
pub use competition::{Competition, CompetitionType, DivisionLevel, TopScorer};
pub use contract::Contract;
pub use fixtures::{Fixture, Fixtures};
pub use history::{ClubHistory, PlayerHistory, PlayerSeasonStats, SeasonRecord};
pub use injury::{Injury, InjuryType};
pub use morale::{Morale, MoraleLevel};
pub use nation::Nation;
pub use player::{Player, Position};
pub use referee::Referee;
pub use reputation::Reputation;
pub use scouting::ScoutReport;
pub use stadium::Stadium;
pub use staff::{Staff, StaffRole};
pub use table::{Table, TableRow};
pub use tactics::{Formation, Mentality, Tactics, TacticsPreset, Tempo};
pub use training::{Training, TrainingFocus};

use crate::ids::*;
use crate::CoreError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The game world containing all entities.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct World {
    pub nations: HashMap<NationId, Nation>,
    pub clubs: HashMap<ClubId, Club>,
    pub players: HashMap<PlayerId, Player>,
    pub staff: HashMap<StaffId, Staff>,
    pub competitions: HashMap<CompetitionId, Competition>,
    pub stadiums: HashMap<StadiumId, Stadium>,
    pub referees: HashMap<RefereeId, Referee>,
}

impl World {
    /// Create a new empty world.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a club by ID.
    pub fn club(&self, id: &str) -> Result<&Club, CoreError> {
        self.clubs
            .get(&ClubId::new(id))
            .ok_or_else(|| CoreError::NotFound {
                entity_type: "Club".into(),
                id: id.into(),
            })
    }

    /// Get a mutable club by ID.
    pub fn club_mut(&mut self, id: &str) -> Result<&mut Club, CoreError> {
        self.clubs
            .get_mut(&ClubId::new(id))
            .ok_or_else(|| CoreError::NotFound {
                entity_type: "Club".into(),
                id: id.into(),
            })
    }

    /// Get a player by ID.
    pub fn player(&self, id: &PlayerId) -> Result<&Player, CoreError> {
        self.players.get(id).ok_or_else(|| CoreError::NotFound {
            entity_type: "Player".into(),
            id: id.to_string(),
        })
    }

    /// Get a mutable player by ID.
    pub fn player_mut(&mut self, id: &PlayerId) -> Result<&mut Player, CoreError> {
        self.players.get_mut(id).ok_or_else(|| CoreError::NotFound {
            entity_type: "Player".into(),
            id: id.to_string(),
        })
    }

    /// Get all players for a club.
    pub fn club_players(&self, club_id: &ClubId) -> Vec<&Player> {
        self.players
            .values()
            .filter(|p| p.club_id.as_ref() == Some(club_id))
            .collect()
    }

    /// Get a competition by ID.
    pub fn competition(&self, id: &CompetitionId) -> Result<&Competition, CoreError> {
        self.competitions
            .get(id)
            .ok_or_else(|| CoreError::NotFound {
                entity_type: "Competition".into(),
                id: id.to_string(),
            })
    }
}
