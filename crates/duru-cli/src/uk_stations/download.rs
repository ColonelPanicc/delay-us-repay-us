use crate::{
    data_cache_dir::{DATA_CACHE_DIR, data_cache_file},
    uk_stations::DEFAULT_LOCAL_JSON_FILE_NAME,
};
use anyhow::Result;
use clap::Args;
use reqwest::header::CONTENT_TYPE;
use std::env::var;
use std::fs::{self, File};

const UK_STATIONS_XML_URL: &str =
    "https://publicdatafeeds.networkrail.co.uk/ntrod/SupportingFileAuthenticate?type=CORPUS";

#[derive(Args)]
pub struct DownloadCommand {
    /// Non-default local file name for output.
    #[arg(short, long)]
    out: Option<String>,
}

impl DownloadCommand {
    pub fn execute(self, dry_run: bool) -> Result<()> {
        let output_xml_path =
            data_cache_file(self.out.as_deref().unwrap_or(DEFAULT_LOCAL_JSON_FILE_NAME));

        let username = var("DURU_NETWORK_RAIL_USERNAME")?;
        let password = Some(var("DURU_NETWORK_RAIL_PASSWORD")?);

        println!(
            "Attempting to download {UK_STATIONS_XML_URL} to {out_file}",
            out_file = output_xml_path.display()
        );
        if dry_run {
            return Ok(());
        }

        fs::create_dir_all(DATA_CACHE_DIR.as_path())?;

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(UK_STATIONS_XML_URL)
            .basic_auth(username, password)
            .header(CONTENT_TYPE, "application/json")
            .send()?
            .bytes()?;
        let mut out_file = File::create(output_xml_path)?;
        gzippy::decompress_to_writer(&response, &mut out_file)?;

        println!("Download complete");
        Ok(())
    }
}
