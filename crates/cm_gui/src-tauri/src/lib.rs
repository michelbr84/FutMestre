// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod models;

use models::DisplayPlayer;
use cm_core::world::player::{Player, Position};
use cm_core::ids::{NationId, PlayerId};
use chrono::NaiveDate;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn start_new_game(name: &str, surname: &str, nation_id: u32, team_id: &str) -> String {
    println!("Starting new game for {} {} (Nation: {}, Team: {})", name, surname, nation_id, team_id);
    format!("Game started for {} {}", name, surname)
}

#[tauri::command]
fn get_team_squad(team_id: u32) -> Vec<DisplayPlayer> {
    // In a real app, we would access the global State<GameState> and get real players.
    // For now, we generate random players on the fly to prove the connection.
    let mut players = Vec::new();
    
    // Generate 25 players
    for i in 1..=25 {
        let pos = if i == 1 { Position::Goalkeeper } 
                  else if i <= 8 { Position::DefenderCenter }
                  else if i <= 16 { Position::MidfielderCenter }
                  else { Position::ForwardCenter };
                  
        let p = Player::new(
            PlayerId::new((i + (team_id * 100)).to_string()), // Unique ID based on team
            format!("Player"), 
            format!("{}", i),
            NationId::new("1"), // Default
            NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            pos
        );
        players.push(DisplayPlayer::from(&p));
    }
    
    players
}

#[tauri::command]
fn advance_day() -> String {
    // Simulating day advancement
    // In real engine: game_state.advance()
    "2026-01-02".to_string() 
}

#[tauri::command]
fn save_game() -> bool {
    // Simulating save
    // In real engine: game_state.save("save1.cmsave")
    true
}

#[tauri::command]
fn get_player_details(player_id: u32) -> Option<models::DisplayPlayerProfile> {
    // Mock data for profile
    let pos = if player_id % 2 == 0 { Position::DefenderCenter } else { Position::ForwardCenter };
    let p = Player::new(
        PlayerId::new(player_id.to_string()), 
        "Player", 
        &player_id.to_string(), 
        NationId::new("1"), 
        NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(), 
        pos
    );
    
    // Mock attributes
    let mut attributes = cm_core::world::attributes::Attributes::default();
    attributes.technical.finishing = 15;
    attributes.physical.pace = 12;

    Some(models::DisplayPlayerProfile {
        display: models::DisplayPlayer::from(&p),
        attributes: models::DisplayAttributes::from(&attributes)
    })
}

#[tauri::command]
fn start_match(home_id: u32, away_id: u32) -> models::DisplayMatchResult {
    use cm_match::model::{MatchInput, TeamStrength};
    use cm_core::ids::ClubId;

    // determine strength (mock based on ID for now, or fetch from state if available)
    // In real app: let home = state.get_club(home_id).strength();
    let home_strength = TeamStrength { attack: 80, midfield: 75, defense: 75, finishing: 70, morale: 80, fitness: 90 };
    let away_strength = TeamStrength { attack: 70, midfield: 70, defense: 70, finishing: 65, morale: 70, fitness: 85 };

    let input = MatchInput {
        home_id: ClubId::new(home_id.to_string()),
        away_id: ClubId::new(away_id.to_string()),
        home: home_strength,
        away: away_strength,
        minutes: 90,
        seed: None, // Random
    };

    let result = cm_match::simulate_match(&input);
    models::DisplayMatchResult::from(result)
}

#[tauri::command]
fn get_league_table() -> models::DisplayLeagueTable {
    use cm_core::competitions::LeagueTable;
    use cm_core::ids::ClubId;

    // Mock league table
    let teams = vec![
        (ClubId::new("1".to_string()), "Man City".to_string()),
        (ClubId::new("2".to_string()), "Arsenal".to_string()),
        (ClubId::new("3".to_string()), "Liverpool".to_string()),
        (ClubId::new("4".to_string()), "Aston Villa".to_string()),
    ];
    let mut table = LeagueTable::new("Premier League".to_string(), teams);
    
    // Simulate some stats
    for row in &mut table.rows {
        row.played = 5;
        row.won = 3;
        row.drawn = 1;
        row.lost = 1;
        row.gf = 10;
        row.ga = 5;
        row.points = 10;
    }
    // Make one winner
    table.rows[0].points = 12;

    models::DisplayLeagueTable::from(table)
}

#[tauri::command]
fn search_players(query: String) -> Vec<models::DisplayPlayer> {
    // Mock search - return a few random players if query matches "silva" or empty
    // In real app: iterate over world.players and filter
    let mut players = Vec::new();
    
    if query.to_lowercase().contains("silva") || query.is_empty() {
        use cm_core::world::player::{Player, Position};
        use cm_core::ids::{PlayerId, ClubId, NationId};
        use chrono::NaiveDate;

        // Mock Player 1
        let mut p1 = Player::new(
            PlayerId::new("100"),
            "Bernardo",
            "Silva",
            NationId::new("1"), // Portugal
            NaiveDate::from_ymd_opt(1994, 8, 10).unwrap(),
            Position::MidfielderAttacking,
        );
        // Set extra fields
        p1.club_id = Some(ClubId::new("1"));
        p1.value = cm_core::economy::Money::from_major(60_000_000);
        
        players.push(models::DisplayPlayer::from(&p1));

        // Mock Player 2
        let mut p2 = Player::new(
            PlayerId::new("101"),
            "David",
            "Silva",
            NationId::new("2"), // Spain
            NaiveDate::from_ymd_opt(1986, 1, 8).unwrap(),
            Position::MidfielderAttacking,
        );
        p2.club_id = Some(ClubId::new("99"));
        p2.value = cm_core::economy::Money::from_major(5_000_000);

        players.push(models::DisplayPlayer::from(&p2));
    }

    players
}

#[tauri::command]
fn offer_transfer(player_id: String, amount: u64) -> String {
    // Mock negotiation logic
    // Accept if amount > 50M, else Reject
    if amount > 50_000_000 {
        format!("Offer accepted! {} joins your club.", player_id)
    } else {
        format!("Offer rejected. That is derisory for a player of this caliber.")
    }
}

#[tauri::command]
fn get_saved_games() -> Vec<models::DisplaySaveSlot> {
    // Mock save list
    vec![
        models::DisplaySaveSlot {
            slot_id: 1,
            manager_name: "Michel Araujo".to_string(),
            club: "Liverpool".to_string(),
            date: "2026-01-01".to_string(),
            timestamp: 1709999999
        },
        models::DisplaySaveSlot {
            slot_id: 2,
            manager_name: "Test User".to_string(),
            club: "Vasco".to_string(),
            date: "2026-02-15".to_string(),
            timestamp: 1708888888
        }
    ]
}

#[tauri::command]
fn load_game(slot_id: u32) -> bool {
    // Mock load
    println!("Loading save slot {}", slot_id);
    true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            start_new_game, 
            get_team_squad, 
            get_player_details, 
            advance_day, 
            save_game,
            start_match,
            get_league_table,
            search_players,
            offer_transfer,
            get_saved_games,
            load_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
