use anyhow::Result;

use domain::{RailwayStation, RailwayStationId};

use crate::DataStore;

#[trait_variant::make]
pub trait RailwayStationDataStore {
    /// Retrieve all details of a single station.
    ///
    /// # Errors
    /// May fail to execute the select query. A missing station is not considered an error, and will simply return `Ok(None)`.
    async fn get_station(&self, station_id: RailwayStationId) -> Result<Option<RailwayStation>>;

    /// Persist a single new station, or replace an existing one.
    ///
    /// # Errors
    /// May fail to execute the upsert query.
    async fn upsert_station(&self, station: RailwayStation) -> Result<()>;

    /// Delete a single station.
    ///
    /// # Errors
    /// May fail to execute the delete query.
    async fn delete_station(&self, station_id: RailwayStationId) -> Result<()>;

    /// Truncate the stations collection, effectively forgetting all stations.
    ///
    /// # Errors
    /// May fail to execute the delete query.
    async fn delete_all_stations(&self) -> Result<()>;
}

impl RailwayStationDataStore for DataStore {
    async fn get_station(&self, station_id: RailwayStationId) -> Result<Option<RailwayStation>> {
        let station = sqlx::query_as!(
            RailwayStation,
            "SELECT crs as id, name, lat, lon FROM stations WHERE crs = ?",
            &station_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(station)
    }

    async fn upsert_station(&self, station: RailwayStation) -> Result<()> {
        sqlx::query!(
            "INSERT OR REPLACE INTO stations (crs, name, lat, lon) VALUES (?, ?, ?, ?)",
            &station.id,
            &station.name,
            &station.lat,
            &station.lon,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_station(&self, station_id: RailwayStationId) -> Result<()> {
        sqlx::query!("DELETE FROM stations WHERE crs = ?", station_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_all_stations(&self) -> Result<()> {
        sqlx::query!("DELETE FROM stations")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn stations() -> Result<()> {
        let store = DataStore::new_for_tests().await?;
        store.delete_all_stations().await?;

        let id_abc = RailwayStationId(['a', 'b', 'c']);
        assert_eq!(None, store.get_station(id_abc).await?);

        let id_def = RailwayStationId(['d', 'e', 'f']);

        let initial_abc = RailwayStation {
            id: id_abc,
            name: "Initial ABC".to_owned(),
            lat: 1.0,
            lon: 1.1,
        };
        store.upsert_station(initial_abc.clone()).await?;

        store
            .upsert_station(RailwayStation {
                id: id_def,
                name: "Initial DEF".to_owned(),
                lat: 0.0,
                lon: 0.1,
            })
            .await?;

        let fetched_abc = store.get_station(id_abc).await?;
        assert_eq!(Some(initial_abc), fetched_abc);

        let replacement_abc = RailwayStation {
            id: id_abc,
            name: "Replacement ABC".to_owned(),
            lat: 2.0,
            lon: 2.1,
        };
        store.upsert_station(replacement_abc.clone()).await?;

        let fetched_replacement_abc = store.get_station(id_abc).await?;
        assert_eq!(Some(replacement_abc), fetched_replacement_abc);

        store.delete_station(id_def).await?;
        assert!(store.get_station(id_abc).await?.is_some());
        store.delete_station(id_abc).await?;
        assert!(store.get_station(id_abc).await?.is_none());

        Ok(())
    }
}
