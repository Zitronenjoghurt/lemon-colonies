use crate::error::CoreResult;
use crate::game::config::GameConfig;
use migration::{Migrator, MigratorTrait};
use sea_orm::sqlx::PgPool;
use sea_orm::{ConnectOptions, DatabaseConnection};
use std::sync::Arc;
use tracing::info;

pub mod entity;
pub mod service;
pub mod store;

pub struct Data {
    connection: DatabaseConnection,
    pub chunk: store::chunk::ChunkStore,
    pub colony: store::colony::ColonyStore,
    pub user: store::user::UserStore,
}

impl Data {
    pub async fn initialize(database_url: impl AsRef<str>) -> CoreResult<Self> {
        let options = ConnectOptions::new(database_url.as_ref());

        info!("Connecting to database...");
        let connection = sea_orm::Database::connect(options).await?;
        info!("Connected to database!");

        let data = Self {
            chunk: store::chunk::ChunkStore::new(&connection),
            colony: store::colony::ColonyStore::new(&connection),
            user: store::user::UserStore::new(&connection),
            connection,
        };
        data.apply_migrations().await?;

        Ok(data)
    }

    pub fn pool(&self) -> &PgPool {
        self.connection.get_postgres_connection_pool()
    }

    async fn apply_migrations(&self) -> CoreResult<()> {
        info!("Applying database migrations...");
        Migrator::up(&self.connection, None).await?;
        info!("Database migrations applied!");
        Ok(())
    }
}
