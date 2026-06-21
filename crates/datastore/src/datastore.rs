pub mod stations;

use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

#[derive(Debug)]
pub struct DataStore {
    pool: SqlitePool,
}

impl DataStore {
    async fn new_for_db_url(database_url: &str) -> Result<Self> {
        let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(DataStore { pool })
    }

    /// Create a new instance using environment variable `DATABASE_URL` from the `.env` or the environment itself.
    ///
    /// # Errors
    /// May fail to determine the `.env` file path.
    /// May fail to read the `DATABASE_URL` var.
    /// May fail to connect to the database or run its migrations.
    pub async fn new_for_env() -> Result<Self> {
        dotenvy::from_path(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join(".env")
                .canonicalize()?,
        )?;
        let database_url = dotenvy::var("DATABASE_URL")?;

        Self::new_for_db_url(&database_url).await
    }
}

#[cfg(test)]
mod test_impl {
    use super::*;

    impl DataStore {
        /// Create a new instance using a hardcoded database url that is used only for testing.
        ///
        /// # Errors
        /// May fail to connect to the database or run its migrations.
        pub async fn new_for_tests() -> Result<Self> {
            Self::new_for_db_url("sqlite:duru-test.db").await
        }
    }
}
