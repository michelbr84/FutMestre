//! Finance system - handles club finances, wages, and income.

use crate::config::GameConfig;
use crate::inbox::generators;
use crate::state::GameState;
use chrono::Datelike;
use cm_core::economy::Money;
use cm_core::ids::ClubId;
use cm_core::world::World;
use cm_finance::merchandising;
use cm_finance::report::{ExpenseBreakdown, IncomeBreakdown, MonthlyReport};
use cm_finance::tv_revenue;

/// Finance system.
pub struct FinanceSystem;

impl FinanceSystem {
    /// Run daily finance logic.
    pub fn run_daily(&self, _cfg: &GameConfig, world: &mut World, state: &mut GameState) {
        // Process weekly wages on Sundays
        if state.date.weekday() == chrono::Weekday::Sun {
            self.process_weekly_wages(world, state);
        }

        // Monthly financial report and loan payments on first of month
        if state.date.is_first_of_month() {
            self.process_loan_payments(world, state);
            self.generate_monthly_report(world, state);
        }
    }

    /// Process weekly wage payments for all clubs.
    fn process_weekly_wages(&self, world: &mut World, state: &mut GameState) {
        // Calculate wage bill for user's club
        let user_club_id = &state.club_id;

        if let Some(club) = world.clubs.get_mut(user_club_id) {
            // Get total wages from players
            let total_wages: Money = world
                .players
                .values()
                .filter(|p| p.club_id.as_ref() == Some(user_club_id))
                .map(|p| p.weekly_wage())
                .fold(Money::ZERO, |acc, w| acc + w);

            // Update wage bill and pay
            club.budget.wage_bill = total_wages;
            club.budget.pay_weekly_wages();
        }
    }

    /// Generate monthly financial report.
    fn generate_monthly_report(&self, world: &World, state: &mut GameState) {
        let user_club_id = &state.club_id;

        if let Some(club) = world.clubs.get(user_club_id) {
            let weekly_wages = club.budget.wage_bill;
            let monthly_wages = weekly_wages.multiply(4.33); // ~4.33 weeks per month

            // Calculate TV revenue (monthly = annual / 12)
            // Default to division 1 position 10 out of 20 if not tracked
            let annual_tv = tv_revenue::calculate_tv_revenue(1, 10, 20);
            let monthly_tv = annual_tv / 12;

            // Calculate merchandising revenue (monthly = annual / 12)
            let annual_merch = merchandising::calculate_merchandising(club.reputation as u32, 0, 0);
            let monthly_merch = annual_merch / 12;

            // Estimate other monthly income based on reputation
            let base_income = Money::from_major(100_000);
            let reputation_bonus = Money::from_major(club.reputation as i64 * 10_000);
            let other_monthly_income = base_income + reputation_bonus;

            let monthly_income = other_monthly_income
                + Money::from_major(monthly_tv)
                + Money::from_major(monthly_merch);

            // Build the structured monthly report
            let income_breakdown = IncomeBreakdown {
                matchday: other_monthly_income.major(),
                tv_rights: monthly_tv,
                sponsorship: 0,
                merchandising: monthly_merch,
                prize_money: 0,
                transfers: 0,
            };
            let expense_breakdown = ExpenseBreakdown {
                wages: monthly_wages.major(),
                transfers: 0,
                stadium: 0,
                other: 0,
            };
            let _report = MonthlyReport::new(
                state.date.date().month(),
                state.date.date().year() as u32,
                income_breakdown,
                expense_breakdown,
                club.budget.balance.major(),
            );

            let msg = generators::monthly_financial_report(
                state.date.date(),
                monthly_income,
                monthly_wages,
                club.budget.balance,
            );
            state.add_message(format!("{}", msg.subject));
        }
    }

    /// Process monthly loan payments for all clubs.
    fn process_loan_payments(&self, world: &mut World, state: &mut GameState) {
        let user_club_id = &state.club_id;

        if let Some(club) = world.clubs.get_mut(user_club_id) {
            if club.budget.has_active_loans() {
                let total_paid = club.budget.process_loan_payments();
                if total_paid > Money::ZERO {
                    state.add_message(format!(
                        "Pagamento mensal de emprestimo: {}. Divida restante: {}",
                        total_paid,
                        club.budget.total_loan_debt()
                    ));
                }
            }
        }
    }

    /// Take a bank loan for a club. Adds the loan amount to balance.
    pub fn take_loan(
        &self,
        world: &mut World,
        club_id: &ClubId,
        amount: Money,
        interest_rate: f64,
        term_months: u16,
        start_date: String,
    ) {
        if let Some(club) = world.clubs.get_mut(club_id) {
            let loan =
                cm_core::economy::BankLoan::new(amount, interest_rate, term_months, start_date);
            club.budget.take_loan(loan);
        }
    }

    /// Process match day income (tickets, etc.).
    pub fn process_match_income(
        &self,
        world: &mut World,
        club_id: &ClubId,
        attendance: u32,
        ticket_price: Money,
    ) {
        if let Some(club) = world.clubs.get_mut(club_id) {
            let income = ticket_price.multiply(attendance as f64);
            club.budget.balance += income;
        }
    }

    /// Process transfer income.
    pub fn process_transfer_income(&self, world: &mut World, club_id: &ClubId, fee: Money) {
        if let Some(club) = world.clubs.get_mut(club_id) {
            club.budget.receive_transfer(fee);
        }
    }

    /// Process transfer expense.
    pub fn process_transfer_expense(&self, world: &mut World, club_id: &ClubId, fee: Money) {
        if let Some(club) = world.clubs.get_mut(club_id) {
            club.budget.spend_transfer(fee);
        }
    }

    /// Add player wages to budget.
    pub fn add_player_wages(&self, world: &mut World, club_id: &ClubId, weekly_wage: Money) {
        if let Some(club) = world.clubs.get_mut(club_id) {
            club.budget.add_wage(weekly_wage);
        }
    }

    /// Remove player wages from budget.
    pub fn remove_player_wages(&self, world: &mut World, club_id: &ClubId, weekly_wage: Money) {
        if let Some(club) = world.clubs.get_mut(club_id) {
            club.budget.remove_wage(weekly_wage);
        }
    }

    /// Check if club can afford a transfer.
    pub fn can_afford_transfer(&self, world: &World, club_id: &ClubId, fee: Money) -> bool {
        world
            .clubs
            .get(club_id)
            .map(|c| c.budget.can_afford_transfer(fee))
            .unwrap_or(false)
    }

    /// Check if club can afford wages.
    pub fn can_afford_wages(&self, world: &World, club_id: &ClubId, weekly_wage: Money) -> bool {
        world
            .clubs
            .get(club_id)
            .map(|c| c.budget.can_afford_wage(weekly_wage))
            .unwrap_or(false)
    }

    /// Get club's financial status.
    pub fn get_financial_status(&self, world: &World, club_id: &ClubId) -> Option<FinancialStatus> {
        world.clubs.get(club_id).map(|club| {
            let budget = &club.budget;
            FinancialStatus {
                balance: budget.balance,
                transfer_budget: budget.transfer_budget,
                wage_budget: budget.wage_budget,
                wage_bill: budget.wage_bill,
                wage_room: budget.available_wage_room(),
                transfer_room: budget.available_for_transfers(),
                is_in_debt: budget.balance.is_negative(),
            }
        })
    }

    /// Process prize money.
    pub fn award_prize_money(&self, world: &mut World, club_id: &ClubId, amount: Money) {
        if let Some(club) = world.clubs.get_mut(club_id) {
            club.budget.balance += amount;
        }
    }
}

/// Club financial status summary.
#[derive(Debug, Clone)]
pub struct FinancialStatus {
    pub balance: Money,
    pub transfer_budget: Money,
    pub wage_budget: Money,
    pub wage_bill: Money,
    pub wage_room: Money,
    pub transfer_room: Money,
    pub is_in_debt: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use cm_core::economy::Budget;
    use cm_core::ids::NationId;
    use cm_core::world::{Club, Player, Position};

    fn setup_test() -> (World, GameState, FinanceSystem) {
        let mut world = World::new();

        // Create club with budget
        let mut club = Club::new("LIV", "Liverpool", NationId::new("ENG"));
        club.budget = Budget::new(
            Money::from_major(50_000_000),
            Money::from_major(20_000_000),
            Money::from_major(500_000),
        );
        world.clubs.insert(club.id.clone(), club);

        // Create player with wage
        let mut player = Player::new(
            "P001",
            "Test",
            "Player",
            NationId::new("ENG"),
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            Position::MidfielderCenter,
        );
        player.club_id = Some(ClubId::new("LIV"));
        world.players.insert(player.id.clone(), player);

        let state = GameState::default();
        let system = FinanceSystem;

        (world, state, system)
    }

    #[test]
    fn test_can_afford_transfer() {
        let (world, _, system) = setup_test();
        let club_id = ClubId::new("LIV");

        assert!(system.can_afford_transfer(&world, &club_id, Money::from_major(10_000_000)));
        assert!(!system.can_afford_transfer(&world, &club_id, Money::from_major(30_000_000)));
    }

    #[test]
    fn test_process_transfer_income() {
        let (mut world, _, system) = setup_test();
        let club_id = ClubId::new("LIV");

        let initial = world.clubs.get(&club_id).unwrap().budget.balance;
        system.process_transfer_income(&mut world, &club_id, Money::from_major(5_000_000));

        let after = world.clubs.get(&club_id).unwrap().budget.balance;
        assert!(after > initial);
    }

    #[test]
    fn test_process_transfer_expense() {
        let (mut world, _, system) = setup_test();
        let club_id = ClubId::new("LIV");

        let initial = world.clubs.get(&club_id).unwrap().budget.balance;
        system.process_transfer_expense(&mut world, &club_id, Money::from_major(5_000_000));

        let after = world.clubs.get(&club_id).unwrap().budget.balance;
        assert!(after < initial);
    }

    #[test]
    fn test_process_match_income() {
        let (mut world, _, system) = setup_test();
        let club_id = ClubId::new("LIV");

        let initial = world.clubs.get(&club_id).unwrap().budget.balance;
        system.process_match_income(&mut world, &club_id, 45_000, Money::from_major(50));

        let after = world.clubs.get(&club_id).unwrap().budget.balance;
        assert!(after > initial);
    }

    #[test]
    fn test_add_remove_wages() {
        let (mut world, _, system) = setup_test();
        let club_id = ClubId::new("LIV");
        let wage = Money::from_major(100_000);

        let initial = world.clubs.get(&club_id).unwrap().budget.wage_bill;
        system.add_player_wages(&mut world, &club_id, wage);

        let after = world.clubs.get(&club_id).unwrap().budget.wage_bill;
        assert!(after > initial);

        system.remove_player_wages(&mut world, &club_id, wage);
        let final_bill = world.clubs.get(&club_id).unwrap().budget.wage_bill;
        assert_eq!(initial, final_bill);
    }

    #[test]
    fn test_get_financial_status() {
        let (world, _, system) = setup_test();
        let club_id = ClubId::new("LIV");

        let status = system.get_financial_status(&world, &club_id);
        assert!(status.is_some());

        let status = status.unwrap();
        assert!(!status.is_in_debt);
        assert_eq!(status.balance.major(), 50_000_000);
    }

    #[test]
    fn test_award_prize_money() {
        let (mut world, _, system) = setup_test();
        let club_id = ClubId::new("LIV");

        let initial = world.clubs.get(&club_id).unwrap().budget.balance;
        system.award_prize_money(&mut world, &club_id, Money::from_major(1_000_000));

        let after = world.clubs.get(&club_id).unwrap().budget.balance;
        assert_eq!(after, initial + Money::from_major(1_000_000));
    }
}
