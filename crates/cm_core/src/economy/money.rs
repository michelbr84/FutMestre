//! Money type for financial operations.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Money amount in the smallest currency unit (cents/pence).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct Money(i64);

impl Money {
    /// Zero money.
    pub const ZERO: Money = Money(0);

    /// Create money from major units (pounds/euros/dollars).
    pub fn from_major(amount: i64) -> Self {
        Self(amount * 100)
    }

    /// Create money from minor units (pence/cents).
    pub fn from_minor(amount: i64) -> Self {
        Self(amount)
    }

    /// Get the amount in minor units.
    pub fn minor(&self) -> i64 {
        self.0
    }

    /// Get the amount in major units.
    pub fn major(&self) -> i64 {
        self.0 / 100
    }

    /// Check if zero.
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Check if negative.
    pub fn is_negative(&self) -> bool {
        self.0 < 0
    }

    /// Get absolute value.
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    /// Multiply by a factor.
    pub fn multiply(&self, factor: f64) -> Self {
        Self((self.0 as f64 * factor) as i64)
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl SubAssign for Money {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let major = self.0.abs() / 100;
        let sign = if self.0 < 0 { "-" } else { "" };

        if major >= 1_000_000 {
            write!(f, "{}£{:.1}M", sign, major as f64 / 1_000_000.0)
        } else if major >= 1_000 {
            write!(f, "{}£{:.0}K", sign, major as f64 / 1_000.0)
        } else {
            write!(f, "{}£{}", sign, major)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_operations() {
        let a = Money::from_major(1000);
        let b = Money::from_major(500);

        assert_eq!((a + b).major(), 1500);
        assert_eq!((a - b).major(), 500);
        assert_eq!(a.multiply(1.5).major(), 1500);
    }

    #[test]
    fn test_money_display() {
        assert_eq!(format!("{}", Money::from_major(500)), "£500");
        assert_eq!(format!("{}", Money::from_major(5_000)), "£5K");
        assert_eq!(format!("{}", Money::from_major(5_000_000)), "£5.0M");
    }
}
