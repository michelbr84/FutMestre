//! Transfer AI - Transfer decision making for AI-controlled clubs.

use chrono::Datelike;
use cm_core::economy::Money;
use cm_core::ids::{ClubId, PlayerId};
use cm_core::world::{Player, Position, World};

use crate::squad_builder::{analyze_squad_needs, analyze_squad_strength, Priority};

/// Transfer decision result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransferDecision {
    Bid { amount: Money, reason: String },
    NoBid { reason: String },
    Negotiate { max_amount: Money },
}

/// Check if the club should bid on a player.
pub fn should_bid(world: &World, club_id: &ClubId, player_id: &PlayerId) -> bool {
    let decision = evaluate_transfer(world, club_id, player_id);
    matches!(
        decision,
        TransferDecision::Bid { .. } | TransferDecision::Negotiate { .. }
    )
}

/// Calculate appropriate bid amount for a player.
pub fn calculate_bid(world: &World, player_id: &PlayerId, budget: Money) -> Money {
    let Some(player) = world.players.get(player_id) else {
        return Money::from_major(100_000);
    };

    let base_value = player.value;
    let quality_factor = calculate_quality_factor(player);
    let age_factor = calculate_age_factor(player);

    // Calculate offer as percentage of value
    let multiplier = quality_factor * age_factor;
    let offer = Money::from_minor((base_value.minor() as f64 * multiplier) as i64);

    // Cap at 80% of budget
    let max_bid = Money::from_minor((budget.minor() as f64 * 0.8) as i64);

    if offer > max_bid {
        max_bid
    } else {
        offer
    }
}

/// Full evaluation of whether to pursue a transfer.
pub fn evaluate_transfer(
    world: &World,
    club_id: &ClubId,
    player_id: &PlayerId,
) -> TransferDecision {
    let Some(player) = world.players.get(player_id) else {
        return TransferDecision::NoBid {
            reason: "Player not found".into(),
        };
    };

    let Some(club) = world.clubs.get(club_id) else {
        return TransferDecision::NoBid {
            reason: "Club not found".into(),
        };
    };

    // Check if player is already at club
    if player.club_id.as_ref() == Some(club_id) {
        return TransferDecision::NoBid {
            reason: "Player already at club".into(),
        };
    }

    // Analyze squad needs
    let needs = analyze_squad_needs(world, club_id);
    let strength = analyze_squad_strength(world, club_id);

    // Check if position is needed
    let position_need = find_position_need(&needs, player.position);

    // Check budget constraints
    let transfer_budget = club.budget.transfer_budget;
    let can_afford = player.value <= transfer_budget;

    // Calculate player quality score
    let quality = player.overall_rating();

    // Decision logic
    match position_need {
        Some(Priority::Critical) => {
            if can_afford {
                let bid = calculate_bid(world, player_id, transfer_budget);
                return TransferDecision::Bid {
                    amount: bid,
                    reason: "Critical position need".into(),
                };
            } else if quality > 75 {
                return TransferDecision::Negotiate {
                    max_amount: transfer_budget,
                };
            }
        }
        Some(Priority::High) => {
            if quality > strength.overall && can_afford {
                let bid = calculate_bid(world, player_id, transfer_budget);
                return TransferDecision::Bid {
                    amount: bid,
                    reason: "High priority upgrade".into(),
                };
            }
        }
        Some(Priority::Medium) => {
            // Only bid if significant upgrade and affordable
            if quality > strength.overall + 5 && can_afford {
                let bid = calculate_bid(world, player_id, transfer_budget);
                return TransferDecision::Bid {
                    amount: bid,
                    reason: "Squad depth improvement".into(),
                };
            }
        }
        Some(Priority::Low) | None => {
            // Only exceptional players
            if quality > 85 && player.potential > 85 && can_afford {
                let bid = calculate_bid(world, player_id, transfer_budget);
                return TransferDecision::Bid {
                    amount: bid,
                    reason: "Exceptional talent opportunity".into(),
                };
            }
        }
    }

    TransferDecision::NoBid {
        reason: "Does not meet transfer criteria".into(),
    }
}

/// Find if there's a need for a specific position.
fn find_position_need(
    needs: &[crate::squad_builder::SquadNeed],
    position: Position,
) -> Option<Priority> {
    let pos_short = position.short_name();

    // Also check for broad position categories
    let is_defender = position.is_defender();
    let is_midfielder = position.is_midfielder();
    let is_forward = position.is_forward();

    needs
        .iter()
        .find(|n| {
            n.position == pos_short
                || (is_defender && n.position == "Defender")
                || (is_midfielder && n.position == "Midfielder")
                || (is_forward && n.position == "Forward")
        })
        .map(|n| n.priority)
}

/// Calculate quality factor for bidding (higher quality = pay more).
fn calculate_quality_factor(player: &Player) -> f64 {
    let quality = player.overall_rating();

    if quality >= 85 {
        1.3 // Premium for elite players
    } else if quality >= 75 {
        1.1 // Slight premium for good players
    } else if quality >= 65 {
        1.0 // Market rate
    } else {
        0.9 // Discount for average players
    }
}

/// Calculate age factor for bidding (younger = pay more).
fn calculate_age_factor(player: &Player) -> f64 {
    let birth_year = player.birth_date.year();
    let current_year = 2024; // Would normally be passed in
    let age = (current_year - birth_year) as u8;

    if age <= 21 {
        1.2 // Premium for youth
    } else if age <= 25 {
        1.1 // Peak value age
    } else if age <= 29 {
        1.0 // Normal
    } else if age <= 32 {
        0.85 // Approaching decline
    } else {
        0.7 // Veteran discount
    }
}

/// Evaluate whether to sell a player.
pub fn should_sell(
    world: &World,
    club_id: &ClubId,
    player_id: &PlayerId,
    offer: Money,
) -> SellDecision {
    let Some(player) = world.players.get(player_id) else {
        return SellDecision::Reject {
            reason: "Player not found".into(),
        };
    };

    let Some(_club) = world.clubs.get(club_id) else {
        return SellDecision::Reject {
            reason: "Club not found".into(),
        };
    };

    // Check if player is at this club
    if player.club_id.as_ref() != Some(club_id) {
        return SellDecision::Reject {
            reason: "Player not at club".into(),
        };
    }

    let player_value = player.value;
    let quality = player.overall_rating();
    let strength = analyze_squad_strength(world, club_id);

    // Count players at this position
    let players_at_position = world
        .club_players(club_id)
        .iter()
        .filter(|p| p.position == player.position)
        .count();

    // Decision factors
    let good_offer = offer >= player_value;
    let great_offer = offer >= Money::from_minor((player_value.minor() as f64 * 1.3) as i64);
    let is_key_player = quality >= strength.overall + 5;
    let has_depth = players_at_position >= 3;
    let is_aging = {
        let age = 2024 - player.birth_date.year();
        age >= 30
    };

    // Never sell star players for less than great offer
    if is_key_player && !great_offer {
        return SellDecision::Reject {
            reason: "Key player - need better offer".into(),
        };
    }

    // Accept great offers for aging players
    if is_aging && great_offer {
        return SellDecision::Accept {
            reason: "Good price for aging player".into(),
        };
    }

    // Accept good offers if we have depth
    if good_offer && has_depth && !is_key_player {
        return SellDecision::Accept {
            reason: "Fair price with squad depth".into(),
        };
    }

    // Negotiate if offer is close
    let offer_ratio = offer.minor() as f64 / player_value.minor().max(1) as f64;
    if offer_ratio >= 0.85 {
        let counter = Money::from_minor((player_value.minor() as f64 * 1.1) as i64);
        return SellDecision::Counter {
            amount: counter,
            reason: "Counter offer - close to value".into(),
        };
    }

    SellDecision::Reject {
        reason: "Offer too low".into(),
    }
}

/// Decision on whether to sell a player.
#[derive(Debug, Clone, PartialEq)]
pub enum SellDecision {
    Accept { reason: String },
    Reject { reason: String },
    Counter { amount: Money, reason: String },
}

/// Identify players to potentially sell.
pub fn identify_sellable_players(world: &World, club_id: &ClubId) -> Vec<(PlayerId, SellReason)> {
    let players = world.club_players(club_id);
    let strength = analyze_squad_strength(world, club_id);

    let mut sellable = Vec::new();

    for player in players {
        let age = 2024 - player.birth_date.year();
        let quality = player.overall_rating();

        // Aging players past prime
        if age >= 31 && quality < strength.overall {
            sellable.push((player.id.clone(), SellReason::Age));
            continue;
        }

        // Below squad average quality
        if quality < strength.overall.saturating_sub(10) {
            sellable.push((player.id.clone(), SellReason::Quality));
            continue;
        }

        // High value relative to quality (sell high)
        if player.value >= Money::from_major(10_000_000) && quality < 75 {
            sellable.push((player.id.clone(), SellReason::HighValue));
            continue;
        }

        // Low potential youth
        if age <= 23 && player.potential < 65 {
            sellable.push((player.id.clone(), SellReason::LowPotential));
        }
    }

    sellable
}

/// Reason for considering selling a player.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SellReason {
    Age,
    Quality,
    HighValue,
    LowPotential,
    SquadSpace,
    Financial,
}

/// Identify transfer targets for the club.
pub fn identify_targets(world: &World, club_id: &ClubId, max_results: usize) -> Vec<PlayerId> {
    let needs = analyze_squad_needs(world, club_id);
    let club = match world.clubs.get(club_id) {
        Some(c) => c,
        None => return Vec::new(),
    };

    let budget = club.budget.transfer_budget;

    // Get critical and high priority needs
    let priority_positions: Vec<_> = needs
        .iter()
        .filter(|n| matches!(n.priority, Priority::Critical | Priority::High))
        .map(|n| &n.position)
        .collect();

    // Find available players that match needs
    let mut candidates: Vec<_> = world
        .players
        .values()
        .filter(|p| {
            // Not at this club
            p.club_id.as_ref() != Some(club_id) &&
            // Affordable
            p.value <= budget &&
            // Matches needed position
            priority_positions.iter().any(|pos| p.position.short_name() == *pos)
        })
        .collect();

    // Sort by quality
    candidates.sort_by(|a, b| b.overall_rating().cmp(&a.overall_rating()));

    candidates
        .iter()
        .take(max_results)
        .map(|p| p.id.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, NaiveDate};
    use cm_core::economy::Budget;
    use cm_core::ids::NationId;
    use cm_core::world::Club;

    fn create_test_player(
        id: &str,
        position: Position,
        quality: u8,
        age_years: i32,
        value: Money,
    ) -> Player {
        let birth_date = NaiveDate::from_ymd_opt(2024 - age_years, 6, 15).unwrap();
        let mut player = Player::new(id, "Test", id, NationId::new("test"), birth_date, position);

        player.value = value;
        player.potential = quality + 5;

        // Set attributes
        player.attributes.technical.finishing = quality;
        player.attributes.technical.dribbling = quality;
        player.attributes.technical.passing = quality;
        player.attributes.technical.tackling = quality;
        player.attributes.technical.marking = quality;
        player.attributes.mental.off_the_ball = quality;
        player.attributes.mental.positioning = quality;
        player.attributes.mental.vision = quality;
        player.attributes.physical.strength = quality;
        player.attributes.physical.stamina = quality;
        player.attributes.technical.first_touch = quality;
        player.attributes.goalkeeper.handling = quality;
        player.attributes.goalkeeper.reflexes = quality;
        player.attributes.goalkeeper.positioning = quality;
        player.attributes.goalkeeper.one_on_ones = quality;

        player
    }

    fn setup_test_world() -> (World, ClubId) {
        let mut world = World::new();
        let club_id = ClubId::new("test_club");

        let mut club = Club::new("test_club", "Test FC", NationId::new("test"));
        club.budget = Budget {
            transfer_budget: Money::from_major(10_000_000),
            wage_budget: Money::from_major(500_000),
            wage_bill: Money::from_major(200_000),
            balance: Money::from_major(15_000_000),
            loans: Vec::new(),
        };
        club.reputation = 70;

        // Minimal squad with gaps
        let players = vec![
            create_test_player(
                "gk1",
                Position::Goalkeeper,
                70,
                28,
                Money::from_major(1_000_000),
            ),
            create_test_player(
                "dc1",
                Position::DefenderCenter,
                72,
                26,
                Money::from_major(2_000_000),
            ),
            create_test_player(
                "dc2",
                Position::DefenderCenter,
                68,
                30,
                Money::from_major(1_500_000),
            ),
            create_test_player(
                "mc1",
                Position::MidfielderCenter,
                75,
                25,
                Money::from_major(3_000_000),
            ),
            create_test_player(
                "fc1",
                Position::ForwardCenter,
                78,
                24,
                Money::from_major(5_000_000),
            ),
        ];

        for mut player in players {
            player.club_id = Some(club_id.clone());
            club.add_player(player.id.clone());
            world.players.insert(player.id.clone(), player);
        }

        // Add some available players not at the club
        let available = vec![
            create_test_player(
                "avail_dl",
                Position::DefenderLeft,
                74,
                23,
                Money::from_major(2_000_000),
            ),
            create_test_player(
                "avail_dr",
                Position::DefenderRight,
                72,
                25,
                Money::from_major(1_800_000),
            ),
            create_test_player(
                "avail_mc",
                Position::MidfielderCenter,
                76,
                22,
                Money::from_major(4_000_000),
            ),
            create_test_player(
                "avail_fc",
                Position::ForwardCenter,
                80,
                26,
                Money::from_major(8_000_000),
            ),
            create_test_player(
                "avail_old",
                Position::ForwardCenter,
                75,
                34,
                Money::from_major(2_000_000),
            ),
        ];

        for player in available {
            world.players.insert(player.id.clone(), player);
        }

        world.clubs.insert(club_id.clone(), club);

        (world, club_id)
    }

    #[test]
    fn test_should_bid_for_needed_position() {
        let (world, club_id) = setup_test_world();

        // Club needs fullbacks
        let should = should_bid(&world, &club_id, &PlayerId::new("avail_dl"));
        assert!(should, "Should bid on needed left back");

        let should = should_bid(&world, &club_id, &PlayerId::new("avail_dr"));
        assert!(should, "Should bid on needed right back");
    }

    #[test]
    fn test_should_not_bid_own_player() {
        let (world, club_id) = setup_test_world();

        let should = should_bid(&world, &club_id, &PlayerId::new("gk1"));
        assert!(!should, "Should not bid on own player");
    }

    #[test]
    fn test_calculate_bid_within_budget() {
        let (world, _) = setup_test_world();

        let budget = Money::from_major(5_000_000);
        let bid = calculate_bid(&world, &PlayerId::new("avail_dl"), budget);

        assert!(bid <= budget, "Bid should not exceed budget");
        assert!(bid > Money::ZERO, "Bid should be positive");
    }

    #[test]
    fn test_quality_factor() {
        let elite = create_test_player(
            "elite",
            Position::ForwardCenter,
            88,
            25,
            Money::from_major(1),
        );
        let good = create_test_player(
            "good",
            Position::ForwardCenter,
            76,
            25,
            Money::from_major(1),
        );
        let avg = create_test_player("avg", Position::ForwardCenter, 62, 25, Money::from_major(1));

        let elite_factor = calculate_quality_factor(&elite);
        let good_factor = calculate_quality_factor(&good);
        let avg_factor = calculate_quality_factor(&avg);

        assert!(elite_factor > good_factor);
        assert!(good_factor > avg_factor);
    }

    #[test]
    fn test_age_factor() {
        let young = create_test_player(
            "young",
            Position::ForwardCenter,
            70,
            20,
            Money::from_major(1),
        );
        let prime = create_test_player(
            "prime",
            Position::ForwardCenter,
            70,
            27,
            Money::from_major(1),
        );
        let old = create_test_player("old", Position::ForwardCenter, 70, 34, Money::from_major(1));

        let young_factor = calculate_age_factor(&young);
        let prime_factor = calculate_age_factor(&prime);
        let old_factor = calculate_age_factor(&old);

        assert!(young_factor > prime_factor);
        assert!(prime_factor > old_factor);
    }

    #[test]
    fn test_should_sell_good_offer_with_depth() {
        let (mut world, club_id) = setup_test_world();

        // Add more defenders for depth
        for i in 3..=5 {
            let mut player = create_test_player(
                &format!("dc{}", i),
                Position::DefenderCenter,
                65,
                27,
                Money::from_major(1_000_000),
            );
            player.club_id = Some(club_id.clone());
            world
                .clubs
                .get_mut(&club_id)
                .unwrap()
                .add_player(player.id.clone());
            world.players.insert(player.id.clone(), player);
        }

        // Non-key defender with depth
        let offer = Money::from_major(1_500_000);
        let decision = should_sell(&world, &club_id, &PlayerId::new("dc2"), offer);

        assert!(matches!(decision, SellDecision::Accept { .. }));
    }

    #[test]
    fn test_should_not_sell_key_player_cheap() {
        let (world, club_id) = setup_test_world();

        // Offer below value for best player
        let offer = Money::from_major(3_000_000);
        let decision = should_sell(&world, &club_id, &PlayerId::new("fc1"), offer);

        assert!(matches!(
            decision,
            SellDecision::Reject { .. } | SellDecision::Counter { .. }
        ));
    }

    #[test]
    fn test_identify_sellable_players() {
        let (mut world, club_id) = setup_test_world();

        // Add an aging player
        let mut old = create_test_player(
            "old_mc",
            Position::MidfielderCenter,
            60,
            33,
            Money::from_major(500_000),
        );
        old.club_id = Some(club_id.clone());
        world
            .clubs
            .get_mut(&club_id)
            .unwrap()
            .add_player(old.id.clone());
        world.players.insert(old.id.clone(), old);

        let sellable = identify_sellable_players(&world, &club_id);

        let old_player_sellable = sellable
            .iter()
            .any(|(id, reason)| *id == PlayerId::new("old_mc") && *reason == SellReason::Age);

        assert!(
            old_player_sellable,
            "Old player should be identified as sellable"
        );
    }

    #[test]
    fn test_identify_targets() {
        let (world, club_id) = setup_test_world();

        let targets = identify_targets(&world, &club_id, 5);

        // Should find fullbacks (critical need)
        let has_fullback = targets.iter().any(|id| {
            world
                .players
                .get(id)
                .map(|p| {
                    p.position == Position::DefenderLeft || p.position == Position::DefenderRight
                })
                .unwrap_or(false)
        });

        assert!(has_fullback, "Should identify fullback targets");
    }

    #[test]
    fn test_evaluate_transfer_critical_need() {
        let (world, club_id) = setup_test_world();

        let decision = evaluate_transfer(&world, &club_id, &PlayerId::new("avail_dl"));

        match decision {
            TransferDecision::Bid { reason, .. } => {
                assert!(
                    reason.contains("Critical") || reason.contains("High"),
                    "Should cite position need: {}",
                    reason
                );
            }
            _ => panic!("Should decide to bid for critical need"),
        }
    }

    #[test]
    fn test_evaluate_transfer_not_affordable() {
        let (mut world, club_id) = setup_test_world();

        // Reduce budget
        world
            .clubs
            .get_mut(&club_id)
            .unwrap()
            .budget
            .transfer_budget = Money::from_major(100_000);

        let decision = evaluate_transfer(&world, &club_id, &PlayerId::new("avail_dl"));

        // Should still potentially negotiate for needed position
        assert!(
            !matches!(decision, TransferDecision::Bid { .. }),
            "Should not bid when can't afford"
        );
    }
}
