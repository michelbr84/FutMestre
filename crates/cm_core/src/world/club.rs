//! Club entity.

use super::{Board, ClubHistory, Tactics};
use crate::economy::{Budget, Money};
use crate::ids::{ClubId, NationId, PlayerId, StadiumId, StaffId};
use serde::{Deserialize, Serialize};

/// A football club.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Club {
    pub id: ClubId,
    pub name: String,
    pub short_name: String,
    pub nation_id: NationId,
    pub stadium_id: Option<StadiumId>,
    pub reputation: u8,
    pub budget: Budget,
    pub board: Board,
    pub tactics: Tactics,
    pub player_ids: Vec<PlayerId>,
    pub staff_ids: Vec<StaffId>,
    pub primary_color: String,
    pub secondary_color: String,
    #[serde(default)]
    pub history: ClubHistory,
    /// IDs dos jogadores na reserva / time B.
    #[serde(default)]
    pub reserve_ids: Vec<PlayerId>,
}

impl Club {
    /// Create a new club.
    pub fn new(id: impl Into<ClubId>, name: impl Into<String>, nation_id: NationId) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            short_name: String::new(),
            nation_id,
            stadium_id: None,
            reputation: 50,
            budget: Budget::default(),
            board: Board::default(),
            tactics: Tactics::default(),
            player_ids: Vec::new(),
            staff_ids: Vec::new(),
            primary_color: "#FF0000".into(),
            secondary_color: "#FFFFFF".into(),
            history: ClubHistory::default(),
            reserve_ids: Vec::new(),
        }
    }

    /// Get weekly wage bill.
    pub fn weekly_wage_bill(&self) -> Money {
        self.budget.wage_bill
    }

    /// Add a player to the squad.
    pub fn add_player(&mut self, player_id: PlayerId) {
        if !self.player_ids.contains(&player_id) {
            self.player_ids.push(player_id);
        }
    }

    /// Remove a player from the squad.
    pub fn remove_player(&mut self, player_id: &PlayerId) {
        self.player_ids.retain(|id| id != player_id);
    }

    /// Get squad size.
    pub fn squad_size(&self) -> usize {
        self.player_ids.len()
    }

    /// Adicionar jogador a reserva.
    pub fn add_to_reserve(&mut self, player_id: PlayerId) {
        if !self.reserve_ids.contains(&player_id) {
            self.reserve_ids.push(player_id);
        }
    }

    /// Remover jogador da reserva.
    pub fn remove_from_reserve(&mut self, player_id: &PlayerId) {
        self.reserve_ids.retain(|id| id != player_id);
    }

    /// Tamanho da reserva.
    pub fn reserve_size(&self) -> usize {
        self.reserve_ids.len()
    }
}
