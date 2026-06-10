use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Result;
use clap::Args;
use reqwest::blocking::get;

use crate::uk_stations::DEFAULT_LOCAL_FILE_NAME;

const UK_STATIONS_JSON_URL: &str =
    "https://raw.githubusercontent.com/davwheat/uk-railway-stations/refs/heads/main/stations.json";

#[derive(Args)]
pub struct DownloadCommand {
    /// Non-default local file name.
    #[arg(short, long)]
    out: Option<String>,
}

impl DownloadCommand {
    pub fn execute(self, dry_run: bool) -> Result<()> {
        let data_cache_dir = env!("CARGO_MANIFEST_DIR")
            .parse::<PathBuf>()?
            .join("data-cache")
            .canonicalize()?;
        let output_json_path =
            data_cache_dir.join(self.out.unwrap_or(DEFAULT_LOCAL_FILE_NAME.to_string()));

        println!(
            "Attempting to download {UK_STATIONS_JSON_URL} to {out_file}",
            out_file = output_json_path.display()
        );
        if dry_run {
            return Ok(());
        }

        fs::create_dir_all(&data_cache_dir)?;

        let file_bytes = get(UK_STATIONS_JSON_URL)?.bytes()?;
        let mut out_file = File::create(output_json_path)?;
        out_file.write_all(&file_bytes)?;

        println!("Download complete");
        Ok(())
    }
}
