//! Wage type for player/staff salaries.

use super::Money;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Wage frequency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WageFrequency {
    Weekly,
    Monthly,
    Yearly,
}

/// Wage with frequency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wage {
    pub amount: Money,
    pub frequency: WageFrequency,
}

impl Wage {
    /// Create a weekly wage.
    pub fn weekly(amount: Money) -> Self {
        Self {
            amount,
            frequency: WageFrequency::Weekly,
        }
    }

    /// Create a monthly wage.
    pub fn monthly(amount: Money) -> Self {
        Self {
            amount,
            frequency: WageFrequency::Monthly,
        }
    }

    /// Create a yearly wage.
    pub fn yearly(amount: Money) -> Self {
        Self {
            amount,
            frequency: WageFrequency::Yearly,
        }
    }

    /// Convert to weekly wage.
    pub fn as_weekly(&self) -> Money {
        match self.frequency {
            WageFrequency::Weekly => self.amount,
            WageFrequency::Monthly => self.amount.multiply(12.0 / 52.0),
            WageFrequency::Yearly => self.amount.multiply(1.0 / 52.0),
        }
    }

    /// Convert to monthly wage.
    pub fn as_monthly(&self) -> Money {
        match self.frequency {
            WageFrequency::Weekly => self.amount.multiply(52.0 / 12.0),
            WageFrequency::Monthly => self.amount,
            WageFrequency::Yearly => self.amount.multiply(1.0 / 12.0),
        }
    }

    /// Convert to yearly wage.
    pub fn as_yearly(&self) -> Money {
        match self.frequency {
            WageFrequency::Weekly => self.amount.multiply(52.0),
            WageFrequency::Monthly => self.amount.multiply(12.0),
            WageFrequency::Yearly => self.amount,
        }
    }
}

impl fmt::Display for Wage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suffix = match self.frequency {
            WageFrequency::Weekly => "p/w",
            WageFrequency::Monthly => "p/m",
            WageFrequency::Yearly => "p/a",
        };
        write!(f, "{} {}", self.amount, suffix)
    }
}

impl Default for Wage {
    fn default() -> Self {
        Self::weekly(Money::ZERO)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wage_weekly_creation() {
        let wage = Wage::weekly(Money::from_major(10_000));
        assert_eq!(wage.amount.major(), 10_000);
        assert_eq!(wage.frequency, WageFrequency::Weekly);
    }

    #[test]
    fn test_wage_monthly_creation() {
        let wage = Wage::monthly(Money::from_major(40_000));
        assert_eq!(wage.frequency, WageFrequency::Monthly);
    }

    #[test]
    fn test_wage_yearly_creation() {
        let wage = Wage::yearly(Money::from_major(500_000));
        assert_eq!(wage.frequency, WageFrequency::Yearly);
    }

    #[test]
    fn test_weekly_to_yearly() {
        let wage = Wage::weekly(Money::from_major(10_000));
        let yearly = wage.as_yearly();
        assert_eq!(yearly.major(), 520_000); // 10k * 52
    }

    #[test]
    fn test_yearly_to_weekly() {
        let wage = Wage::yearly(Money::from_major(520_000));
        let weekly = wage.as_weekly();
        assert_eq!(weekly.major(), 10_000); // 520k / 52
    }

    #[test]
    fn test_monthly_to_yearly() {
        let wage = Wage::monthly(Money::from_major(50_000));
        let yearly = wage.as_yearly();
        assert_eq!(yearly.major(), 600_000); // 50k * 12
    }

    #[test]
    fn test_wage_display_weekly() {
        let wage = Wage::weekly(Money::from_major(5_000));
        let display = format!("{}", wage);
        assert!(display.contains("p/w"));
    }

    #[test]
    fn test_wage_display_monthly() {
        let wage = Wage::monthly(Money::from_major(20_000));
        let display = format!("{}", wage);
        assert!(display.contains("p/m"));
    }

    #[test]
    fn test_wage_display_yearly() {
        let wage = Wage::yearly(Money::from_major(250_000));
        let display = format!("{}", wage);
        assert!(display.contains("p/a"));
    }

    #[test]
    fn test_wage_default() {
        let wage = Wage::default();
        assert_eq!(wage.amount, Money::ZERO);
        assert_eq!(wage.frequency, WageFrequency::Weekly);
    }

    #[test]
    fn test_wage_serialization() {
        let wage = Wage::weekly(Money::from_major(15_000));
        let json = serde_json::to_string(&wage).unwrap();
        let parsed: Wage = serde_json::from_str(&json).unwrap();
        assert_eq!(wage, parsed);
    }

    #[test]
    fn test_conversion_roundtrip() {
        let original = Wage::weekly(Money::from_major(10_000));
        let yearly = original.as_yearly();
        let weekly_back = Wage::yearly(yearly).as_weekly();
        // Allow small rounding difference
        assert!((weekly_back.major() - 10_000).abs() <= 1);
    }
}
