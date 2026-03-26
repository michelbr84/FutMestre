//! Simulate match command.

use cm_data::import::json_importer::JsonWorldImporter;
use cm_match::model::{MatchInput, TeamStrength};
use cm_match::probabilistic::simulate_match;

#[derive(clap::Args, Debug)]
pub struct SimulateMatchArgs {
    /// Directory containing JSON data files
    #[arg(long, default_value = "assets/data")]
    pub data_dir: String,

    /// Home team ID
    #[arg(long, default_value = "LIV")]
    pub home: String,

    /// Away team ID
    #[arg(long, default_value = "ARS")]
    pub away: String,

    /// Random seed (optional, for deterministic results)
    #[arg(long)]
    pub seed: Option<u64>,
}

pub fn run(args: SimulateMatchArgs) -> anyhow::Result<()> {
    println!("Loading world data...");
    let importer = JsonWorldImporter::new(&args.data_dir);
    let world = importer.load_world()?;

    let home = world.club(&args.home)?;
    let away = world.club(&args.away)?;

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  {} vs {}", home.name, away.name);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let home_strength = TeamStrength::from_club(home);
    let away_strength = TeamStrength::from_club(away);

    let input = MatchInput {
        home_id: home.id.clone(),
        away_id: away.id.clone(),
        home: home_strength,
        away: away_strength,
        minutes: 90,
        seed: args.seed,
    };

    let result = simulate_match(&input);

    println!("━━━━━━━━━ FULL TIME ━━━━━━━━━");
    println!(
        "  {}  {}  -  {}  {}",
        home.name, result.home_goals, result.away_goals, away.name
    );
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    if !result.highlights.is_empty() {
        println!("Match Highlights:");
        for highlight in &result.highlights {
            println!("  {}", highlight);
        }
    }

    Ok(())
}
