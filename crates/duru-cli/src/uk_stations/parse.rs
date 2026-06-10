use std::{
    fs::{self, File},
    io::{BufWriter, Write},
};

use anyhow::Result;
use clap::Args;
use serde::Deserialize;

use crate::{
    data_cache_dir::data_cache_file,
    uk_stations::{DEFAULT_LOCAL_JSON_FILE_NAME, DEFAULT_LOCAL_TEXT_FILE_NAME},
};

#[derive(Args)]
pub struct ParseCommand {
    /// Non-default local file name for input.
    #[arg(short, long)]
    r#in: Option<String>,

    /// Non-default local file name for output.
    #[arg(short, long)]
    out: Option<String>,
}

impl ParseCommand {
    pub fn execute(self, dry_run: bool) -> Result<()> {
        let input_json_path =
            data_cache_file(self.r#in.as_deref().unwrap_or(DEFAULT_LOCAL_JSON_FILE_NAME));
        let output_txt_path =
            data_cache_file(self.out.as_deref().unwrap_or(DEFAULT_LOCAL_TEXT_FILE_NAME));

        println!(
            "Attempting to parse {in_file} to store output in {out_file}",
            in_file = input_json_path.display(),
            out_file = output_txt_path.display(),
        );
        if dry_run {
            return Ok(());
        }

        let json_str = fs::read_to_string(input_json_path)?;
        let stations: Vec<Station> = serde_json::from_str(&json_str)?;

        let out = File::create(output_txt_path)?;
        let mut out = BufWriter::new(out);

        for station in stations {
            writeln!(out, "{crs}", crs = station.crs_code)?;
        }

        out.flush()?;

        println!("Parse complete");
        Ok(())
    }
}

/// A single entry of the JSON array in the default data set. Omits fields not currently used.
///
/// For example,
/// ```json
/// {
///     "stationName": "Abbey Wood",
///     "lat": 51.490719,
///     "long": 0.120343,
///     "crsCode": "ABW",
///     "constituentCountry": "england"
/// }
/// ```
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Station {
    crs_code: String,
}
