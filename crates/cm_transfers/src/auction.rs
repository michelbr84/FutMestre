//! Leilao estilo Elifoot - sistema de leilao de jogadores.
//!
//! Inspirado no Elifoot 98, onde jogadores sao vendidos em leilao
//! com rodadas de lances entre clubes interessados.

use cm_core::economy::Money;
use cm_core::ids::{ClubId, PlayerId};
use serde::{Deserialize, Serialize};

/// Resultado de um lance no leilao.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuctionResult {
    /// Lance aceito (supera o lance atual).
    Accepted,
    /// Lance muito baixo (abaixo do lance atual ou do minimo).
    TooLow,
    /// Leilao ja encerrado.
    AuctionEnded,
}

/// Lance individual em um leilao.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionBid {
    pub club_id: ClubId,
    pub amount: Money,
    pub round: u8,
}

/// Leilao de jogador estilo Elifoot.
///
/// O leilao funciona em rodadas: cada clube interessado pode dar um lance
/// por rodada. O lance deve superar o lance atual. Quando uma rodada
/// termina sem novos lances, ou quando as rodadas se esgotam, o leilao
/// encerra e o maior lance vence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auction {
    /// Jogador sendo leiloado.
    pub player_id: PlayerId,
    /// Clube vendedor.
    pub seller_club_id: ClubId,
    /// Lance minimo para participar.
    pub minimum_bid: Money,
    /// Lance mais alto atual.
    pub current_bid: Money,
    /// Clube com o lance mais alto.
    pub current_bidder: Option<ClubId>,
    /// Rodadas restantes.
    pub rounds_remaining: u8,
    /// Historico de todos os lances.
    pub bids: Vec<AuctionBid>,
    /// Indica se houve lance na rodada atual.
    had_bid_this_round: bool,
}

/// Numero padrao de rodadas em um leilao.
const DEFAULT_ROUNDS: u8 = 5;

impl Auction {
    /// Cria um novo leilao para um jogador.
    pub fn new(player_id: PlayerId, seller: ClubId, min_bid: Money) -> Self {
        Self {
            player_id,
            seller_club_id: seller,
            minimum_bid: min_bid,
            current_bid: Money::ZERO,
            current_bidder: None,
            rounds_remaining: DEFAULT_ROUNDS,
            bids: Vec::new(),
            had_bid_this_round: false,
        }
    }

    /// Dar um lance no leilao.
    ///
    /// O lance deve ser maior que o lance atual e maior ou igual ao lance minimo.
    /// O clube vendedor nao pode dar lances.
    pub fn place_bid(&mut self, club_id: ClubId, amount: Money) -> AuctionResult {
        if self.is_finished() {
            return AuctionResult::AuctionEnded;
        }

        // Lance deve ser >= minimo
        if amount < self.minimum_bid {
            return AuctionResult::TooLow;
        }

        // Lance deve superar o lance atual
        if amount <= self.current_bid {
            return AuctionResult::TooLow;
        }

        // Registrar o lance
        self.bids.push(AuctionBid {
            club_id: club_id.clone(),
            amount,
            round: DEFAULT_ROUNDS - self.rounds_remaining + 1,
        });

        self.current_bid = amount;
        self.current_bidder = Some(club_id);
        self.had_bid_this_round = true;

        AuctionResult::Accepted
    }

    /// Avanca para a proxima rodada.
    ///
    /// Retorna `true` se o leilao continua (ainda ha rodadas e houve lance).
    /// Retorna `false` se o leilao encerrou.
    pub fn advance_round(&mut self) -> bool {
        if self.is_finished() {
            return false;
        }

        // Se nao houve lance nesta rodada, leilao encerra
        if !self.had_bid_this_round {
            self.rounds_remaining = 0;
            return false;
        }

        // Avanca rodada
        self.rounds_remaining = self.rounds_remaining.saturating_sub(1);
        self.had_bid_this_round = false;

        // Retorna true se ainda ha rodadas
        self.rounds_remaining > 0
    }

    /// Retorna o vencedor e o valor do lance vencedor.
    ///
    /// Retorna `None` se nao houve lances ou o leilao nao encerrou.
    pub fn winner(&self) -> Option<(&ClubId, Money)> {
        if !self.is_finished() {
            return None;
        }
        self.current_bidder
            .as_ref()
            .map(|bidder| (bidder, self.current_bid))
    }

    /// Verifica se o leilao terminou.
    pub fn is_finished(&self) -> bool {
        self.rounds_remaining == 0
    }

    /// Numero total de lances recebidos.
    pub fn total_bids(&self) -> usize {
        self.bids.len()
    }

    /// Rodada atual (1-based).
    pub fn current_round(&self) -> u8 {
        DEFAULT_ROUNDS - self.rounds_remaining + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seller() -> ClubId {
        ClubId::new("SELLER")
    }

    fn bidder_a() -> ClubId {
        ClubId::new("CLUB_A")
    }

    fn bidder_b() -> ClubId {
        ClubId::new("CLUB_B")
    }

    fn player() -> PlayerId {
        PlayerId::new("P001")
    }

    #[test]
    fn test_new_auction() {
        let auction = Auction::new(player(), seller(), Money::from_major(5_000_000));

        assert_eq!(auction.player_id, player());
        assert_eq!(auction.seller_club_id, seller());
        assert_eq!(auction.minimum_bid, Money::from_major(5_000_000));
        assert_eq!(auction.current_bid, Money::ZERO);
        assert!(auction.current_bidder.is_none());
        assert_eq!(auction.rounds_remaining, DEFAULT_ROUNDS);
        assert!(!auction.is_finished());
        assert!(auction.bids.is_empty());
    }

    #[test]
    fn test_place_valid_bid() {
        let mut auction = Auction::new(player(), seller(), Money::from_major(5_000_000));

        let result = auction.place_bid(bidder_a(), Money::from_major(6_000_000));
        assert_eq!(result, AuctionResult::Accepted);
        assert_eq!(auction.current_bid, Money::from_major(6_000_000));
        assert_eq!(auction.current_bidder, Some(bidder_a()));
        assert_eq!(auction.total_bids(), 1);
    }

    #[test]
    fn test_bid_below_minimum_rejected() {
        let mut auction = Auction::new(player(), seller(), Money::from_major(5_000_000));

        let result = auction.place_bid(bidder_a(), Money::from_major(3_000_000));
        assert_eq!(result, AuctionResult::TooLow);
        assert!(auction.current_bidder.is_none());
        assert_eq!(auction.total_bids(), 0);
    }

    #[test]
    fn test_bid_at_minimum_accepted() {
        let mut auction = Auction::new(player(), seller(), Money::from_major(5_000_000));

        let result = auction.place_bid(bidder_a(), Money::from_major(5_000_000));
        assert_eq!(result, AuctionResult::Accepted);
    }

    #[test]
    fn test_bid_must_exceed_current() {
        let mut auction = Auction::new(player(), seller(), Money::from_major(5_000_000));

        auction.place_bid(bidder_a(), Money::from_major(6_000_000));

        // Same amount should fail
        let result = auction.place_bid(bidder_b(), Money::from_major(6_000_000));
        assert_eq!(result, AuctionResult::TooLow);

        // Lower amount should fail
        let result = auction.place_bid(bidder_b(), Money::from_major(5_500_000));
        assert_eq!(result, AuctionResult::TooLow);

        // Higher amount should succeed
        let result = auction.place_bid(bidder_b(), Money::from_major(7_000_000));
        assert_eq!(result, AuctionResult::Accepted);
        assert_eq!(auction.current_bidder, Some(bidder_b()));
    }

    #[test]
    fn test_advance_round_with_bids() {
        let mut auction = Auction::new(player(), seller(), Money::from_major(5_000_000));

        auction.place_bid(bidder_a(), Money::from_major(6_000_000));
        let continues = auction.advance_round();
        assert!(continues);
        assert_eq!(auction.rounds_remaining, DEFAULT_ROUNDS - 1);
    }

    #[test]
    fn test_advance_round_without_bids_ends_auction() {
        let mut auction = Auction::new(player(), seller(), Money::from_major(5_000_000));

        auction.place_bid(bidder_a(), Money::from_major(6_000_000));
        auction.advance_round(); // Round 1 -> 2

        // No bids in round 2 -> auction ends
        let continues = auction.advance_round();
        assert!(!continues);
        assert!(auction.is_finished());
    }

    #[test]
    fn test_auction_ends_after_all_rounds() {
        let mut auction = Auction::new(player(), seller(), Money::from_major(1_000_000));

        for i in 0..DEFAULT_ROUNDS {
            assert!(!auction.is_finished());
            let amount = Money::from_major(2_000_000 + (i as i64) * 500_000);
            auction.place_bid(bidder_a(), amount);
            auction.advance_round();
        }

        assert!(auction.is_finished());
    }

    #[test]
    fn test_winner_before_finish_returns_none() {
        let mut auction = Auction::new(player(), seller(), Money::from_major(5_000_000));
        auction.place_bid(bidder_a(), Money::from_major(6_000_000));

        assert!(auction.winner().is_none());
    }

    #[test]
    fn test_winner_after_finish() {
        let mut auction = Auction::new(player(), seller(), Money::from_major(5_000_000));

        auction.place_bid(bidder_a(), Money::from_major(6_000_000));
        auction.advance_round();

        auction.place_bid(bidder_b(), Money::from_major(8_000_000));
        auction.advance_round();

        // No bids -> ends
        auction.advance_round();

        assert!(auction.is_finished());
        let (winner, amount) = auction.winner().unwrap();
        assert_eq!(*winner, bidder_b());
        assert_eq!(amount, Money::from_major(8_000_000));
    }

    #[test]
    fn test_no_winner_if_no_bids() {
        let mut auction = Auction::new(player(), seller(), Money::from_major(5_000_000));

        // Advance without any bids
        let continues = auction.advance_round();
        assert!(!continues);
        assert!(auction.is_finished());
        assert!(auction.winner().is_none());
    }

    #[test]
    fn test_bid_on_finished_auction() {
        let mut auction = Auction::new(player(), seller(), Money::from_major(5_000_000));

        // End the auction immediately (no bids)
        auction.advance_round();
        assert!(auction.is_finished());

        let result = auction.place_bid(bidder_a(), Money::from_major(10_000_000));
        assert_eq!(result, AuctionResult::AuctionEnded);
    }

    #[test]
    fn test_bidding_war_multiple_rounds() {
        let mut auction = Auction::new(player(), seller(), Money::from_major(5_000_000));

        // Round 1: A bids 6M
        auction.place_bid(bidder_a(), Money::from_major(6_000_000));
        assert!(auction.advance_round());

        // Round 2: B bids 7M, A bids 8M
        auction.place_bid(bidder_b(), Money::from_major(7_000_000));
        auction.place_bid(bidder_a(), Money::from_major(8_000_000));
        assert!(auction.advance_round());

        // Round 3: B bids 9M
        auction.place_bid(bidder_b(), Money::from_major(9_000_000));
        assert!(auction.advance_round());

        // Round 4: no bids -> ends
        let continues = auction.advance_round();
        assert!(!continues);

        let (winner, amount) = auction.winner().unwrap();
        assert_eq!(*winner, bidder_b());
        assert_eq!(amount, Money::from_major(9_000_000));
        assert_eq!(auction.total_bids(), 4);
    }

    #[test]
    fn test_current_round_tracking() {
        let mut auction = Auction::new(player(), seller(), Money::from_major(1_000_000));

        assert_eq!(auction.current_round(), 1);

        auction.place_bid(bidder_a(), Money::from_major(2_000_000));
        auction.advance_round();
        assert_eq!(auction.current_round(), 2);

        auction.place_bid(bidder_b(), Money::from_major(3_000_000));
        auction.advance_round();
        assert_eq!(auction.current_round(), 3);
    }

    #[test]
    fn test_bid_history_records_rounds() {
        let mut auction = Auction::new(player(), seller(), Money::from_major(1_000_000));

        auction.place_bid(bidder_a(), Money::from_major(2_000_000));
        auction.advance_round();

        auction.place_bid(bidder_b(), Money::from_major(3_000_000));

        assert_eq!(auction.bids[0].round, 1);
        assert_eq!(auction.bids[0].club_id, bidder_a());
        assert_eq!(auction.bids[1].round, 2);
        assert_eq!(auction.bids[1].club_id, bidder_b());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut auction = Auction::new(player(), seller(), Money::from_major(5_000_000));
        auction.place_bid(bidder_a(), Money::from_major(6_000_000));

        let json = serde_json::to_string(&auction).unwrap();
        let deserialized: Auction = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.player_id, auction.player_id);
        assert_eq!(deserialized.current_bid, auction.current_bid);
        assert_eq!(deserialized.current_bidder, auction.current_bidder);
        assert_eq!(deserialized.rounds_remaining, auction.rounds_remaining);
        assert_eq!(deserialized.bids.len(), auction.bids.len());
    }
}
