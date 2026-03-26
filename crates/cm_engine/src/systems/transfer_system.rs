//! Transfer system.

use crate::config::GameConfig;
use crate::state::GameState;
use chrono::Weekday;
use cm_ai::squad_builder::{analyze_squad_needs, Priority};
use cm_ai::transfer_ai::{calculate_bid, identify_targets};
use cm_core::economy::Money;
use cm_core::ids::{ClubId, PlayerId};
use cm_core::world::World;
use cm_transfers::negotiation::{
    calculate_asking_price, evaluate_bid, NegotiationContext, NegotiationResponse, TransferBid,
};
use cm_transfers::window::is_window_open;

/// Limite de transferencias por clube por janela.
const MAX_TRANSFERS_PER_WINDOW: u8 = 2;

/// Transfer system.
pub struct TransferSystem;

impl TransferSystem {
    /// Run daily transfer logic.
    pub fn run_daily(&self, _cfg: &GameConfig, world: &mut World, state: &mut GameState) {
        state.flags.transfer_window_open = is_window_open(state.date.date());

        // On the 1st of each month, check for expired contracts.
        if state.date.is_first_of_month() {
            self.process_expired_contracts(world, state);
        }

        // AI clubs make transfers once per week (Mondays) during transfer windows.
        if state.flags.transfer_window_open && state.date.weekday() == Weekday::Mon {
            self.process_ai_transfers(world, state);
        }

        // Generate transfer rumors every 3 days during transfer windows.
        if state.flags.transfer_window_open && state.days_played % 3 == 0 {
            self.generate_rumors(world, state);
        }
    }

    /// Gera rumores de transferencia aleatorios para o inbox.
    fn generate_rumors(&self, world: &World, state: &mut GameState) {
        use crate::inbox::generators::generate_random_rumors;

        // Coletar jogadores notaveis (overall >= 70) com clubes.
        let player_names: Vec<(String, String)> = world
            .players
            .values()
            .filter(|p| p.overall_rating() >= 70 && p.club_id.is_some())
            .take(50)
            .map(|p| {
                let club_name = p
                    .club_id
                    .as_ref()
                    .and_then(|cid| world.clubs.get(cid))
                    .map(|c| c.short_name.clone())
                    .unwrap_or_else(|| "???".to_string());
                (p.full_name(), club_name)
            })
            .collect();

        let club_names: Vec<String> = world
            .clubs
            .values()
            .filter(|c| c.reputation >= 50)
            .take(20)
            .map(|c| c.short_name.clone())
            .collect();

        let seed = state.days_played;
        let rumors = generate_random_rumors(state.date.date(), &player_names, &club_names, seed);

        for rumor in rumors {
            state.add_message(format!("{}: {}", rumor.subject, rumor.body));
        }
    }

    /// AI clubs evaluate needs and make transfers.
    fn process_ai_transfers(&self, world: &mut World, state: &mut GameState) {
        let user_club_id = state.club_id.clone();
        let current_date = state.date.date();

        // Collect AI club IDs (exclude user club).
        let ai_club_ids: Vec<ClubId> = world
            .clubs
            .keys()
            .filter(|cid| **cid != user_club_id)
            .cloned()
            .collect();

        // Collect transfer actions first to avoid borrow issues.
        let mut transfers: Vec<(ClubId, PlayerId, Money, String, Option<ClubId>)> = Vec::new();

        for club_id in &ai_club_ids {
            // Check squad needs.
            let needs = analyze_squad_needs(world, club_id);
            let has_urgent_need = needs
                .iter()
                .any(|n| matches!(n.priority, Priority::Critical | Priority::High));

            if !has_urgent_need {
                continue;
            }

            // Find targets.
            let targets = identify_targets(world, club_id, 5);
            if targets.is_empty() {
                continue;
            }

            let budget = world
                .clubs
                .get(club_id)
                .map(|c| c.budget.transfer_budget)
                .unwrap_or(Money::ZERO);

            let club_rep = world.clubs.get(club_id).map(|c| c.reputation).unwrap_or(50);

            // Try to sign the best target (limit MAX_TRANSFERS_PER_WINDOW per club).
            let mut signed_this_window = 0u8;
            for target_id in &targets {
                if signed_this_window >= MAX_TRANSFERS_PER_WINDOW {
                    break;
                }

                let Some(player) = world.players.get(target_id) else {
                    continue;
                };

                // Skip if already at this club.
                if player.club_id.as_ref() == Some(club_id) {
                    continue;
                }

                let player_value = player.value;
                let player_name = player.full_name();
                let from_club_id = player.club_id.clone();

                let contract_years = player
                    .contract
                    .as_ref()
                    .map(|c| c.years_remaining(current_date))
                    .unwrap_or(0.5);

                let selling_rep = from_club_id
                    .as_ref()
                    .and_then(|cid| world.clubs.get(cid))
                    .map(|c| c.reputation)
                    .unwrap_or(30);

                let asking_price = calculate_asking_price(player_value, contract_years, 50, false);

                let bid_amount = calculate_bid(world, target_id, budget);

                // Build negotiation context.
                let context = NegotiationContext {
                    player_value,
                    asking_price,
                    selling_club_reputation: selling_rep,
                    buying_club_reputation: club_rep,
                    player_wants_to_leave: false,
                    contract_remaining_years: contract_years,
                    selling_desperation: 0,
                };

                let bid = TransferBid::new(bid_amount);
                let response = evaluate_bid(&bid, &context);

                match response {
                    NegotiationResponse::Accept => {
                        transfers.push((
                            club_id.clone(),
                            target_id.clone(),
                            bid_amount,
                            player_name,
                            from_club_id,
                        ));
                        signed_this_window += 1;
                    }
                    NegotiationResponse::Counter(counter_amount) => {
                        // AI accepts counter if they can afford it.
                        if counter_amount <= budget {
                            transfers.push((
                                club_id.clone(),
                                target_id.clone(),
                                counter_amount,
                                player_name,
                                from_club_id,
                            ));
                            signed_this_window += 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Execute transfers.
        for (buying_club_id, player_id, fee, player_name, from_club_id) in transfers {
            let buying_club_name = world
                .clubs
                .get(&buying_club_id)
                .map(|c| c.short_name.clone())
                .unwrap_or_else(|| "???".to_string());

            // Deduct from buying club budget.
            if let Some(club) = world.clubs.get_mut(&buying_club_id) {
                club.budget.spend_transfer(fee);
                club.add_player(player_id.clone());
            }

            // Remove from selling club and add income.
            if let Some(ref old_club_id) = from_club_id {
                if let Some(old_club) = world.clubs.get_mut(old_club_id) {
                    old_club.remove_player(&player_id);
                    old_club.budget.receive_transfer(fee);
                }
            }

            // Update player's club_id.
            if let Some(player) = world.players.get_mut(&player_id) {
                player.club_id = Some(buying_club_id.clone());
            }

            // Generate inbox message for notable transfers.
            state.add_message(format!(
                "TRANSFERENCIA: {} assinou com {} por {}.",
                player_name, buying_club_name, fee
            ));
        }
    }

    /// Check all players for expired contracts. Players whose contracts have
    /// expired lose their club_id and become free agents.
    fn process_expired_contracts(&self, world: &mut World, state: &mut GameState) {
        let current_date = state.date.date();
        let user_club_id = state.club_id.clone();

        // Collect players with expired contracts first to avoid borrow issues.
        let expired: Vec<(PlayerId, String, Option<ClubId>)> = world
            .players
            .values()
            .filter(|p| {
                p.club_id.is_some()
                    && p.contract
                        .as_ref()
                        .map(|c| c.is_expired(current_date))
                        .unwrap_or(false)
            })
            .map(|p| (p.id.clone(), p.full_name(), p.club_id.clone()))
            .collect();

        for (player_id, player_name, old_club_id) in expired {
            // Remove player from club roster.
            if let Some(club_id) = &old_club_id {
                if let Some(club) = world.clubs.get_mut(club_id) {
                    club.remove_player(&player_id);
                }
            }

            // Set player as free agent.
            if let Some(player) = world.players.get_mut(&player_id) {
                player.club_id = None;
                player.contract = None;
            }

            // Notify user if it's their club's player.
            if old_club_id.as_ref() == Some(&user_club_id) {
                state.add_message(format!(
                    "O contrato de {} expirou. Ele agora e um agente livre.",
                    player_name
                ));
            }
        }
    }
}
