//! Export save command - Export save data to various formats.

use tracing::info;

use cm_save::snapshot::SaveSnapshot;

#[derive(clap::Args, Debug)]
pub struct ExportSaveArgs {
    /// Path to the save file
    #[arg(long)]
    pub save: String,

    /// Output file path
    #[arg(long)]
    pub out: String,

    /// Export format (json, summary, world)
    #[arg(long, default_value = "summary")]
    pub format: String,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

pub fn run(args: ExportSaveArgs) -> anyhow::Result<()> {
    info!("Exporting save file: {}", args.save);

    // Load the save file
    let snapshot = SaveSnapshot::read_from_file(&args.save)?;

    if args.verbose {
        println!("Loaded save file: {}", args.save);
        println!("Save version: {}", snapshot.version);
    }

    match args.format.as_str() {
        "json" => export_json(&snapshot, &args.out, args.verbose)?,
        "summary" => export_summary(&snapshot, &args.out, args.verbose)?,
        "world" => export_world(&snapshot, &args.out, args.verbose)?,
        _ => {
            return Err(anyhow::anyhow!(
                "Unknown format: {}. Valid formats: json, summary, world",
                args.format
            ));
        }
    }

    println!("✓ Export complete: {}", args.out);

    Ok(())
}

fn export_json(snapshot: &SaveSnapshot, out_path: &str, verbose: bool) -> anyhow::Result<()> {
    use std::fs::File;
    use std::io::Write;

    if verbose {
        println!("Exporting full save as JSON...");
    }

    let json = serde_json::to_string_pretty(&snapshot)?;

    let mut file = File::create(out_path)?;
    file.write_all(json.as_bytes())?;

    if verbose {
        println!("JSON size: {} bytes", json.len());
    }

    Ok(())
}

fn export_summary(snapshot: &SaveSnapshot, out_path: &str, verbose: bool) -> anyhow::Result<()> {
    use std::fs::File;
    use std::io::Write;

    if verbose {
        println!("Generating save summary...");
    }

    let world = snapshot.world();
    let state = snapshot.state();
    let config = &snapshot.payload.game_config;

    let mut output = String::new();

    output.push_str("=== CM Save Summary ===\n\n");

    // Game info
    output.push_str("[Game Info]\n");
    output.push_str(&format!("Save Version: {}\n", snapshot.version));
    output.push_str(&format!("Manager: {}\n", state.manager_name));
    output.push_str(&format!("Club: {}\n", state.club_id));
    output.push_str(&format!("Date: {}\n", state.date));
    output.push_str(&format!("Difficulty: {}\n", config.difficulty));
    output.push_str("\n");

    // World stats
    output.push_str("[World Statistics]\n");
    output.push_str(&format!("Nations: {}\n", world.nations.len()));
    output.push_str(&format!("Clubs: {}\n", world.clubs.len()));
    output.push_str(&format!("Players: {}\n", world.players.len()));
    output.push_str(&format!("Staff: {}\n", world.staff.len()));
    output.push_str(&format!("Competitions: {}\n", world.competitions.len()));
    output.push_str(&format!("Stadiums: {}\n", world.stadiums.len()));
    output.push_str(&format!("Referees: {}\n", world.referees.len()));
    output.push_str("\n");

    // Inbox summary
    output.push_str("[Inbox]\n");
    output.push_str(&format!("Messages: {}\n", state.inbox.len()));
    output.push_str("\n");

    // Club details if available
    if let Some(club) = world.clubs.get(&cm_core::ids::ClubId::new(&state.club_id)) {
        output.push_str("[Managed Club Details]\n");
        output.push_str(&format!("Name: {}\n", club.name));
        output.push_str(&format!("Squad Size: {}\n", club.player_ids.len()));
        output.push_str(&format!("Reputation: {}\n", club.reputation));
        output.push_str(&format!(
            "Transfer Budget: {}\n",
            club.budget.transfer_budget
        ));
        output.push_str(&format!("Wage Budget: {}\n", club.budget.wage_budget));
        output.push_str("\n");
    }

    let mut file = File::create(out_path)?;
    file.write_all(output.as_bytes())?;

    if verbose {
        println!("Summary generated");
    }

    Ok(())
}

fn export_world(snapshot: &SaveSnapshot, out_path: &str, verbose: bool) -> anyhow::Result<()> {
    use std::fs::File;
    use std::io::Write;

    if verbose {
        println!("Exporting world data as JSON...");
    }

    let json = serde_json::to_string_pretty(snapshot.world())?;

    let mut file = File::create(out_path)?;
    file.write_all(json.as_bytes())?;

    if verbose {
        println!("World JSON size: {} bytes", json.len());
    }

    Ok(())
}
