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
    async fn upsert_station(&self, station: &RailwayStation) -> Result<()>;

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

    async fn upsert_station(&self, station: &RailwayStation) -> Result<()> {
        sqlx::query!(
            "INSERT OR REPLACE INTO stations (crs, name, lat, lon) VALUES (?, ?, ?, ?)",
            station.id,
            station.name,
            station.lat,
            station.lon,
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
    use sqlx::SqlitePool;

    const ID_ABC: RailwayStationId = RailwayStationId(['A', 'B', 'C']);
    const ID_DEF: RailwayStationId = RailwayStationId(['D', 'E', 'F']);

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn upsert_then_get(pool: SqlitePool) -> Result<()> {
        let store = DataStore { pool };

        assert_eq!(None, store.get_station(ID_ABC).await?);

        let station_abc = RailwayStation {
            id: ID_ABC,
            name: "Station ABC".to_owned(),
            lat: 1.0,
            lon: 1.1,
        };
        store.upsert_station(&station_abc).await?;

        assert_eq!(Some(station_abc), store.get_station(ID_ABC).await?);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn upsert_then_get_two(pool: SqlitePool) -> Result<()> {
        let store = DataStore { pool };

        assert_eq!(None, store.get_station(ID_ABC).await?);
        assert_eq!(None, store.get_station(ID_DEF).await?);

        let station_abc = RailwayStation {
            id: ID_ABC,
            name: "Station ABC".to_owned(),
            lat: 1.0,
            lon: 1.1,
        };
        store.upsert_station(&station_abc).await?;

        let station_def = RailwayStation {
            id: ID_DEF,
            name: "Station DEF".to_owned(),
            lat: 2.5,
            lon: 2.6,
        };
        store.upsert_station(&station_def).await?;

        assert_eq!(Some(station_abc), store.get_station(ID_ABC).await?);
        assert_eq!(Some(station_def), store.get_station(ID_DEF).await?);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn replace_existing(pool: SqlitePool) -> Result<()> {
        let store = DataStore { pool };

        let station_abc = RailwayStation {
            id: ID_ABC,
            name: "Station ABC".to_owned(),
            lat: 1.0,
            lon: 1.1,
        };
        store.upsert_station(&station_abc).await?;
        assert_eq!(Some(station_abc), store.get_station(ID_ABC).await?);

        let station_abc_updated = RailwayStation {
            id: ID_ABC,
            name: "Station ABC Updated".to_owned(),
            lat: 0.1234,
            lon: 0.5678,
        };
        store.upsert_station(&station_abc_updated).await?;
        assert_eq!(Some(station_abc_updated), store.get_station(ID_ABC).await?);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn delete_one(pool: SqlitePool) -> Result<()> {
        let store = DataStore { pool };

        let station_abc = RailwayStation {
            id: ID_ABC,
            name: "Station ABC".to_owned(),
            lat: 1.0,
            lon: 1.1,
        };
        store.upsert_station(&station_abc).await?;

        let station_def = RailwayStation {
            id: ID_DEF,
            name: "Station DEF".to_owned(),
            lat: 2.5,
            lon: 2.6,
        };
        store.upsert_station(&station_def).await?;

        assert!(store.get_station(ID_ABC).await?.is_some());
        assert!(store.get_station(ID_DEF).await?.is_some());

        store.delete_station(ID_ABC).await?;

        assert!(store.get_station(ID_ABC).await?.is_none());
        assert!(store.get_station(ID_DEF).await?.is_some());

        Ok(())
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn delete_all(pool: SqlitePool) -> Result<()> {
        let store = DataStore { pool };

        let station_abc = RailwayStation {
            id: ID_ABC,
            name: "Station ABC".to_owned(),
            lat: 1.0,
            lon: 1.1,
        };
        store.upsert_station(&station_abc).await?;

        let station_def = RailwayStation {
            id: ID_DEF,
            name: "Station DEF".to_owned(),
            lat: 2.5,
            lon: 2.6,
        };
        store.upsert_station(&station_def).await?;

        assert!(store.get_station(ID_ABC).await?.is_some());
        assert!(store.get_station(ID_DEF).await?.is_some());

        store.delete_all_stations().await?;

        assert!(store.get_station(ID_ABC).await?.is_none());
        assert!(store.get_station(ID_DEF).await?.is_none());

        Ok(())
    }
}
