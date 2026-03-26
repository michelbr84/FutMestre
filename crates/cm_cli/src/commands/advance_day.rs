//! Advance day command.

use chrono::NaiveDate;

use cm_core::ids::ClubId;
use cm_engine::config::GameConfig;
use cm_engine::game::Game;
use cm_engine::state::GameState;
use cm_save::snapshot::{GameConfigData, GameStateData, SaveSnapshot};

#[derive(clap::Args, Debug)]
pub struct AdvanceDayArgs {
    /// Save file path
    #[arg(long, default_value = "saves/slot1.cmsave")]
    pub save: String,

    /// Number of days to advance
    #[arg(long, default_value_t = 1)]
    pub days: u32,
}

pub fn run(args: AdvanceDayArgs) -> anyhow::Result<()> {
    println!("Loading save: {}", args.save);
    let snap = SaveSnapshot::read_from_file(&args.save)?;

    // Create game from save
    let cfg = GameConfig {
        difficulty: snap.payload.game_config.difficulty,
        auto_save: snap.payload.game_config.auto_save,
        ..Default::default()
    };

    let start_date = NaiveDate::parse_from_str(&snap.payload.game_state.date, "%Y-%m-%d")
        .unwrap_or_else(|_| NaiveDate::from_ymd_opt(2001, 7, 1).unwrap());

    let state = GameState::new(
        start_date,
        snap.payload.game_state.manager_name.clone(),
        ClubId::new(&snap.payload.game_state.club_id),
    );

    let mut game = Game::new(cfg, snap.payload.world.clone(), state);

    println!("Advancing {} day(s)...", args.days);
    for i in 0..args.days {
        game.process_day();
        if (i + 1) % 7 == 0 {
            println!("  Week {} complete: {}", (i + 1) / 7, game.state().date);
        }
    }

    // Save updated state
    let config_data = GameConfigData {
        difficulty: game.cfg().difficulty,
        auto_save: game.cfg().auto_save,
    };

    let state_data = GameStateData {
        date: game.state().date.date().to_string(),
        manager_name: game.state().manager_name.clone(),
        club_id: game.state().club_id.to_string(),
        inbox: game.state().inbox.clone(),
    };

    let new_snap = SaveSnapshot::new(game.world().clone(), config_data, state_data)?;
    new_snap.write_to_file(&args.save)?;

    println!(
        "✓ Advanced {} day(s). Current date: {}",
        args.days,
        game.state().date
    );
    println!("  {} messages in inbox", game.state().inbox.len());

    Ok(())
}
