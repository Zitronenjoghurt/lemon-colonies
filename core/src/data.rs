use crate::error::CoreResult;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, DatabaseConnection};
use tracing::info;

pub struct Data {
    connection: DatabaseConnection,
}

impl Data {
    pub async fn initialize(database_url: impl AsRef<str>) -> CoreResult<Self> {
        let options = ConnectOptions::new(database_url.as_ref());

        info!("Connecting to database...");
        let connection = sea_orm::Database::connect(options).await?;
        info!("Connected to database!");

        let data = Self { connection };
        data.apply_migrations().await?;

        Ok(data)
    }

    async fn apply_migrations(&self) -> CoreResult<()> {
        info!("Applying database migrations...");
        Migrator::up(&self.connection, None).await?;
        info!("Database migrations applied!");
        Ok(())
    }
}
