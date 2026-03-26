//! Player entity.

use super::{Attributes, Contract, Injury, Morale, PlayerHistory};
use crate::economy::Money;
use crate::ids::{ClubId, NationId, PlayerId};
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// Player position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Position {
    Goalkeeper,
    DefenderCenter,
    DefenderLeft,
    DefenderRight,
    MidfielderCenter,
    MidfielderLeft,
    MidfielderRight,
    MidfielderDefensive,
    MidfielderAttacking,
    ForwardCenter,
    ForwardLeft,
    ForwardRight,
}

impl Position {
    /// Get short display name.
    pub fn short_name(&self) -> &'static str {
        match self {
            Self::Goalkeeper => "GK",
            Self::DefenderCenter => "DC",
            Self::DefenderLeft => "DL",
            Self::DefenderRight => "DR",
            Self::MidfielderCenter => "MC",
            Self::MidfielderLeft => "ML",
            Self::MidfielderRight => "MR",
            Self::MidfielderDefensive => "DM",
            Self::MidfielderAttacking => "AM",
            Self::ForwardCenter => "FC",
            Self::ForwardLeft => "FL",
            Self::ForwardRight => "FR",
        }
    }

    /// Check if position is defensive.
    pub fn is_defender(&self) -> bool {
        matches!(
            self,
            Self::DefenderCenter | Self::DefenderLeft | Self::DefenderRight
        )
    }

    /// Check if position is midfield.
    pub fn is_midfielder(&self) -> bool {
        matches!(
            self,
            Self::MidfielderCenter
                | Self::MidfielderLeft
                | Self::MidfielderRight
                | Self::MidfielderDefensive
                | Self::MidfielderAttacking
        )
    }

    /// Check if position is forward.
    pub fn is_forward(&self) -> bool {
        matches!(
            self,
            Self::ForwardCenter | Self::ForwardLeft | Self::ForwardRight
        )
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::MidfielderCenter
    }
}

/// A football player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub first_name: String,
    pub last_name: String,
    pub nationality: NationId,
    pub birth_date: NaiveDate,
    pub position: Position,
    pub secondary_positions: Vec<Position>,
    pub preferred_foot: PreferredFoot,
    pub club_id: Option<ClubId>,
    pub attributes: Attributes,
    pub contract: Option<Contract>,
    pub value: Money,
    pub morale: Morale,
    pub injury: Option<Injury>,
    pub fitness: u8,
    pub form: u8,
    pub potential: u8,
    #[serde(default)]
    pub history: PlayerHistory,
}

/// Preferred foot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreferredFoot {
    Left,
    Right,
    Either,
}

impl Default for PreferredFoot {
    fn default() -> Self {
        Self::Right
    }
}

impl Player {
    /// Create a new player.
    pub fn new(
        id: impl Into<PlayerId>,
        first_name: impl Into<String>,
        last_name: impl Into<String>,
        nationality: NationId,
        birth_date: NaiveDate,
        position: Position,
    ) -> Self {
        Self {
            id: id.into(),
            first_name: first_name.into(),
            last_name: last_name.into(),
            nationality,
            birth_date,
            position,
            secondary_positions: Vec::new(),
            preferred_foot: PreferredFoot::Right,
            club_id: None,
            attributes: Attributes::default(),
            contract: None,
            value: Money::from_major(100_000),
            morale: Morale::default(),
            injury: None,
            fitness: 100,
            form: 50,
            potential: 70,
            history: PlayerHistory::default(),
        }
    }

    /// Get full name.
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    /// Get age on a given date.
    pub fn age_on(&self, date: NaiveDate) -> u8 {
        let years = date.year() - self.birth_date.year();
        if date.ordinal() < self.birth_date.ordinal() {
            (years - 1) as u8
        } else {
            years as u8
        }
    }

    /// Check if player is injured.
    pub fn is_injured(&self) -> bool {
        self.injury.is_some()
    }

    /// Check if player is available for selection.
    pub fn is_available(&self) -> bool {
        !self.is_injured() && self.fitness >= 50
    }

    /// Get weekly wage.
    pub fn weekly_wage(&self) -> Money {
        self.contract
            .as_ref()
            .map(|c| c.wage.as_weekly())
            .unwrap_or(Money::ZERO)
    }

    /// Get overall rating based on position.
    pub fn overall_rating(&self) -> u8 {
        match self.position {
            Position::Goalkeeper => self.attributes.keeper_rating(),
            pos if pos.is_defender() => self.attributes.defense_rating(),
            pos if pos.is_forward() => self.attributes.attack_rating(),
            _ => self.attributes.midfield_rating(),
        }
    }
}
