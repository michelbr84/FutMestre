//! Transfer tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::window::is_window_open;
    use chrono::NaiveDate;

    #[test]
    fn test_transfer_window() {
        assert!(is_window_open(
            NaiveDate::from_ymd_opt(2001, 7, 15).unwrap()
        ));
        assert!(is_window_open(
            NaiveDate::from_ymd_opt(2001, 1, 10).unwrap()
        ));
        assert!(!is_window_open(
            NaiveDate::from_ymd_opt(2001, 10, 15).unwrap()
        ));
    }
}
