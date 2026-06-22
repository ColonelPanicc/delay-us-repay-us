pub mod stations;

use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePool},
};

use crate::stations::RailwayStationDataStore;

/// A shared migrator for prod and tests to use, without duplicating the codegen.
pub(crate) static MIGRATOR: Migrator = sqlx::migrate!();

/// A trait to group together the more specialised data store traits into a single named trait for easy use in trait bounds.
pub trait RailwayDataStore: RailwayStationDataStore {}

#[derive(Debug)]
pub struct DataStore {
    pool: SqlitePool,
}

impl DataStore {
    async fn new_for_db_url(database_url: &str) -> Result<Self> {
        let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await?;
        MIGRATOR.run(&pool).await?;
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
