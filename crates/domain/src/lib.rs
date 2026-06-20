use std::time::Duration;

#[derive(Clone, Debug)]
pub struct RailwayStationId(pub [char; 3]);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DelayRepayMode {
    DR15,
    DR30,
}

impl DelayRepayMode {
    #[must_use]
    pub fn minimum_delay(self) -> Duration {
        match self {
            DelayRepayMode::DR15 => Duration::from_mins(15),
            DelayRepayMode::DR30 => Duration::from_mins(30),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RailwayOperator {
    pub short_code: String,
    pub full_name: String,
    pub delay_repay_mode: DelayRepayMode,
}

#[derive(Clone, Debug)]
pub struct Route {
    pub starting_station: RailwayStationId,
    pub terminus: RailwayStationId,
    pub stops: Vec<RailwayStationId>,
    pub operator: RailwayOperator,
}

#[cfg(test)]
mod tests {
    use crate::{DelayRepayMode, RailwayOperator, RailwayStationId, Route};
    use pretty_assertions::assert_eq;
    use std::time::Duration;

    #[test]
    fn can_construct_railway_station() {
        let station = RailwayStationId(['L', 'S', 'T']);
        assert_eq!(station.0.len(), 3);
    }

    #[test]
    fn can_construct_routes() {
        let start = RailwayStationId(['L', 'S', 'T']);
        let mid = RailwayStationId(['S', 'R', 'A']);
        let end = RailwayStationId(['S', 'N', 'F']);

        let operator = RailwayOperator {
            short_code: "GA".to_string(),
            full_name: "Greater Anglia".to_string(),
            delay_repay_mode: DelayRepayMode::DR15,
        };

        let route = Route {
            starting_station: start.clone(),
            terminus: end.clone(),
            operator,
            stops: vec![start, mid, end],
        };

        assert_eq!(route.operator.short_code, "GA");
        assert_eq!(route.operator.full_name, "Greater Anglia");
        assert_eq!(route.operator.delay_repay_mode, DelayRepayMode::DR15);
        assert_eq!(route.stops.len(), 3);
    }

    #[test]
    fn dr15_applies_from_15_mins_delay() {
        assert_eq!(
            DelayRepayMode::DR15.minimum_delay(),
            Duration::from_mins(15)
        );
    }

    #[test]
    fn dr30_applies_from_30_mins_delay() {
        assert_eq!(
            DelayRepayMode::DR30.minimum_delay(),
            Duration::from_mins(30)
        );
    }
}
