//! Calendar system.

use crate::ids::{CompetitionId, MatchId};
use chrono::{Datelike, NaiveDate};
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

    /// Check if a date falls within an international break.
    pub fn is_international_break(&self, date: NaiveDate) -> bool {
        self.entries.iter().any(|e| {
            e.date == date && matches!(e.entry_type, CalendarEntryType::InternationalBreak)
        })
    }

    /// Check if a date falls within any FIFA international break period.
    /// Uses standard FIFA calendar windows.
    pub fn is_fifa_break(date: NaiveDate) -> bool {
        let month = date.month();
        let day = date.day();
        matches!(
            (month, day),
            // Marco: semana FIFA (normalmente dias 20-28)
            (3, 20..=28) |
            // Junho: janela longa (1-14)
            (6, 1..=14) |
            // Setembro: semana FIFA (4-12)
            (9, 4..=12) |
            // Outubro: semana FIFA (9-17)
            (10, 9..=17) |
            // Novembro: semana FIFA (13-21)
            (11, 13..=21)
        )
    }

    /// Populate FIFA international break entries for a given season year.
    /// Season runs from July of `year` to June of `year+1`.
    pub fn add_fifa_breaks(&mut self, year: i32) {
        let breaks = [
            // Setembro do ano da temporada
            (year, 9, 4, 12),
            // Outubro do ano da temporada
            (year, 10, 9, 17),
            // Novembro do ano da temporada
            (year, 11, 13, 21),
            // Marco do ano seguinte
            (year + 1, 3, 20, 28),
            // Junho do ano seguinte
            (year + 1, 6, 1, 14),
        ];

        for (y, m, start_day, end_day) in breaks {
            for d in start_day..=end_day {
                if let Some(date) = NaiveDate::from_ymd_opt(y, m, d) {
                    self.add(CalendarEntry {
                        date,
                        entry_type: CalendarEntryType::InternationalBreak,
                        competition_id: None,
                    });
                }
            }
        }
    }
}
