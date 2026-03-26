//! Calendar system.

use crate::ids::{CompetitionId, MatchId};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Calendar entry type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CalendarEntryType {
    Match { match_id: MatchId },
    Training,
    TransferDeadline,
    SeasonStart,
    SeasonEnd,
    InternationalBreak,
    Other { description: String },
}

/// A calendar entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEntry {
    pub date: NaiveDate,
    pub entry_type: CalendarEntryType,
    pub competition_id: Option<CompetitionId>,
}

impl CalendarEntry {
    /// Create a match entry.
    pub fn match_entry(date: NaiveDate, match_id: MatchId, competition_id: CompetitionId) -> Self {
        Self {
            date,
            entry_type: CalendarEntryType::Match { match_id },
            competition_id: Some(competition_id),
        }
    }

    /// Create a training entry.
    pub fn training(date: NaiveDate) -> Self {
        Self {
            date,
            entry_type: CalendarEntryType::Training,
            competition_id: None,
        }
    }

    /// Check if match day.
    pub fn is_match_day(&self) -> bool {
        matches!(self.entry_type, CalendarEntryType::Match { .. })
    }
}

/// Game calendar.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Calendar {
    pub entries: Vec<CalendarEntry>,
}

impl Calendar {
    /// Create a new calendar.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Add an entry.
    pub fn add(&mut self, entry: CalendarEntry) {
        self.entries.push(entry);
        self.entries.sort_by_key(|e| e.date);
    }

    /// Get entries for a date.
    pub fn on_date(&self, date: NaiveDate) -> Vec<&CalendarEntry> {
        self.entries.iter().filter(|e| e.date == date).collect()
    }

    /// Check if date has a match.
    pub fn is_match_day(&self, date: NaiveDate) -> bool {
        self.on_date(date).iter().any(|e| e.is_match_day())
    }

    /// Get next match date.
    pub fn next_match_date(&self, from: NaiveDate) -> Option<NaiveDate> {
        self.entries
            .iter()
            .filter(|e| e.date >= from && e.is_match_day())
            .map(|e| e.date)
            .next()
    }
}
