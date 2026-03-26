//! League table.

use crate::ids::ClubId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A row in the league table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    pub club_id: ClubId,
    pub played: u16,
    pub won: u16,
    pub drawn: u16,
    pub lost: u16,
    pub goals_for: u16,
    pub goals_against: u16,
    pub points: u16,
}

impl TableRow {
    /// Create a new table row.
    pub fn new(club_id: ClubId) -> Self {
        Self {
            club_id,
            played: 0,
            won: 0,
            drawn: 0,
            lost: 0,
            goals_for: 0,
            goals_against: 0,
            points: 0,
        }
    }

    /// Goal difference.
    pub fn goal_difference(&self) -> i16 {
        self.goals_for as i16 - self.goals_against as i16
    }

    /// Record a win.
    pub fn record_win(&mut self, goals_for: u8, goals_against: u8, points: u8) {
        self.played += 1;
        self.won += 1;
        self.goals_for += goals_for as u16;
        self.goals_against += goals_against as u16;
        self.points += points as u16;
    }

    /// Record a draw.
    pub fn record_draw(&mut self, goals_for: u8, goals_against: u8, points: u8) {
        self.played += 1;
        self.drawn += 1;
        self.goals_for += goals_for as u16;
        self.goals_against += goals_against as u16;
        self.points += points as u16;
    }

    /// Record a loss.
    pub fn record_loss(&mut self, goals_for: u8, goals_against: u8, points: u8) {
        self.played += 1;
        self.lost += 1;
        self.goals_for += goals_for as u16;
        self.goals_against += goals_against as u16;
        self.points += points as u16;
    }
}

/// League table.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Table {
    pub rows: Vec<TableRow>,
    /// Head-to-head record: maps (ClubA, ClubB) to (goals scored by A, goals scored by B)
    /// across all matches between the two clubs.
    #[serde(default)]
    pub head_to_head: HashMap<(ClubId, ClubId), (u16, u16)>,
}

impl Table {
    /// Create a new table.
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            head_to_head: HashMap::new(),
        }
    }

    /// Add a team.
    pub fn add_team(&mut self, club_id: ClubId) {
        if !self.rows.iter().any(|r| r.club_id == club_id) {
            self.rows.push(TableRow::new(club_id));
        }
    }

    /// Get row for a team.
    pub fn get_team(&self, club_id: &ClubId) -> Option<&TableRow> {
        self.rows.iter().find(|r| &r.club_id == club_id)
    }

    /// Get mutable row for a team.
    pub fn get_team_mut(&mut self, club_id: &ClubId) -> Option<&mut TableRow> {
        self.rows.iter_mut().find(|r| &r.club_id == club_id)
    }

    /// Record a head-to-head result between two clubs.
    pub fn record_head_to_head(
        &mut self,
        home_id: &ClubId,
        away_id: &ClubId,
        home_goals: u8,
        away_goals: u8,
    ) {
        // Record from home's perspective
        let entry_home = self
            .head_to_head
            .entry((home_id.clone(), away_id.clone()))
            .or_insert((0, 0));
        entry_home.0 += home_goals as u16;
        entry_home.1 += away_goals as u16;

        // Record from away's perspective
        let entry_away = self
            .head_to_head
            .entry((away_id.clone(), home_id.clone()))
            .or_insert((0, 0));
        entry_away.0 += away_goals as u16;
        entry_away.1 += home_goals as u16;
    }

    /// Sort table by points, then goal difference, then goals for,
    /// then head-to-head, then club ID as final tiebreak.
    pub fn sort(&mut self) {
        // Clone the h2h map so the closure can reference it without borrowing self
        let h2h = self.head_to_head.clone();
        self.rows.sort_by(|a, b| {
            b.points
                .cmp(&a.points)
                .then_with(|| b.goal_difference().cmp(&a.goal_difference()))
                .then_with(|| b.goals_for.cmp(&a.goals_for))
                .then_with(|| {
                    // Head-to-head: compare goal difference between the two clubs
                    let key_a = (a.club_id.clone(), b.club_id.clone());
                    let key_b = (b.club_id.clone(), a.club_id.clone());
                    let a_diff = h2h
                        .get(&key_a)
                        .map(|&(s, c)| s as i16 - c as i16)
                        .unwrap_or(0);
                    let b_diff = h2h
                        .get(&key_b)
                        .map(|&(s, c)| s as i16 - c as i16)
                        .unwrap_or(0);
                    b_diff.cmp(&a_diff)
                })
                .then_with(|| a.club_id.0.cmp(&b.club_id.0))
        });
    }

    /// Get position for a team (1-indexed).
    pub fn position(&self, club_id: &ClubId) -> Option<usize> {
        self.rows
            .iter()
            .position(|r| &r.club_id == club_id)
            .map(|p| p + 1)
    }

    /// Record a match result.
    pub fn record_result(
        &mut self,
        home_id: &ClubId,
        away_id: &ClubId,
        home_goals: u8,
        away_goals: u8,
        win_points: u8,
        draw_points: u8,
    ) {
        if home_goals > away_goals {
            if let Some(row) = self.get_team_mut(home_id) {
                row.record_win(home_goals, away_goals, win_points);
            }
            if let Some(row) = self.get_team_mut(away_id) {
                row.record_loss(away_goals, home_goals, 0);
            }
        } else if away_goals > home_goals {
            if let Some(row) = self.get_team_mut(home_id) {
                row.record_loss(home_goals, away_goals, 0);
            }
            if let Some(row) = self.get_team_mut(away_id) {
                row.record_win(away_goals, home_goals, win_points);
            }
        } else {
            if let Some(row) = self.get_team_mut(home_id) {
                row.record_draw(home_goals, away_goals, draw_points);
            }
            if let Some(row) = self.get_team_mut(away_id) {
                row.record_draw(away_goals, home_goals, draw_points);
            }
        }
        self.record_head_to_head(home_id, away_id, home_goals, away_goals);
        self.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_table() -> Table {
        let mut table = Table::new();
        table.add_team(ClubId::new("FLA"));
        table.add_team(ClubId::new("PAL"));
        table.add_team(ClubId::new("SAO"));
        table.add_team(ClubId::new("COR"));
        table
    }

    #[test]
    fn test_table_sort_by_points() {
        let mut table = setup_table();
        let fla = ClubId::new("FLA");
        let pal = ClubId::new("PAL");
        let sao = ClubId::new("SAO");
        let cor = ClubId::new("COR");

        // FLA wins 2-0 vs COR -> FLA 3pts, COR 0pts
        table.record_result(&fla, &cor, 2, 0, 3, 1);
        // PAL wins 1-0 vs SAO -> PAL 3pts, SAO 0pts
        table.record_result(&pal, &sao, 1, 0, 3, 1);
        // FLA wins 1-0 vs SAO -> FLA 6pts
        table.record_result(&fla, &sao, 1, 0, 3, 1);

        // FLA should be first (6pts), PAL second (3pts)
        assert_eq!(table.position(&fla), Some(1));
        assert_eq!(table.position(&pal), Some(2));
    }

    #[test]
    fn test_table_sort_by_goal_difference() {
        let mut table = Table::new();
        let fla = ClubId::new("FLA");
        let pal = ClubId::new("PAL");
        let sao = ClubId::new("SAO");
        let cor = ClubId::new("COR");

        table.add_team(fla.clone());
        table.add_team(pal.clone());
        table.add_team(sao.clone());
        table.add_team(cor.clone());

        // Both FLA and PAL win, same points, but FLA wins by bigger margin
        table.record_result(&fla, &sao, 3, 0, 3, 1); // FLA +3 GD
        table.record_result(&pal, &cor, 1, 0, 3, 1); // PAL +1 GD

        assert_eq!(table.position(&fla), Some(1));
        assert_eq!(table.position(&pal), Some(2));
    }

    #[test]
    fn test_table_head_to_head() {
        let mut table = Table::new();
        let fla = ClubId::new("FLA");
        let pal = ClubId::new("PAL");

        table.add_team(fla.clone());
        table.add_team(pal.clone());

        // Record head-to-head
        table.record_head_to_head(&fla, &pal, 2, 1);

        // Check from FLA's perspective
        let h2h_fla = table.head_to_head.get(&(fla.clone(), pal.clone()));
        assert!(h2h_fla.is_some());
        let (scored, conceded) = h2h_fla.unwrap();
        assert_eq!(*scored, 2);
        assert_eq!(*conceded, 1);

        // Check from PAL's perspective
        let h2h_pal = table.head_to_head.get(&(pal.clone(), fla.clone()));
        assert!(h2h_pal.is_some());
        let (scored, conceded) = h2h_pal.unwrap();
        assert_eq!(*scored, 1);
        assert_eq!(*conceded, 2);
    }

    #[test]
    fn test_table_record_result_home_win() {
        let mut table = setup_table();
        let fla = ClubId::new("FLA");
        let pal = ClubId::new("PAL");

        table.record_result(&fla, &pal, 2, 1, 3, 1);

        let fla_row = table.get_team(&fla).unwrap();
        assert_eq!(fla_row.played, 1);
        assert_eq!(fla_row.won, 1);
        assert_eq!(fla_row.points, 3);
        assert_eq!(fla_row.goals_for, 2);
        assert_eq!(fla_row.goals_against, 1);

        let pal_row = table.get_team(&pal).unwrap();
        assert_eq!(pal_row.played, 1);
        assert_eq!(pal_row.lost, 1);
        assert_eq!(pal_row.points, 0);
    }

    #[test]
    fn test_table_record_result_draw() {
        let mut table = setup_table();
        let fla = ClubId::new("FLA");
        let pal = ClubId::new("PAL");

        table.record_result(&fla, &pal, 1, 1, 3, 1);

        let fla_row = table.get_team(&fla).unwrap();
        assert_eq!(fla_row.drawn, 1);
        assert_eq!(fla_row.points, 1);

        let pal_row = table.get_team(&pal).unwrap();
        assert_eq!(pal_row.drawn, 1);
        assert_eq!(pal_row.points, 1);
    }

    #[test]
    fn test_table_record_result_away_win() {
        let mut table = setup_table();
        let fla = ClubId::new("FLA");
        let pal = ClubId::new("PAL");

        table.record_result(&fla, &pal, 0, 2, 3, 1);

        let fla_row = table.get_team(&fla).unwrap();
        assert_eq!(fla_row.lost, 1);
        assert_eq!(fla_row.points, 0);

        let pal_row = table.get_team(&pal).unwrap();
        assert_eq!(pal_row.won, 1);
        assert_eq!(pal_row.points, 3);
    }

    #[test]
    fn test_table_goal_difference() {
        let row = TableRow {
            club_id: ClubId::new("FLA"),
            played: 5,
            won: 3,
            drawn: 1,
            lost: 1,
            goals_for: 10,
            goals_against: 4,
            points: 10,
        };
        assert_eq!(row.goal_difference(), 6);
    }

    #[test]
    fn test_table_goal_difference_negative() {
        let row = TableRow {
            club_id: ClubId::new("FLA"),
            played: 5,
            won: 0,
            drawn: 1,
            lost: 4,
            goals_for: 2,
            goals_against: 12,
            points: 1,
        };
        assert_eq!(row.goal_difference(), -10);
    }

    #[test]
    fn test_table_add_team_no_duplicate() {
        let mut table = Table::new();
        let fla = ClubId::new("FLA");
        table.add_team(fla.clone());
        table.add_team(fla.clone());
        assert_eq!(table.rows.len(), 1);
    }
}
