//! Time utilities.

use chrono::{Datelike, NaiveDate, Weekday};

/// Get the current season year from a date.
/// Season runs from July to June, so dates before July belong to previous season.
pub fn season_year(date: NaiveDate) -> i32 {
    if date.month() >= 7 {
        date.year()
    } else {
        date.year() - 1
    }
}

/// Check if a date is a weekend.
pub fn is_weekend(date: NaiveDate) -> bool {
    matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}

/// Get days until next occurrence of a weekday.
pub fn days_until_weekday(from: NaiveDate, target: Weekday) -> i64 {
    let current = from.weekday().num_days_from_monday() as i64;
    let target = target.num_days_from_monday() as i64;
    let diff = target - current;
    if diff <= 0 {
        diff + 7
    } else {
        diff
    }
}

/// Format date as display string.
pub fn format_date(date: NaiveDate) -> String {
    date.format("%d %b %Y").to_string()
}

/// Parse date from string (YYYY-MM-DD).
pub fn parse_date(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn test_season_year_after_july() {
        assert_eq!(season_year(date(2024, 8, 15)), 2024);
        assert_eq!(season_year(date(2024, 12, 25)), 2024);
    }

    #[test]
    fn test_season_year_before_july() {
        assert_eq!(season_year(date(2024, 1, 15)), 2023);
        assert_eq!(season_year(date(2024, 6, 30)), 2023);
    }

    #[test]
    fn test_season_year_july() {
        assert_eq!(season_year(date(2024, 7, 1)), 2024);
    }

    #[test]
    fn test_is_weekend_saturday() {
        // 2024-01-06 is Saturday
        assert!(is_weekend(date(2024, 1, 6)));
    }

    #[test]
    fn test_is_weekend_sunday() {
        // 2024-01-07 is Sunday
        assert!(is_weekend(date(2024, 1, 7)));
    }

    #[test]
    fn test_is_weekend_weekday() {
        // 2024-01-08 is Monday
        assert!(!is_weekend(date(2024, 1, 8)));
        // 2024-01-10 is Wednesday
        assert!(!is_weekend(date(2024, 1, 10)));
        // 2024-01-12 is Friday
        assert!(!is_weekend(date(2024, 1, 12)));
    }

    #[test]
    fn test_days_until_weekday_same_day() {
        // 2024-01-08 is Monday, asking for Monday
        assert_eq!(days_until_weekday(date(2024, 1, 8), Weekday::Mon), 7);
    }

    #[test]
    fn test_days_until_weekday_forward() {
        // 2024-01-08 is Monday, asking for Wednesday
        assert_eq!(days_until_weekday(date(2024, 1, 8), Weekday::Wed), 2);
        // 2024-01-08 is Monday, asking for Saturday
        assert_eq!(days_until_weekday(date(2024, 1, 8), Weekday::Sat), 5);
    }

    #[test]
    fn test_days_until_weekday_wrap() {
        // 2024-01-12 is Friday, asking for Monday
        assert_eq!(days_until_weekday(date(2024, 1, 12), Weekday::Mon), 3);
    }

    #[test]
    fn test_format_date() {
        let formatted = format_date(date(2024, 1, 15));
        assert_eq!(formatted, "15 Jan 2024");
    }

    #[test]
    fn test_format_date_single_digit_day() {
        let formatted = format_date(date(2024, 12, 5));
        assert_eq!(formatted, "05 Dec 2024");
    }

    #[test]
    fn test_parse_date_valid() {
        let parsed = parse_date("2024-01-15");
        assert_eq!(parsed, Some(date(2024, 1, 15)));
    }

    #[test]
    fn test_parse_date_invalid() {
        assert!(parse_date("invalid").is_none());
        assert!(parse_date("2024/01/15").is_none());
        assert!(parse_date("15-01-2024").is_none());
    }

    #[test]
    fn test_parse_date_roundtrip() {
        let original = date(2024, 7, 1);
        let formatted = original.format("%Y-%m-%d").to_string();
        let parsed = parse_date(&formatted);
        assert_eq!(parsed, Some(original));
    }
}
