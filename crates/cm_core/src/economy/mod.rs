//! Economy module - money, wages, budgets, loans.

mod budget;
mod loan;
mod money;
mod wage;

pub use budget::Budget;
pub use loan::BankLoan;
pub use money::Money;
pub use wage::Wage;
