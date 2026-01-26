// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn start_new_game(name: &str, surname: &str, nation_id: u32, team_id: &str) -> String {
    println!("Starting new game for {} {} (Nation: {}, Team: {})", name, surname, nation_id, team_id);
    // In a real implementation, this would call cm_engine or cm_cli logic to create the save
    format!("Game started for {} {}", name, surname)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, start_new_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
