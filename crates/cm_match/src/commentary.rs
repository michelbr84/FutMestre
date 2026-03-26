//! Match commentary generation (stub).

/// Generate commentary for a goal.
pub fn goal_commentary(minute: u8, scorer: &str, home: bool) -> String {
    let location = if home { "mandante" } else { "visitante" };
    format!(
        "{}' GOOOL! {} marca para o {}!",
        minute, scorer, location
    )
}

/// Generate commentary for a save.
pub fn save_commentary(minute: u8, keeper: &str) -> String {
    format!("{}' Grande defesa de {}!", minute, keeper)
}

/// Generate commentary for a card.
pub fn card_commentary(minute: u8, player: &str, yellow: bool) -> String {
    let card_type = if yellow { "amarelo" } else { "vermelho" };
    format!(
        "{}' {} recebe cartao {}!",
        minute, player, card_type
    )
}

/// Generate half-time commentary.
pub fn halftime_commentary(home_goals: u8, away_goals: u8) -> String {
    format!(
        "Intervalo: O placar e {} - {}.",
        home_goals, away_goals
    )
}

/// Generate full-time commentary.
pub fn fulltime_commentary(
    home_goals: u8,
    away_goals: u8,
    home_name: &str,
    away_name: &str,
) -> String {
    let result = if home_goals > away_goals {
        format!("{} vence!", home_name)
    } else if away_goals > home_goals {
        format!("{} vence!", away_name)
    } else {
        "Empate!".to_string()
    };

    format!(
        "Fim de jogo: {} {} - {} {}. {}",
        home_name, home_goals, away_goals, away_name, result
    )
}

/// Generate injury commentary.
pub fn injury_commentary(minute: u8, player: &str) -> String {
    format!("{}' {} sofre uma lesao e precisa sair!", minute, player)
}

/// Generate foul commentary.
pub fn foul_commentary(minute: u8, player: &str) -> String {
    format!("{}' Falta cometida por {}.", minute, player)
}

/// Generate corner commentary.
pub fn corner_commentary(minute: u8, home: bool) -> String {
    let side = if home { "mandante" } else { "visitante" };
    format!("{}' Escanteio para o {}.", minute, side)
}

/// Generate penalty commentary.
pub fn penalty_commentary(minute: u8, scored: bool) -> String {
    if scored {
        format!("{}' GOOOL de penalti!", minute)
    } else {
        format!("{}' Penalti perdido!", minute)
    }
}

/// Generate extra time commentary.
pub fn extra_time_commentary() -> String {
    "Prorrogacao! Mais 30 minutos serao disputados.".to_string()
}

/// Generate penalty shootout header.
pub fn penalty_shootout_commentary() -> String {
    "Disputa de penaltis!".to_string()
}
