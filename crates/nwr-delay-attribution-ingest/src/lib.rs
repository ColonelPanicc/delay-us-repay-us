use serde::Deserialize;
use serde_aux::prelude::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(dead_code)]
pub struct DelayAttributionRecord {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub planned_origin_location_code: u16,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub planned_dest_location_code: u16,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub train_service_code: u32,
    pub toc_code: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub pfpi_minutes: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_deserialize_csv_record() {
        let example_json = r#"
        {
            "PLANNED_ORIGIN_LOCATION_CODE": "5381",
            "PLANNED_DEST_LOCATION_CODE": "535",
            "TRAIN_SERVICE_CODE": "101",
            "TOC_CODE": "GA",
            "PFPI_MINUTES": "1.25"
        }
        "#;

        let record: DelayAttributionRecord = serde_json::from_str(example_json).unwrap();
        assert_eq!(record.planned_origin_location_code, 5381);
        assert_eq!(record.planned_dest_location_code, 535);
        assert_eq!(record.train_service_code, 101);
        assert_eq!(record.toc_code, "GA");

        let pfpi_minutes_within_tolerance =
            record.pfpi_minutes >= 1.25 && record.pfpi_minutes <= 1.251;
        assert!(pfpi_minutes_within_tolerance);
    }
}
