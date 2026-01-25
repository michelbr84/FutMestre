//! Game events.

use serde::{Deserialize, Serialize};
use crate::ids::{ClubId, MatchId, PlayerId};

/// Match events during simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchEvent {
    Goal {
        minute: u8,
        scorer: PlayerId,
        assist: Option<PlayerId>,
        club: ClubId,
    },
    OwnGoal {
        minute: u8,
        player: PlayerId,
        club: ClubId,
    },
    YellowCard {
        minute: u8,
        player: PlayerId,
    },
    RedCard {
        minute: u8,
        player: PlayerId,
    },
    Substitution {
        minute: u8,
        player_off: PlayerId,
        player_on: PlayerId,
    },
    Injury {
        minute: u8,
        player: PlayerId,
    },
    PenaltyMissed {
        minute: u8,
        taker: PlayerId,
    },
    PenaltyScored {
        minute: u8,
        taker: PlayerId,
    },
    HalfTime,
    FullTime,
    ExtraTimeStart,
    PenaltyShootout,
}

impl MatchEvent {
    /// Get the minute of the event.
    pub fn minute(&self) -> Option<u8> {
        match self {
            Self::Goal { minute, .. } => Some(*minute),
            Self::OwnGoal { minute, .. } => Some(*minute),
            Self::YellowCard { minute, .. } => Some(*minute),
            Self::RedCard { minute, .. } => Some(*minute),
            Self::Substitution { minute, .. } => Some(*minute),
            Self::Injury { minute, .. } => Some(*minute),
            Self::PenaltyMissed { minute, .. } => Some(*minute),
            Self::PenaltyScored { minute, .. } => Some(*minute),
            _ => None,
        }
    }
}

/// General game events (not match-specific).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    MatchPlayed {
        match_id: MatchId,
        home: ClubId,
        away: ClubId,
        home_goals: u8,
        away_goals: u8,
    },
    TransferCompleted {
        player: PlayerId,
        from: ClubId,
        to: ClubId,
    },
    ContractSigned {
        player: PlayerId,
        club: ClubId,
    },
    InjuryUpdate {
        player: PlayerId,
        days_out: u16,
    },
    SeasonEnd {
        season: String,
    },
    NewSeason {
        season: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goal_event() {
        let event = MatchEvent::Goal {
            minute: 45,
            scorer: PlayerId::new("P001"),
            assist: Some(PlayerId::new("P002")),
            club: ClubId::new("LIV"),
        };
        assert_eq!(event.minute(), Some(45));
    }

    #[test]
    fn test_goal_without_assist() {
        let event = MatchEvent::Goal {
            minute: 90,
            scorer: PlayerId::new("P001"),
            assist: None,
            club: ClubId::new("LIV"),
        };
        assert_eq!(event.minute(), Some(90));
    }

    #[test]
    fn test_own_goal_event() {
        let event = MatchEvent::OwnGoal {
            minute: 30,
            player: PlayerId::new("P003"),
            club: ClubId::new("MAN"),
        };
        assert_eq!(event.minute(), Some(30));
    }

    #[test]
    fn test_yellow_card_event() {
        let event = MatchEvent::YellowCard {
            minute: 60,
            player: PlayerId::new("P005"),
        };
        assert_eq!(event.minute(), Some(60));
    }

    #[test]
    fn test_red_card_event() {
        let event = MatchEvent::RedCard {
            minute: 75,
            player: PlayerId::new("P006"),
        };
        assert_eq!(event.minute(), Some(75));
    }

    #[test]
    fn test_substitution_event() {
        let event = MatchEvent::Substitution {
            minute: 65,
            player_off: PlayerId::new("P007"),
            player_on: PlayerId::new("P008"),
        };
        assert_eq!(event.minute(), Some(65));
    }

    #[test]
    fn test_injury_event() {
        let event = MatchEvent::Injury {
            minute: 20,
            player: PlayerId::new("P009"),
        };
        assert_eq!(event.minute(), Some(20));
    }

    #[test]
    fn test_penalty_events() {
        let missed = MatchEvent::PenaltyMissed {
            minute: 88,
            taker: PlayerId::new("P010"),
        };
        assert_eq!(missed.minute(), Some(88));

        let scored = MatchEvent::PenaltyScored {
            minute: 90,
            taker: PlayerId::new("P011"),
        };
        assert_eq!(scored.minute(), Some(90));
    }

    #[test]
    fn test_timeless_events() {
        assert_eq!(MatchEvent::HalfTime.minute(), None);
        assert_eq!(MatchEvent::FullTime.minute(), None);
        assert_eq!(MatchEvent::ExtraTimeStart.minute(), None);
        assert_eq!(MatchEvent::PenaltyShootout.minute(), None);
    }

    #[test]
    fn test_match_event_serialization() {
        let event = MatchEvent::Goal {
            minute: 45,
            scorer: PlayerId::new("P001"),
            assist: None,
            club: ClubId::new("LIV"),
        };
        let json = serde_json::to_string(&event).unwrap();
        let parsed: MatchEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.minute(), Some(45));
    }

    #[test]
    fn test_game_event_match_played() {
        let event = GameEvent::MatchPlayed {
            match_id: MatchId::new("M001"),
            home: ClubId::new("LIV"),
            away: ClubId::new("MAN"),
            home_goals: 3,
            away_goals: 1,
        };
        
        if let GameEvent::MatchPlayed { home_goals, away_goals, .. } = event {
            assert_eq!(home_goals, 3);
            assert_eq!(away_goals, 1);
        } else {
            panic!("Wrong event type");
        }
    }

    #[test]
    fn test_game_event_transfer() {
        let event = GameEvent::TransferCompleted {
            player: PlayerId::new("P001"),
            from: ClubId::new("LIV"),
            to: ClubId::new("MAN"),
        };
        
        if let GameEvent::TransferCompleted { from, to, .. } = event {
            assert_eq!(from.as_str(), "LIV");
            assert_eq!(to.as_str(), "MAN");
        } else {
            panic!("Wrong event type");
        }
    }

    #[test]
    fn test_game_event_contract() {
        let event = GameEvent::ContractSigned {
            player: PlayerId::new("P001"),
            club: ClubId::new("LIV"),
        };
        
        if let GameEvent::ContractSigned { club, .. } = event {
            assert_eq!(club.as_str(), "LIV");
        } else {
            panic!("Wrong event type");
        }
    }

    #[test]
    fn test_game_event_injury() {
        let event = GameEvent::InjuryUpdate {
            player: PlayerId::new("P001"),
            days_out: 21,
        };
        
        if let GameEvent::InjuryUpdate { days_out, .. } = event {
            assert_eq!(days_out, 21);
        } else {
            panic!("Wrong event type");
        }
    }

    #[test]
    fn test_game_event_season() {
        let end = GameEvent::SeasonEnd {
            season: "2024-25".to_string(),
        };
        let new = GameEvent::NewSeason {
            season: "2025-26".to_string(),
        };
        
        if let GameEvent::SeasonEnd { season } = end {
            assert_eq!(season, "2024-25");
        }
        if let GameEvent::NewSeason { season } = new {
            assert_eq!(season, "2025-26");
        }
    }

    #[test]
    fn test_game_event_serialization() {
        let event = GameEvent::MatchPlayed {
            match_id: MatchId::new("M001"),
            home: ClubId::new("LIV"),
            away: ClubId::new("MAN"),
            home_goals: 2,
            away_goals: 2,
        };
        let json = serde_json::to_string(&event).unwrap();
        let parsed: GameEvent = serde_json::from_str(&json).unwrap();
        
        if let GameEvent::MatchPlayed { home_goals, away_goals, .. } = parsed {
            assert_eq!(home_goals, 2);
            assert_eq!(away_goals, 2);
        }
    }
}
