use crate::data::entity::object;
use crate::data::store::Store;
use crate::error::CoreResult;
use crate::math::coords::ChunkCoords;
use futures::Stream;
use migration::{Condition, ExprTrait};
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use std::collections::HashSet;
use std::pin::Pin;

pub struct ObjectStore {
    connection: DatabaseConnection,
}

impl Store for ObjectStore {
    type Entity = object::Entity;
    type ActiveModel = object::ActiveModel;

    fn db(&self) -> &DatabaseConnection {
        &self.connection
    }
}

impl ObjectStore {
    pub fn new(connection: &DatabaseConnection) -> Self {
        Self {
            connection: connection.clone(),
        }
    }

    pub async fn stream_by_chunks(
        &self,
        chunks: &HashSet<ChunkCoords>,
    ) -> CoreResult<Pin<Box<dyn Stream<Item = CoreResult<object::Model>> + Send + '_>>> {
        let mut condition = Condition::any();
        for coords in chunks {
            condition = condition.add(
                object::Column::ChunkX
                    .eq(coords.x)
                    .and(object::Column::ChunkY.eq(coords.y)),
            );
        }
        Ok(Box::pin(self.stream_by(condition).await?))
    }
}
