//! Import data command - Import world data from JSON files.

use tracing::info;

use cm_data::import::json_importer::JsonWorldImporter;

#[derive(clap::Args, Debug)]
pub struct ImportDataArgs {
    /// Directory containing JSON data files
    #[arg(long, default_value = "assets/data")]
    pub data_dir: String,

    /// Output file for validation report
    #[arg(long)]
    pub report: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Validate data only (don't save)
    #[arg(long)]
    pub validate_only: bool,
}

pub fn run(args: ImportDataArgs) -> anyhow::Result<()> {
    info!("Importing world data from: {}", args.data_dir);
    
    let importer = JsonWorldImporter::new(&args.data_dir);
    
    if args.verbose {
        println!("Reading data files from: {}", args.data_dir);
    }
    
    // Load the world data
    let world = match importer.load_world() {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Error loading world data: {}", e);
            return Err(e.into());
        }
    };
    
    // Generate statistics
    let stats = ImportStats {
        nations: world.nations.len(),
        clubs: world.clubs.len(),
        players: world.players.len(),
        staff: world.staff.len(),
        competitions: world.competitions.len(),
        stadiums: world.stadiums.len(),
        referees: world.referees.len(),
    };
    
    // Output results
    println!("\n=== Import Summary ===");
    println!("Nations:      {}", stats.nations);
    println!("Clubs:        {}", stats.clubs);
    println!("Players:      {}", stats.players);
    println!("Staff:        {}", stats.staff);
    println!("Competitions: {}", stats.competitions);
    println!("Stadiums:     {}", stats.stadiums);
    println!("Referees:     {}", stats.referees);
    
    if args.verbose {
        println!("\n=== Detailed Information ===");
        
        println!("\nNations:");
        for (id, nation) in world.nations.iter().take(10) {
            println!("  {} - {}", id, nation.name);
        }
        if world.nations.len() > 10 {
            println!("  ... and {} more", world.nations.len() - 10);
        }
        
        println!("\nClubs:");
        for (id, club) in world.clubs.iter().take(10) {
            println!("  {} - {} ({} players)", id, club.name, club.player_ids.len());
        }
        if world.clubs.len() > 10 {
            println!("  ... and {} more", world.clubs.len() - 10);
        }
        
        println!("\nCompetitions:");
        for (id, comp) in world.competitions.iter().take(10) {
            println!("  {} - {}", id, comp.name);
        }
        if world.competitions.len() > 10 {
            println!("  ... and {} more", world.competitions.len() - 10);
        }
    }
    
    // Validation checks
    let mut warnings = Vec::new();
    let mut errors = Vec::new();
    
    // Check for clubs with no players
    for (id, club) in &world.clubs {
        if club.player_ids.is_empty() {
            warnings.push(format!("Club {} ({}) has no players", id, club.name));
        }
    }
    
    // Check for players without clubs
    let orphan_players: Vec<_> = world.players.values()
        .filter(|p| p.club_id.is_none())
        .collect();
    if !orphan_players.is_empty() {
        warnings.push(format!("{} players have no club assignment", orphan_players.len()));
    }
    
    // Check for competitions without teams
    for (id, comp) in &world.competitions {
        if comp.teams.is_empty() {
            warnings.push(format!("Competition {} ({}) has no teams", id, comp.name));
        }
    }
    
    // Output validation results
    if !warnings.is_empty() {
        println!("\n=== Warnings ===");
        for warning in &warnings {
            println!("⚠ {}", warning);
        }
    }
    
    if !errors.is_empty() {
        println!("\n=== Errors ===");
        for error in &errors {
            println!("✗ {}", error);
        }
        return Err(anyhow::anyhow!("Validation failed with {} errors", errors.len()));
    }
    
    // Write report if requested
    if let Some(report_path) = args.report {
        write_report(&report_path, &stats, &warnings, &errors)?;
        println!("\nReport written to: {}", report_path);
    }
    
    if args.validate_only {
        println!("\n✓ Validation complete (data not saved)");
    } else {
        println!("\n✓ Import complete");
    }
    
    Ok(())
}

struct ImportStats {
    nations: usize,
    clubs: usize,
    players: usize,
    staff: usize,
    competitions: usize,
    stadiums: usize,
    referees: usize,
}

fn write_report(
    path: &str,
    stats: &ImportStats,
    warnings: &[String],
    errors: &[String],
) -> anyhow::Result<()> {
    use std::fs::File;
    use std::io::Write;
    
    let mut file = File::create(path)?;
    
    writeln!(file, "CM Data Import Report")?;
    writeln!(file, "=====================")?;
    writeln!(file)?;
    writeln!(file, "Statistics:")?;
    writeln!(file, "  Nations:      {}", stats.nations)?;
    writeln!(file, "  Clubs:        {}", stats.clubs)?;
    writeln!(file, "  Players:      {}", stats.players)?;
    writeln!(file, "  Staff:        {}", stats.staff)?;
    writeln!(file, "  Competitions: {}", stats.competitions)?;
    writeln!(file, "  Stadiums:     {}", stats.stadiums)?;
    writeln!(file, "  Referees:     {}", stats.referees)?;
    writeln!(file)?;
    
    if !warnings.is_empty() {
        writeln!(file, "Warnings ({}):", warnings.len())?;
        for warning in warnings {
            writeln!(file, "  - {}", warning)?;
        }
        writeln!(file)?;
    }
    
    if !errors.is_empty() {
        writeln!(file, "Errors ({}):", errors.len())?;
        for error in errors {
            writeln!(file, "  - {}", error)?;
        }
        writeln!(file)?;
    }
    
    writeln!(file, "Status: {}", if errors.is_empty() { "PASS" } else { "FAIL" })?;
    
    Ok(())
}
