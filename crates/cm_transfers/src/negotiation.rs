//! Transfer negotiation system.

use cm_core::economy::Money;
use crate::model::{Transfer, TransferStatus};
use chrono::NaiveDate;

/// Negotiation response from the selling club.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NegotiationResponse {
    Accept,
    Counter(Money),
    Reject,
    WaitingForPlayer,
}

/// Player response to contract offer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerResponse {
    Accept,
    WantMoreWages(Money),
    WantLongerContract,
    NotInterested,
    ClubNotGoodEnough,
}

/// Contract negotiation parameters.
#[derive(Debug, Clone)]
pub struct ContractOffer {
    pub weekly_wage: Money,
    pub contract_years: u8,
    pub signing_bonus: Money,
    pub release_clause: Option<Money>,
    pub performance_bonuses: Money,
}

impl ContractOffer {
    /// Create a basic contract offer.
    pub fn new(weekly_wage: Money, years: u8) -> Self {
        Self {
            weekly_wage,
            contract_years: years,
            signing_bonus: Money::ZERO,
            release_clause: None,
            performance_bonuses: Money::ZERO,
        }
    }

    /// Total value of the contract.
    pub fn total_value(&self) -> Money {
        let weeks = (self.contract_years as i64) * 52;
        let base = self.weekly_wage.multiply(weeks as f64);
        base + self.signing_bonus + self.performance_bonuses
    }
}

/// Transfer bid parameters.
#[derive(Debug, Clone)]
pub struct TransferBid {
    pub fee: Money,
    pub add_ons: Money,
    pub payment_schedule: PaymentSchedule,
    pub exchange_player: Option<String>,
}

/// Payment schedule for transfer fee.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentSchedule {
    Upfront,
    Installments(u8), // Number of yearly installments
    PerformanceBased, // Pay based on appearances
}

impl TransferBid {
    /// Create a simple bid.
    pub fn new(fee: Money) -> Self {
        Self {
            fee,
            add_ons: Money::ZERO,
            payment_schedule: PaymentSchedule::Upfront,
            exchange_player: None,
        }
    }

    /// Total potential value.
    pub fn total_value(&self) -> Money {
        self.fee + self.add_ons
    }
}

/// Negotiation context.
#[derive(Debug, Clone)]
pub struct NegotiationContext {
    pub player_value: Money,
    pub asking_price: Money,
    pub selling_club_reputation: u8,
    pub buying_club_reputation: u8,
    pub player_wants_to_leave: bool,
    pub contract_remaining_years: f32,
    pub selling_desperation: u8, // 0-100
}

/// Evaluate a transfer bid.
pub fn evaluate_bid(bid: &TransferBid, context: &NegotiationContext) -> NegotiationResponse {
    let ratio = bid.total_value().major() as f64 / context.asking_price.major().max(1) as f64;
    
    // Adjust threshold based on context
    let mut threshold = 0.90;
    
    // Player wants to leave - more likely to accept lower bid
    if context.player_wants_to_leave {
        threshold -= 0.15;
    }
    
    // Short contract - more pressure to sell
    if context.contract_remaining_years < 1.0 {
        threshold -= 0.20;
    } else if context.contract_remaining_years < 2.0 {
        threshold -= 0.10;
    }
    
    // Reputation difference - big clubs can negotiate harder
    let rep_diff = context.buying_club_reputation as i16 - context.selling_club_reputation as i16;
    if rep_diff > 20 {
        threshold -= 0.05;
    } else if rep_diff < -20 {
        threshold += 0.05;
    }
    
    // Desperation
    threshold -= context.selling_desperation as f64 / 500.0;
    
    // Payment schedule affects acceptance
    let schedule_modifier = match bid.payment_schedule {
        PaymentSchedule::Upfront => 0.0,
        PaymentSchedule::Installments(years) => -(years as f64 * 0.02),
        PaymentSchedule::PerformanceBased => -0.05,
    };
    
    let adjusted_ratio = ratio + schedule_modifier;
    
    if adjusted_ratio >= 1.0 {
        NegotiationResponse::Accept
    } else if adjusted_ratio >= threshold {
        // Counter with a price between bid and asking
        let counter_factor = threshold + (1.0 - threshold) * 0.5;
        NegotiationResponse::Counter(context.asking_price.multiply(counter_factor))
    } else if adjusted_ratio >= threshold - 0.15 {
        // Counter with asking price
        NegotiationResponse::Counter(context.asking_price)
    } else {
        NegotiationResponse::Reject
    }
}

/// Evaluate a contract offer from player's perspective.
pub fn evaluate_contract(
    offer: &ContractOffer,
    current_wage: Money,
    player_age: u8,
    club_reputation: u8,
    player_ambition: u8,
) -> PlayerResponse {
    let wage_ratio = offer.weekly_wage.major() as f64 / current_wage.major().max(1) as f64;
    
    // Younger players prefer longer contracts
    let min_years = match player_age {
        16..=22 => 3,
        23..=28 => 2,
        29..=32 => 1,
        _ => 1,
    };
    
    if offer.contract_years < min_years {
        return PlayerResponse::WantLongerContract;
    }
    
    // Ambitious players want better clubs
    if player_ambition > 70 && club_reputation < 60 {
        return PlayerResponse::ClubNotGoodEnough;
    }
    
    // Wage expectations
    let min_wage_ratio = 1.0 + (player_ambition as f64 / 200.0); // 1.0-1.5
    
    if wage_ratio < min_wage_ratio {
        let desired = current_wage.multiply(min_wage_ratio + 0.1);
        return PlayerResponse::WantMoreWages(desired);
    }
    
    // Consider total package
    let total_value = offer.total_value();
    let expected_value = current_wage.multiply((offer.contract_years as f64) * 52.0 * min_wage_ratio);
    
    if total_value.major() >= expected_value.major() {
        PlayerResponse::Accept
    } else {
        let desired = current_wage.multiply(min_wage_ratio + 0.15);
        PlayerResponse::WantMoreWages(desired)
    }
}

/// Calculate a fair asking price for a player.
pub fn calculate_asking_price(
    player_value: Money,
    contract_years: f32,
    importance: u8, // How important the player is to the club (0-100)
    want_to_sell: bool,
) -> Money {
    let mut multiplier = 1.0;
    
    // Contract length affects price
    multiplier *= match contract_years {
        y if y > 4.0 => 1.3,
        y if y > 3.0 => 1.2,
        y if y > 2.0 => 1.1,
        y if y > 1.0 => 1.0,
        y if y > 0.5 => 0.8,
        _ => 0.5, // Expiring contract
    };
    
    // Important players cost more
    multiplier *= 1.0 + (importance as f64 / 200.0);
    
    // Willing to sell = lower price
    if want_to_sell {
        multiplier *= 0.85;
    }
    
    player_value.multiply(multiplier)
}

/// Process a negotiation round.
pub fn process_negotiation(
    transfer: &mut Transfer,
    response: NegotiationResponse,
    counter_bid: Option<TransferBid>,
) {
    match response {
        NegotiationResponse::Accept => {
            transfer.status = TransferStatus::Accepted;
        }
        NegotiationResponse::Counter(_) => {
            if let Some(bid) = counter_bid {
                transfer.fee = bid.fee;
            }
            transfer.status = TransferStatus::Negotiating;
        }
        NegotiationResponse::Reject => {
            transfer.status = TransferStatus::Rejected;
        }
        NegotiationResponse::WaitingForPlayer => {
            transfer.status = TransferStatus::Negotiating;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cm_core::ids::{ClubId, PlayerId};

    fn test_context() -> NegotiationContext {
        NegotiationContext {
            player_value: Money::from_major(10_000_000),
            asking_price: Money::from_major(12_000_000),
            selling_club_reputation: 70,
            buying_club_reputation: 75,
            player_wants_to_leave: false,
            contract_remaining_years: 3.0,
            selling_desperation: 0,
        }
    }

    #[test]
    fn test_bid_accept_at_asking_price() {
        let context = test_context();
        let bid = TransferBid::new(Money::from_major(12_000_000));
        
        let response = evaluate_bid(&bid, &context);
        assert_eq!(response, NegotiationResponse::Accept);
    }

    #[test]
    fn test_bid_accept_over_asking_price() {
        let context = test_context();
        let bid = TransferBid::new(Money::from_major(15_000_000));
        
        let response = evaluate_bid(&bid, &context);
        assert_eq!(response, NegotiationResponse::Accept);
    }

    #[test]
    fn test_bid_reject_too_low() {
        let context = test_context();
        let bid = TransferBid::new(Money::from_major(5_000_000));
        
        let response = evaluate_bid(&bid, &context);
        assert_eq!(response, NegotiationResponse::Reject);
    }

    #[test]
    fn test_bid_counter_close_to_asking() {
        let context = test_context();
        let bid = TransferBid::new(Money::from_major(11_000_000));
        
        let response = evaluate_bid(&bid, &context);
        assert!(matches!(response, NegotiationResponse::Counter(_)));
    }

    #[test]
    fn test_player_wants_to_leave_easier_to_buy() {
        let mut context = test_context();
        context.player_wants_to_leave = true;
        
        let bid = TransferBid::new(Money::from_major(9_000_000));
        let response = evaluate_bid(&bid, &context);
        
        // Should be more accepting when player wants to leave
        assert!(!matches!(response, NegotiationResponse::Reject));
    }

    #[test]
    fn test_expiring_contract_cheaper() {
        let mut context = test_context();
        context.contract_remaining_years = 0.5;
        
        let bid = TransferBid::new(Money::from_major(8_000_000));
        let response = evaluate_bid(&bid, &context);
        
        // Should accept lower bids for expiring contracts
        assert!(!matches!(response, NegotiationResponse::Reject));
    }

    #[test]
    fn test_contract_offer_accept() {
        let offer = ContractOffer::new(Money::from_major(50_000), 3);
        let current_wage = Money::from_major(40_000);
        
        let response = evaluate_contract(&offer, current_wage, 25, 70, 50);
        assert_eq!(response, PlayerResponse::Accept);
    }

    #[test]
    fn test_contract_offer_want_more_wages() {
        let offer = ContractOffer::new(Money::from_major(40_000), 3);
        let current_wage = Money::from_major(50_000);
        
        let response = evaluate_contract(&offer, current_wage, 25, 70, 50);
        assert!(matches!(response, PlayerResponse::WantMoreWages(_)));
    }

    #[test]
    fn test_contract_young_player_wants_longer() {
        let offer = ContractOffer::new(Money::from_major(50_000), 2);
        let current_wage = Money::from_major(40_000);
        
        let response = evaluate_contract(&offer, current_wage, 19, 70, 50);
        assert_eq!(response, PlayerResponse::WantLongerContract);
    }

    #[test]
    fn test_ambitious_player_rejects_small_club() {
        let offer = ContractOffer::new(Money::from_major(100_000), 4);
        let current_wage = Money::from_major(50_000);
        
        let response = evaluate_contract(&offer, current_wage, 25, 40, 85);
        assert_eq!(response, PlayerResponse::ClubNotGoodEnough);
    }

    #[test]
    fn test_asking_price_with_long_contract() {
        let value = Money::from_major(10_000_000);
        let asking = calculate_asking_price(value, 5.0, 50, false);
        
        assert!(asking.major() > value.major());
    }

    #[test]
    fn test_asking_price_with_expiring_contract() {
        let value = Money::from_major(10_000_000);
        let asking = calculate_asking_price(value, 0.5, 50, false);
        
        assert!(asking.major() < value.major());
    }

    #[test]
    fn test_asking_price_want_to_sell() {
        let value = Money::from_major(10_000_000);
        let normal_asking = calculate_asking_price(value, 3.0, 50, false);
        let sell_asking = calculate_asking_price(value, 3.0, 50, true);
        
        assert!(sell_asking.major() < normal_asking.major());
    }

    #[test]
    fn test_contract_offer_total_value() {
        let mut offer = ContractOffer::new(Money::from_major(50_000), 2);
        offer.signing_bonus = Money::from_major(500_000);
        offer.performance_bonuses = Money::from_major(100_000);
        
        let total = offer.total_value();
        // 50k * 52 * 2 + 500k + 100k = 5.2M + 600k = 5.8M
        assert!(total.major() > 5_000_000);
    }

    #[test]
    fn test_bid_total_value_with_addons() {
        let mut bid = TransferBid::new(Money::from_major(10_000_000));
        bid.add_ons = Money::from_major(2_000_000);
        
        assert_eq!(bid.total_value().major(), 12_000_000);
    }

    #[test]
    fn test_payment_schedule_affects_acceptance() {
        let context = test_context();
        
        let upfront_bid = TransferBid::new(Money::from_major(11_000_000));
        
        let mut installment_bid = TransferBid::new(Money::from_major(11_000_000));
        installment_bid.payment_schedule = PaymentSchedule::Installments(3);
        
        let upfront_response = evaluate_bid(&upfront_bid, &context);
        let installment_response = evaluate_bid(&installment_bid, &context);
        
        // Upfront payment should be preferred
        // Both might be counters, but installments are less favorable
        if let (NegotiationResponse::Counter(up), NegotiationResponse::Counter(inst)) = 
            (upfront_response, installment_response) 
        {
            // Counter for installments should be higher
            assert!(inst.major() >= up.major());
        }
    }
}
