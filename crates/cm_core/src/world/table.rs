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
