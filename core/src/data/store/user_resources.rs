use crate::data::chrono_now;
use crate::data::entity::user_resources;
use crate::data::store::Store;
use crate::error::{CoreError, CoreResult};
use crate::game::resource::{ResourceBag, ResourceId};
use sea_orm::sea_query::OnConflict;
use sea_orm::QueryFilter;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DatabaseTransaction, EntityTrait, QuerySelect, Set,
};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub struct UserResourcesStore {
    connection: DatabaseConnection,
}

impl UserResourcesStore {
    pub fn new(connection: &DatabaseConnection) -> Self {
        Self {
            connection: connection.clone(),
        }
    }
}

impl Store for UserResourcesStore {
    type Entity = user_resources::Entity;
    type ActiveModel = user_resources::ActiveModel;

    fn db(&self) -> &DatabaseConnection {
        &self.connection
    }
}

impl UserResourcesStore {
    pub async fn adjust(
        &self,
        txn: &DatabaseTransaction,
        user_id: Uuid,
        adjustments: &[(ResourceId, i64)],
    ) -> CoreResult<ResourceBag> {
        if adjustments.is_empty() {
            return Ok(ResourceBag::default());
        }

        let resource_ids: Vec<i16> = adjustments
            .iter()
            .map(|&(rid, _)| rid as u16 as i16)
            .collect();

        let existing: HashMap<i16, u64> = user_resources::Entity::find()
            .filter(
                Condition::all()
                    .add(user_resources::Column::UserId.eq(user_id))
                    .add(user_resources::Column::ResourceId.is_in(resource_ids)),
            )
            .lock_exclusive()
            .all(txn)
            .await?
            .into_iter()
            .map(|r| (r.resource_id, r.amount as u64))
            .collect();

        let now = chrono_now();
        let mut models = Vec::with_capacity(adjustments.len());
        let mut result = HashMap::with_capacity(adjustments.len());

        for &(resource_id, delta) in adjustments {
            let rid = resource_id as u16 as i16;
            let current = existing.get(&rid).copied().unwrap_or(0);

            let new_amount = if delta >= 0 {
                current.saturating_add(delta as u64)
            } else {
                let abs = delta.unsigned_abs();
                current
                    .checked_sub(abs)
                    .ok_or(CoreError::InsufficientResources {
                        resource_id,
                        available: current,
                        requested: abs,
                    })?
            };

            result.insert(resource_id, new_amount);
            models.push(user_resources::ActiveModel {
                user_id: Set(user_id),
                resource_id: Set(rid),
                amount: Set(new_amount as i64),
                created_at: Set(now),
                updated_at: Set(now),
            });
        }

        user_resources::Entity::insert_many(models)
            .on_conflict(
                OnConflict::columns([
                    user_resources::Column::UserId,
                    user_resources::Column::ResourceId,
                ])
                .update_columns([
                    user_resources::Column::Amount,
                    user_resources::Column::UpdatedAt,
                ])
                .to_owned(),
            )
            .exec(txn)
            .await?;

        Ok(ResourceBag::new(result))
    }

    pub async fn get_multiple(
        &self,
        user_id: Uuid,
        resource_ids: &HashSet<ResourceId>,
    ) -> CoreResult<ResourceBag> {
        let ids: Vec<i16> = resource_ids.iter().map(|&rid| rid as u16 as i16).collect();

        let resources = user_resources::Entity::find()
            .filter(
                Condition::all()
                    .add(user_resources::Column::UserId.eq(user_id))
                    .add(user_resources::Column::ResourceId.is_in(ids)),
            )
            .select_only()
            .columns([
                user_resources::Column::ResourceId,
                user_resources::Column::Amount,
            ])
            .into_tuple::<(i16, i64)>()
            .all(self.db())
            .await?
            .into_iter()
            .map(|(rid, amt)| (ResourceId::from_repr(rid as u16).unwrap(), amt as u64))
            .collect::<HashMap<_, _>>();

        Ok(ResourceBag::new(resources))
    }

    pub async fn get_all(&self, user_id: Uuid) -> CoreResult<ResourceBag> {
        let resources = user_resources::Entity::find()
            .filter(user_resources::Column::UserId.eq(user_id))
            .select_only()
            .columns([
                user_resources::Column::ResourceId,
                user_resources::Column::Amount,
            ])
            .into_tuple::<(i16, i64)>()
            .all(self.db())
            .await?
            .into_iter()
            .map(|(rid, amt)| (ResourceId::from_repr(rid as u16).unwrap(), amt as u64))
            .collect::<HashMap<_, _>>();

        Ok(ResourceBag::new(resources))
    }
}
