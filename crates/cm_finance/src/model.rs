//! Financial model.

use cm_core::economy::Money;
use serde::{Deserialize, Serialize};

/// Club financial statement.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FinancialStatement {
    pub income: Income,
    pub expenses: Expenses,
}

/// Income sources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Income {
    pub matchday: Money,
    pub tv_rights: Money,
    pub sponsorship: Money,
    pub merchandise: Money,
    pub prize_money: Money,
    pub transfers: Money,
}

impl Income {
    pub fn total(&self) -> Money {
        self.matchday
            + self.tv_rights
            + self.sponsorship
            + self.merchandise
            + self.prize_money
            + self.transfers
    }
}

/// Expense categories.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Expenses {
    pub wages: Money,
    pub transfers: Money,
    pub stadium: Money,
    pub other: Money,
}

impl Expenses {
    pub fn total(&self) -> Money {
        self.wages + self.transfers + self.stadium + self.other
    }
}

impl FinancialStatement {
    pub fn net(&self) -> Money {
        self.income.total() - self.expenses.total()
    }
}
