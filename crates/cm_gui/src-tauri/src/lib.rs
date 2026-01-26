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
            start_match
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
