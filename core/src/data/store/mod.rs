use crate::error::{CoreError, CoreResult};
use sea_orm::sea_query::IntoCondition;
use sea_orm::*;

pub mod user;

#[async_trait::async_trait]
pub trait Store {
    type Entity: EntityTrait;
    type ActiveModel: ActiveModelTrait<Entity = Self::Entity> + ActiveModelBehavior + Send;

    fn db(&self) -> &DatabaseConnection;

    async fn find_by_id<K>(&self, id: K) -> CoreResult<Option<<Self::Entity as EntityTrait>::Model>>
    where
        K: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send,
    {
        Self::Entity::find_by_id(id)
            .one(self.db())
            .await
            .map_err(Into::into)
    }

    async fn find_one_by<F>(
        &self,
        filter: F,
    ) -> CoreResult<Option<<Self::Entity as EntityTrait>::Model>>
    where
        F: IntoCondition + Send,
    {
        Self::Entity::find()
            .filter(filter)
            .one(self.db())
            .await
            .map_err(Into::into)
    }

    async fn insert(
        &self,
        model: Self::ActiveModel,
    ) -> CoreResult<<Self::Entity as EntityTrait>::Model>
    where
        <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self::ActiveModel>,
    {
        model.insert(self.db()).await.map_err(Into::into)
    }

    async fn update(
        &self,
        model: Self::ActiveModel,
    ) -> CoreResult<<Self::Entity as EntityTrait>::Model>
    where
        <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self::ActiveModel>,
    {
        model.update(self.db()).await.map_err(Into::into)
    }

    async fn delete_by_id<K>(&self, id: K) -> CoreResult<DeleteResult>
    where
        K: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send,
    {
        Self::Entity::delete_by_id(id)
            .exec(self.db())
            .await
            .map_err(Into::into)
    }

    async fn create_if_not_exists<K, F>(
        &self,
        id: K,
        create: F,
    ) -> CoreResult<<Self::Entity as EntityTrait>::Model>
    where
        K: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>
            + Send
            + Clone,
        F: FnOnce() -> Self::ActiveModel + Send,
        <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self::ActiveModel>,
    {
        let existing = Self::Entity::find_by_id(id.clone())
            .one(self.db())
            .await
            .map_err(CoreError::from)?;

        match existing {
            Some(model) => Ok(model),
            None => create().insert(self.db()).await.map_err(CoreError::from),
        }
    }
}
