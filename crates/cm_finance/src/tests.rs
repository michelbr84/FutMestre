//! Finance tests.

#[cfg(test)]
mod ticketing_tests {
    use cm_core::economy::Money;
    use crate::ticketing::*;

    #[test]
    fn test_matchday_revenue() {
        let revenue = calculate_matchday_revenue(40_000, Money::from_major(30));
        assert_eq!(revenue.major(), 1_200_000);
    }

    #[test]
    fn test_zero_attendance() {
        let revenue = calculate_matchday_revenue(0, Money::from_major(30));
        assert_eq!(revenue.major(), 0);
    }

    #[test]
    fn test_small_stadium() {
        let revenue = calculate_matchday_revenue(5_000, Money::from_major(25));
        assert_eq!(revenue.major(), 125_000);
    }
}

#[cfg(test)]
mod sponsorship_tests {
    use cm_core::economy::Money;
    use crate::sponsorship::*;

    #[test]
    fn test_sponsorship_high_reputation() {
        let sponsorship = calculate_sponsorship(90);
        assert!(sponsorship.major() > 5_000_000);
    }

    #[test]
    fn test_sponsorship_low_reputation() {
        let sponsorship = calculate_sponsorship(30);
        assert!(sponsorship.major() > 0);
        assert!(sponsorship.major() < 5_000_000);
    }

    #[test]
    fn test_sponsorship_scales_with_reputation() {
        let low_rep = calculate_sponsorship(30);
        let high_rep = calculate_sponsorship(90);
        assert!(high_rep.major() > low_rep.major());
    }
}

#[cfg(test)]
mod prize_tests {
    use cm_core::economy::Money;
    use crate::prizes::*;

    #[test]
    fn test_league_prize_first_place() {
        let prize = league_prize(1, 1);
        assert!(prize.major() > 10_000_000);
    }

    #[test]
    fn test_league_prize_relegation_zone() {
        let prize = league_prize(20, 1);
        assert!(prize.major() > 0);
    }

    #[test]
    fn test_league_prize_lower_tier() {
        let tier1 = league_prize(1, 1);
        let tier2 = league_prize(1, 2);
        assert!(tier1.major() > tier2.major());
    }

    #[test]
    fn test_higher_position_more_money() {
        let first = league_prize(1, 1);
        let fifth = league_prize(5, 1);
        let tenth = league_prize(10, 1);
        
        assert!(first.major() > fifth.major());
        assert!(fifth.major() > tenth.major());
    }
}

#[cfg(test)]
mod wage_tests {
    use cm_core::economy::Money;
    use crate::wage::*;

    #[test]
    fn test_total_wages() {
        let wages = vec![
            Money::from_major(100_000),
            Money::from_major(80_000),
            Money::from_major(50_000),
        ];
        
        let total = calculate_weekly_wages(&wages);
        assert_eq!(total.major(), 230_000);
    }

    #[test]
    fn test_empty_wages() {
        let wages: Vec<Money> = vec![];
        let total = calculate_weekly_wages(&wages);
        assert_eq!(total.major(), 0);
    }

    #[test]
    fn test_single_wage() {
        let wages = vec![Money::from_major(75_000)];
        let total = calculate_weekly_wages(&wages);
        assert_eq!(total.major(), 75_000);
    }
}

#[cfg(test)]
mod debt_tests {
    use cm_core::economy::Money;
    use crate::debt::*;

    #[test]
    fn test_interest_calculation() {
        let debt = Money::from_major(10_000_000);
        let interest = calculate_interest(debt, 0.05);
        assert_eq!(interest.major(), 500_000);
    }

    #[test]
    fn test_zero_debt() {
        let debt = Money::ZERO;
        let interest = calculate_interest(debt, 0.05);
        assert_eq!(interest.major(), 0);
    }

    #[test]
    fn test_zero_rate() {
        let debt = Money::from_major(10_000_000);
        let interest = calculate_interest(debt, 0.0);
        assert_eq!(interest.major(), 0);
    }

    #[test]
    fn test_high_rate() {
        let debt = Money::from_major(1_000_000);
        let interest = calculate_interest(debt, 0.15);
        assert_eq!(interest.major(), 150_000);
    }
}

#[cfg(test)]
mod ffp_tests {
    use cm_core::economy::Money;
    use crate::ffp::*;

    #[test]
    fn test_ffp_compliant() {
        let income = Money::from_major(100_000_000);
        let expenses = Money::from_major(90_000_000);
        assert!(check_ffp_compliance(income, expenses));
    }

    #[test]
    fn test_ffp_exact_threshold() {
        let income = Money::from_major(100_000_000);
        let expenses = Money::from_major(111_000_000); // 10% over
        assert!(check_ffp_compliance(income, expenses));
    }

    #[test]
    fn test_ffp_non_compliant() {
        let income = Money::from_major(50_000_000);
        let expenses = Money::from_major(100_000_000);
        assert!(!check_ffp_compliance(income, expenses));
    }

    #[test]
    fn test_ffp_profit() {
        let income = Money::from_major(150_000_000);
        let expenses = Money::from_major(100_000_000);
        assert!(check_ffp_compliance(income, expenses));
    }
}

#[cfg(test)]
mod model_tests {
    use cm_core::economy::Money;
    use crate::model::*;

    #[test]
    fn test_income_total() {
        let income = Income {
            matchday: Money::from_major(1_000_000),
            tv_rights: Money::from_major(5_000_000),
            sponsorship: Money::from_major(2_000_000),
            merchandise: Money::from_major(500_000),
            prize_money: Money::from_major(3_000_000),
            transfers: Money::from_major(10_000_000),
        };
        
        assert_eq!(income.total().major(), 21_500_000);
    }

    #[test]
    fn test_expenses_total() {
        let expenses = Expenses {
            wages: Money::from_major(5_000_000),
            transfers: Money::from_major(8_000_000),
            stadium: Money::from_major(500_000),
            other: Money::from_major(200_000),
        };
        
        assert_eq!(expenses.total().major(), 13_700_000);
    }

    #[test]
    fn test_financial_statement_net_profit() {
        let statement = FinancialStatement {
            income: Income {
                matchday: Money::from_major(1_000_000),
                tv_rights: Money::from_major(10_000_000),
                sponsorship: Money::from_major(2_000_000),
                merchandise: Money::from_major(500_000),
                prize_money: Money::from_major(1_000_000),
                transfers: Money::from_major(0),
            },
            expenses: Expenses {
                wages: Money::from_major(8_000_000),
                transfers: Money::from_major(0),
                stadium: Money::from_major(500_000),
                other: Money::from_major(500_000),
            },
        };
        
        let net = statement.net();
        assert!(net.major() > 0); // Profitable
    }

    #[test]
    fn test_financial_statement_net_loss() {
        let statement = FinancialStatement {
            income: Income {
                matchday: Money::from_major(500_000),
                tv_rights: Money::from_major(2_000_000),
                sponsorship: Money::from_major(500_000),
                merchandise: Money::from_major(100_000),
                prize_money: Money::from_major(0),
                transfers: Money::from_major(0),
            },
            expenses: Expenses {
                wages: Money::from_major(5_000_000),
                transfers: Money::from_major(2_000_000),
                stadium: Money::from_major(300_000),
                other: Money::from_major(200_000),
            },
        };
        
        let net = statement.net();
        assert!(net.is_negative()); // Loss
    }
}

#[cfg(test)]
mod tv_revenue_tests {
    use crate::tv_revenue::*;

    #[test]
    fn test_serie_a_top4() {
        let revenue = calculate_tv_revenue(1, 1, 20);
        assert_eq!(revenue, 7_500_000);
    }

    #[test]
    fn test_serie_a_mid_table() {
        let revenue = calculate_tv_revenue(1, 10, 20);
        assert_eq!(revenue, 5_000_000);
    }

    #[test]
    fn test_serie_a_bottom3() {
        let revenue = calculate_tv_revenue(1, 20, 20);
        assert_eq!(revenue, 4_000_000);
    }

    #[test]
    fn test_serie_b_base() {
        let revenue = calculate_tv_revenue(2, 10, 20);
        assert_eq!(revenue, 2_000_000);
    }

    #[test]
    fn test_serie_c_base() {
        let revenue = calculate_tv_revenue(3, 10, 20);
        assert_eq!(revenue, 500_000);
    }

    #[test]
    fn test_serie_d_base() {
        let revenue = calculate_tv_revenue(4, 10, 20);
        assert_eq!(revenue, 100_000);
    }

    #[test]
    fn test_higher_division_more_revenue() {
        let a = calculate_tv_revenue(1, 10, 20);
        let b = calculate_tv_revenue(2, 10, 20);
        let c = calculate_tv_revenue(3, 10, 20);
        assert!(a > b);
        assert!(b > c);
    }

    #[test]
    fn test_tv_revenue_as_money() {
        let money = tv_revenue_as_money(1, 1, 20);
        assert_eq!(money.major(), 7_500_000);
    }
}

#[cfg(test)]
mod merchandising_tests {
    use crate::merchandising::*;

    #[test]
    fn test_base_merchandising() {
        let revenue = calculate_merchandising(100, 0, 0);
        assert_eq!(revenue, 100_000);
    }

    #[test]
    fn test_wins_boost() {
        let base = calculate_merchandising(100, 0, 0);
        let boosted = calculate_merchandising(100, 10, 0);
        assert!(boosted > base);
    }

    #[test]
    fn test_losses_reduce() {
        let base = calculate_merchandising(100, 0, 0);
        let reduced = calculate_merchandising(100, 0, 10);
        assert!(reduced < base);
    }

    #[test]
    fn test_floor_at_50_percent() {
        let revenue = calculate_merchandising(100, 0, 100);
        assert_eq!(revenue, 50_000); // 50% floor
    }

    #[test]
    fn test_cap_at_200_percent() {
        let revenue = calculate_merchandising(100, 100, 0);
        assert_eq!(revenue, 200_000); // 200% cap
    }

    #[test]
    fn test_merchandising_as_money() {
        let money = merchandising_as_money(100, 0, 0);
        assert_eq!(money.major(), 100_000);
    }

    #[test]
    fn test_zero_reputation() {
        let revenue = calculate_merchandising(0, 10, 0);
        assert_eq!(revenue, 0);
    }
}

#[cfg(test)]
mod report_tests {
    use crate::report::*;

    #[test]
    fn test_report_profitable() {
        let income = IncomeBreakdown {
            matchday: 2_000_000,
            tv_rights: 5_000_000,
            sponsorship: 1_000_000,
            merchandising: 500_000,
            prize_money: 0,
            transfers: 0,
        };
        let expenses = ExpenseBreakdown {
            wages: 3_000_000,
            transfers: 0,
            stadium: 200_000,
            other: 100_000,
        };
        let report = MonthlyReport::new(1, 2026, income, expenses, 50_000_000);
        assert!(report.is_profitable());
        assert_eq!(report.net_profit, 5_200_000);
        assert_eq!(report.balance_end, 55_200_000);
    }

    #[test]
    fn test_report_loss() {
        let income = IncomeBreakdown {
            matchday: 500_000,
            ..Default::default()
        };
        let expenses = ExpenseBreakdown {
            wages: 2_000_000,
            ..Default::default()
        };
        let report = MonthlyReport::new(6, 2026, income, expenses, 10_000_000);
        assert!(!report.is_profitable());
        assert_eq!(report.net_profit, -1_500_000);
        assert_eq!(report.balance_end, 8_500_000);
    }

    #[test]
    fn test_breakdown_totals() {
        let income = IncomeBreakdown {
            matchday: 1,
            tv_rights: 2,
            sponsorship: 3,
            merchandising: 4,
            prize_money: 5,
            transfers: 6,
        };
        assert_eq!(income.total(), 21);

        let expenses = ExpenseBreakdown {
            wages: 10,
            transfers: 20,
            stadium: 30,
            other: 40,
        };
        assert_eq!(expenses.total(), 100);
    }
}

#[cfg(test)]
mod budget_transfer_tests {
    use cm_core::economy::{Budget, Money};

    #[test]
    fn test_process_transfer_expense() {
        let mut budget = Budget::new(
            Money::from_major(50_000_000),
            Money::from_major(20_000_000),
            Money::from_major(500_000),
        );
        budget.process_transfer_expense(Money::from_major(5_000_000));
        assert_eq!(budget.transfer_budget.major(), 15_000_000);
        assert_eq!(budget.balance.major(), 45_000_000);
    }

    #[test]
    fn test_process_transfer_income() {
        let mut budget = Budget::new(
            Money::from_major(50_000_000),
            Money::from_major(20_000_000),
            Money::from_major(500_000),
        );
        budget.process_transfer_income(Money::from_major(10_000_000));
        assert_eq!(budget.transfer_budget.major(), 30_000_000);
        assert_eq!(budget.balance.major(), 60_000_000);
    }
}

#[cfg(test)]
mod integration_tests {
    use cm_core::economy::Money;
    use crate::*;

    #[test]
    fn test_season_financial_flow() {
        // Simulate a season's finances for a mid-table club
        
        // Income
        let matchday_per_game = ticketing::calculate_matchday_revenue(30_000, Money::from_major(25));
        let home_games = 19; // Half the season
        let total_matchday = matchday_per_game.multiply(home_games as f64);
        
        let sponsorship = sponsorship::calculate_sponsorship(60);
        let prize = prizes::league_prize(10, 1);
        
        // Expenses
        let weekly_wages = vec![
            Money::from_major(50_000),
            Money::from_major(40_000),
            Money::from_major(35_000),
        ];
        let total_weekly = wage::calculate_weekly_wages(&weekly_wages);
        let annual_wages = total_weekly.multiply(52.0);
        
        // Check FFP
        let total_income = total_matchday + sponsorship + prize;
        let total_expenses = annual_wages;
        
        // Mid-table club should be able to stay FFP compliant
        assert!(ffp::check_ffp_compliance(total_income, total_expenses) || 
                total_income.major() > total_expenses.major() / 2);
    }

    #[test]
    fn test_debt_service() {
        // Club with debt
        let debt = Money::from_major(50_000_000);
        let annual_interest = debt::calculate_interest(debt, 0.05);
        
        // They need enough income to cover interest
        let income = Money::from_major(100_000_000);
        assert!(income.major() > annual_interest.major());
    }
}
