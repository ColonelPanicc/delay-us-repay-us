use domain::{RailwayOperator, RailwayStationId, Route};

pub trait RailwayLoader {
    fn load_railway_stations(self) -> Vec<RailwayStationId>;
    fn load_routes(self) -> Vec<Route>;
    fn load_train_operators(self) -> Vec<RailwayOperator>;
}
