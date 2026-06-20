use anyhow::Result;

use crate::DataStore;

impl DataStore {
    /// Truncate the stations collection, effectively forgetting all stations.
    ///
    /// # Errors
    /// May fail to execute the delete query.
    pub async fn truncate_stations(&self) -> Result<()> {
        sqlx::query!("DELETE FROM stations")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Count all known stations.
    ///
    /// # Errors
    /// May fail to execute the count query.
    pub async fn count_stations(&self) -> Result<i64> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) as count from stations")
            .fetch_one(&self.pool)
            .await?;
        Ok(count)
    }

    /// Persist a new station.
    ///
    /// # Errors
    /// May fail to execute the insert query, for example if that station code is already present.
    pub async fn add_station(&self, code: &str, name: &str, lat: f32, lon: f32) -> Result<()> {
        sqlx::query!(
            "INSERT INTO stations (crs, name, lat, lon) VALUES (?, ?, ?, ?)",
            code,
            name,
            lat,
            lon,
        )
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
        store.truncate_stations().await?;

        assert_eq!(0, store.count_stations().await?);

        store.add_station("abc", "AyyBeeSee", 1.0, 2.0).await?;
        store.add_station("def", "DeeEeeEff", 2.0, 1.0).await?;

        assert_eq!(2, store.count_stations().await?);

        Ok(())
    }
}
