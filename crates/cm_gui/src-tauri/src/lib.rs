mod models;

use std::sync::Mutex;

use chrono::NaiveDate;
use cm_core::ids::{ClubId, CompetitionId, PlayerId};
use cm_core::world::{Fixtures, Table, World};
use cm_data::import::JsonWorldImporter;
use cm_engine::config::GameMode;
use cm_engine::{Game, GameConfig, GameState};
use cm_match::model::{MatchInput, TeamStrength};
use cm_save::snapshot::{GameConfigData, GameStateData, SaveSnapshot};
use cm_save::list_saves;
use models::*;
use tauri::State;

// ─── Global State ────────────────────────────────────────────────────────────

struct AppState {
    game: Mutex<Option<Game>>,
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn resolve_data_dir() -> String {
    let candidates = [
        "assets/data",
        "../../assets/data",
        "../../../assets/data",
        "crates/cm_gui/src-tauri/assets/data",
    ];
    for c in &candidates {
        if std::path::Path::new(c).exists() {
            return c.to_string();
        }
    }
    "assets/data".to_string()
}

fn nation_name(world: &World, nation_id: &cm_core::ids::NationId) -> String {
    world
        .nations
        .get(nation_id)
        .map(|n| n.name.clone())
        .unwrap_or_else(|| nation_id.to_string())
}

fn club_name(world: &World, club_id: &ClubId) -> String {
    world
        .clubs
        .get(club_id)
        .map(|c| c.name.clone())
        .unwrap_or_else(|| club_id.to_string())
}

fn find_user_competition(world: &World, club_id: &ClubId) -> Option<CompetitionId> {
    world
        .competitions
        .values()
        .find(|c| c.is_league() && c.teams.contains(club_id))
        .map(|c| c.id.clone())
}

fn saves_dir() -> std::path::PathBuf {
    let candidates = ["saves", "../../saves", "../../../saves"];
    for c in &candidates {
        let p = std::path::Path::new(c);
        if p.exists() {
            return p.to_path_buf();
        }
    }
    let p = std::path::PathBuf::from("saves");
    let _ = std::fs::create_dir_all(&p);
    p
}

// ─── Commands: Game Lifecycle ────────────────────────────────────────────────

#[tauri::command]
fn get_available_clubs(game_mode: Option<String>, _state: State<AppState>) -> Vec<DisplayClubOption> {
    let importer = JsonWorldImporter::new(resolve_data_dir());
    let world = match importer.load_world() {
        Ok(w) => w,
        Err(_) => return Vec::new(),
    };

    let mode = match game_mode.as_deref() {
        Some("CareerSerieD") => GameMode::CareerSerieD,
        _ => GameMode::Sandbox,
    };

    let mut clubs: Vec<DisplayClubOption> = world
        .clubs
        .values()
        .filter(|c| {
            if mode == GameMode::CareerSerieD {
                // Only Serie D clubs
                world.competitions.values().any(|comp| {
                    comp.is_league()
                        && comp.teams.contains(&c.id)
                        && comp.division_level == Some(cm_core::world::DivisionLevel::SerieD)
                })
            } else {
                true
            }
        })
        .map(|c| {
            let division = world
                .competitions
                .values()
                .find(|comp| comp.is_league() && comp.teams.contains(&c.id))
                .and_then(|comp| comp.division_level)
                .map(|d| d.name().to_string())
                .unwrap_or_else(|| "Sem divisao".to_string());

            DisplayClubOption {
                id: c.id.to_string(),
                nome: c.name.clone(),
                cor_primaria: c.primary_color.clone(),
                cor_secundaria: c.secondary_color.clone(),
                reputation: c.reputation,
                division,
            }
        })
        .collect();

    clubs.sort_by(|a, b| b.reputation.cmp(&a.reputation));
    clubs
}

#[tauri::command]
fn start_new_game(
    name: &str,
    surname: &str,
    team_id: &str,
    game_mode: Option<String>,
    state: State<AppState>,
) -> Result<DisplayGameState, String> {
    let importer = JsonWorldImporter::new(resolve_data_dir());
    let world = importer.load_world().map_err(|e| format!("{e}"))?;

    let club_id = ClubId::new(team_id);
    let manager = format!("{} {}", name, surname);
    let start_date = NaiveDate::from_ymd_opt(2001, 7, 1).unwrap();

    let mode = match game_mode.as_deref() {
        Some("CareerSerieD") => GameMode::CareerSerieD,
        _ => GameMode::Sandbox,
    };

    let game_state = GameState::new(start_date, manager, club_id.clone());
    let mut cfg = GameConfig::default();
    cfg.game_mode = mode;
    let mut game = Game::new(cfg, world, game_state);

    game.bootstrap_inbox();

    // Generate initial fixtures for all competitions
    let comp_system = cm_engine::systems::competition_system::CompetitionSystem;
    let fixture_start = NaiveDate::from_ymd_opt(2001, 8, 4).unwrap();
    let comp_info: Vec<(CompetitionId, Vec<ClubId>)> = game
        .world()
        .competitions
        .values()
        .filter(|c| c.is_league() && c.division_level.is_some())
        .map(|c| (c.id.clone(), c.teams.clone()))
        .collect();

    for (comp_id, clubs) in comp_info {
        let fixtures = comp_system.generate_league_fixtures(&comp_id, &clubs, fixture_start);
        if let Some(comp) = game.world_mut().competitions.get_mut(&comp_id) {
            comp.fixtures = Fixtures::new();
            for fixture in fixtures {
                comp.fixtures.add(fixture);
            }
            // Initialize table
            comp.table = Table::new();
            for club in &comp.teams {
                comp.table.add_team(club.clone());
            }
            if !comp.fixtures.matches.is_empty() {
                comp.total_rounds = comp
                    .fixtures
                    .matches
                    .iter()
                    .map(|f| f.round)
                    .max()
                    .unwrap_or(0);
            }
        }
    }

    let result = build_game_state(&game);

    let mut lock = state.game.lock().unwrap();
    *lock = Some(game);

    Ok(result)
}

#[tauri::command]
fn get_game_state(state: State<AppState>) -> Option<DisplayGameState> {
    let lock = state.game.lock().unwrap();
    lock.as_ref().map(build_game_state)
}

fn build_game_state(game: &Game) -> DisplayGameState {
    let world = game.world();
    let gs = game.state();
    let cid = &gs.club_id;

    let c = world.clubs.get(cid);
    let cname = c.map(|c| c.name.clone()).unwrap_or_default();
    let balance = c.map(|c| format_money(c.budget.balance)).unwrap_or_default();
    let transfer_budget = c
        .map(|c| format_money(c.budget.transfer_budget))
        .unwrap_or_default();

    let (division, position) = find_user_competition(world, cid)
        .and_then(|comp_id| world.competitions.get(&comp_id))
        .map(|comp| {
            let div = comp
                .division_level
                .map(|d| d.name().to_string())
                .unwrap_or_else(|| comp.name.clone());
            let pos = comp
                .table
                .position(cid)
                .map(|p| format!("{}o", p))
                .unwrap_or_else(|| "-".to_string());
            (div, pos)
        })
        .unwrap_or_else(|| ("-".to_string(), "-".to_string()));

    DisplayGameState {
        club_name: cname,
        club_id: cid.to_string(),
        manager_name: gs.manager_name.clone(),
        date: format!("{}", gs.date),
        season: gs.season(),
        balance,
        transfer_budget,
        division,
        position,
    }
}

// ─── Commands: Squad ─────────────────────────────────────────────────────────

#[tauri::command]
fn get_team_squad(team_id: String, state: State<AppState>) -> Vec<DisplayPlayer> {
    let lock = state.game.lock().unwrap();
    let game = match lock.as_ref() {
        Some(g) => g,
        None => return Vec::new(),
    };
    let world = game.world();
    let date = game.state().date.date();
    let cid = ClubId::new(&team_id);

    let club = match world.clubs.get(&cid) {
        Some(c) => c,
        None => return Vec::new(),
    };

    let mut players: Vec<DisplayPlayer> = club
        .player_ids
        .iter()
        .filter_map(|pid| world.players.get(pid))
        .map(|p| {
            let nat = nation_name(world, &p.nationality);
            DisplayPlayer::from_player(p, date, &nat, &club.name)
        })
        .collect();

    players.sort_by(|a, b| b.overall.cmp(&a.overall));
    players
}

#[tauri::command]
fn get_player_details(player_id: String, state: State<AppState>) -> Option<DisplayPlayerProfile> {
    let lock = state.game.lock().unwrap();
    let game = lock.as_ref()?;
    let world = game.world();
    let date = game.state().date.date();

    let pid = PlayerId::new(&player_id);
    let player = world.players.get(&pid)?;

    let nat = nation_name(world, &player.nationality);
    let cname = player
        .club_id
        .as_ref()
        .map(|cid| club_name(world, cid))
        .unwrap_or_else(|| "Livre".into());

    Some(DisplayPlayerProfile {
        display: DisplayPlayer::from_player(player, date, &nat, &cname),
        attributes: DisplayAttributes::from(&player.attributes),
    })
}

#[tauri::command]
fn swap_starter(player_id: String, state: State<AppState>) -> bool {
    let mut lock = state.game.lock().unwrap();
    let game = match lock.as_mut() {
        Some(g) => g,
        None => return false,
    };

    let user_club_id = game.state().club_id.clone();
    let pid = PlayerId::new(&player_id);

    let club = match game.world_mut().clubs.get_mut(&user_club_id) {
        Some(c) => c,
        None => return false,
    };

    if let Some(pos) = club.player_ids.iter().position(|p| *p == pid) {
        // Swap: if in first 11, move to end; if in reserves, move to position 10
        let squad_size = club.player_ids.len();
        if squad_size < 2 {
            return false;
        }
        let removed = club.player_ids.remove(pos);
        if pos < 11 {
            // Was starter -> move to reserves
            club.player_ids.push(removed);
        } else {
            // Was reserve -> move to starters (position 10 = last starter)
            let insert_pos = 10.min(club.player_ids.len());
            club.player_ids.insert(insert_pos, removed);
        }
        true
    } else {
        false
    }
}

// ─── Commands: Day Advancement ───────────────────────────────────────────────

#[tauri::command]
fn advance_day(state: State<AppState>) -> Option<DisplayGameState> {
    let mut lock = state.game.lock().unwrap();
    let game = lock.as_mut()?;
    game.process_day();
    Some(build_game_state(game))
}

#[tauri::command]
fn check_match_today(state: State<AppState>) -> Option<DisplayFixturePreview> {
    let lock = state.game.lock().unwrap();
    let game = lock.as_ref()?;
    let world = game.world();
    let today = game.state().date.date();
    let user_club = &game.state().club_id;

    for comp in world.competitions.values() {
        for fixture in &comp.fixtures.matches {
            if fixture.date == today
                && !fixture.is_played()
                && (fixture.home_id == *user_club || fixture.away_id == *user_club)
            {
                return Some(DisplayFixturePreview {
                    home_name: club_name(world, &fixture.home_id),
                    away_name: club_name(world, &fixture.away_id),
                    home_id: fixture.home_id.to_string(),
                    away_id: fixture.away_id.to_string(),
                    competition: comp.name.clone(),
                    is_home: fixture.home_id == *user_club,
                });
            }
        }
    }
    None
}

// ─── Commands: Match ─────────────────────────────────────────────────────────

#[tauri::command]
fn start_match(home_id: String, away_id: String, state: State<AppState>) -> Option<DisplayMatchResult> {
    let mut lock = state.game.lock().unwrap();
    let game = lock.as_mut()?;

    let home_cid = ClubId::new(&home_id);
    let away_cid = ClubId::new(&away_id);

    let home_strength = game
        .world()
        .clubs
        .get(&home_cid)
        .map(TeamStrength::from_club)
        .unwrap_or_default();
    let away_strength = game
        .world()
        .clubs
        .get(&away_cid)
        .map(TeamStrength::from_club)
        .unwrap_or_default();

    let home_name = club_name(game.world(), &home_cid);
    let away_name = club_name(game.world(), &away_cid);

    let input = MatchInput {
        home_id: home_cid.clone(),
        away_id: away_cid.clone(),
        home: home_strength,
        away: away_strength,
        minutes: 90,
        seed: None,
    };

    let result = cm_match::simulate_match(&input);

    // Update fixture as played
    let today = game.state().date.date();
    for comp in game.world_mut().competitions.values_mut() {
        for fixture in &mut comp.fixtures.matches {
            if fixture.date == today
                && fixture.home_id == home_cid
                && fixture.away_id == away_cid
                && !fixture.is_played()
            {
                fixture.set_result(result.home_goals, result.away_goals, 30000);
                // Update league table
                comp.table.record_result(
                    &home_cid,
                    &away_cid,
                    result.home_goals,
                    result.away_goals,
                    3,
                    1,
                );
                break;
            }
        }
    }

    // Store last match result
    game.state_mut().last_match_result = Some(result.clone());

    Some(DisplayMatchResult {
        home_goals: result.home_goals,
        away_goals: result.away_goals,
        home_name,
        away_name,
        highlights: result.highlights,
    })
}

// ─── Commands: League Table ──────────────────────────────────────────────────

#[tauri::command]
fn get_league_table(state: State<AppState>) -> Option<DisplayLeagueTable> {
    let lock = state.game.lock().unwrap();
    let game = lock.as_ref()?;
    let world = game.world();
    let user_club = &game.state().club_id;

    let comp = world
        .competitions
        .values()
        .find(|c| c.is_league() && c.teams.contains(user_club))?;

    let mut rows: Vec<DisplayLeagueRow> = comp
        .table
        .rows
        .iter()
        .map(|r| DisplayLeagueRow {
            position: 0,
            club_name: club_name(world, &r.club_id),
            played: r.played,
            won: r.won,
            drawn: r.drawn,
            lost: r.lost,
            gf: r.goals_for,
            ga: r.goals_against,
            points: r.points,
        })
        .collect();

    rows.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| (b.gf as i32 - b.ga as i32).cmp(&(a.gf as i32 - a.ga as i32)))
            .then_with(|| b.gf.cmp(&a.gf))
    });

    for (i, r) in rows.iter_mut().enumerate() {
        r.position = (i + 1) as u8;
    }

    Some(DisplayLeagueTable {
        name: comp.name.clone(),
        rows,
    })
}

// ─── Commands: Fixtures ──────────────────────────────────────────────────────

#[tauri::command]
fn get_fixtures(state: State<AppState>) -> Vec<DisplayFixture> {
    let lock = state.game.lock().unwrap();
    let game = match lock.as_ref() {
        Some(g) => g,
        None => return Vec::new(),
    };
    let world = game.world();
    let user_club = &game.state().club_id;

    let mut fixtures: Vec<DisplayFixture> = world
        .competitions
        .values()
        .flat_map(|comp| {
            comp.fixtures
                .for_team(user_club)
                .into_iter()
                .map(move |f| {
                    let result_str = f.result.as_ref().map(|r| {
                        format!("{} x {}", r.home_goals, r.away_goals)
                    });
                    DisplayFixture {
                        id: f.id.to_string(),
                        competition: comp.name.clone(),
                        round: f.round,
                        date: f.date.format("%d %b %Y").to_string(),
                        home_name: club_name(world, &f.home_id),
                        away_name: club_name(world, &f.away_id),
                        home_id: f.home_id.to_string(),
                        away_id: f.away_id.to_string(),
                        result: result_str,
                        played: f.is_played(),
                    }
                })
        })
        .collect();

    fixtures.sort_by_key(|f| f.date.clone());
    fixtures
}

// ─── Commands: Inbox ─────────────────────────────────────────────────────────

#[tauri::command]
fn get_inbox(state: State<AppState>) -> Vec<DisplayMessage> {
    let lock = state.game.lock().unwrap();
    let game = match lock.as_ref() {
        Some(g) => g,
        None => return Vec::new(),
    };

    game.state()
        .inbox
        .iter()
        .rev()
        .enumerate()
        .map(|(i, msg)| DisplayMessage {
            id: format!("msg-{}", i),
            msg_type: "system".into(),
            title: msg.lines().next().unwrap_or("Mensagem").to_string(),
            text: msg.clone(),
            date: format!("{}", game.state().date),
            time: "09:00".into(),
            unread: i < 3,
            tags: vec!["Info".into()],
        })
        .collect()
}

// ─── Commands: Finance ───────────────────────────────────────────────────────

#[tauri::command]
fn get_finances(state: State<AppState>) -> Option<DisplayFinances> {
    let lock = state.game.lock().unwrap();
    let game = lock.as_ref()?;
    let world = game.world();
    let club = world.clubs.get(&game.state().club_id)?;

    Some(DisplayFinances {
        balance: format_money(club.budget.balance),
        transfer_budget: format_money(club.budget.transfer_budget),
        wage_budget: format_money(club.budget.wage_budget),
        wage_bill: format_money(club.budget.wage_bill),
        wage_room: format_money(club.budget.available_wage_room()),
    })
}

// ─── Commands: Transfers ─────────────────────────────────────────────────────

#[tauri::command]
fn search_players(query: String, state: State<AppState>) -> Vec<DisplayPlayer> {
    let lock = state.game.lock().unwrap();
    let game = match lock.as_ref() {
        Some(g) => g,
        None => return Vec::new(),
    };
    let world = game.world();
    let date = game.state().date.date();
    let query_lower = query.to_lowercase();

    let mut results: Vec<DisplayPlayer> = world
        .players
        .values()
        .filter(|p| {
            if query_lower.is_empty() {
                p.overall_rating() >= 60
            } else {
                p.full_name().to_lowercase().contains(&query_lower)
            }
        })
        .take(50)
        .map(|p| {
            let nat = nation_name(world, &p.nationality);
            let cname = p
                .club_id
                .as_ref()
                .map(|cid| club_name(world, cid))
                .unwrap_or_else(|| "Livre".into());
            DisplayPlayer::from_player(p, date, &nat, &cname)
        })
        .collect();

    results.sort_by(|a, b| b.overall.cmp(&a.overall));
    results
}

#[tauri::command]
fn offer_transfer(player_id: String, amount: u64, state: State<AppState>) -> String {
    let mut lock = state.game.lock().unwrap();
    let game = match lock.as_mut() {
        Some(g) => g,
        None => return "Nenhum jogo em andamento.".into(),
    };

    let pid = PlayerId::new(&player_id);
    let user_club_id = game.state().club_id.clone();

    // Check player exists
    let (player_name, player_value, player_club) = {
        let world = game.world();
        match world.players.get(&pid) {
            Some(p) => (
                p.full_name(),
                p.value.major() as u64,
                p.club_id.clone(),
            ),
            None => return "Jogador nao encontrado.".into(),
        }
    };

    // Check budget
    {
        let world = game.world();
        let club = match world.clubs.get(&user_club_id) {
            Some(c) => c,
            None => return "Clube nao encontrado.".into(),
        };
        if !club.budget.can_afford_transfer(cm_core::economy::Money::from_major(amount as i64)) {
            return "Orcamento insuficiente para esta transferencia.".into();
        }
    }

    // Simple negotiation: accept if bid >= 90% of value
    let threshold = (player_value as f64 * 0.9) as u64;
    if amount >= threshold {
        let fee = cm_core::economy::Money::from_major(amount as i64);

        // Remove from old club
        if let Some(old_club_id) = &player_club {
            if let Some(old_club) = game.world_mut().clubs.get_mut(old_club_id) {
                old_club.remove_player(&pid);
                old_club.budget.receive_transfer(fee);
            }
        }

        // Add to user club
        if let Some(new_club) = game.world_mut().clubs.get_mut(&user_club_id) {
            new_club.add_player(pid.clone());
            new_club.budget.spend_transfer(fee);
        }

        // Update player's club_id
        if let Some(player) = game.world_mut().players.get_mut(&pid) {
            player.club_id = Some(user_club_id);
        }

        format!("Oferta aceita! {} se juntou ao seu clube.", player_name)
    } else {
        format!(
            "Oferta recusada. O clube pede pelo menos {}.",
            format_money(cm_core::economy::Money::from_major(threshold as i64))
        )
    }
}

// ─── Commands: Tactics ───────────────────────────────────────────────────────

#[tauri::command]
fn update_tactics(
    formation: String,
    mentality: String,
    tempo: String,
    pressing: u8,
    def_line: u8,
    width: u8,
    direct: u8,
    state: State<AppState>,
) -> bool {
    use cm_core::world::{Formation, Mentality, Tempo};

    let mut lock = state.game.lock().unwrap();
    let game = match lock.as_mut() {
        Some(g) => g,
        None => return false,
    };

    let user_club_id = game.state().club_id.clone();
    let club = match game.world_mut().clubs.get_mut(&user_club_id) {
        Some(c) => c,
        None => return false,
    };

    club.tactics.formation = match formation.as_str() {
        "4-4-2" => Formation::F442,
        "4-3-3" => Formation::F433,
        "3-5-2" => Formation::F352,
        "4-5-1" => Formation::F451,
        "4-2-3-1" => Formation::F4231,
        "3-4-1-2" => Formation::F3412,
        "5-3-2" => Formation::F532,
        "4-1-4-1" => Formation::F4141,
        "4-4-1-1" => Formation::F4411,
        "3-4-3" => Formation::F343,
        _ => club.tactics.formation,
    };

    club.tactics.mentality = match mentality.as_str() {
        "Defensive" | "Defensivo" => Mentality::Defensive,
        "Cautious" | "Cauteloso" => Mentality::Cautious,
        "Balanced" | "Equilibrado" => Mentality::Balanced,
        "Attacking" | "Ofensivo" => Mentality::Attacking,
        "AllOutAttack" | "AtaqueTotal" => Mentality::AllOutAttack,
        _ => club.tactics.mentality,
    };

    club.tactics.tempo = match tempo.as_str() {
        "Slow" | "Lento" => Tempo::Slow,
        "Normal" => Tempo::Normal,
        "Fast" | "Rapido" => Tempo::Fast,
        _ => club.tactics.tempo,
    };

    club.tactics.pressing = pressing;
    club.tactics.defensive_line = def_line;
    club.tactics.width = width;
    club.tactics.direct_passing = direct;

    true
}

// ─── Commands: Save/Load ─────────────────────────────────────────────────────

#[tauri::command]
fn save_game(state: State<AppState>) -> Result<bool, String> {
    let lock = state.game.lock().unwrap();
    let game = match lock.as_ref() {
        Some(g) => g,
        None => return Err("Nenhum jogo em andamento.".into()),
    };

    let gs = game.state();
    let cfg = game.cfg();

    let config_data = GameConfigData {
        difficulty: cfg.difficulty,
        auto_save: cfg.auto_save,
    };

    let state_data = GameStateData {
        date: format!("{}", gs.date.date()),
        manager_name: gs.manager_name.clone(),
        club_id: gs.club_id.to_string(),
        inbox: gs.inbox.clone(),
    };

    let snap = SaveSnapshot::new(game.world().clone(), config_data, state_data)
        .map_err(|e| format!("{e}"))?;

    let dir = saves_dir();
    let _ = std::fs::create_dir_all(&dir);
    let filename = format!("{}_{}.cmsave", gs.club_id, gs.date.date());
    let path = dir.join(&filename);

    snap.write_to_file(path.to_str().unwrap_or("save.cmsave"))
        .map_err(|e| format!("{e}"))?;

    Ok(true)
}

#[tauri::command]
fn get_saved_games() -> Vec<DisplaySaveSlot> {
    let dir = saves_dir();
    let saves = list_saves(&dir);

    saves
        .into_iter()
        .enumerate()
        .map(|(i, meta)| DisplaySaveSlot {
            slot_id: i as u32,
            manager_name: meta.manager_name,
            club: meta.club_name,
            date: meta.game_date,
            timestamp: meta.created_at.timestamp() as u64,
        })
        .collect()
}

#[tauri::command]
fn load_game(slot_id: u32, state: State<AppState>) -> Result<DisplayGameState, String> {
    let dir = saves_dir();
    let saves = list_saves(&dir);

    let meta = saves
        .get(slot_id as usize)
        .ok_or_else(|| "Save nao encontrado.".to_string())?;

    let file_path = dir.join(format!("{}.cmsave", meta.save_name));
    let snap = SaveSnapshot::read_from_file(file_path.to_str().unwrap_or(""))
        .map_err(|e| format!("{e}"))?;

    let world = snap.payload.world;
    let state_data = &snap.payload.game_state;
    let config_data = &snap.payload.game_config;

    let date = NaiveDate::parse_from_str(&state_data.date, "%Y-%m-%d")
        .unwrap_or_else(|_| NaiveDate::from_ymd_opt(2001, 7, 1).unwrap());

    let game_state = GameState {
        date: cm_core::sim::GameDate::from(date),
        manager_name: state_data.manager_name.clone(),
        club_id: ClubId::new(&state_data.club_id),
        inbox: state_data.inbox.clone(),
        flags: cm_engine::state::GameFlags::default(),
        days_played: 0,
        last_match_result: None,
    };

    let cfg = GameConfig {
        difficulty: config_data.difficulty,
        auto_save: config_data.auto_save,
        ..GameConfig::default()
    };

    let game = Game::new(cfg, world, game_state);
    let result = build_game_state(&game);

    let mut lock = state.game.lock().unwrap();
    *lock = Some(game);

    Ok(result)
}

// ─── Commands: Round Results ─────────────────────────────────────────────

#[derive(serde::Serialize)]
struct DisplayRoundResult {
    home_name: String,
    away_name: String,
    home_goals: u8,
    away_goals: u8,
    competition: String,
}

#[tauri::command]
fn get_round_results(state: State<AppState>) -> Vec<DisplayRoundResult> {
    let lock = state.game.lock().unwrap();
    let game = match lock.as_ref() {
        Some(g) => g,
        None => return Vec::new(),
    };
    let world = game.world();
    let today = game.state().date.date();

    let mut results = Vec::new();
    for comp in world.competitions.values() {
        for fixture in &comp.fixtures.matches {
            if fixture.date == today && fixture.is_played() {
                if let Some(ref r) = fixture.result {
                    results.push(DisplayRoundResult {
                        home_name: club_name(world, &fixture.home_id),
                        away_name: club_name(world, &fixture.away_id),
                        home_goals: r.home_goals,
                        away_goals: r.away_goals,
                        competition: comp.name.clone(),
                    });
                }
            }
        }
    }
    results
}

// ─── Commands: All Fixtures (all competitions) ──────────────────────────

#[tauri::command]
fn get_all_fixtures(state: State<AppState>) -> Vec<DisplayFixture> {
    let lock = state.game.lock().unwrap();
    let game = match lock.as_ref() {
        Some(g) => g,
        None => return Vec::new(),
    };
    let world = game.world();
    let today = game.state().date.date();

    let mut fixtures: Vec<DisplayFixture> = world
        .competitions
        .values()
        .flat_map(|comp| {
            comp.fixtures.matches.iter()
                .filter(|f| {
                    // Show: played today, or upcoming within 7 days
                    (f.is_played() && f.date == today)
                        || (!f.is_played() && f.date >= today && f.date <= today + chrono::Duration::days(14))
                })
                .map(move |f| {
                    let result_str = f.result.as_ref().map(|r| {
                        format!("{} x {}", r.home_goals, r.away_goals)
                    });
                    DisplayFixture {
                        id: f.id.to_string(),
                        competition: comp.name.clone(),
                        round: f.round,
                        date: f.date.format("%d %b %Y").to_string(),
                        home_name: club_name(world, &f.home_id),
                        away_name: club_name(world, &f.away_id),
                        home_id: f.home_id.to_string(),
                        away_id: f.away_id.to_string(),
                        result: result_str,
                        played: f.is_played(),
                    }
                })
        })
        .collect();

    fixtures.sort_by(|a, b| a.date.cmp(&b.date));
    fixtures
}

// ─── Tauri Entry Point ───────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            game: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            get_available_clubs,
            start_new_game,
            get_game_state,
            get_team_squad,
            get_player_details,
            swap_starter,
            advance_day,
            check_match_today,
            start_match,
            get_league_table,
            get_fixtures,
            get_inbox,
            get_finances,
            search_players,
            offer_transfer,
            update_tactics,
            save_game,
            get_saved_games,
            load_game,
            get_round_results,
            get_all_fixtures,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
