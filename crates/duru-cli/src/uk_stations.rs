pub mod download;
pub mod parse;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::uk_stations::download::DownloadCommand;
use crate::uk_stations::parse::ParseCommand;

const DEFAULT_LOCAL_JSON_FILE_NAME: &str = "uk-stations.json";
const DEFAULT_LOCAL_TEXT_FILE_NAME: &str = "uk-stations.txt";

/// The set of actions relating to UK stations.
#[derive(Args)]
pub struct UkStationCommand {
    #[command(subcommand)]
    pub command: UkStationCommands,
}

#[derive(Subcommand)]
pub enum UkStationCommands {
    /// Download raw data to a local cache directory.
    Download(DownloadCommand),

    /// Parse downloaded raw data to extract useful bits.
    Parse(ParseCommand),
}

impl UkStationCommand {
    pub fn execute(self, dry_run: bool) -> Result<()> {
        match self.command {
            UkStationCommands::Download(cmd) => cmd.execute(dry_run),
            UkStationCommands::Parse(cmd) => cmd.execute(dry_run),
        }
    }
}
