use domain::{RailwayOperator, RailwayStation, Route};

pub trait RailwayLoader {
    fn load_railway_stations(self) -> Vec<RailwayStation>;
    fn load_routes(self) -> Vec<Route>;
    fn load_train_operators(self) -> Vec<RailwayOperator>;
}
