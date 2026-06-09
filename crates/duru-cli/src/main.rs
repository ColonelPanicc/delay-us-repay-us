use clap::{Parser, Subcommand};

/// A CLI to enable devs to trigger actions within DURU easily.
///
/// Devs can run locally from the repo root directory with `cargo r -p duru-cli -- --help`
#[derive(Parser)]
#[command(version, arg_required_else_help = true)]
struct Cli {
    /// Do not perform the provided action, but show what it would do.
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,

    /// The provided action, if any. If none, show help.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The set of actions runnable from the CLI.
#[derive(Subcommand)]
enum Commands {}

fn main() {
    let cli = Cli::parse();
    let Some(command) = cli.command else {
        println!("No command provided");
        return;
    };

    if cli.dry_run {
        println!("Dry run is on");
    }

    match command {}
}
