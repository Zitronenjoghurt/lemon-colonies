use crate::data::entity::colony;
use crate::data::store::Store;
use crate::error::CoreResult;
use crate::math::coords::ChunkCoords;
use sea_orm::prelude::Uuid;
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{QueryFilter, QuerySelect};

pub struct ColonyStore {
    connection: DatabaseConnection,
}

impl Store for ColonyStore {
    type Entity = colony::Entity;
    type ActiveModel = colony::ActiveModel;

    fn db(&self) -> &DatabaseConnection {
        &self.connection
    }
}

impl ColonyStore {
    pub fn new(connection: &DatabaseConnection) -> Self {
        Self {
            connection: connection.clone(),
        }
    }

    pub async fn find_coords_by_user_id(&self, user_id: Uuid) -> CoreResult<Vec<ChunkCoords>> {
        let coords = colony::Entity::find()
            .filter(colony::Column::UserId.eq(user_id))
            .select_only()
            .column(colony::Column::ChunkX)
            .column(colony::Column::ChunkY)
            .into_tuple::<(i32, i32)>()
            .all(&self.connection)
            .await?
            .iter()
            .map(|(x, y)| ChunkCoords::new(*x, *y))
            .collect();
        Ok(coords)
    }
}
