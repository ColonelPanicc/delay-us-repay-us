#[derive(Clone)]
#[allow(dead_code)]
// TODO remove lint once we use this.
pub(crate) struct RailwayStation {
    short_code: String,
    full_name: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
// TODO remove lint once we use this.
#[allow(dead_code)]
pub(crate) enum DelayRepayBehaviour {
    DR15,
    DR30,
    Other,
}

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct RailwayOperator {
    short_code: String,
    full_name: String,
    delay_repay_mode: DelayRepayBehaviour,
}

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct Route {
    starting_station: RailwayStation,
    terminus: RailwayStation,
    stops: Vec<RailwayStation>,
    operators: RailwayOperator,
}

// Allow dead code for now as this doesn't have an implementation.
// TODO: remove this once we have implemented.
#[allow(dead_code)]
pub(crate) trait RailwayLoader {
    fn load_railway_stations(self) -> Vec<RailwayStation>;
    fn load_routes(self) -> Vec<Route>;
    fn load_train_operators(self) -> Vec<RailwayOperator>;
}

#[cfg(test)]
mod tests {
    use crate::{DelayRepayBehaviour, RailwayOperator, RailwayStation, Route};
    use pretty_assertions::assert_eq;

    #[test]
    fn can_construct_railway_station() {
        let station = RailwayStation {
            short_code: "LST".to_string(),
            full_name: "London Liverpool Street".to_string(),
        };
        assert_eq!(station.short_code, "LST");
        assert_eq!(station.full_name, "London Liverpool Street");
    }

    #[test]
    fn can_construct_routes() {
        let start = RailwayStation {
            short_code: "LST".to_string(),
            full_name: "London Liverpool Street".to_string(),
        };
        let mid = RailwayStation {
            short_code: "SRA".to_string(),
            full_name: "Stratford (London)".to_string(),
        };
        let end = RailwayStation {
            short_code: "SNF".to_string(),
            full_name: "Shenfield".to_string(),
        };

        let operator = RailwayOperator {
            short_code: "GA".to_string(),
            full_name: "Greater Anglia".to_string(),
            delay_repay_mode: DelayRepayBehaviour::DR15,
        };

        let route = Route {
            starting_station: start.clone(),
            terminus: end.clone(),
            operators: operator,
            stops: vec![start.clone(), mid.clone(), end.clone()],
        };

        assert_eq!(route.starting_station.full_name, "London Liverpool Street");
        assert_eq!(route.terminus.full_name, "Shenfield");
        assert_eq!(route.operators.short_code, "GA");
        assert_eq!(route.operators.full_name, "Greater Anglia");
        assert_eq!(route.operators.delay_repay_mode, DelayRepayBehaviour::DR15);
        assert_eq!(route.stops.iter().count(), 3);
    }
}
