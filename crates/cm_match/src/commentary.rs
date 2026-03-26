//! Match commentary generation — narração rica estilo CM/Elifoot.
//!
//! Múltiplos templates por evento, selecionados via seed/minuto para variedade.

/// Seleciona um template baseado no minuto para variedade sem RNG extra.
fn pick<'a>(minute: u8, options: &[&'a str]) -> &'a str {
    options[(minute as usize) % options.len()]
}

// ─── Atmosfera ──────────────────────────────────────────────────────────────

/// Texto de abertura da partida.
pub fn kickoff_commentary(home: &str, away: &str) -> String {
    format!(
        "O arbitro apita e a bola rola! {} recebe {} nesta partida!",
        home, away
    )
}

/// Narração de atmosfera baseada no minuto (preenche períodos sem eventos).
pub fn atmosphere_commentary(minute: u8, home: &str, away: &str) -> Option<String> {
    match minute {
        5 => Some(format!(
            "Inicio cauteloso. {} e {} se estudam no meio-campo.",
            home, away
        )),
        15 => Some("A partida vai ganhando ritmo. A torcida comeca a empurrar.".into()),
        25 => Some("Jogo equilibrado neste primeiro tempo. Os dois times criam jogadas.".into()),
        35 => Some("A pressao aumenta com o intervalo se aproximando.".into()),
        44 => Some("Ultimo minuto do primeiro tempo...".into()),
        50 => Some("Segundo tempo comecando com intensidade!".into()),
        60 => Some("Uma hora de jogo. As pernas comecam a pesar.".into()),
        70 => Some("Reta final se aproximando. A tensao aumenta nas arquibancadas.".into()),
        80 => Some("Dez minutos para o fim! A torcida esta de pe!".into()),
        85 => Some("Ultimos minutos! Coracao acelerado nas duas torcidas.".into()),
        89 => Some("Minuto final do tempo regulamentar...".into()),
        _ => None,
    }
}

// ─── Gols ───────────────────────────────────────────────────────────────────

/// Narração de gol com múltiplas variações.
pub fn goal_commentary(minute: u8, scorer: &str, team: &str) -> String {
    let templates: &[&str] = &[
        "GOOOOOL! {} marca para o {}! A torcida explode!",
        "GOOOL! {} balanca as redes pelo {}! Que momento!",
        "E GOOOL! {} nao perdoa e faz o gol do {}!",
        "GOLAAAAACO! {} marca um golaço pelo {}!",
        "GOL! {} amplia para o {}! A torcida vai a loucura!",
        "GOL GOL GOL! {} faz mais um para o {}!",
    ];
    let template = pick(minute, templates);
    format!(
        "{}' {}",
        minute,
        template.replacen("{}", scorer, 1).replacen("{}", team, 1)
    )
}

/// Narração de gol de falta.
pub fn freekick_goal_commentary(minute: u8, team: &str) -> String {
    let templates: &[&str] = &[
        "GOOOL DE FALTA! {} marca em cobranca magistral!",
        "GOLAACO DE FALTA! A bola entra no angulo pelo {}!",
        "GOL! Cobranca perfeita de falta para o {}!",
    ];
    let template = pick(minute, templates);
    format!("{}' {}", minute, template.replacen("{}", team, 1))
}

/// Narração de gol de escanteio.
pub fn corner_goal_commentary(minute: u8, team: &str) -> String {
    let templates: &[&str] = &[
        "GOOOL! Na cobranca de escanteio, {} marca de cabeca!",
        "GOL! Escanteio perfeito e {} aproveita na area!",
        "GOOOL! Bola na area e {} nao perdoa!",
    ];
    let template = pick(minute, templates);
    format!("{}' {}", minute, template.replacen("{}", team, 1))
}

// ─── Defesas ────────────────────────────────────────────────────────────────

/// Narração de grande defesa.
pub fn save_commentary(minute: u8, keeper: &str) -> String {
    let templates: &[&str] = &[
        "Grande defesa de {}! O goleiro salva o time!",
        "Que defesa de {}! A bola ia no angulo!",
        "{} se estica todo e faz uma defesaca!",
    ];
    let template = pick(minute, templates);
    format!("{}' {}", minute, template.replacen("{}", keeper, 1))
}

// ─── Cartoes ────────────────────────────────────────────────────────────────

/// Narração de cartão.
pub fn card_commentary(minute: u8, player: &str, yellow: bool) -> String {
    if yellow {
        let templates: &[&str] = &[
            "{} recebe cartao AMARELO! Falta dura no meio-campo.",
            "AMARELO para {}! O arbitro nao perdoa.",
            "Cartao amarelo! {} entra no caderninho do juiz.",
        ];
        let template = pick(minute, templates);
        format!("{}' {}", minute, template.replacen("{}", player, 1))
    } else {
        let templates: &[&str] = &[
            "CARTAO VERMELHO! {} e expulso! Falta violenta!",
            "VERMELHO! {} esta fora do jogo! A torcida protesta!",
            "Expulsao! {} recebe o vermelho e deixa o time com um a menos!",
        ];
        let template = pick(minute, templates);
        format!("{}' {}", minute, template.replacen("{}", player, 1))
    }
}

// ─── Intervalo e Fim ────────────────────────────────────────────────────────

/// Narração do intervalo.
pub fn halftime_commentary(home_goals: u8, away_goals: u8, home: &str, away: &str) -> String {
    let situation = if home_goals > away_goals {
        format!("{} vai vencendo!", home)
    } else if away_goals > home_goals {
        format!("{} vai vencendo!", away)
    } else {
        "Tudo igual ate aqui!".into()
    };
    format!(
        "INTERVALO! {} {} x {} {}. {}",
        home, home_goals, away_goals, away, situation
    )
}

/// Narração de fim de jogo.
pub fn fulltime_commentary(
    home_goals: u8,
    away_goals: u8,
    home_name: &str,
    away_name: &str,
) -> String {
    let result = if home_goals > away_goals {
        format!("VITORIA do {}!", home_name)
    } else if away_goals > home_goals {
        format!("VITORIA do {}!", away_name)
    } else {
        "EMPATE! Os dois times ficam no X.".into()
    };
    format!(
        "APITA O ARBITRO! FIM DE JOGO! {} {} x {} {}. {}",
        home_name, home_goals, away_goals, away_name, result
    )
}

// ─── Lesoes ─────────────────────────────────────────────────────────────────

/// Narração de lesão.
pub fn injury_commentary(minute: u8, player: &str) -> String {
    let templates: &[&str] = &[
        "{} fica caido no gramado! Parece lesao serio.",
        "{} sente dores e pede substituicao. Sai de maca!",
        "Preocupacao! {} se machuca e nao consegue continuar.",
    ];
    let template = pick(minute, templates);
    format!("{}' {}", minute, template.replacen("{}", player, 1))
}

// ─── Faltas e Escanteios ────────────────────────────────────────────────────

/// Narração de falta.
pub fn foul_commentary(minute: u8, player: &str) -> String {
    let templates: &[&str] = &[
        "Falta de {}! O juiz marca.",
        "Entrada forte de {}! Falta marcada.",
        "{} comete falta. Jogo parado.",
    ];
    let template = pick(minute, templates);
    format!("{}' {}", minute, template.replacen("{}", player, 1))
}

/// Narração de escanteio.
pub fn corner_commentary(minute: u8, team: &str) -> String {
    let templates: &[&str] = &[
        "Escanteio para o {}! Bola na area...",
        "Canto para o {}. A defesa se posiciona.",
        "Escanteio cobrado pelo {}!",
    ];
    let template = pick(minute, templates);
    format!("{}' {}", minute, template.replacen("{}", team, 1))
}

// ─── Penaltis ───────────────────────────────────────────────────────────────

/// Narração de pênalti.
pub fn penalty_commentary(minute: u8, scored: bool) -> String {
    if scored {
        let templates: &[&str] = &[
            "PENALTI CONVERTIDO! GOL! A bola entra sem chance para o goleiro!",
            "GOOOL DE PENALTI! Batida firme no canto!",
            "PENALTI! GOL! O cobrador nao treme e marca!",
        ];
        let template = pick(minute, templates);
        format!("{}' {}", minute, template)
    } else {
        let templates: &[&str] = &[
            "PENALTI PERDIDO! O goleiro defende! Que momento!",
            "ERROU! A cobranca vai para fora! Que desperdicio!",
            "DEFENDEU! O goleiro adivinha o canto e salva!",
        ];
        let template = pick(minute, templates);
        format!("{}' {}", minute, template)
    }
}

// ─── Prorrogação e Disputa ──────────────────────────────────────────────────

/// Narração de início de prorrogação.
pub fn extra_time_commentary() -> String {
    "PRORROGACAO! A partida nao terminou! Mais 30 minutos de emocao!".into()
}

/// Narração de início de disputa de pênaltis.
pub fn penalty_shootout_commentary() -> String {
    "DISPUTA DE PENALTIS! Hora da verdade! Quem vai ter mais nervos de aco?".into()
}

// ─── Estatísticas formatadas ────────────────────────────────────────────────

/// Gera um resumo de estatísticas para exibição.
pub fn stats_summary(
    home: &str,
    away: &str,
    home_poss: f64,
    away_poss: f64,
    home_shots: u32,
    away_shots: u32,
    home_on_target: u32,
    away_on_target: u32,
    home_fouls: u32,
    away_fouls: u32,
    home_corners: u32,
    away_corners: u32,
    home_yellows: u32,
    away_yellows: u32,
    home_reds: u32,
    away_reds: u32,
) -> Vec<String> {
    vec![
        format!("{:>20}   {:^12}   {:<20}", home, "ESTATISTICAS", away),
        format!(
            "{:>20}   {:^12}   {:<20}",
            format!("{:.0}%", home_poss * 100.0),
            "Posse",
            format!("{:.0}%", away_poss * 100.0)
        ),
        format!(
            "{:>20}   {:^12}   {:<20}",
            home_shots, "Finalizacoes", away_shots
        ),
        format!(
            "{:>20}   {:^12}   {:<20}",
            home_on_target, "No gol", away_on_target
        ),
        format!("{:>20}   {:^12}   {:<20}", home_fouls, "Faltas", away_fouls),
        format!(
            "{:>20}   {:^12}   {:<20}",
            home_corners, "Escanteios", away_corners
        ),
        format!(
            "{:>20}   {:^12}   {:<20}",
            home_yellows, "Amarelos", away_yellows
        ),
        format!(
            "{:>20}   {:^12}   {:<20}",
            home_reds, "Vermelhos", away_reds
        ),
    ]
}
