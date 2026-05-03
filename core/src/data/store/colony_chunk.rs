use crate::data::entity::{colony, colony_chunk, user};
use crate::data::store::Store;
use crate::error::CoreResult;
use crate::math::coords::ChunkCoords;
use crate::math::rect::Rect;
use futures::{StreamExt, TryStreamExt};
use sea_orm::sea_query::IntoCondition;
use sea_orm::{ColumnTrait, ExprTrait, QueryFilter};
use sea_orm::{DatabaseConnection, EntityTrait, Linked, RelationDef, RelationTrait};
use std::collections::HashSet;

pub struct ColonyChunkStore {
    connection: DatabaseConnection,
}

impl Store for ColonyChunkStore {
    type Entity = colony_chunk::Entity;
    type ActiveModel = colony_chunk::ActiveModel;

    fn db(&self) -> &DatabaseConnection {
        &self.connection
    }
}

impl ColonyChunkStore {
    pub fn new(connection: &DatabaseConnection) -> Self {
        Self {
            connection: connection.clone(),
        }
    }

    pub async fn find_one_by_with_owner<F>(
        &self,
        filter: F,
    ) -> CoreResult<Option<(colony_chunk::Model, user::Model)>>
    where
        F: IntoCondition + Send,
    {
        let result = colony_chunk::Entity::find()
            .filter(filter)
            .find_also_linked(ColonyChunkToUser)
            .one(self.db())
            .await?;

        Ok(result.map(|(chunk, user)| {
            (
                chunk,
                user.expect("colony_chunk always has an owner via colony"),
            )
        }))
    }

    pub async fn find_owned_coords_in_rect(
        &self,
        rect: Rect<i32>,
    ) -> CoreResult<HashSet<ChunkCoords>> {
        let mut stream = self
            .stream_by(
                colony_chunk::Column::ChunkX
                    .between(rect.min.x, rect.max.x)
                    .and(colony_chunk::Column::ChunkY.between(rect.min.y, rect.max.y)),
            )
            .await?;

        let mut coords = HashSet::new();
        while let Some(m) = stream.try_next().await? {
            coords.insert(ChunkCoords::new(m.chunk_x, m.chunk_y));
        }
        Ok(coords)
    }
}

pub struct ColonyChunkToUser;

impl Linked for ColonyChunkToUser {
    type FromEntity = colony_chunk::Entity;
    type ToEntity = user::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            colony_chunk::Relation::Colony.def(),
            colony::Relation::User.def(),
        ]
    }
}
