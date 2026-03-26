//! CM CLI - Command-line interface for CM game.

use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

use cm_telemetry::tracing::init_tracing;

mod commands;
mod errors;
mod output;

#[derive(Parser, Debug)]
#[command(name = "cm", version, about = "CM01/02-style manager sim (Rust)")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Create a new game
    NewGame(commands::new_game::NewGameArgs),
    /// Advance simulation by days
    AdvanceDay(commands::advance_day::AdvanceDayArgs),
    /// Simulate a single match
    SimulateMatch(commands::simulate_match::SimulateMatchArgs),
    /// Import world data from JSON files
    ImportData(commands::import_data::ImportDataArgs),
    /// Export save data to various formats
    ExportSave(commands::export_save::ExportSaveArgs),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Set up tracing based on verbosity
    let filter = if cli.verbose {
        EnvFilter::from_default_env().add_directive("debug".parse()?)
    } else {
        EnvFilter::from_default_env().add_directive("info".parse()?)
    };
    init_tracing(filter);

    match cli.cmd {
        Command::NewGame(args) => commands::new_game::run(args),
        Command::AdvanceDay(args) => commands::advance_day::run(args),
        Command::SimulateMatch(args) => commands::simulate_match::run(args),
        Command::ImportData(args) => commands::import_data::run(args),
        Command::ExportSave(args) => commands::export_save::run(args),
    }
}
