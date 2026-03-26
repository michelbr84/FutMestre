//! Contract entity.

use crate::economy::{Money, Wage};
use crate::ids::ContractId;
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// A player or staff contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub id: ContractId,
    pub wage: Wage,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub signing_fee: Money,
    pub release_clause: Option<Money>,
    pub loyalty_bonus: Money,
    pub appearance_bonus: Money,
    pub goal_bonus: Money,
}

impl Contract {
    /// Create a new contract.
    pub fn new(wage: Wage, start_date: NaiveDate, end_date: NaiveDate) -> Self {
        Self {
            id: ContractId::new(uuid::Uuid::new_v4().to_string()),
            wage,
            start_date,
            end_date,
            signing_fee: Money::ZERO,
            release_clause: None,
            loyalty_bonus: Money::ZERO,
            appearance_bonus: Money::ZERO,
            goal_bonus: Money::ZERO,
        }
    }

    /// Check if contract is active on date.
    pub fn is_active(&self, date: NaiveDate) -> bool {
        date >= self.start_date && date <= self.end_date
    }

    /// Check if contract is expiring soon (within 6 months).
    pub fn is_expiring_soon(&self, date: NaiveDate) -> bool {
        let months_remaining = (self.end_date.year() - date.year()) * 12
            + (self.end_date.month() as i32 - date.month() as i32);
        months_remaining <= 6 && months_remaining > 0
    }

    /// Check if the contract has expired (date is past end_date).
    pub fn is_expired(&self, date: NaiveDate) -> bool {
        date > self.end_date
    }

    /// Get years remaining.
    pub fn years_remaining(&self, date: NaiveDate) -> f32 {
        let days = (self.end_date - date).num_days();
        days as f32 / 365.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::economy::Wage;

    fn make_contract(start: (i32, u32, u32), end: (i32, u32, u32)) -> Contract {
        Contract::new(
            Wage::weekly(Money::from_major(10_000)),
            NaiveDate::from_ymd_opt(start.0, start.1, start.2).unwrap(),
            NaiveDate::from_ymd_opt(end.0, end.1, end.2).unwrap(),
        )
    }

    #[test]
    fn test_is_expired_after_end_date() {
        let c = make_contract((2024, 1, 1), (2025, 6, 30));
        let after = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();
        assert!(c.is_expired(after));
    }

    #[test]
    fn test_is_not_expired_on_end_date() {
        let c = make_contract((2024, 1, 1), (2025, 6, 30));
        let on = NaiveDate::from_ymd_opt(2025, 6, 30).unwrap();
        assert!(!c.is_expired(on));
    }

    #[test]
    fn test_is_not_expired_before_end_date() {
        let c = make_contract((2024, 1, 1), (2025, 6, 30));
        let before = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        assert!(!c.is_expired(before));
    }

    #[test]
    fn test_is_active_within_range() {
        let c = make_contract((2024, 1, 1), (2025, 6, 30));
        let mid = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap();
        assert!(c.is_active(mid));
    }

    #[test]
    fn test_expired_implies_not_active() {
        let c = make_contract((2024, 1, 1), (2025, 6, 30));
        let after = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();
        assert!(c.is_expired(after));
        assert!(!c.is_active(after));
    }
}
