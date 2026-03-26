//! Staff entity.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::Contract;
use crate::ids::{ClubId, NationId, StaffId};

/// Staff role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StaffRole {
    Manager,
    AssistantManager,
    Coach,
    GoalkeeperCoach,
    FitnessCoach,
    Scout,
    Physio,
    YouthCoach,
    DataAnalyst,
}

impl StaffRole {
    /// Get display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Manager => "Manager",
            Self::AssistantManager => "Assistant Manager",
            Self::Coach => "Coach",
            Self::GoalkeeperCoach => "Goalkeeper Coach",
            Self::FitnessCoach => "Fitness Coach",
            Self::Scout => "Scout",
            Self::Physio => "Physiotherapist",
            Self::YouthCoach => "Youth Coach",
            Self::DataAnalyst => "Data Analyst",
        }
    }
}

impl Default for StaffRole {
    fn default() -> Self {
        Self::Coach
    }
}

/// A staff member.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Staff {
    pub id: StaffId,
    pub first_name: String,
    pub last_name: String,
    pub nationality: NationId,
    pub birth_date: NaiveDate,
    pub role: StaffRole,
    pub club_id: Option<ClubId>,
    pub contract: Option<Contract>,
    // Staff attributes (1-20)
    pub coaching: u8,
    pub man_management: u8,
    pub tactics: u8,
    pub scouting: u8,
    pub youth_development: u8,
    pub physiotherapy: u8,
    pub fitness: u8,
}

impl Staff {
    /// Create a new staff member.
    pub fn new(
        id: impl Into<StaffId>,
        first_name: impl Into<String>,
        last_name: impl Into<String>,
        nationality: NationId,
        role: StaffRole,
    ) -> Self {
        Self {
            id: id.into(),
            first_name: first_name.into(),
            last_name: last_name.into(),
            nationality,
            birth_date: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            role,
            club_id: None,
            contract: None,
            coaching: 10,
            man_management: 10,
            tactics: 10,
            scouting: 10,
            youth_development: 10,
            physiotherapy: 10,
            fitness: 10,
        }
    }

    /// Get full name.
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    /// Get overall rating for role.
    pub fn role_rating(&self) -> u8 {
        match self.role {
            StaffRole::Manager | StaffRole::AssistantManager => {
                ((self.coaching as u16 + self.man_management as u16 + self.tactics as u16) / 3)
                    as u8
            }
            StaffRole::Coach | StaffRole::GoalkeeperCoach | StaffRole::FitnessCoach => {
                self.coaching
            }
            StaffRole::Scout => self.scouting,
            StaffRole::Physio => self.physiotherapy,
            StaffRole::YouthCoach => self.youth_development,
            StaffRole::DataAnalyst => self.tactics,
        }
    }
}
