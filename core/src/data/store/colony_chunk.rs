use crate::data::entity::{colony, colony_chunk, user};
use crate::data::store::Store;
use crate::error::CoreResult;
use sea_orm::sea_query::IntoCondition;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait, Linked, RelationDef, RelationTrait};

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
