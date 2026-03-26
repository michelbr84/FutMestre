//! Financial reporting structures.

use serde::{Deserialize, Serialize};

/// Income breakdown for monthly reports.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IncomeBreakdown {
    pub matchday: i64,
    pub tv_rights: i64,
    pub sponsorship: i64,
    pub merchandising: i64,
    pub prize_money: i64,
    pub transfers: i64,
}

impl IncomeBreakdown {
    /// Total of all income categories.
    pub fn total(&self) -> i64 {
        self.matchday
            + self.tv_rights
            + self.sponsorship
            + self.merchandising
            + self.prize_money
            + self.transfers
    }
}

/// Expense breakdown for monthly reports.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExpenseBreakdown {
    pub wages: i64,
    pub transfers: i64,
    pub stadium: i64,
    pub other: i64,
}

impl ExpenseBreakdown {
    /// Total of all expense categories.
    pub fn total(&self) -> i64 {
        self.wages + self.transfers + self.stadium + self.other
    }
}

/// Monthly financial report for a club.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyReport {
    pub month: u32,
    pub year: u32,
    pub total_income: i64,
    pub total_expenses: i64,
    pub net_profit: i64,
    pub income_breakdown: IncomeBreakdown,
    pub expense_breakdown: ExpenseBreakdown,
    pub balance_start: i64,
    pub balance_end: i64,
}

impl MonthlyReport {
    /// Create a new monthly report.
    pub fn new(
        month: u32,
        year: u32,
        income_breakdown: IncomeBreakdown,
        expense_breakdown: ExpenseBreakdown,
        balance_start: i64,
    ) -> Self {
        let total_income = income_breakdown.total();
        let total_expenses = expense_breakdown.total();
        let net_profit = total_income - total_expenses;
        let balance_end = balance_start + net_profit;

        Self {
            month,
            year,
            total_income,
            total_expenses,
            net_profit,
            income_breakdown,
            expense_breakdown,
            balance_start,
            balance_end,
        }
    }

    /// Whether the month was profitable.
    pub fn is_profitable(&self) -> bool {
        self.net_profit > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_income_breakdown_total() {
        let income = IncomeBreakdown {
            matchday: 1_000_000,
            tv_rights: 5_000_000,
            sponsorship: 2_000_000,
            merchandising: 500_000,
            prize_money: 0,
            transfers: 3_000_000,
        };
        assert_eq!(income.total(), 11_500_000);
    }

    #[test]
    fn test_expense_breakdown_total() {
        let expenses = ExpenseBreakdown {
            wages: 4_000_000,
            transfers: 2_000_000,
            stadium: 300_000,
            other: 200_000,
        };
        assert_eq!(expenses.total(), 6_500_000);
    }

    #[test]
    fn test_monthly_report_profitable() {
        let income = IncomeBreakdown {
            matchday: 1_000_000,
            tv_rights: 5_000_000,
            sponsorship: 2_000_000,
            merchandising: 500_000,
            prize_money: 0,
            transfers: 0,
        };
        let expenses = ExpenseBreakdown {
            wages: 4_000_000,
            transfers: 0,
            stadium: 300_000,
            other: 200_000,
        };
        let report = MonthlyReport::new(1, 2026, income, expenses, 50_000_000);

        assert_eq!(report.total_income, 8_500_000);
        assert_eq!(report.total_expenses, 4_500_000);
        assert_eq!(report.net_profit, 4_000_000);
        assert_eq!(report.balance_start, 50_000_000);
        assert_eq!(report.balance_end, 54_000_000);
        assert!(report.is_profitable());
    }

    #[test]
    fn test_monthly_report_loss() {
        let income = IncomeBreakdown {
            matchday: 500_000,
            tv_rights: 1_000_000,
            ..Default::default()
        };
        let expenses = ExpenseBreakdown {
            wages: 3_000_000,
            stadium: 200_000,
            ..Default::default()
        };
        let report = MonthlyReport::new(6, 2026, income, expenses, 10_000_000);

        assert_eq!(report.total_income, 1_500_000);
        assert_eq!(report.total_expenses, 3_200_000);
        assert_eq!(report.net_profit, -1_700_000);
        assert_eq!(report.balance_end, 8_300_000);
        assert!(!report.is_profitable());
    }

    #[test]
    fn test_monthly_report_break_even() {
        let income = IncomeBreakdown {
            matchday: 2_000_000,
            ..Default::default()
        };
        let expenses = ExpenseBreakdown {
            wages: 2_000_000,
            ..Default::default()
        };
        let report = MonthlyReport::new(3, 2026, income, expenses, 20_000_000);

        assert_eq!(report.net_profit, 0);
        assert_eq!(report.balance_end, 20_000_000);
        assert!(!report.is_profitable());
    }

    #[test]
    fn test_default_breakdowns() {
        let income = IncomeBreakdown::default();
        assert_eq!(income.total(), 0);

        let expenses = ExpenseBreakdown::default();
        assert_eq!(expenses.total(), 0);
    }
}
