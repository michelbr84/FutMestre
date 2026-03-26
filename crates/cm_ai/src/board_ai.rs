//! Board AI - Simulates board expectations and satisfaction.

use cm_core::economy::Money;

/// Board satisfaction factors and weights.
#[derive(Debug, Clone, Copy)]
pub struct BoardExpectations {
    pub expected_position: u8,
    pub financial_target: i64, // Target balance in minor units
    pub cup_expectation: CupExpectation,
}

impl Default for BoardExpectations {
    fn default() -> Self {
        Self {
            expected_position: 10,
            financial_target: 0,
            cup_expectation: CupExpectation::Progress,
        }
    }
}

/// Cup competition expectations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CupExpectation {
    NoExpectation,
    Progress, // Get past early rounds
    Quarterfinals,
    Semifinals,
    Final,
    Win,
}

/// Calculate board satisfaction score (0-100).
///
/// Factors in league position, financial health, and cup progress.
pub fn board_satisfaction(league_position: u8, expected_position: u8, financial_health: i64) -> u8 {
    let mut satisfaction: i32 = 50; // Start neutral

    // League position factor (most important)
    let position_diff = expected_position as i32 - league_position as i32;

    if position_diff >= 5 {
        // Way above expectations
        satisfaction += 35;
    } else if position_diff >= 2 {
        // Above expectations
        satisfaction += 20;
    } else if position_diff >= 0 {
        // Meeting expectations
        satisfaction += 10;
    } else if position_diff >= -2 {
        // Slightly below
        satisfaction -= 5;
    } else if position_diff >= -5 {
        // Below expectations
        satisfaction -= 20;
    } else {
        // Way below expectations
        satisfaction -= 40;
    }

    // Financial health factor
    if financial_health > 10_000_000 {
        satisfaction += 15;
    } else if financial_health > 0 {
        satisfaction += 5;
    } else if financial_health > -5_000_000 {
        satisfaction -= 5;
    } else if financial_health > -20_000_000 {
        satisfaction -= 15;
    } else {
        satisfaction -= 30; // Severe debt
    }

    satisfaction.clamp(0, 100) as u8
}

/// Calculate detailed board satisfaction with all factors.
pub fn calculate_board_satisfaction(
    expectations: &BoardExpectations,
    league_position: u8,
    financial_balance: Money,
    cup_round_reached: u8, // 0 = didn't enter, 1 = first round, etc.
) -> BoardSatisfaction {
    let mut total_score: i32 = 0;
    let mut max_score: i32 = 0;

    // League position (40% weight)
    let league_score = calculate_league_score(league_position, expectations.expected_position);
    total_score += (league_score as i32) * 40;
    max_score += 100 * 40;

    // Financial health (35% weight)
    let financial_score =
        calculate_financial_score(financial_balance, expectations.financial_target);
    total_score += (financial_score as i32) * 35;
    max_score += 100 * 35;

    // Cup progress (25% weight)
    let cup_score = calculate_cup_score(cup_round_reached, expectations.cup_expectation);
    total_score += (cup_score as i32) * 25;
    max_score += 100 * 25;

    let overall = ((total_score * 100) / max_score) as u8;

    BoardSatisfaction {
        overall,
        league_score,
        financial_score,
        cup_score,
        risk_level: assess_job_risk(overall),
    }
}

fn calculate_league_score(position: u8, expected: u8) -> u8 {
    let diff = expected as i32 - position as i32;

    let score = 50 + (diff * 10);
    score.clamp(0, 100) as u8
}

fn calculate_financial_score(balance: Money, target: i64) -> u8 {
    // Convert to major units for comparison (balance is in minor units = cents)
    let balance_major = balance.major();
    let target_major = target / 100; // target is also in minor units
    let diff = balance_major - target_major;

    // Thresholds in major units (millions)
    const THRESHOLD: i64 = 10_000_000;

    if diff >= THRESHOLD {
        100 // Well above target
    } else if diff >= 0 {
        // Linear scale from 70-100
        70 + ((diff * 30) / THRESHOLD).min(30) as u8
    } else if diff >= -THRESHOLD {
        // Linear scale from 40-70
        (70 + (diff * 30 / THRESHOLD)) as u8
    } else {
        // Severe deficit
        let score: f64 = 40.0 + (diff as f64 / THRESHOLD as f64 * 10.0);
        score.max(0.0) as u8
    }
}

fn calculate_cup_score(round_reached: u8, expectation: CupExpectation) -> u8 {
    let expected_round = match expectation {
        CupExpectation::NoExpectation => 0,
        CupExpectation::Progress => 2,
        CupExpectation::Quarterfinals => 4,
        CupExpectation::Semifinals => 5,
        CupExpectation::Final => 6,
        CupExpectation::Win => 7,
    };

    if round_reached >= expected_round {
        100.min(50 + (round_reached as u16 * 10)) as u8
    } else {
        let shortfall = expected_round - round_reached;
        (50_i16 - (shortfall as i16 * 15)).max(0) as u8
    }
}

fn assess_job_risk(satisfaction: u8) -> JobRisk {
    if satisfaction >= 70 {
        JobRisk::Safe
    } else if satisfaction >= 50 {
        JobRisk::Secure
    } else if satisfaction >= 35 {
        JobRisk::Warning
    } else if satisfaction >= 20 {
        JobRisk::AtRisk
    } else {
        JobRisk::Imminent
    }
}

/// Detailed board satisfaction breakdown.
#[derive(Debug, Clone)]
pub struct BoardSatisfaction {
    pub overall: u8,
    pub league_score: u8,
    pub financial_score: u8,
    pub cup_score: u8,
    pub risk_level: JobRisk,
}

/// Job security risk levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobRisk {
    Safe,     // 70+
    Secure,   // 50-69
    Warning,  // 35-49
    AtRisk,   // 20-34
    Imminent, // <20
}

/// Board decisions the AI can make.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardDecision {
    IncreaseBudget { amount: Money, reason: String },
    DecreaseBudget { amount: Money, reason: String },
    ExtendContract,
    Warning,
    Termination,
    NoAction,
}

/// Evaluate and decide board actions based on performance.
pub fn evaluate_board_action(
    satisfaction: &BoardSatisfaction,
    months_since_last_action: u8,
) -> BoardDecision {
    // Don't take frequent actions
    if months_since_last_action < 2 {
        return BoardDecision::NoAction;
    }

    match satisfaction.risk_level {
        JobRisk::Imminent => BoardDecision::Termination,
        JobRisk::AtRisk => BoardDecision::Warning,
        JobRisk::Warning => {
            if satisfaction.financial_score < 40 {
                BoardDecision::DecreaseBudget {
                    amount: Money::from_major(500_000),
                    reason: "Financial concerns".into(),
                }
            } else {
                BoardDecision::NoAction
            }
        }
        JobRisk::Secure => BoardDecision::NoAction,
        JobRisk::Safe => {
            if satisfaction.overall >= 85 && satisfaction.financial_score >= 70 {
                BoardDecision::IncreaseBudget {
                    amount: Money::from_major(1_000_000),
                    reason: "Excellent performance rewarded".into(),
                }
            } else if satisfaction.overall >= 90 {
                BoardDecision::ExtendContract
            } else {
                BoardDecision::NoAction
            }
        }
    }
}

/// Generate board expectations based on club reputation.
pub fn generate_expectations(reputation: u8, current_balance: Money) -> BoardExpectations {
    let expected_position = match reputation {
        90..=100 => 1,
        80..=89 => 3,
        70..=79 => 6,
        60..=69 => 10,
        50..=59 => 14,
        40..=49 => 17,
        _ => 20,
    };

    let cup_expectation = match reputation {
        80..=100 => CupExpectation::Win,
        70..=79 => CupExpectation::Semifinals,
        60..=69 => CupExpectation::Quarterfinals,
        50..=59 => CupExpectation::Progress,
        _ => CupExpectation::NoExpectation,
    };

    BoardExpectations {
        expected_position,
        financial_target: current_balance.minor(),
        cup_expectation,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_satisfaction_exceeding_expectations() {
        // Position 3 when expected 10
        let satisfaction = board_satisfaction(3, 10, 5_000_000);
        assert!(satisfaction >= 70, "Should be highly satisfied");
    }

    #[test]
    fn test_board_satisfaction_meeting_expectations() {
        let satisfaction = board_satisfaction(10, 10, 0);
        assert!(
            satisfaction >= 50 && satisfaction <= 70,
            "Should be moderately satisfied"
        );
    }

    #[test]
    fn test_board_satisfaction_below_expectations() {
        // Position 15 when expected 5
        let satisfaction = board_satisfaction(15, 5, 0);
        assert!(satisfaction < 40, "Should be unsatisfied");
    }

    #[test]
    fn test_board_satisfaction_with_debt() {
        let healthy = board_satisfaction(10, 10, 5_000_000);
        let in_debt = board_satisfaction(10, 10, -15_000_000);

        assert!(healthy > in_debt, "Debt should reduce satisfaction");
    }

    #[test]
    fn test_calculate_board_satisfaction() {
        let expectations = BoardExpectations {
            expected_position: 8,
            financial_target: 0,
            cup_expectation: CupExpectation::Quarterfinals,
        };

        let satisfaction = calculate_board_satisfaction(
            &expectations,
            5, // Better than expected
            Money::from_major(5_000_000),
            5, // Reached semifinals
        );

        assert!(satisfaction.overall >= 70);
        assert!(satisfaction.league_score >= 70);
        assert!(satisfaction.cup_score >= 70);
    }

    #[test]
    fn test_job_risk_levels() {
        assert_eq!(assess_job_risk(90), JobRisk::Safe);
        assert_eq!(assess_job_risk(60), JobRisk::Secure);
        assert_eq!(assess_job_risk(40), JobRisk::Warning);
        assert_eq!(assess_job_risk(25), JobRisk::AtRisk);
        assert_eq!(assess_job_risk(10), JobRisk::Imminent);
    }

    #[test]
    fn test_evaluate_board_action_termination() {
        let satisfaction = BoardSatisfaction {
            overall: 15,
            league_score: 10,
            financial_score: 20,
            cup_score: 20,
            risk_level: JobRisk::Imminent,
        };

        let action = evaluate_board_action(&satisfaction, 3);
        assert_eq!(action, BoardDecision::Termination);
    }

    #[test]
    fn test_evaluate_board_action_budget_increase() {
        let satisfaction = BoardSatisfaction {
            overall: 90,
            league_score: 95,
            financial_score: 85,
            cup_score: 80,
            risk_level: JobRisk::Safe,
        };

        let action = evaluate_board_action(&satisfaction, 3);
        assert!(matches!(action, BoardDecision::IncreaseBudget { .. }));
    }

    #[test]
    fn test_evaluate_board_action_no_frequent_actions() {
        let satisfaction = BoardSatisfaction {
            overall: 10,
            league_score: 10,
            financial_score: 10,
            cup_score: 10,
            risk_level: JobRisk::Imminent,
        };

        // Only 1 month since last action
        let action = evaluate_board_action(&satisfaction, 1);
        assert_eq!(action, BoardDecision::NoAction);
    }

    #[test]
    fn test_generate_expectations() {
        let high_rep = generate_expectations(85, Money::from_major(10_000_000));
        assert!(high_rep.expected_position <= 5);
        assert_eq!(high_rep.cup_expectation, CupExpectation::Win);

        let low_rep = generate_expectations(35, Money::from_major(500_000));
        assert!(low_rep.expected_position >= 15);
        assert_eq!(low_rep.cup_expectation, CupExpectation::NoExpectation);
    }

    #[test]
    fn test_league_score_calculation() {
        let exceeding = calculate_league_score(3, 10);
        let meeting = calculate_league_score(10, 10);
        let below = calculate_league_score(15, 10);

        assert!(exceeding > meeting);
        assert!(meeting > below);
    }

    #[test]
    fn test_financial_score_calculation() {
        let wealthy = calculate_financial_score(Money::from_major(20_000_000), 0);
        let stable = calculate_financial_score(Money::from_major(1_000_000), 0);
        let in_debt = calculate_financial_score(Money::from_major(-5_000_000), 0);

        assert!(wealthy > stable);
        assert!(stable > in_debt);
    }

    #[test]
    fn test_cup_score_calculation() {
        // Exceeded expectations
        let exceeded = calculate_cup_score(6, CupExpectation::Quarterfinals);
        assert!(exceeded >= 80);

        // Met expectations
        let met = calculate_cup_score(4, CupExpectation::Quarterfinals);
        assert!(met >= 50);

        // Failed expectations
        let failed = calculate_cup_score(1, CupExpectation::Semifinals);
        assert!(failed < 40);
    }
}
