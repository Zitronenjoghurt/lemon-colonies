use crate::data::entity::{colony, colony_chunk, user};
use crate::data::store::Store;
use crate::error::CoreResult;
use sea_orm::sea_query::IntoCondition;
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, EntityTrait, Linked, RelationDef, RelationTrait};
use sea_orm::{DatabaseConnection, Set};

pub struct UserStore {
    connection: DatabaseConnection,
}

impl Store for UserStore {
    type Entity = user::Entity;
    type ActiveModel = user::ActiveModel;

    fn db(&self) -> &DatabaseConnection {
        &self.connection
    }
}

impl UserStore {
    pub fn new(connection: &DatabaseConnection) -> Self {
        Self {
            connection: connection.clone(),
        }
    }

    pub async fn find_by_discord_id(
        &self,
        discord_id: impl AsRef<str>,
    ) -> CoreResult<Option<user::Model>> {
        self.find_one_by(user::Column::DiscordId.eq(discord_id.as_ref()))
            .await
    }

    pub async fn create_from_discord(
        &self,
        discord_id: impl AsRef<str>,
        username: impl AsRef<str>,
    ) -> CoreResult<user::Model> {
        let new = user::ActiveModel {
            discord_id: Set(Some(discord_id.as_ref().to_string())),
            username: Set(username.as_ref().to_string()),
            ..Default::default()
        };
        self.insert(new).await
    }

    pub async fn find_by_with_owned_chunks<F>(
        &self,
        filter: F,
    ) -> CoreResult<Option<(user::Model, Vec<colony_chunk::Model>)>>
    where
        F: IntoCondition + Send,
    {
        let result = user::Entity::find()
            .filter(filter)
            .find_with_linked(UserToColonyChunks)
            .all(self.db())
            .await?;

        Ok(result.into_iter().next())
    }
}

pub struct UserToColonyChunks;

impl Linked for UserToColonyChunks {
    type FromEntity = user::Entity;
    type ToEntity = colony_chunk::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            colony::Relation::User.def().rev(),
            colony_chunk::Relation::Colony.def().rev(),
        ]
    }
}
