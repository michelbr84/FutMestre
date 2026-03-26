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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::economy::{Money, Wage};
    use crate::ids::NationId;
    use crate::world::{Contract, Injury, InjuryType};

    fn make_player(position: Position) -> Player {
        Player::new(
            "P001",
            "Test",
            "Player",
            NationId::new("BRA"),
            NaiveDate::from_ymd_opt(1995, 6, 15).unwrap(),
            position,
        )
    }

    #[test]
    fn test_player_overall_rating_goalkeeper() {
        let mut player = make_player(Position::Goalkeeper);
        player.attributes.goalkeeper.handling = 80;
        player.attributes.goalkeeper.reflexes = 70;
        player.attributes.goalkeeper.positioning = 60;
        player.attributes.goalkeeper.one_on_ones = 90;
        // keeper_rating = (80 + 70 + 60 + 90) / 4 = 75
        assert_eq!(player.overall_rating(), 75);
    }

    #[test]
    fn test_player_overall_rating_defender() {
        let mut player = make_player(Position::DefenderCenter);
        player.attributes.technical.tackling = 80;
        player.attributes.technical.marking = 70;
        player.attributes.mental.positioning = 60;
        player.attributes.physical.strength = 90;
        // defense_rating = (80 + 70 + 60 + 90) / 4 = 75
        assert_eq!(player.overall_rating(), 75);
    }

    #[test]
    fn test_player_overall_rating_midfielder() {
        let mut player = make_player(Position::MidfielderCenter);
        player.attributes.technical.passing = 80;
        player.attributes.technical.first_touch = 70;
        player.attributes.mental.vision = 60;
        player.attributes.physical.stamina = 90;
        // midfield_rating = (80 + 70 + 60 + 90) / 4 = 75
        assert_eq!(player.overall_rating(), 75);
    }

    #[test]
    fn test_player_overall_rating_forward() {
        let mut player = make_player(Position::ForwardCenter);
        player.attributes.technical.finishing = 80;
        player.attributes.technical.dribbling = 70;
        player.attributes.technical.passing = 60;
        player.attributes.mental.off_the_ball = 90;
        // attack_rating = (80 + 70 + 60 + 90) / 4 = 75
        assert_eq!(player.overall_rating(), 75);
    }

    #[test]
    fn test_player_overall_rating_left_positions() {
        // DefenderLeft uses defense_rating
        let player = make_player(Position::DefenderLeft);
        assert_eq!(player.overall_rating(), player.attributes.defense_rating());

        // ForwardLeft uses attack_rating
        let player = make_player(Position::ForwardLeft);
        assert_eq!(player.overall_rating(), player.attributes.attack_rating());

        // MidfielderLeft uses midfield_rating
        let player = make_player(Position::MidfielderLeft);
        assert_eq!(player.overall_rating(), player.attributes.midfield_rating());
    }

    #[test]
    fn test_player_age_calculation() {
        let player = Player::new(
            "P001",
            "Test",
            "Player",
            NationId::new("BRA"),
            NaiveDate::from_ymd_opt(1995, 6, 15).unwrap(),
            Position::ForwardCenter,
        );

        // Before birthday in 2024
        let date_before = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
        assert_eq!(player.age_on(date_before), 28);

        // After birthday in 2024
        let date_after = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();
        assert_eq!(player.age_on(date_after), 29);

        // On birthday
        let date_on = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        assert_eq!(player.age_on(date_on), 29);
    }

    #[test]
    fn test_player_availability_healthy() {
        let mut player = make_player(Position::MidfielderCenter);
        player.fitness = 80;
        player.injury = None;
        assert!(player.is_available());
    }

    #[test]
    fn test_player_availability_injured() {
        let mut player = make_player(Position::MidfielderCenter);
        player.fitness = 80;
        player.injury = Some(Injury::new(
            InjuryType::Knee,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            30,
        ));
        assert!(!player.is_available());
        assert!(player.is_injured());
    }

    #[test]
    fn test_player_availability_low_fitness() {
        let mut player = make_player(Position::MidfielderCenter);
        player.fitness = 40; // below 50
        player.injury = None;
        assert!(!player.is_available());
    }

    #[test]
    fn test_player_availability_exactly_50_fitness() {
        let mut player = make_player(Position::MidfielderCenter);
        player.fitness = 50;
        player.injury = None;
        assert!(player.is_available());
    }

    #[test]
    fn test_player_weekly_wage_with_contract() {
        let mut player = make_player(Position::ForwardCenter);
        let contract = Contract::new(
            Wage::weekly(Money::from_major(50_000)),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 6, 30).unwrap(),
        );
        player.contract = Some(contract);
        assert_eq!(player.weekly_wage().major(), 50_000);
    }

    #[test]
    fn test_player_weekly_wage_without_contract() {
        let player = make_player(Position::ForwardCenter);
        assert_eq!(player.weekly_wage(), Money::ZERO);
    }

    #[test]
    fn test_player_full_name() {
        let player = Player::new(
            "P001",
            "Roberto",
            "Carlos",
            NationId::new("BRA"),
            NaiveDate::from_ymd_opt(1973, 4, 10).unwrap(),
            Position::DefenderLeft,
        );
        assert_eq!(player.full_name(), "Roberto Carlos");
    }

    #[test]
    fn test_position_short_names() {
        assert_eq!(Position::Goalkeeper.short_name(), "GK");
        assert_eq!(Position::ForwardCenter.short_name(), "FC");
        assert_eq!(Position::MidfielderDefensive.short_name(), "DM");
        assert_eq!(Position::MidfielderAttacking.short_name(), "AM");
    }

    #[test]
    fn test_position_categories() {
        assert!(Position::DefenderCenter.is_defender());
        assert!(Position::DefenderLeft.is_defender());
        assert!(Position::DefenderRight.is_defender());
        assert!(!Position::Goalkeeper.is_defender());
        assert!(!Position::ForwardCenter.is_defender());

        assert!(Position::MidfielderCenter.is_midfielder());
        assert!(Position::MidfielderDefensive.is_midfielder());
        assert!(Position::MidfielderAttacking.is_midfielder());
        assert!(!Position::Goalkeeper.is_midfielder());

        assert!(Position::ForwardCenter.is_forward());
        assert!(Position::ForwardLeft.is_forward());
        assert!(Position::ForwardRight.is_forward());
        assert!(!Position::Goalkeeper.is_forward());
    }
}
