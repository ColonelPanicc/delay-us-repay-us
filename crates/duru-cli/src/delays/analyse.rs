use crate::input_dir::INPUT_DIR;
use clap::Args;
use nwr_delay_attribution_ingest::DelayAttributionRecord;
use std::fs;
use std::path::Path;
use std::string::String;

#[derive(Args)]
pub struct Analyse {
    /// Filter by an operator code
    #[arg(short, long)]
    operator: Option<String>,
    /// Filter for events with a delay less than the provided value
    #[arg(short, long)]
    lt_mins: Option<f32>,
    /// Filter for events with a delay greater than the provided value
    #[arg(short, long)]
    gt_mins: Option<f32>,
}

impl Analyse {
    pub fn execute(self, dry_run: bool) -> anyhow::Result<()> {
        if dry_run {
            return Ok(());
        }

        print_table_header_to_console();
        for file in fs::read_dir(INPUT_DIR.as_path())? {
            if file.is_ok() {
                let path = file?.path();
                parse_csv(path.as_path())
                    .filter(|record| {
                        // Apply filter for operator if present.
                        self.operator
                            .as_ref()
                            .is_none_or(|op| &record.toc_code == op)
                    })
                    .filter(|record| {
                        // Apply a filter for the delay being greater than a provided value.
                        self.gt_mins
                            .is_none_or(|delay_gt_mins| record.pfpi_minutes > delay_gt_mins)
                    })
                    .filter(|record| {
                        // Apply a filter for the delay being less than a provided value.
                        self.lt_mins
                            .is_none_or(|delay_lt_mins| record.pfpi_minutes < delay_lt_mins)
                    })
                    .for_each(|record| {
                        // Print outputs to console.
                        print_record_to_console(&record);
                    });
            }
        }

        Ok(())
    }
}

fn print_table_header_to_console() {
    println!("Operator,TrainServiceCode,OriginLocationCode,DestLocationCode,DelayMins");
}
fn print_record_to_console(record: &DelayAttributionRecord) {
    println!(
        "{},{},{},{},{}",
        record.toc_code,
        record.train_service_code,
        record.planned_origin_location_code,
        record.planned_dest_location_code,
        record.pfpi_minutes,
    );
}

fn parse_csv(path: &Path) -> Box<dyn Iterator<Item = DelayAttributionRecord>> {
    let reader = csv::Reader::from_path(path).expect("Failed to open csv file");
    let iter = reader
        .into_deserialize::<DelayAttributionRecord>()
        .filter_map(Result::ok);
    Box::new(iter)
}
