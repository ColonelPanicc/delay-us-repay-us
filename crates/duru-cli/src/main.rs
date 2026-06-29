mod completions;
mod data_cache_dir;
mod delays;
mod input_dir;
mod uk_stations;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{
    completions::CompletionsCommand, delays::DelaysCommand, uk_stations::UkStationCommand,
};

/// A CLI to enable devs to trigger actions within DURU easily.
///
/// Devs can run locally from the repo root directory with `cargo r -p duru-cli -- --help`,
/// or with the provided cargo alias `cargo duru --help`.
#[derive(Parser)]
#[command(version, arg_required_else_help = true)]
struct Cli {
    /// Do not perform the provided action, but show what it would do.
    #[arg(short, long, default_value_t = false, global = true)]
    dry_run: bool,

    /// The provided action, if any. If none, show help.
    #[command(subcommand)]
    command: Commands,
}

/// The set of actions runnable from the CLI.
#[derive(Subcommand)]
enum Commands {
    #[command(subcommand_negates_reqs(true))]
    Completions(CompletionsCommand),

    #[command(visible_alias("uk"))]
    UkStations(UkStationCommand),

    #[command(visible_alias("delay"))]
    Delays(DelaysCommand),
}

impl Commands {
    fn execute(self, dry_run: bool) -> Result<()> {
        match self {
            Commands::Completions(cmd) => {
                cmd.execute();
                Ok(())
            }
            Commands::UkStations(cmd) => cmd.execute(dry_run),
            Commands::Delays(cmd) => cmd.execute(dry_run),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let dry_run = cli.dry_run;
    if dry_run {
        println!("Dry run is on");
    }

    if let Err(e) = cli.command.execute(dry_run) {
        panic!("Failed to execute command - {e:?}");
    }
}
