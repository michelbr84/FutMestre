//! Merchandising revenue calculations.

use cm_core::economy::Money;

/// Calculates merchandising revenue based on reputation and recent form.
///
/// Base revenue = reputation * 1000 (in major currency units).
/// Modified by recent results:
/// - Each win adds 5% to the base
/// - Each loss subtracts 3% from the base
/// - The modifier is clamped so revenue never goes below 50% of base
///   and never exceeds 200% of base.
pub fn calculate_merchandising(reputation: u32, recent_wins: u32, recent_losses: u32) -> i64 {
    let base = reputation as i64 * 1_000;

    let win_bonus = recent_wins as f64 * 0.05;
    let loss_penalty = recent_losses as f64 * 0.03;
    let modifier = (1.0 + win_bonus - loss_penalty).clamp(0.5, 2.0);

    (base as f64 * modifier) as i64
}

/// Calculate merchandising revenue as Money type.
pub fn merchandising_as_money(reputation: u32, recent_wins: u32, recent_losses: u32) -> Money {
    Money::from_major(calculate_merchandising(
        reputation,
        recent_wins,
        recent_losses,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_merchandising() {
        // No wins or losses, pure base
        let revenue = calculate_merchandising(100, 0, 0);
        assert_eq!(revenue, 100_000); // 100 * 1000
    }

    #[test]
    fn test_high_reputation() {
        let revenue = calculate_merchandising(200, 0, 0);
        assert_eq!(revenue, 200_000); // 200 * 1000
    }

    #[test]
    fn test_zero_reputation() {
        let revenue = calculate_merchandising(0, 10, 0);
        assert_eq!(revenue, 0); // 0 base
    }

    #[test]
    fn test_wins_increase_revenue() {
        let base = calculate_merchandising(100, 0, 0);
        let with_wins = calculate_merchandising(100, 10, 0);
        assert!(with_wins > base);
    }

    #[test]
    fn test_losses_decrease_revenue() {
        let base = calculate_merchandising(100, 0, 0);
        let with_losses = calculate_merchandising(100, 0, 10);
        assert!(with_losses < base);
    }

    #[test]
    fn test_modifier_clamped_min() {
        // Many losses, modifier should be clamped to 0.5
        let revenue = calculate_merchandising(100, 0, 50);
        assert_eq!(revenue, 50_000); // 100_000 * 0.5
    }

    #[test]
    fn test_modifier_clamped_max() {
        // Many wins, modifier should be clamped to 2.0
        let revenue = calculate_merchandising(100, 100, 0);
        assert_eq!(revenue, 200_000); // 100_000 * 2.0
    }

    #[test]
    fn test_wins_and_losses_combined() {
        // 5 wins (+25%) and 5 losses (-15%) => net +10%
        let revenue = calculate_merchandising(100, 5, 5);
        assert_eq!(revenue, 110_000); // 100_000 * 1.10
    }

    #[test]
    fn test_merchandising_as_money() {
        let money = merchandising_as_money(100, 5, 0);
        assert_eq!(money.major(), 125_000); // 100_000 * 1.25
    }

    #[test]
    fn test_single_win_bonus() {
        let revenue = calculate_merchandising(100, 1, 0);
        assert_eq!(revenue, 105_000); // 100_000 * 1.05
    }

    #[test]
    fn test_single_loss_penalty() {
        let revenue = calculate_merchandising(100, 0, 1);
        assert_eq!(revenue, 97_000); // 100_000 * 0.97
    }
}
