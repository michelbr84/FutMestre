//! TV revenue calculations.

use cm_core::economy::Money;

/// Calculates TV revenue based on division level and league position.
///
/// Base revenue by division:
/// - Serie A (level 1): 5M
/// - Serie B (level 2): 2M
/// - Serie C (level 3): 500K
/// - Serie D (level 4+): 100K
///
/// Position bonus:
/// - Top 4: +50%
/// - Mid-table: +0%
/// - Bottom 3: -20%
pub fn calculate_tv_revenue(division_level: u32, position: u32, total_teams: u32) -> i64 {
    let base = match division_level {
        1 => 5_000_000i64,
        2 => 2_000_000,
        3 => 500_000,
        _ => 100_000,
    };

    let multiplier = if position <= 4 {
        1.50
    } else if total_teams > 3 && position > total_teams - 3 {
        0.80
    } else {
        1.00
    };

    (base as f64 * multiplier) as i64
}

/// Calculate TV revenue as Money type.
pub fn tv_revenue_as_money(division_level: u32, position: u32, total_teams: u32) -> Money {
    Money::from_major(calculate_tv_revenue(division_level, position, total_teams))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serie_a_top4() {
        let revenue = calculate_tv_revenue(1, 1, 20);
        assert_eq!(revenue, 7_500_000); // 5M * 1.5
    }

    #[test]
    fn test_serie_a_mid_table() {
        let revenue = calculate_tv_revenue(1, 10, 20);
        assert_eq!(revenue, 5_000_000); // 5M * 1.0
    }

    #[test]
    fn test_serie_a_bottom3() {
        let revenue = calculate_tv_revenue(1, 18, 20);
        assert_eq!(revenue, 4_000_000); // 5M * 0.8
    }

    #[test]
    fn test_serie_a_last_place() {
        let revenue = calculate_tv_revenue(1, 20, 20);
        assert_eq!(revenue, 4_000_000); // 5M * 0.8
    }

    #[test]
    fn test_serie_b_top4() {
        let revenue = calculate_tv_revenue(2, 3, 20);
        assert_eq!(revenue, 3_000_000); // 2M * 1.5
    }

    #[test]
    fn test_serie_b_mid_table() {
        let revenue = calculate_tv_revenue(2, 10, 20);
        assert_eq!(revenue, 2_000_000); // 2M * 1.0
    }

    #[test]
    fn test_serie_c() {
        let revenue = calculate_tv_revenue(3, 1, 20);
        assert_eq!(revenue, 750_000); // 500K * 1.5
    }

    #[test]
    fn test_serie_d() {
        let revenue = calculate_tv_revenue(4, 1, 20);
        assert_eq!(revenue, 150_000); // 100K * 1.5
    }

    #[test]
    fn test_unknown_division() {
        let revenue = calculate_tv_revenue(5, 10, 20);
        assert_eq!(revenue, 100_000); // defaults to 100K, mid-table
    }

    #[test]
    fn test_position_4_is_top4() {
        let revenue = calculate_tv_revenue(1, 4, 20);
        assert_eq!(revenue, 7_500_000); // 5M * 1.5
    }

    #[test]
    fn test_position_5_is_mid_table() {
        let revenue = calculate_tv_revenue(1, 5, 20);
        assert_eq!(revenue, 5_000_000); // 5M * 1.0
    }

    #[test]
    fn test_bottom3_boundary() {
        // With 20 teams, bottom 3 = positions 18, 19, 20
        let mid = calculate_tv_revenue(1, 17, 20);
        let bottom = calculate_tv_revenue(1, 18, 20);
        assert_eq!(mid, 5_000_000);
        assert_eq!(bottom, 4_000_000);
    }

    #[test]
    fn test_small_league() {
        // With 4 teams, bottom 3 = positions 2, 3, 4
        // But position 4 is both top4 and bottom3 -- top4 wins
        let revenue = calculate_tv_revenue(1, 4, 4);
        assert_eq!(revenue, 7_500_000); // top 4 bonus applies first
    }

    #[test]
    fn test_tv_revenue_as_money() {
        let money = tv_revenue_as_money(1, 1, 20);
        assert_eq!(money.major(), 7_500_000);
    }
}
