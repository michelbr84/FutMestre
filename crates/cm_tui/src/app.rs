//! TUI application logic - complete football manager interface.

use chrono::NaiveDate;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Tabs, Wrap};

use cm_core::ids::{ClubId, CompetitionId};
use cm_core::world::{
    CompetitionType, Formation, Mentality, Player, Position, Tempo,
    World,
};
use cm_data::import::JsonWorldImporter;
use cm_engine::{Game, GameConfig, GameState};

// ─── App State ──────────────────────────────────────────────────────────────

/// Top-level application state.
pub enum AppScreen {
    MainMenu(MainMenuState),
    NewGame(NewGameState),
    InGame(InGameState),
    Settings(SettingsState),
    Quit,
}

pub struct MainMenuState {
    pub selected: usize,
}

pub struct NewGameState {
    pub club_index: usize,
    pub manager_name: String,
    pub editing_name: bool,
    pub available_clubs: Vec<(ClubId, String, String, u8)>, // id, name, division, reputation
}

pub struct InGameState {
    pub game: Game,
    pub tab: GameTab,
    pub squad_scroll: usize,
    pub squad_selected: usize,
    pub inbox_scroll: usize,
    pub standings_division: usize,
    pub tactics_field: usize,
    pub training_field: usize,
    pub match_live: Option<MatchLiveState>,
    pub show_popup: Option<String>,
}

pub struct MatchLiveState {
    pub events: Vec<String>,
    pub home_name: String,
    pub away_name: String,
    pub home_goals: u8,
    pub away_goals: u8,
    pub minute: u8,
    pub finished: bool,
}

pub struct SettingsState {
    pub selected: usize,
    pub language: String,
    pub currency: String,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameTab {
    Squad,
    Tactics,
    Training,
    Fixtures,
    Standings,
    Finance,
    Transfers,
    Inbox,
}

impl GameTab {
    fn all() -> &'static [GameTab] {
        &[
            GameTab::Squad,
            GameTab::Tactics,
            GameTab::Training,
            GameTab::Fixtures,
            GameTab::Standings,
            GameTab::Finance,
            GameTab::Transfers,
            GameTab::Inbox,
        ]
    }

    fn name(&self) -> &'static str {
        match self {
            GameTab::Squad => "Elenco",
            GameTab::Tactics => "Taticas",
            GameTab::Training => "Treino",
            GameTab::Fixtures => "Jogos",
            GameTab::Standings => "Classificacao",
            GameTab::Finance => "Financas",
            GameTab::Transfers => "Transferencias",
            GameTab::Inbox => "Noticias",
        }
    }

    fn index(&self) -> usize {
        GameTab::all().iter().position(|t| t == self).unwrap_or(0)
    }

    fn from_index(i: usize) -> Self {
        *GameTab::all().get(i).unwrap_or(&GameTab::Squad)
    }
}

// ─── Run ────────────────────────────────────────────────────────────────────

/// Run the TUI application.
pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> anyhow::Result<()> {
    let mut screen = AppScreen::MainMenu(MainMenuState { selected: 0 });

    loop {
        terminal.draw(|f| render(f, &screen))?;

        if let Event::Key(key) = event::read()? {
            screen = handle_input(screen, key.code, key.modifiers);
        }

        if matches!(screen, AppScreen::Quit) {
            break;
        }
    }

    Ok(())
}

// ─── Input Handling ─────────────────────────────────────────────────────────

fn handle_input(screen: AppScreen, key: KeyCode, _mods: KeyModifiers) -> AppScreen {
    match screen {
        AppScreen::MainMenu(mut st) => handle_main_menu(key, &mut st),
        AppScreen::NewGame(mut st) => handle_new_game(key, &mut st),
        AppScreen::InGame(mut st) => handle_in_game(key, &mut st),
        AppScreen::Settings(mut st) => handle_settings(key, &mut st),
        AppScreen::Quit => AppScreen::Quit,
    }
}

fn handle_main_menu(key: KeyCode, st: &mut MainMenuState) -> AppScreen {
    match key {
        KeyCode::Up => {
            st.selected = st.selected.saturating_sub(1);
            AppScreen::MainMenu(MainMenuState { selected: st.selected })
        }
        KeyCode::Down => {
            st.selected = (st.selected + 1).min(2);
            AppScreen::MainMenu(MainMenuState { selected: st.selected })
        }
        KeyCode::Enter => match st.selected {
            0 => {
                // New Game - load world and show club selection
                let world = load_world();
                let mut clubs: Vec<(ClubId, String, String, u8)> = Vec::new();

                // Sort clubs by division then reputation
                let mut sorted: Vec<_> = world.clubs.values().collect();
                sorted.sort_by(|a, b| b.reputation.cmp(&a.reputation));

                for club in sorted {
                    let div = find_club_division(&world, &club.id);
                    clubs.push((
                        club.id.clone(),
                        club.name.clone(),
                        div,
                        club.reputation,
                    ));
                }

                AppScreen::NewGame(NewGameState {
                    club_index: 0,
                    manager_name: String::new(),
                    editing_name: true,
                    available_clubs: clubs,
                })
            }
            1 => {
                // Settings
                AppScreen::Settings(SettingsState {
                    selected: 0,
                    language: "Portugues".into(),
                    currency: "R$ (Real)".into(),
                })
            }
            2 => AppScreen::Quit,
            _ => AppScreen::MainMenu(MainMenuState { selected: st.selected }),
        },
        KeyCode::Char('q') => AppScreen::Quit,
        _ => AppScreen::MainMenu(MainMenuState { selected: st.selected }),
    }
}

fn handle_new_game(key: KeyCode, st: &mut NewGameState) -> AppScreen {
    if st.editing_name {
        match key {
            KeyCode::Enter => {
                if st.manager_name.is_empty() {
                    st.manager_name = "Tecnico".into();
                }
                st.editing_name = false;
                return AppScreen::NewGame(std::mem::replace(st, NewGameState {
                    club_index: 0,
                    manager_name: String::new(),
                    editing_name: true,
                    available_clubs: Vec::new(),
                }));
            }
            KeyCode::Char(c) => {
                st.manager_name.push(c);
                return AppScreen::NewGame(std::mem::replace(st, NewGameState {
                    club_index: 0,
                    manager_name: String::new(),
                    editing_name: true,
                    available_clubs: Vec::new(),
                }));
            }
            KeyCode::Backspace => {
                st.manager_name.pop();
                return AppScreen::NewGame(std::mem::replace(st, NewGameState {
                    club_index: 0,
                    manager_name: String::new(),
                    editing_name: true,
                    available_clubs: Vec::new(),
                }));
            }
            KeyCode::Esc => return AppScreen::MainMenu(MainMenuState { selected: 0 }),
            _ => {}
        }
        return AppScreen::NewGame(std::mem::replace(st, NewGameState {
            club_index: 0,
            manager_name: String::new(),
            editing_name: true,
            available_clubs: Vec::new(),
        }));
    }

    match key {
        KeyCode::Up => {
            st.club_index = st.club_index.saturating_sub(1);
        }
        KeyCode::Down => {
            st.club_index = (st.club_index + 1).min(st.available_clubs.len().saturating_sub(1));
        }
        KeyCode::PageUp => {
            st.club_index = st.club_index.saturating_sub(10);
        }
        KeyCode::PageDown => {
            st.club_index = (st.club_index + 10).min(st.available_clubs.len().saturating_sub(1));
        }
        KeyCode::Enter => {
            if let Some((club_id, _, _, _)) = st.available_clubs.get(st.club_index) {
                let manager_name = st.manager_name.clone();
                let club_id = club_id.clone();
                return start_game(manager_name, club_id);
            }
        }
        KeyCode::Esc => return AppScreen::MainMenu(MainMenuState { selected: 0 }),
        _ => {}
    }

    AppScreen::NewGame(std::mem::replace(st, NewGameState {
        club_index: 0,
        manager_name: String::new(),
        editing_name: true,
        available_clubs: Vec::new(),
    }))
}

fn handle_in_game(key: KeyCode, st: &mut InGameState) -> AppScreen {
    // Handle popup dismissal
    if st.show_popup.is_some() {
        if matches!(key, KeyCode::Enter | KeyCode::Esc) {
            st.show_popup = None;
        }
        return AppScreen::InGame(std::mem::replace(st, dummy_in_game()));
    }

    // Handle live match
    if let Some(ref mut live) = st.match_live {
        if live.finished {
            if matches!(key, KeyCode::Enter | KeyCode::Esc) {
                st.match_live = None;
            }
        }
        return AppScreen::InGame(std::mem::replace(st, dummy_in_game()));
    }

    match key {
        KeyCode::Tab => {
            let idx = (st.tab.index() + 1) % GameTab::all().len();
            st.tab = GameTab::from_index(idx);
        }
        KeyCode::BackTab => {
            let idx = if st.tab.index() == 0 {
                GameTab::all().len() - 1
            } else {
                st.tab.index() - 1
            };
            st.tab = GameTab::from_index(idx);
        }
        KeyCode::Char('1') => st.tab = GameTab::Squad,
        KeyCode::Char('2') => st.tab = GameTab::Tactics,
        KeyCode::Char('3') => st.tab = GameTab::Training,
        KeyCode::Char('4') => st.tab = GameTab::Fixtures,
        KeyCode::Char('5') => st.tab = GameTab::Standings,
        KeyCode::Char('6') => st.tab = GameTab::Finance,
        KeyCode::Char('7') => st.tab = GameTab::Transfers,
        KeyCode::Char('8') => st.tab = GameTab::Inbox,
        KeyCode::Char(' ') | KeyCode::Char('n') => {
            // Advance day
            st.game.process_day();
        }
        KeyCode::Char('a') => {
            // Advance week (7 days)
            for _ in 0..7 {
                st.game.process_day();
            }
        }
        KeyCode::Esc => {
            return AppScreen::MainMenu(MainMenuState { selected: 0 });
        }
        // Tab-specific input
        _ => {
            handle_tab_input(st, key);
        }
    }

    AppScreen::InGame(std::mem::replace(st, dummy_in_game()))
}

fn handle_tab_input(st: &mut InGameState, key: KeyCode) {
    match st.tab {
        GameTab::Squad => {
            let max = st.game.world().club_players(&st.game.state().club_id).len().saturating_sub(1);
            match key {
                KeyCode::Up => st.squad_selected = st.squad_selected.saturating_sub(1),
                KeyCode::Down => st.squad_selected = (st.squad_selected + 1).min(max),
                _ => {}
            }
        }
        GameTab::Tactics => {
            match key {
                KeyCode::Up => st.tactics_field = st.tactics_field.saturating_sub(1),
                KeyCode::Down => st.tactics_field = (st.tactics_field + 1).min(6),
                KeyCode::Left | KeyCode::Right => {
                    let club_id = st.game.state().club_id.clone();
                    let left = matches!(key, KeyCode::Left);
                    if let Some(club) = st.game.world_mut().clubs.get_mut(&club_id) {
                        match st.tactics_field {
                            0 => {
                                // Formation
                                let formations = [
                                    Formation::F442, Formation::F433, Formation::F352,
                                    Formation::F451, Formation::F4231, Formation::F3412,
                                    Formation::F532, Formation::F4141, Formation::F4411,
                                    Formation::F343,
                                ];
                                let cur = formations.iter().position(|f| *f == club.tactics.formation).unwrap_or(0);
                                let next = if left {
                                    if cur == 0 { formations.len() - 1 } else { cur - 1 }
                                } else {
                                    (cur + 1) % formations.len()
                                };
                                club.tactics.formation = formations[next];
                            }
                            1 => {
                                // Mentality
                                let mentalities = [
                                    Mentality::Defensive, Mentality::Cautious,
                                    Mentality::Balanced, Mentality::Attacking,
                                    Mentality::AllOutAttack,
                                ];
                                let cur = mentalities.iter().position(|m| *m == club.tactics.mentality).unwrap_or(2);
                                let next = if left {
                                    if cur == 0 { mentalities.len() - 1 } else { cur - 1 }
                                } else {
                                    (cur + 1) % mentalities.len()
                                };
                                club.tactics.mentality = mentalities[next];
                            }
                            2 => {
                                // Tempo
                                let tempos = [Tempo::Slow, Tempo::Normal, Tempo::Fast];
                                let cur = tempos.iter().position(|t| *t == club.tactics.tempo).unwrap_or(1);
                                let next = if left {
                                    if cur == 0 { tempos.len() - 1 } else { cur - 1 }
                                } else {
                                    (cur + 1) % tempos.len()
                                };
                                club.tactics.tempo = tempos[next];
                            }
                            3 => {
                                // Pressing
                                if left { club.tactics.pressing = club.tactics.pressing.saturating_sub(10); }
                                else { club.tactics.pressing = (club.tactics.pressing + 10).min(100); }
                            }
                            4 => {
                                // Defensive line
                                if left { club.tactics.defensive_line = club.tactics.defensive_line.saturating_sub(10); }
                                else { club.tactics.defensive_line = (club.tactics.defensive_line + 10).min(100); }
                            }
                            5 => {
                                // Width
                                if left { club.tactics.width = club.tactics.width.saturating_sub(10); }
                                else { club.tactics.width = (club.tactics.width + 10).min(100); }
                            }
                            6 => {
                                // Direct passing
                                if left { club.tactics.direct_passing = club.tactics.direct_passing.saturating_sub(10); }
                                else { club.tactics.direct_passing = (club.tactics.direct_passing + 10).min(100); }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        GameTab::Training => {
            match key {
                KeyCode::Up => st.training_field = st.training_field.saturating_sub(1),
                KeyCode::Down => st.training_field = (st.training_field + 1).min(1),
                KeyCode::Left | KeyCode::Right => {
                    // Cycle training options (simplified - would need Training on GameState)
                }
                _ => {}
            }
        }
        GameTab::Standings => {
            match key {
                KeyCode::Left => st.standings_division = st.standings_division.saturating_sub(1),
                KeyCode::Right => st.standings_division = (st.standings_division + 1).min(3),
                _ => {}
            }
        }
        GameTab::Inbox => {
            let max = st.game.state().inbox.len().saturating_sub(1);
            match key {
                KeyCode::Up => st.inbox_scroll = st.inbox_scroll.saturating_sub(1),
                KeyCode::Down => st.inbox_scroll = (st.inbox_scroll + 1).min(max),
                _ => {}
            }
        }
        _ => {}
    }
}

fn handle_settings(key: KeyCode, st: &mut SettingsState) -> AppScreen {
    match key {
        KeyCode::Up => st.selected = st.selected.saturating_sub(1),
        KeyCode::Down => st.selected = (st.selected + 1).min(1),
        KeyCode::Left | KeyCode::Right => {
            match st.selected {
                0 => {
                    st.language = if st.language == "Portugues" {
                        "English".into()
                    } else {
                        "Portugues".into()
                    };
                }
                1 => {
                    let currencies = ["R$ (Real)", "$ (Dollar)", "EUR (Euro)", "GBP (Libra)"];
                    let cur = currencies.iter().position(|c| *c == st.currency.as_str()).unwrap_or(0);
                    let next = if matches!(key, KeyCode::Left) {
                        if cur == 0 { currencies.len() - 1 } else { cur - 1 }
                    } else {
                        (cur + 1) % currencies.len()
                    };
                    st.currency = currencies[next].into();
                }
                _ => {}
            }
        }
        KeyCode::Esc => return AppScreen::MainMenu(MainMenuState { selected: 0 }),
        _ => {}
    }
    AppScreen::Settings(SettingsState {
        selected: st.selected,
        language: st.language.clone(),
        currency: st.currency.clone(),
    })
}

// ─── Rendering ──────────────────────────────────────────────────────────────

fn render(f: &mut Frame, screen: &AppScreen) {
    let area = f.area();
    // Clear background
    f.render_widget(Clear, area);

    match screen {
        AppScreen::MainMenu(st) => render_main_menu(f, area, st),
        AppScreen::NewGame(st) => render_new_game(f, area, st),
        AppScreen::InGame(st) => render_in_game(f, area, st),
        AppScreen::Settings(st) => render_settings(f, area, st),
        AppScreen::Quit => {}
    }
}

fn render_main_menu(f: &mut Frame, area: Rect, st: &MainMenuState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(8),
            Constraint::Min(6),
            Constraint::Length(3),
        ])
        .split(area);

    // Title
    let title = Paragraph::new(
        "╔══════════════════════════════════════════╗\n\
         ║          FutMestre                       ║\n\
         ║    Football Manager Simulator            ║\n\
         ║                                          ║\n\
         ║    Inspirado em CM 01/02 & Elifoot 98    ║\n\
         ╚══════════════════════════════════════════╝"
    )
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::Cyan));
    f.render_widget(title, chunks[0]);

    // Menu items
    let items = vec!["Novo Jogo", "Configuracoes", "Sair"];
    let menu_items: Vec<ListItem> = items.iter().enumerate().map(|(i, item)| {
        let style = if i == st.selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let prefix = if i == st.selected { ">> " } else { "   " };
        ListItem::new(format!("{}{}", prefix, item)).style(style)
    }).collect();

    let menu = List::new(menu_items)
        .block(Block::default().borders(Borders::ALL).title(" Menu "));
    f.render_widget(menu, chunks[1]);

    // Help
    let help = Paragraph::new("[↑↓] Navegar  [Enter] Selecionar  [Q] Sair")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, chunks[2]);
}

fn render_new_game(f: &mut Frame, area: Rect, st: &NewGameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(area);

    let title = Paragraph::new(" Novo Jogo ")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    if st.editing_name {
        let input = Paragraph::new(format!("Nome do Tecnico: {}|", st.manager_name))
            .block(Block::default().borders(Borders::ALL).title(" Digite seu nome "))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(input, chunks[1]);

        let hint = Paragraph::new("[Enter] Confirmar  [Esc] Voltar")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(hint, chunks[3]);
    } else {
        let name_display = Paragraph::new(format!("Tecnico: {}", st.manager_name))
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Green));
        f.render_widget(name_display, chunks[1]);

        // Club selection table
        let header = Row::new(vec![
            Cell::from("Clube").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Divisao").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Rep").style(Style::default().add_modifier(Modifier::BOLD)),
        ]);

        let visible_start = st.club_index.saturating_sub(10);
        let visible_end = (visible_start + 25).min(st.available_clubs.len());

        let rows: Vec<Row> = st.available_clubs[visible_start..visible_end]
            .iter()
            .enumerate()
            .map(|(i, (_, name, div, rep))| {
                let actual_idx = visible_start + i;
                let style = if actual_idx == st.club_index {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                Row::new(vec![
                    Cell::from(name.as_str()),
                    Cell::from(div.as_str()),
                    Cell::from(format!("{}", rep)),
                ]).style(style)
            })
            .collect();

        let table = Table::new(rows, [
            Constraint::Percentage(50),
            Constraint::Percentage(30),
            Constraint::Percentage(20),
        ])
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(format!(
            " Escolha seu clube ({}/{}) ",
            st.club_index + 1,
            st.available_clubs.len()
        )));
        f.render_widget(table, chunks[2]);

        let hint = Paragraph::new("[↑↓] Navegar  [PgUp/PgDn] Pagina  [Enter] Selecionar  [Esc] Voltar")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(hint, chunks[3]);
    }
}

fn render_in_game(f: &mut Frame, area: Rect, st: &InGameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(1), // Tabs
            Constraint::Min(10),  // Content
            Constraint::Length(2), // Footer
        ])
        .split(area);

    // Header - Club info
    let club_name = st.game.world().clubs.get(&st.game.state().club_id)
        .map(|c| c.name.as_str())
        .unwrap_or("???");
    let balance = st.game.world().clubs.get(&st.game.state().club_id)
        .map(|c| format!("{}", c.budget.balance))
        .unwrap_or_default();
    let date = st.game.state().date.to_string();
    let season = st.game.state().season();
    let div = find_club_division(st.game.world(), &st.game.state().club_id);

    let header = Paragraph::new(format!(
        " {} | {} | {} | {} | Tecnico: {}",
        club_name, div, date, balance, st.game.state().manager_name
    ))
    .block(Block::default().borders(Borders::ALL).title(format!(" FutMestre - Temporada {} ", season)))
    .style(Style::default().fg(Color::Cyan));
    f.render_widget(header, chunks[0]);

    // Tabs
    let tab_titles: Vec<Line> = GameTab::all().iter().enumerate().map(|(i, t)| {
        let style = if *t == st.tab {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        Line::from(format!(" {}.{} ", i + 1, t.name())).style(style)
    }).collect();

    let tabs = Tabs::new(tab_titles)
        .select(st.tab.index())
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow));
    f.render_widget(tabs, chunks[1]);

    // Content area
    match st.tab {
        GameTab::Squad => render_squad(f, chunks[2], st),
        GameTab::Tactics => render_tactics(f, chunks[2], st),
        GameTab::Training => render_training(f, chunks[2], st),
        GameTab::Fixtures => render_fixtures(f, chunks[2], st),
        GameTab::Standings => render_standings(f, chunks[2], st),
        GameTab::Finance => render_finance(f, chunks[2], st),
        GameTab::Transfers => render_transfers(f, chunks[2], st),
        GameTab::Inbox => render_inbox(f, chunks[2], st),
    }

    // Footer
    let footer_text = match st.tab {
        GameTab::Squad => "[↑↓] Selecionar  [Tab] Aba  [Espaco] Avancar dia  [A] Avancar semana",
        GameTab::Tactics => "[↑↓] Campo  [←→] Alterar  [Tab] Aba  [Espaco] Avancar dia",
        GameTab::Training => "[↑↓] Campo  [←→] Alterar  [Tab] Aba  [Espaco] Avancar dia",
        GameTab::Standings => "[←→] Divisao  [Tab] Aba  [Espaco] Avancar dia",
        GameTab::Inbox => "[↑↓] Navegar  [Tab] Aba  [Espaco] Avancar dia",
        _ => "[Tab] Aba  [Espaco/N] Avancar dia  [A] Avancar semana  [Esc] Menu",
    };
    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[3]);

    // Render popup if any
    if let Some(ref msg) = st.show_popup {
        render_popup(f, area, msg);
    }
}

fn render_squad(f: &mut Frame, area: Rect, st: &InGameState) {
    let players = st.game.world().club_players(&st.game.state().club_id);
    let mut sorted_players: Vec<&Player> = players;
    sorted_players.sort_by(|a, b| {
        position_order(&a.position).cmp(&position_order(&b.position))
            .then_with(|| b.overall_rating().cmp(&a.overall_rating()))
    });

    let header = Row::new(vec![
        Cell::from("#").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Nome").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Pos").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Idade").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("OVR").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Fit").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Forma").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Moral").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Valor").style(Style::default().add_modifier(Modifier::BOLD)),
    ]);

    let game_date = st.game.state().date.date();
    let rows: Vec<Row> = sorted_players.iter().enumerate().map(|(i, p)| {
        let style = if i == st.squad_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else if p.is_injured() {
            Style::default().fg(Color::Red)
        } else if i < 11 {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        };

        let age = p.age_on(game_date);
        let morale_str = format!("{}", p.morale.level());
        let injury_mark = if p.is_injured() { " [L]" } else { "" };

        Row::new(vec![
            Cell::from(format!("{}", i + 1)),
            Cell::from(format!("{}{}", p.full_name(), injury_mark)),
            Cell::from(p.position.short_name()),
            Cell::from(format!("{}", age)),
            Cell::from(format!("{}", p.overall_rating())),
            Cell::from(format!("{}%", p.fitness)),
            Cell::from(format!("{}", p.form)),
            Cell::from(morale_str),
            Cell::from(format!("{}", p.value)),
        ]).style(style)
    }).collect();

    let table = Table::new(rows, [
        Constraint::Length(3),
        Constraint::Min(20),
        Constraint::Length(4),
        Constraint::Length(5),
        Constraint::Length(4),
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(8),
        Constraint::Length(10),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(format!(
        " Elenco ({} jogadores) - Verde=Titular ",
        sorted_players.len()
    )));
    f.render_widget(table, area);
}

fn render_tactics(f: &mut Frame, area: Rect, st: &InGameState) {
    let club_id = &st.game.state().club_id;
    let tactics = st.game.world().clubs.get(club_id)
        .map(|c| c.tactics.clone())
        .unwrap_or_default();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Tactics settings
    let fields = vec![
        ("Formacao", tactics.formation.display_name().to_string()),
        ("Mentalidade", format!("{:?}", tactics.mentality)),
        ("Ritmo", format!("{:?}", tactics.tempo)),
        ("Pressao", format!("{}%", tactics.pressing)),
        ("Linha Defensiva", format!("{}%", tactics.defensive_line)),
        ("Largura", format!("{}%", tactics.width)),
        ("Passe Direto", format!("{}%", tactics.direct_passing)),
    ];

    let items: Vec<ListItem> = fields.iter().enumerate().map(|(i, (label, value))| {
        let style = if i == st.tactics_field {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let prefix = if i == st.tactics_field { ">> " } else { "   " };
        ListItem::new(format!("{}{}: < {} >", prefix, label, value)).style(style)
    }).collect();

    let tactics_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Taticas [←→ para alterar] "));
    f.render_widget(tactics_list, chunks[0]);

    // Formation visualization
    let formation_text = render_formation_ascii(&tactics.formation);
    let formation_widget = Paragraph::new(formation_text)
        .block(Block::default().borders(Borders::ALL).title(" Formacao "))
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center);
    f.render_widget(formation_widget, chunks[1]);
}

fn render_training(f: &mut Frame, area: Rect, st: &InGameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Min(5),
        ])
        .split(area);

    let focuses = [
        ("Geral", "Treinamento equilibrado de todos os atributos"),
        ("Fisico", "Pace, stamina, forca, aceleracao, agilidade"),
        ("Tecnico", "Finalizacao, passe, drible, cruzamento, desarme"),
        ("Tatico", "Posicionamento, decisoes, antecipacao, visao"),
        ("Recuperacao", "Restaura fitness, sem ganho de atributos"),
        ("Ataque", "Foco em habilidades ofensivas"),
        ("Defesa", "Foco em habilidades defensivas"),
        ("Bola Parada", "Escanteios, faltas, penaltis"),
    ];

    let items: Vec<ListItem> = focuses.iter().enumerate().map(|(i, (name, desc))| {
        let style = if i == st.training_field {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let prefix = if i == st.training_field { ">> " } else { "   " };
        ListItem::new(format!("{}{} - {}", prefix, name, desc)).style(style)
    }).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Foco do Treino [↑↓ para selecionar] "));
    f.render_widget(list, chunks[0]);

    // Squad fitness overview
    let players = st.game.world().club_players(&st.game.state().club_id);
    let avg_fitness: f32 = if players.is_empty() { 0.0 } else {
        players.iter().map(|p| p.fitness as f32).sum::<f32>() / players.len() as f32
    };
    let injured_count = players.iter().filter(|p| p.is_injured()).count();

    let info = Paragraph::new(format!(
        "Fitness Medio: {:.0}%  |  Jogadores Lesionados: {}  |  Elenco Total: {}",
        avg_fitness, injured_count, players.len()
    ))
    .block(Block::default().borders(Borders::ALL).title(" Status do Elenco "))
    .style(Style::default().fg(Color::Cyan));
    f.render_widget(info, chunks[1]);
}

fn render_fixtures(f: &mut Frame, area: Rect, st: &InGameState) {
    let club_id = &st.game.state().club_id;

    // Find user's competition
    let mut user_fixtures: Vec<String> = Vec::new();
    let mut next_match: Option<String> = None;

    for comp in st.game.world().competitions.values() {
        if !comp.teams.contains(club_id) {
            continue;
        }

        user_fixtures.push(format!("--- {} ---", comp.name));

        for fixture in comp.fixtures.for_team(club_id) {
            let home_name = st.game.world().clubs.get(&fixture.home_id)
                .map(|c| c.short_name.as_str())
                .unwrap_or("???");
            let away_name = st.game.world().clubs.get(&fixture.away_id)
                .map(|c| c.short_name.as_str())
                .unwrap_or("???");

            let result_str = if let Some(ref res) = fixture.result {
                format!("{} x {}", res.home_goals, res.away_goals)
            } else {
                if next_match.is_none() {
                    next_match = Some(format!("{} vs {} em {}", home_name, away_name, fixture.date));
                }
                "vs".to_string()
            };

            let is_home = &fixture.home_id == club_id;
            let marker = if is_home { "(C)" } else { "(F)" };

            user_fixtures.push(format!(
                "  R{:02} {} | {} {} {} {}",
                fixture.round, fixture.date, home_name, result_str, away_name, marker
            ));
        }
    }

    if user_fixtures.is_empty() {
        user_fixtures.push("Nenhum jogo agendado. Aguarde o inicio da temporada.".into());
    }

    let items: Vec<ListItem> = user_fixtures.iter().map(|line| {
        let style = if line.starts_with("---") {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else if line.contains(" vs ") {
            Style::default().fg(Color::Yellow) // upcoming
        } else {
            Style::default().fg(Color::White)
        };
        ListItem::new(line.as_str()).style(style)
    }).collect();

    let title = match next_match {
        Some(ref m) => format!(" Jogos | Proximo: {} ", m),
        None => " Jogos ".to_string(),
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(list, area);
}

fn render_standings(f: &mut Frame, area: Rect, st: &InGameState) {
    let divisions = ["BRA1", "BRA2", "BRA3", "BRA4"];
    let div_names = ["Serie A", "Serie B", "Serie C", "Serie D"];
    let div_idx = st.standings_division.min(3);

    // Find user's division and highlight it
    let user_div = divisions.iter().position(|d| {
        st.game.world().competitions.get(&CompetitionId::new(*d))
            .map(|c| c.teams.contains(&st.game.state().club_id))
            .unwrap_or(false)
    }).unwrap_or(0);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(5)])
        .split(area);

    // Division tabs
    let div_tabs: Vec<Line> = div_names.iter().enumerate().map(|(i, name)| {
        let style = if i == div_idx {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else if i == user_div {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        };
        Line::from(format!(" {} ", name)).style(style)
    }).collect();

    let tabs = Tabs::new(div_tabs)
        .select(div_idx)
        .style(Style::default().fg(Color::White));
    f.render_widget(tabs, chunks[0]);

    // Table
    let comp_id = CompetitionId::new(divisions[div_idx]);
    if let Some(comp) = st.game.world().competitions.get(&comp_id) {
        let header = Row::new(vec![
            Cell::from("#").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Clube").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("J").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("V").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("E").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("D").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("GP").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("GC").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("SG").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Pts").style(Style::default().add_modifier(Modifier::BOLD)),
        ]);

        let rows: Vec<Row> = comp.table.rows.iter().enumerate().map(|(i, row)| {
            let club_name = st.game.world().clubs.get(&row.club_id)
                .map(|c| c.name.as_str())
                .unwrap_or("???");

            let is_user = row.club_id == st.game.state().club_id;
            let style = if is_user {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else if i < 3 {
                Style::default().fg(Color::Green) // Promotion zone
            } else if i >= comp.table.rows.len().saturating_sub(3) {
                Style::default().fg(Color::Red) // Relegation zone
            } else {
                Style::default().fg(Color::White)
            };

            Row::new(vec![
                Cell::from(format!("{}", i + 1)),
                Cell::from(if is_user { format!(">> {} <<", club_name) } else { club_name.to_string() }),
                Cell::from(format!("{}", row.played)),
                Cell::from(format!("{}", row.won)),
                Cell::from(format!("{}", row.drawn)),
                Cell::from(format!("{}", row.lost)),
                Cell::from(format!("{}", row.goals_for)),
                Cell::from(format!("{}", row.goals_against)),
                Cell::from(format!("{}", row.goal_difference())),
                Cell::from(format!("{}", row.points)),
            ]).style(style)
        }).collect();

        let table = Table::new(rows, [
            Constraint::Length(3),
            Constraint::Min(20),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(5),
        ])
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(format!(
            " {} [←→ para trocar divisao] ",
            div_names[div_idx]
        )));
        f.render_widget(table, chunks[1]);
    } else {
        let msg = Paragraph::new("Competicao nao encontrada")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(msg, chunks[1]);
    }
}

fn render_finance(f: &mut Frame, area: Rect, st: &InGameState) {
    let club_id = &st.game.state().club_id;
    let club = st.game.world().clubs.get(club_id);

    if let Some(club) = club {
        let budget = &club.budget;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10),
                Constraint::Min(5),
            ])
            .split(area);

        let info = format!(
            "  Saldo Total:           {}\n\
             \n\
              Orcamento Transferencias: {}\n\
              Orcamento Salarial:       {}\n\
              Folha Salarial Atual:     {}\n\
              Espaco Salarial:          {}\n\
             \n\
              Pode Pagar Transferencia: Ate {}",
            budget.balance,
            budget.transfer_budget,
            budget.wage_budget,
            budget.wage_bill,
            budget.available_wage_room(),
            budget.available_for_transfers(),
        );

        let finance = Paragraph::new(info)
            .block(Block::default().borders(Borders::ALL).title(" Financas "))
            .style(Style::default().fg(Color::Green));
        f.render_widget(finance, chunks[0]);

        // Player wages
        let mut wage_list: Vec<(&Player, String)> = Vec::new();
        for pid in &club.player_ids {
            if let Ok(player) = st.game.world().player(pid) {
                let wage = player.weekly_wage();
                wage_list.push((player, format!("{}/sem", wage)));
            }
        }
        wage_list.sort_by(|a, b| b.0.weekly_wage().minor().cmp(&a.0.weekly_wage().minor()));

        let items: Vec<ListItem> = wage_list.iter().map(|(p, wage)| {
            ListItem::new(format!("  {} ({}) - {}", p.full_name(), p.position.short_name(), wage))
                .style(Style::default().fg(Color::White))
        }).collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(" Salarios dos Jogadores "));
        f.render_widget(list, chunks[1]);
    }
}

fn render_transfers(f: &mut Frame, area: Rect, st: &InGameState) {
    let window_status = if st.game.state().flags.transfer_window_open {
        "ABERTA"
    } else {
        "FECHADA"
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(5),
        ])
        .split(area);

    let info = Paragraph::new(format!(
        "  Janela de Transferencias: {}\n\
         \n\
          Orcamento disponivel: {}",
        window_status,
        st.game.world().clubs.get(&st.game.state().club_id)
            .map(|c| format!("{}", c.budget.available_for_transfers()))
            .unwrap_or_default()
    ))
    .block(Block::default().borders(Borders::ALL).title(" Transferencias "))
    .style(Style::default().fg(if st.game.state().flags.transfer_window_open { Color::Green } else { Color::Red }));
    f.render_widget(info, chunks[0]);

    // Show other clubs' notable players as potential targets
    let mut targets: Vec<String> = Vec::new();
    let user_club = st.game.state().club_id.clone();

    for (_pid, player) in &st.game.world().players {
        if player.club_id.as_ref() != Some(&user_club) && player.overall_rating() >= 60 {
            let club_name = player.club_id.as_ref()
                .and_then(|cid| st.game.world().clubs.get(cid))
                .map(|c| c.short_name.as_str())
                .unwrap_or("Livre");
            targets.push(format!(
                "  {} ({}) - {} - OVR:{} - {}",
                player.full_name(), player.position.short_name(),
                club_name, player.overall_rating(), player.value
            ));
        }
        if targets.len() >= 30 { break; }
    }

    let items: Vec<ListItem> = targets.iter().map(|t| {
        ListItem::new(t.as_str()).style(Style::default().fg(Color::White))
    }).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Jogadores Disponiveis "));
    f.render_widget(list, chunks[1]);
}

fn render_inbox(f: &mut Frame, area: Rect, st: &InGameState) {
    let messages = &st.game.state().inbox;

    if messages.is_empty() {
        let empty = Paragraph::new("Nenhuma mensagem. Avance os dias para receber noticias!")
            .block(Block::default().borders(Borders::ALL).title(" Noticias "))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(empty, area);
        return;
    }

    // Show messages in reverse order (newest first)
    let items: Vec<ListItem> = messages.iter().rev().enumerate().map(|(i, msg)| {
        let style = if i == st.inbox_scroll {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        ListItem::new(format!("  {}", msg)).style(style)
    }).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!(
            " Noticias ({} mensagens) ",
            messages.len()
        )));
    f.render_widget(list, area);
}

fn render_settings(f: &mut Frame, area: Rect, st: &SettingsState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(2),
        ])
        .split(area);

    let title = Paragraph::new(" Configuracoes ")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let settings = vec![
        ("Idioma", st.language.as_str()),
        ("Moeda", st.currency.as_str()),
    ];

    let items: Vec<ListItem> = settings.iter().enumerate().map(|(i, (label, value))| {
        let style = if i == st.selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let prefix = if i == st.selected { ">> " } else { "   " };
        ListItem::new(format!("{}{}: < {} >", prefix, label, value)).style(style)
    }).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Opcoes "));
    f.render_widget(list, chunks[1]);

    let help = Paragraph::new("[↑↓] Navegar  [←→] Alterar  [Esc] Voltar")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, chunks[2]);
}

fn render_popup(f: &mut Frame, area: Rect, message: &str) {
    let popup_area = centered_rect(60, 30, area);
    f.render_widget(Clear, popup_area);
    let popup = Paragraph::new(message)
        .block(Block::default().borders(Borders::ALL).title(" Aviso "))
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(popup, popup_area);
}

// ─── Helpers ────────────────────────────────────────────────────────────────

fn load_world() -> World {
    // Try loading from assets/data, fall back to defaults
    let data_dir = if std::path::Path::new("assets/data").exists() {
        "assets/data"
    } else if std::path::Path::new("../../assets/data").exists() {
        "../../assets/data"
    } else {
        "assets/data" // will use defaults
    };
    let importer = JsonWorldImporter::new(data_dir);
    importer.load_world().unwrap_or_default()
}

fn start_game(manager_name: String, club_id: ClubId) -> AppScreen {
    let world = load_world();
    let state = GameState::new(
        NaiveDate::from_ymd_opt(2001, 7, 1).unwrap(),
        manager_name,
        club_id,
    );
    let cfg = GameConfig::default();
    let mut game = Game::new(cfg, world, state);
    game.bootstrap_inbox();

    // Generate initial fixtures for all competitions
    generate_initial_fixtures(&mut game);

    AppScreen::InGame(InGameState {
        game,
        tab: GameTab::Squad,
        squad_scroll: 0,
        squad_selected: 0,
        inbox_scroll: 0,
        standings_division: find_user_division_index(&game_ref_workaround()),
        tactics_field: 0,
        training_field: 0,
        match_live: None,
        show_popup: None,
    })
}

/// Workaround: we can't borrow game after moving it, so just default to 0.
fn game_ref_workaround() -> usize { 0 }

fn generate_initial_fixtures(game: &mut Game) {
    let _start_date = game.state().date.date();
    let comp_system = cm_engine::systems::competition_system::CompetitionSystem;

    // Generate fixtures for each league competition
    let comp_ids: Vec<CompetitionId> = game.world().competitions.keys().cloned().collect();

    for comp_id in &comp_ids {
        let teams: Vec<ClubId> = game.world().competitions.get(comp_id)
            .map(|c| c.teams.clone())
            .unwrap_or_default();

        let comp_type = game.world().competitions.get(comp_id)
            .map(|c| c.competition_type)
            .unwrap_or(CompetitionType::League);

        if comp_type == CompetitionType::League && teams.len() >= 2 {
            // Start leagues on August 2 (first Saturday of August)
            let league_start = NaiveDate::from_ymd_opt(2001, 8, 4).unwrap();
            let fixtures = comp_system.generate_league_fixtures(comp_id, &teams, league_start);

            if let Some(comp) = game.world_mut().competitions.get_mut(comp_id) {
                comp.fixtures.matches = fixtures;
                // Initialize table
                for team in &comp.teams {
                    comp.table.add_team(team.clone());
                }
            }
        }
    }
}

fn find_club_division(world: &World, club_id: &ClubId) -> String {
    for comp in world.competitions.values() {
        if comp.is_league() && comp.teams.contains(club_id) {
            if let Some(div) = comp.division_level {
                return div.name().to_string();
            }
            return comp.short_name.clone();
        }
    }
    "Sem divisao".to_string()
}

fn find_user_division_index(default: &usize) -> usize {
    *default
}

fn position_order(pos: &Position) -> u8 {
    match pos {
        Position::Goalkeeper => 0,
        Position::DefenderLeft => 1,
        Position::DefenderCenter => 2,
        Position::DefenderRight => 3,
        Position::MidfielderDefensive => 4,
        Position::MidfielderLeft => 5,
        Position::MidfielderCenter => 6,
        Position::MidfielderRight => 7,
        Position::MidfielderAttacking => 8,
        Position::ForwardLeft => 9,
        Position::ForwardCenter => 10,
        Position::ForwardRight => 11,
    }
}

fn render_formation_ascii(formation: &Formation) -> String {
    match formation {
        Formation::F442 => {
            "          GK\n\
             \n\
             DR    DC    DC    DL\n\
             \n\
             MR    MC    MC    ML\n\
             \n\
             \n\
                  FC    FC"
        }
        Formation::F433 => {
            "          GK\n\
             \n\
             DR    DC    DC    DL\n\
             \n\
                MC  MC  MC\n\
             \n\
             \n\
              FR       FC       FL"
        }
        Formation::F352 => {
            "          GK\n\
             \n\
               DC    DC    DC\n\
             \n\
             MR   MC   MC   MC   ML\n\
             \n\
             \n\
                  FC    FC"
        }
        Formation::F4231 => {
            "          GK\n\
             \n\
             DR    DC    DC    DL\n\
             \n\
                DM    DM\n\
             \n\
              MR     AM     ML\n\
             \n\
                   FC"
        }
        _ => {
            return format!(
                "          GK\n\
                 \n\
                    Formacao: {}\n\
                 \n\
                 DEF: {}  MID: {}  ATK: {}",
                formation.display_name(),
                formation.defenders(),
                formation.midfielders(),
                formation.forwards()
            );
        }
    }.to_string()
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn dummy_in_game() -> InGameState {
    // This is a workaround for Rust ownership - it creates a placeholder
    // that gets replaced by std::mem::replace
    let world = World::default();
    let state = GameState::default();
    let cfg = GameConfig::default();
    let game = Game::new(cfg, world, state);
    InGameState {
        game,
        tab: GameTab::Squad,
        squad_scroll: 0,
        squad_selected: 0,
        inbox_scroll: 0,
        standings_division: 0,
        tactics_field: 0,
        training_field: 0,
        match_live: None,
        show_popup: None,
    }
}
