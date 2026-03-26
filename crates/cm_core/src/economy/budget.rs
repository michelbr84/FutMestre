//! Budget management for clubs.

use super::Money;
use serde::{Deserialize, Serialize};

/// Club budget allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    /// Total balance.
    pub balance: Money,
    /// Transfer budget remaining.
    pub transfer_budget: Money,
    /// Wage budget (weekly).
    pub wage_budget: Money,
    /// Current weekly wage bill.
    pub wage_bill: Money,
}

impl Budget {
    /// Create a new budget.
    pub fn new(balance: Money, transfer_budget: Money, wage_budget: Money) -> Self {
        Self {
            balance,
            transfer_budget,
            wage_budget,
            wage_bill: Money::ZERO,
        }
    }

    /// Available transfer funds.
    pub fn available_for_transfers(&self) -> Money {
        self.transfer_budget
    }

    /// Available wage room (weekly).
    pub fn available_wage_room(&self) -> Money {
        self.wage_budget - self.wage_bill
    }

    /// Check if can afford a transfer.
    pub fn can_afford_transfer(&self, amount: Money) -> bool {
        amount <= self.transfer_budget
    }

    /// Check if can afford a wage.
    pub fn can_afford_wage(&self, weekly_wage: Money) -> bool {
        self.wage_bill + weekly_wage <= self.wage_budget
    }

    /// Spend on transfer.
    pub fn spend_transfer(&mut self, amount: Money) {
        self.transfer_budget -= amount;
        self.balance -= amount;
    }

    /// Receive transfer income.
    pub fn receive_transfer(&mut self, amount: Money) {
        self.transfer_budget += amount;
        self.balance += amount;
    }

    /// Add to wage bill.
    pub fn add_wage(&mut self, weekly_wage: Money) {
        self.wage_bill += weekly_wage;
    }

    /// Remove from wage bill.
    pub fn remove_wage(&mut self, weekly_wage: Money) {
        self.wage_bill -= weekly_wage;
    }

    /// Process weekly wage payment.
    pub fn pay_weekly_wages(&mut self) {
        self.balance -= self.wage_bill;
    }

    /// Process a transfer expense (alias for spend_transfer).
    pub fn process_transfer_expense(&mut self, fee: Money) {
        self.spend_transfer(fee);
    }

    /// Process transfer income (alias for receive_transfer).
    pub fn process_transfer_income(&mut self, fee: Money) {
        self.receive_transfer(fee);
    }
}

impl Default for Budget {
    fn default() -> Self {
        Self::new(Money::ZERO, Money::ZERO, Money::ZERO)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_budget() -> Budget {
        Budget::new(
            Money::from_major(50_000_000),  // balance
            Money::from_major(20_000_000),  // transfer budget
            Money::from_major(500_000),     // wage budget
        )
    }

    #[test]
    fn test_budget_creation() {
        let budget = test_budget();
        assert_eq!(budget.balance.major(), 50_000_000);
        assert_eq!(budget.transfer_budget.major(), 20_000_000);
        assert_eq!(budget.wage_budget.major(), 500_000);
        assert_eq!(budget.wage_bill, Money::ZERO);
    }

    #[test]
    fn test_available_for_transfers() {
        let budget = test_budget();
        assert_eq!(budget.available_for_transfers().major(), 20_000_000);
    }

    #[test]
    fn test_available_wage_room() {
        let mut budget = test_budget();
        assert_eq!(budget.available_wage_room().major(), 500_000);
        
        budget.add_wage(Money::from_major(200_000));
        assert_eq!(budget.available_wage_room().major(), 300_000);
    }

    #[test]
    fn test_can_afford_transfer() {
        let budget = test_budget();
        assert!(budget.can_afford_transfer(Money::from_major(10_000_000)));
        assert!(budget.can_afford_transfer(Money::from_major(20_000_000)));
        assert!(!budget.can_afford_transfer(Money::from_major(25_000_000)));
    }

    #[test]
    fn test_can_afford_wage() {
        let mut budget = test_budget();
        budget.add_wage(Money::from_major(400_000));
        
        assert!(budget.can_afford_wage(Money::from_major(100_000)));
        assert!(!budget.can_afford_wage(Money::from_major(200_000)));
    }

    #[test]
    fn test_spend_transfer() {
        let mut budget = test_budget();
        let fee = Money::from_major(5_000_000);
        budget.spend_transfer(fee);
        
        assert_eq!(budget.transfer_budget.major(), 15_000_000);
        assert_eq!(budget.balance.major(), 45_000_000);
    }

    #[test]
    fn test_receive_transfer() {
        let mut budget = test_budget();
        let fee = Money::from_major(10_000_000);
        budget.receive_transfer(fee);
        
        assert_eq!(budget.transfer_budget.major(), 30_000_000);
        assert_eq!(budget.balance.major(), 60_000_000);
    }

    #[test]
    fn test_wage_operations() {
        let mut budget = test_budget();
        
        budget.add_wage(Money::from_major(100_000));
        assert_eq!(budget.wage_bill.major(), 100_000);
        
        budget.add_wage(Money::from_major(50_000));
        assert_eq!(budget.wage_bill.major(), 150_000);
        
        budget.remove_wage(Money::from_major(30_000));
        assert_eq!(budget.wage_bill.major(), 120_000);
    }

    #[test]
    fn test_pay_weekly_wages() {
        let mut budget = test_budget();
        budget.add_wage(Money::from_major(200_000));
        
        budget.pay_weekly_wages();
        assert_eq!(budget.balance.major(), 49_800_000);
        
        // Paying wages multiple times
        budget.pay_weekly_wages();
        assert_eq!(budget.balance.major(), 49_600_000);
    }

    #[test]
    fn test_budget_default() {
        let budget = Budget::default();
        assert_eq!(budget.balance, Money::ZERO);
        assert_eq!(budget.transfer_budget, Money::ZERO);
        assert_eq!(budget.wage_budget, Money::ZERO);
        assert_eq!(budget.wage_bill, Money::ZERO);
    }

    #[test]
    fn test_budget_serialization() {
        let budget = test_budget();
        let json = serde_json::to_string(&budget).unwrap();
        let parsed: Budget = serde_json::from_str(&json).unwrap();
        
        assert_eq!(budget.balance, parsed.balance);
        assert_eq!(budget.transfer_budget, parsed.transfer_budget);
    }
}
