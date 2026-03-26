//! Game date and time management.

use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

/// Game date wrapper with football-specific utilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct GameDate(pub NaiveDate);

impl GameDate {
    /// Create a new game date.
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self(
            NaiveDate::from_ymd_opt(year, month, day)
                .unwrap_or_else(|| NaiveDate::from_ymd_opt(2001, 7, 1).unwrap()),
        )
    }

    /// Get the inner NaiveDate.
    pub fn date(&self) -> NaiveDate {
        self.0
    }

    /// Advance by one day.
    pub fn advance_day(&mut self) {
        self.0 = self.0.succ_opt().unwrap_or(self.0);
    }

    /// Advance by n days.
    pub fn advance_days(&mut self, n: u32) {
        for _ in 0..n {
            self.advance_day();
        }
    }

    /// Get the current season year.
    /// Season runs July-June, so dates before July belong to previous season.
    pub fn season_year(&self) -> i32 {
        if self.0.month() >= 7 {
            self.0.year()
        } else {
            self.0.year() - 1
        }
    }

    /// Get the season string (e.g., "2001-02").
    pub fn season_string(&self) -> String {
        let start = self.season_year();
        let end = (start + 1) % 100;
        format!("{}-{:02}", start, end)
    }

    /// Check if it's a weekend.
    pub fn is_weekend(&self) -> bool {
        matches!(self.0.weekday(), Weekday::Sat | Weekday::Sun)
    }

    /// Check if it's a Saturday (typical match day).
    pub fn is_saturday(&self) -> bool {
        self.0.weekday() == Weekday::Sat
    }

    /// Check if it's the first day of the month.
    pub fn is_first_of_month(&self) -> bool {
        self.0.day() == 1
    }

    /// Get the day of week.
    pub fn weekday(&self) -> Weekday {
        self.0.weekday()
    }

    /// Get year.
    pub fn year(&self) -> i32 {
        self.0.year()
    }

    /// Get month.
    pub fn month(&self) -> u32 {
        self.0.month()
    }

    /// Get day.
    pub fn day(&self) -> u32 {
        self.0.day()
    }

    /// Parse from string.
    pub fn parse(s: &str) -> Option<Self> {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").ok().map(Self)
    }
}

impl Default for GameDate {
    fn default() -> Self {
        Self::new(2001, 7, 1)
    }
}

impl std::fmt::Display for GameDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%d %b %Y"))
    }
}

impl From<NaiveDate> for GameDate {
    fn from(date: NaiveDate) -> Self {
        Self(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_date_creation() {
        let date = GameDate::new(2024, 1, 15);
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 15);
    }

    #[test]
    fn test_game_date_invalid_fallback() {
        let date = GameDate::new(2024, 13, 40); // invalid
        assert_eq!(date.year(), 2001);
        assert_eq!(date.month(), 7);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_advance_day() {
        let mut date = GameDate::new(2024, 1, 15);
        date.advance_day();
        assert_eq!(date.day(), 16);
    }

    #[test]
    fn test_advance_day_month_boundary() {
        let mut date = GameDate::new(2024, 1, 31);
        date.advance_day();
        assert_eq!(date.month(), 2);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_advance_days() {
        let mut date = GameDate::new(2024, 1, 1);
        date.advance_days(10);
        assert_eq!(date.day(), 11);
    }

    #[test]
    fn test_season_year_after_july() {
        let date = GameDate::new(2024, 8, 1);
        assert_eq!(date.season_year(), 2024);
    }

    #[test]
    fn test_season_year_before_july() {
        let date = GameDate::new(2025, 3, 15);
        assert_eq!(date.season_year(), 2024);
    }

    #[test]
    fn test_season_string() {
        let date = GameDate::new(2024, 8, 1);
        assert_eq!(date.season_string(), "2024-25");
    }

    #[test]
    fn test_is_weekend() {
        // 2024-01-06 is Saturday
        let saturday = GameDate::new(2024, 1, 6);
        assert!(saturday.is_weekend());

        // 2024-01-07 is Sunday
        let sunday = GameDate::new(2024, 1, 7);
        assert!(sunday.is_weekend());

        // 2024-01-08 is Monday
        let monday = GameDate::new(2024, 1, 8);
        assert!(!monday.is_weekend());
    }

    #[test]
    fn test_is_saturday() {
        let saturday = GameDate::new(2024, 1, 6);
        assert!(saturday.is_saturday());

        let sunday = GameDate::new(2024, 1, 7);
        assert!(!sunday.is_saturday());
    }

    #[test]
    fn test_is_first_of_month() {
        let first = GameDate::new(2024, 1, 1);
        assert!(first.is_first_of_month());

        let second = GameDate::new(2024, 1, 2);
        assert!(!second.is_first_of_month());
    }

    #[test]
    fn test_weekday() {
        let monday = GameDate::new(2024, 1, 8);
        assert_eq!(monday.weekday(), Weekday::Mon);
    }

    #[test]
    fn test_parse_valid() {
        let date = GameDate::parse("2024-01-15");
        assert!(date.is_some());
        let date = date.unwrap();
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 15);
    }

    #[test]
    fn test_parse_invalid() {
        assert!(GameDate::parse("invalid").is_none());
        assert!(GameDate::parse("2024/01/15").is_none());
    }

    #[test]
    fn test_display() {
        let date = GameDate::new(2024, 1, 15);
        let display = format!("{}", date);
        assert!(display.contains("15"));
        assert!(display.contains("Jan"));
        assert!(display.contains("2024"));
    }

    #[test]
    fn test_default() {
        let date = GameDate::default();
        assert_eq!(date.year(), 2001);
        assert_eq!(date.month(), 7);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_from_naive_date() {
        let naive = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let game_date = GameDate::from(naive);
        assert_eq!(game_date.date(), naive);
    }

    #[test]
    fn test_ordering() {
        let date1 = GameDate::new(2024, 1, 1);
        let date2 = GameDate::new(2024, 1, 15);
        let date3 = GameDate::new(2024, 2, 1);

        assert!(date1 < date2);
        assert!(date2 < date3);
        assert!(date1 < date3);
    }

    #[test]
    fn test_serialization() {
        let date = GameDate::new(2024, 6, 15);
        let json = serde_json::to_string(&date).unwrap();
        let parsed: GameDate = serde_json::from_str(&json).unwrap();
        assert_eq!(date, parsed);
    }
}
