pub mod download;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::uk_stations::download::DownloadCommand;

const DEFAULT_LOCAL_FILE_NAME: &str = "uk-stations.json";

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
}

impl UkStationCommand {
    pub fn execute(self, dry_run: bool) -> Result<()> {
        match self.command {
            UkStationCommands::Download(cmd) => cmd.execute(dry_run),
        }
    }
}
