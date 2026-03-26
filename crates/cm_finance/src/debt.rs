//! Debt management and bank loans.

use cm_core::economy::Money;

// Re-export BankLoan from cm_core for convenience.
pub use cm_core::economy::BankLoan;

/// Calculate interest payment.
pub fn calculate_interest(debt: Money, rate: f32) -> Money {
    Money::from_minor((debt.minor() as f64 * rate as f64) as i64)
}
