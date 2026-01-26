use serde::{Serialize, Deserialize};
use crate::ids::ClubId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeagueRow {
    pub club_id: ClubId,
    pub club_name: String,
    pub played: u8,
    pub won: u8,
    pub drawn: u8,
    pub lost: u8,
    pub gf: u16,
    pub ga: u16,
    pub points: u16,
}

impl LeagueRow {
    pub fn new(club_id: ClubId, club_name: String) -> Self {
        Self {
            club_id,
            club_name,
            played: 0,
            won: 0,
            drawn: 0,
            lost: 0,
            gf: 0,
            ga: 0,
            points: 0,
        }
    }

    pub fn gd(&self) -> i16 {
        (self.gf as i16) - (self.ga as i16)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeagueTable {
    pub competition_name: String,
    pub rows: Vec<LeagueRow>,
}

impl LeagueTable {
    pub fn new(name: String, teams: Vec<(ClubId, String)>) -> Self {
        let mut rows = teams.into_iter().map(|(id, name)| LeagueRow::new(id, name)).collect::<Vec<_>>();
        Self {
            competition_name: name,
            rows,
        }
    }
}
