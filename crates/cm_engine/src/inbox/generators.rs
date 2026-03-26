//! Message generators for creating inbox messages.

use chrono::NaiveDate;
use cm_core::economy::Money;

use super::message::{InboxMessage, MessageCategory, MessagePriority};

/// Generate welcome message.
pub fn welcome_message(manager: &str, club: &str) -> String {
    format!("Welcome {} to {}! Your journey begins now.", manager, club)
}

/// Generate a welcome inbox message.
pub fn welcome_inbox(date: NaiveDate, manager: &str, club: &str) -> InboxMessage {
    InboxMessage::new(
        date,
        MessageCategory::Board,
        format!("Welcome to {}", club),
        format!(
            "Dear {},\n\n\
            The board is delighted to welcome you as our new manager. \
            We have high hopes for this season and trust in your ability \
            to lead us to success.\n\n\
            Please familiarize yourself with the squad and set up your tactics.\n\n\
            Best regards,\nThe Board",
            manager
        ),
    )
}

/// Generate match result message.
pub fn match_result(
    date: NaiveDate,
    home_club: &str,
    away_club: &str,
    home_goals: u8,
    away_goals: u8,
    is_user_match: bool,
) -> InboxMessage {
    let result_str = if home_goals > away_goals {
        "Victory"
    } else if home_goals < away_goals {
        "Defeat"
    } else {
        "Draw"
    };

    let subject = if is_user_match {
        format!(
            "Match Report: {} {} - {} {}",
            home_club, home_goals, away_goals, away_club
        )
    } else {
        format!(
            "{} {} - {} {}",
            home_club, home_goals, away_goals, away_club
        )
    };

    let body = if is_user_match {
        format!(
            "Final Score: {} {} - {} {}\n\n\
            Result: {}\n\n\
            A detailed match report is available in the match section.",
            home_club, home_goals, away_goals, away_club, result_str
        )
    } else {
        format!(
            "{} {} - {} {}",
            home_club, home_goals, away_goals, away_club
        )
    };

    InboxMessage::new(date, MessageCategory::Match, subject, body)
}

/// Generate injury report message.
pub fn injury_report(
    date: NaiveDate,
    player_name: &str,
    injury_type: &str,
    days_out: u16,
) -> InboxMessage {
    let subject = format!("Injury: {} - {} days", player_name, days_out);
    let body = format!(
        "Medical Report\n\n\
        Player: {}\n\
        Injury: {}\n\
        Expected Recovery: {} days\n\n\
        The medical team will monitor the situation and provide updates.",
        player_name, injury_type, days_out
    );

    InboxMessage::new(date, MessageCategory::Injury, subject, body)
}

/// Generate injury recovery message.
pub fn injury_recovered(date: NaiveDate, player_name: &str) -> InboxMessage {
    InboxMessage::new(
        date,
        MessageCategory::Injury,
        format!("{} has recovered", player_name),
        format!(
            "{} has fully recovered from injury and is available for selection.",
            player_name
        ),
    )
}

/// Generate transfer offer received message.
pub fn transfer_offer_received(
    date: NaiveDate,
    player_name: &str,
    from_club: &str,
    fee: Money,
) -> InboxMessage {
    InboxMessage::new(
        date,
        MessageCategory::Transfer,
        format!("Transfer Offer: {}", player_name),
        format!(
            "Transfer Offer Received\n\n\
            Player: {}\n\
            From: {}\n\
            Offered Fee: {}\n\n\
            Please respond to this offer in the transfers section.",
            player_name, from_club, fee
        ),
    )
}

/// Generate transfer completed message.
pub fn transfer_completed(
    date: NaiveDate,
    player_name: &str,
    to_club: &str,
    fee: Money,
    is_incoming: bool,
) -> InboxMessage {
    let direction = if is_incoming { "Signing" } else { "Sale" };
    InboxMessage::new(
        date,
        MessageCategory::Transfer,
        format!("Transfer Complete: {}", player_name),
        format!(
            "Transfer {}\n\n\
            Player: {}\n\
            Club: {}\n\
            Fee: {}\n\n\
            All paperwork has been completed.",
            direction, player_name, to_club, fee
        ),
    )
}

/// Generate contract expiring message.
pub fn contract_expiring(date: NaiveDate, player_name: &str, months_remaining: u8) -> InboxMessage {
    InboxMessage::new(
        date,
        MessageCategory::Contract,
        format!("Contract Expiring: {}", player_name),
        format!(
            "{}'s contract expires in {} months.\n\n\
            Consider negotiating a new contract or listing for transfer.",
            player_name, months_remaining
        ),
    )
}

/// Generate contract renewal message.
pub fn contract_renewed(date: NaiveDate, player_name: &str, years: u8) -> InboxMessage {
    InboxMessage::new(
        date,
        MessageCategory::Contract,
        format!("Contract Renewed: {}", player_name),
        format!(
            "{} has signed a new {}-year contract with the club.",
            player_name, years
        ),
    )
}

/// Generate board expectation message.
pub fn board_expectations(
    date: NaiveDate,
    club_name: &str,
    league_target: &str,
    cup_target: &str,
) -> InboxMessage {
    InboxMessage::new(
        date,
        MessageCategory::Board,
        format!("Season Expectations - {}", club_name),
        format!(
            "The board has set the following expectations for this season:\n\n\
            League: {}\n\
            Cup: {}\n\n\
            Good luck!",
            league_target, cup_target
        ),
    )
}

/// Generate board confidence message.
pub fn board_confidence(date: NaiveDate, confidence_level: &str, reason: &str) -> InboxMessage {
    InboxMessage::new(
        date,
        MessageCategory::Board,
        format!("Board Confidence: {}", confidence_level),
        format!(
            "The board's confidence in you is currently {}.\n\n\
            Reason: {}",
            confidence_level, reason
        ),
    )
}

/// Generate monthly financial report message.
pub fn monthly_financial_report(
    date: NaiveDate,
    income: Money,
    expenses: Money,
    balance: Money,
) -> InboxMessage {
    let net = income - expenses;
    let status = if net.is_negative() { "Loss" } else { "Profit" };

    InboxMessage::new(
        date,
        MessageCategory::Other,
        format!("Monthly Financial Report - {}", date.format("%B %Y")),
        format!(
            "Financial Summary\n\n\
            Income: {}\n\
            Expenses: {}\n\
            Net {}: {}\n\
            Current Balance: {}",
            income,
            expenses,
            status,
            net.abs(),
            balance
        ),
    )
}

/// Generate youth academy graduate message.
pub fn youth_graduate(
    date: NaiveDate,
    player_name: &str,
    position: &str,
    potential: u8,
) -> InboxMessage {
    let potential_str = match potential {
        80..=100 => "exceptional",
        70..=79 => "very good",
        60..=69 => "good",
        50..=59 => "decent",
        _ => "average",
    };

    InboxMessage::new(
        date,
        MessageCategory::Training,
        format!("Youth Graduate: {}", player_name),
        format!(
            "A new talent has graduated from the youth academy!\n\n\
            Name: {}\n\
            Position: {}\n\
            Potential: {} ({})\n\n\
            Consider giving them some first-team opportunities.",
            player_name, position, potential, potential_str
        ),
    )
}

/// Generate press conference message.
pub fn press_conference_request(date: NaiveDate, topic: &str) -> InboxMessage {
    InboxMessage::new(
        date,
        MessageCategory::Press,
        "Press Conference Requested",
        format!(
            "The press has requested a conference regarding: {}\n\n\
            Would you like to attend?",
            topic
        ),
    )
}

/// Generate match preview message.
pub fn match_preview(
    date: NaiveDate,
    opponent: &str,
    competition: &str,
    venue: &str,
) -> InboxMessage {
    InboxMessage::new(
        date,
        MessageCategory::Match,
        format!("Match Preview: vs {}", opponent),
        format!(
            "Upcoming Match\n\n\
            Opponent: {}\n\
            Competition: {}\n\
            Venue: {}\n\n\
            Make sure to check your squad and tactics before the match.",
            opponent, competition, venue
        ),
    )
}

/// Generate season end summary message.
/// Uses Urgent priority since season results (promotion/relegation/title) are critical.
pub fn season_end(date: NaiveDate, season: &str, league_position: u8, points: u16) -> InboxMessage {
    InboxMessage::with_priority(
        date,
        MessageCategory::Other,
        format!("Season End: {}", season),
        format!(
            "The {} season has concluded.\n\n\
            Final League Position: {}\n\
            Points: {}\n\n\
            Review your season performance and prepare for the next campaign.",
            season, league_position, points
        ),
        MessagePriority::Urgent,
    )
}

/// Generate new season message.
pub fn new_season(date: NaiveDate, season: &str) -> InboxMessage {
    InboxMessage::new(
        date,
        MessageCategory::Other,
        format!("New Season: {}", season),
        format!(
            "The {} season is about to begin!\n\n\
            Pre-season preparations are underway. Check the fixture list \
            and make any necessary transfers before the window closes.",
            season
        ),
    )
}

/// Generate transfer window message.
pub fn transfer_window_status(date: NaiveDate, is_opening: bool) -> InboxMessage {
    let (subject, body) = if is_opening {
        (
            "Transfer Window Opens",
            "The transfer window is now open. You may buy and sell players.",
        )
    } else {
        (
            "Transfer Window Closes",
            "The transfer window has closed. No more transfers until the next window.",
        )
    };

    InboxMessage::new(date, MessageCategory::Transfer, subject, body)
}

/// Tipos de rumor de transferencia.
#[derive(Debug, Clone, Copy)]
pub enum RumorType {
    /// Jogador sendo observado por um clube.
    BeingWatched,
    /// Clube prepara oferta.
    PreparingBid,
    /// Jogador quer sair.
    WantsToLeave,
}

/// Generate a transfer rumor message.
pub fn transfer_rumor(
    date: NaiveDate,
    player_name: &str,
    player_club: &str,
    interested_club: &str,
    rumor_type: RumorType,
) -> InboxMessage {
    let (subject, body) = match rumor_type {
        RumorType::BeingWatched => (
            format!("Rumor: {} sendo observado", player_name),
            format!(
                "{} esta sendo observado por {}.\n\n\
                Olheiros do clube foram vistos acompanhando as ultimas partidas do jogador.",
                player_name, interested_club
            ),
        ),
        RumorType::PreparingBid => (
            format!("Rumor: Oferta por {}", player_name),
            format!(
                "{} prepara oferta por {}, atualmente no {}.\n\n\
                Fontes indicam que o clube esta disposto a investir pesado na contratacao.",
                interested_club, player_name, player_club
            ),
        ),
        RumorType::WantsToLeave => (
            format!("Rumor: {} quer sair", player_name),
            format!(
                "Rumores indicam que {} quer sair de {}.\n\n\
                O jogador estaria insatisfeito com seu papel no time e busca novos desafios.",
                player_name, player_club
            ),
        ),
    };

    InboxMessage::new(date, MessageCategory::Transfer, subject, body)
}

/// Gera rumores de transferencia aleatorios.
///
/// Seleciona jogadores de clubes notaveis e gera rumores variados.
/// `seed` e usado para determinismo (normalmente dias_played ou similar).
pub fn generate_random_rumors(
    date: NaiveDate,
    player_names: &[(String, String)], // (player_name, player_club)
    club_names: &[String],
    seed: u32,
) -> Vec<InboxMessage> {
    let mut rumors = Vec::new();

    if player_names.is_empty() || club_names.is_empty() {
        return rumors;
    }

    // Usar seed para selecionar jogador e tipo de rumor deterministicamente.
    let player_idx = (seed as usize) % player_names.len();
    let club_idx = ((seed as usize) * 7 + 3) % club_names.len();
    let rumor_type_idx = (seed as usize) % 3;

    let (ref player_name, ref player_club) = player_names[player_idx];
    let interested_club = &club_names[club_idx];

    // Evitar rumor sobre jogador indo para o proprio clube.
    if interested_club == player_club {
        return rumors;
    }

    let rumor_type = match rumor_type_idx {
        0 => RumorType::BeingWatched,
        1 => RumorType::PreparingBid,
        _ => RumorType::WantsToLeave,
    };

    rumors.push(transfer_rumor(
        date,
        player_name,
        player_club,
        interested_club,
        rumor_type,
    ));

    rumors
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()
    }

    #[test]
    fn test_welcome_message() {
        let msg = welcome_message("John", "Liverpool");
        assert!(msg.contains("John"));
        assert!(msg.contains("Liverpool"));
    }

    #[test]
    fn test_welcome_inbox() {
        let msg = welcome_inbox(test_date(), "John", "Liverpool");
        assert_eq!(msg.category, MessageCategory::Board);
        assert!(msg.subject.contains("Liverpool"));
        assert!(msg.body.contains("John"));
        assert!(!msg.read);
    }

    #[test]
    fn test_match_result_victory() {
        let msg = match_result(test_date(), "Liverpool", "Manchester", 3, 1, true);
        assert_eq!(msg.category, MessageCategory::Match);
        assert!(msg.body.contains("Victory"));
    }

    #[test]
    fn test_match_result_defeat() {
        let msg = match_result(test_date(), "Liverpool", "Manchester", 0, 2, true);
        assert!(msg.body.contains("Defeat"));
    }

    #[test]
    fn test_match_result_draw() {
        let msg = match_result(test_date(), "Liverpool", "Manchester", 1, 1, true);
        assert!(msg.body.contains("Draw"));
    }

    #[test]
    fn test_injury_report() {
        let msg = injury_report(test_date(), "Steven Gerrard", "Hamstring strain", 14);
        assert_eq!(msg.category, MessageCategory::Injury);
        assert!(msg.subject.contains("14 days"));
        assert!(msg.body.contains("Steven Gerrard"));
    }

    #[test]
    fn test_injury_recovered() {
        let msg = injury_recovered(test_date(), "Steven Gerrard");
        assert!(msg.subject.contains("recovered"));
    }

    #[test]
    fn test_transfer_offer() {
        let fee = Money::from_major(10_000_000);
        let msg = transfer_offer_received(test_date(), "Player", "Real Madrid", fee);
        assert_eq!(msg.category, MessageCategory::Transfer);
    }

    #[test]
    fn test_contract_expiring() {
        let msg = contract_expiring(test_date(), "Player", 6);
        assert_eq!(msg.category, MessageCategory::Contract);
        assert!(msg.body.contains("6 months"));
    }

    #[test]
    fn test_board_expectations() {
        let msg = board_expectations(test_date(), "Liverpool", "Top 4", "Quarter Finals");
        assert_eq!(msg.category, MessageCategory::Board);
        assert!(msg.body.contains("Top 4"));
    }

    #[test]
    fn test_monthly_report() {
        let income = Money::from_major(1_000_000);
        let expenses = Money::from_major(800_000);
        let balance = Money::from_major(5_000_000);
        let msg = monthly_financial_report(test_date(), income, expenses, balance);
        assert!(msg.body.contains("Profit"));
    }

    #[test]
    fn test_youth_graduate() {
        let msg = youth_graduate(test_date(), "Young Player", "Midfielder", 85);
        assert_eq!(msg.category, MessageCategory::Training);
        assert!(msg.body.contains("exceptional"));
    }

    #[test]
    fn test_transfer_window() {
        let open_msg = transfer_window_status(test_date(), true);
        assert!(open_msg.subject.contains("Opens"));

        let close_msg = transfer_window_status(test_date(), false);
        assert!(close_msg.subject.contains("Closes"));
    }

    #[test]
    fn test_message_id_unique() {
        let msg1 = welcome_inbox(test_date(), "A", "B");
        std::thread::sleep(std::time::Duration::from_nanos(1));
        let msg2 = welcome_inbox(test_date(), "A", "B");
        assert_ne!(msg1.id, msg2.id);
    }

    #[test]
    fn test_transfer_rumor_being_watched() {
        let msg = transfer_rumor(
            test_date(),
            "Ronaldinho",
            "Flamengo",
            "Barcelona",
            RumorType::BeingWatched,
        );
        assert_eq!(msg.category, MessageCategory::Transfer);
        assert!(msg.subject.contains("Ronaldinho"));
        assert!(msg.body.contains("Barcelona"));
    }

    #[test]
    fn test_transfer_rumor_preparing_bid() {
        let msg = transfer_rumor(
            test_date(),
            "Neymar",
            "Santos",
            "Real Madrid",
            RumorType::PreparingBid,
        );
        assert!(msg.body.contains("Real Madrid"));
        assert!(msg.body.contains("Neymar"));
        assert!(msg.body.contains("Santos"));
    }

    #[test]
    fn test_transfer_rumor_wants_to_leave() {
        let msg = transfer_rumor(
            test_date(),
            "Kaka",
            "Milan",
            "Chelsea",
            RumorType::WantsToLeave,
        );
        assert!(msg.body.contains("Kaka"));
        assert!(msg.body.contains("Milan"));
    }

    #[test]
    fn test_generate_random_rumors() {
        let players = vec![
            ("Jogador A".to_string(), "Clube A".to_string()),
            ("Jogador B".to_string(), "Clube B".to_string()),
        ];
        let clubs = vec!["Clube C".to_string(), "Clube D".to_string()];
        let rumors = generate_random_rumors(test_date(), &players, &clubs, 42);
        assert!(!rumors.is_empty());
        assert_eq!(rumors[0].category, MessageCategory::Transfer);
    }

    #[test]
    fn test_generate_random_rumors_empty() {
        let rumors = generate_random_rumors(test_date(), &[], &[], 0);
        assert!(rumors.is_empty());
    }
}
