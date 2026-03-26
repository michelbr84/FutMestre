//! Type-safe IDs for all entities.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Macro to create newtype ID wrappers.
macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(pub String);

        impl $name {
            pub fn new(s: impl Into<String>) -> Self {
                Self(s.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&String> for $name {
            fn from(s: &String) -> Self {
                Self(s.clone())
            }
        }
    };
}

define_id!(PlayerId);
define_id!(ClubId);
define_id!(StaffId);
define_id!(NationId);
define_id!(CompetitionId);
define_id!(StadiumId);
define_id!(RefereeId);
define_id!(MatchId);
define_id!(ContractId);
define_id!(TransferId);

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_player_id_creation() {
        let id1 = PlayerId::new("P001");
        let id2 = PlayerId::from("P001");
        let id3 = PlayerId::from(String::from("P001"));

        assert_eq!(id1, id2);
        assert_eq!(id2, id3);
        assert_eq!(id1.as_str(), "P001");
    }

    #[test]
    fn test_club_id_display() {
        let id = ClubId::new("LIV");
        assert_eq!(format!("{}", id), "LIV");
    }

    #[test]
    fn test_id_hashing() {
        let mut set = HashSet::new();
        set.insert(PlayerId::new("P001"));
        set.insert(PlayerId::new("P002"));
        set.insert(PlayerId::new("P001")); // duplicate

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_id_equality() {
        let id1 = ClubId::new("LIV");
        let id2 = ClubId::new("LIV");
        let id3 = ClubId::new("MAN");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_id_serialization() {
        let id = NationId::new("ENG");
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"ENG\"");

        let parsed: NationId = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, id);
    }

    #[test]
    fn test_different_id_types() {
        // Ensure different ID types can coexist
        let player = PlayerId::new("ID001");
        let club = ClubId::new("ID001");
        let staff = StaffId::new("ID001");

        // They have the same internal value but are different types
        assert_eq!(player.as_str(), club.as_str());
        assert_eq!(club.as_str(), staff.as_str());
    }

    #[test]
    fn test_id_from_string_reference() {
        let s = String::from("TEST");
        let id = MatchId::from(&s);
        assert_eq!(id.as_str(), "TEST");
    }

    #[test]
    fn test_id_clone() {
        let original = CompetitionId::new("PL");
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_id_debug() {
        let id = StadiumId::new("STD001");
        let debug = format!("{:?}", id);
        assert!(debug.contains("STD001"));
    }
}
