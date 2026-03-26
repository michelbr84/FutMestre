//! Bank loan model.

use super::Money;
use serde::{Deserialize, Serialize};

/// A bank loan with interest and monthly payments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankLoan {
    /// Total amount borrowed.
    pub amount: Money,
    /// Annual interest rate (e.g. 0.05 = 5%).
    pub interest_rate: f64,
    /// Fixed monthly payment.
    pub monthly_payment: Money,
    /// Remaining balance to pay.
    pub remaining: Money,
    /// Start date (ISO format).
    pub start_date: String,
    /// Loan term in months.
    pub term_months: u16,
    /// Number of months already paid.
    pub months_paid: u16,
}

impl BankLoan {
    /// Create a new bank loan. Calculates monthly payment using standard
    /// amortization formula: M = P * [r(1+r)^n] / [(1+r)^n - 1]
    /// where r = monthly rate, n = term_months, P = amount.
    pub fn new(amount: Money, interest_rate: f64, term_months: u16, start_date: String) -> Self {
        let monthly_payment = Self::calc_monthly_payment(amount, interest_rate, term_months);
        Self {
            amount,
            interest_rate,
            monthly_payment,
            remaining: amount,
            start_date,
            term_months,
            months_paid: 0,
        }
    }

    /// Calculate monthly payment using amortization formula.
    fn calc_monthly_payment(amount: Money, annual_rate: f64, term_months: u16) -> Money {
        if annual_rate == 0.0 {
            let minor = amount.minor() / term_months as i64;
            return Money::from_minor(minor);
        }
        let r = annual_rate / 12.0;
        let n = term_months as f64;
        let factor = (1.0 + r).powf(n);
        let payment = amount.minor() as f64 * (r * factor) / (factor - 1.0);
        Money::from_minor(payment.round() as i64)
    }

    /// Make a monthly payment. Returns the amount actually paid (could be less
    /// on the final payment). Updates remaining balance and months_paid.
    pub fn make_payment(&mut self) -> Money {
        if self.remaining <= Money::ZERO {
            return Money::ZERO;
        }
        let payment = if self.monthly_payment > self.remaining {
            self.remaining
        } else {
            self.monthly_payment
        };
        self.remaining -= payment;
        self.months_paid += 1;
        payment
    }

    /// Total interest over the life of the loan.
    pub fn total_interest(&self) -> Money {
        let total_paid = self.monthly_payment.multiply(self.term_months as f64);
        if total_paid > self.amount {
            total_paid - self.amount
        } else {
            Money::ZERO
        }
    }

    /// Check if the loan is fully paid off.
    pub fn is_paid_off(&self) -> bool {
        self.remaining <= Money::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank_loan_new() {
        let loan = BankLoan::new(Money::from_major(1_000_000), 0.06, 12, "2026-01-01".into());
        assert_eq!(loan.amount.major(), 1_000_000);
        assert_eq!(loan.remaining.major(), 1_000_000);
        assert_eq!(loan.months_paid, 0);
        assert!(!loan.is_paid_off());
        // Monthly payment should be > principal/12 due to interest
        assert!(loan.monthly_payment > Money::from_minor(1_000_000_00 / 12));
    }

    #[test]
    fn test_bank_loan_zero_interest() {
        let loan = BankLoan::new(Money::from_major(1_200_000), 0.0, 12, "2026-01-01".into());
        assert_eq!(loan.monthly_payment.major(), 100_000);
    }

    #[test]
    fn test_make_payment() {
        let mut loan = BankLoan::new(Money::from_major(1_200_000), 0.0, 12, "2026-01-01".into());
        let paid = loan.make_payment();
        assert_eq!(paid.major(), 100_000);
        assert_eq!(loan.remaining.major(), 1_100_000);
        assert_eq!(loan.months_paid, 1);
    }

    #[test]
    fn test_full_repayment() {
        let mut loan = BankLoan::new(Money::from_major(120_000), 0.0, 12, "2026-01-01".into());
        for _ in 0..12 {
            loan.make_payment();
        }
        assert!(loan.is_paid_off());
        assert_eq!(loan.remaining, Money::ZERO);
        // Extra payment returns zero
        assert_eq!(loan.make_payment(), Money::ZERO);
    }

    #[test]
    fn test_total_interest() {
        let loan = BankLoan::new(Money::from_major(1_000_000), 0.06, 12, "2026-01-01".into());
        let interest = loan.total_interest();
        assert!(interest > Money::ZERO);
        // For a 6% annual rate, 12 months, interest should be roughly ~3.3% of principal
        assert!(interest.major() > 20_000);
        assert!(interest.major() < 100_000);
    }

    #[test]
    fn test_final_payment_caps_at_remaining() {
        let mut loan = BankLoan::new(Money::from_major(120_000), 0.0, 12, "2026-01-01".into());
        // Pay 11 months (each 10_000 major, evenly divisible)
        for _ in 0..11 {
            loan.make_payment();
        }
        let remaining_before = loan.remaining;
        let final_pay = loan.make_payment();
        assert_eq!(final_pay, remaining_before);
        assert!(loan.is_paid_off());
    }
}
