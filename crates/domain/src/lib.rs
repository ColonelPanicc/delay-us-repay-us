use std::time::Duration;

#[derive(Clone, Debug)]
pub struct RailwayStation {
    pub short_code: [char; 3],
    pub full_name: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DelayRepayScheme {
    DR15,
    DR30,
}

impl DelayRepayScheme {
    #[must_use]
    pub fn minimum_delay(self) -> Duration {
        match self {
            DelayRepayScheme::DR15 => Duration::from_mins(15),
            DelayRepayScheme::DR30 => Duration::from_mins(30),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RailwayOperator {
    pub short_code: String,
    pub full_name: String,
    pub delay_repay_mode: DelayRepayScheme,
}

#[derive(Clone, Debug)]
pub struct Route {
    pub starting_station: RailwayStation,
    pub terminus: RailwayStation,
    pub stops: Vec<RailwayStation>,
    pub operator: RailwayOperator,
}

#[cfg(test)]
mod tests {
    use crate::{DelayRepayScheme, RailwayOperator, RailwayStation, Route};
    use pretty_assertions::assert_eq;
    use std::time::Duration;

    #[test]
    fn can_construct_railway_station() {
        let station = RailwayStation {
            short_code: ['L', 'S', 'T'],
            full_name: "London Liverpool Street".to_string(),
        };
        assert_eq!(station.short_code.len(), 3);
        assert_eq!(station.full_name, "London Liverpool Street");
    }

    #[test]
    fn can_construct_routes() {
        let start = RailwayStation {
            short_code: ['L', 'S', 'T'],
            full_name: "London Liverpool Street".to_string(),
        };
        let mid = RailwayStation {
            short_code: ['S', 'R', 'A'],
            full_name: "Stratford (London)".to_string(),
        };
        let end = RailwayStation {
            short_code: ['S', 'N', 'F'],
            full_name: "Shenfield".to_string(),
        };

        let operator = RailwayOperator {
            short_code: "GA".to_string(),
            full_name: "Greater Anglia".to_string(),
            delay_repay_mode: DelayRepayScheme::DR15,
        };

        let route = Route {
            starting_station: start.clone(),
            terminus: end.clone(),
            operator,
            stops: vec![start, mid, end],
        };

        assert_eq!(route.starting_station.full_name, "London Liverpool Street");
        assert_eq!(route.terminus.full_name, "Shenfield");
        assert_eq!(route.operator.short_code, "GA");
        assert_eq!(route.operator.full_name, "Greater Anglia");
        assert_eq!(route.operator.delay_repay_mode, DelayRepayScheme::DR15);
        assert_eq!(route.stops.len(), 3);
    }

    #[test]
    fn dr15_applies_from_15_mins_delay() {
        assert_eq!(
            DelayRepayScheme::DR15.minimum_delay(),
            Duration::from_mins(15)
        );
    }

    #[test]
    fn dr30_applies_from_30_mins_delay() {
        assert_eq!(
            DelayRepayScheme::DR30.minimum_delay(),
            Duration::from_mins(30)
        );
    }
}
