use std::{
    fs::{self, File},
    io::Write,
};

use anyhow::Result;
use clap::Args;
use reqwest::blocking::get;

use crate::{
    data_cache_dir::{DATA_CACHE_DIR, data_cache_file},
    uk_stations::DEFAULT_LOCAL_JSON_FILE_NAME,
};

const UK_STATIONS_JSON_URL: &str =
    "https://raw.githubusercontent.com/davwheat/uk-railway-stations/refs/heads/main/stations.json";

#[derive(Args)]
pub struct DownloadCommand {
    /// Non-default local file name for output.
    #[arg(short, long)]
    out: Option<String>,
}

impl DownloadCommand {
    pub fn execute(self, dry_run: bool) -> Result<()> {
        let output_json_path =
            data_cache_file(self.out.as_deref().unwrap_or(DEFAULT_LOCAL_JSON_FILE_NAME));

        println!(
            "Attempting to download {UK_STATIONS_JSON_URL} to {out_file}",
            out_file = output_json_path.display()
        );
        if dry_run {
            return Ok(());
        }

        fs::create_dir_all(DATA_CACHE_DIR.as_path())?;

        let file_bytes = get(UK_STATIONS_JSON_URL)?.bytes()?;
        let mut out_file = File::create(output_json_path)?;
        out_file.write_all(&file_bytes)?;

        println!("Download complete");
        Ok(())
    }
}
