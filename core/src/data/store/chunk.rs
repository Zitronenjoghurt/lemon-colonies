use crate::data::entity::{chunk, object};
use crate::data::store::Store;
use crate::error::CoreResult;
use crate::game::chunk::Chunk;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{Condition, DatabaseConnection, EntityTrait, ExprTrait};

pub struct ChunkStore {
    connection: DatabaseConnection,
}

impl Store for ChunkStore {
    type Entity = chunk::Entity;
    type ActiveModel = chunk::ActiveModel;

    fn db(&self) -> &DatabaseConnection {
        &self.connection
    }
}

impl ChunkStore {
    pub fn new(connection: &DatabaseConnection) -> Self {
        Self {
            connection: connection.clone(),
        }
    }

    pub async fn load_existing(&self, x: i32, y: i32) -> CoreResult<Option<Chunk>> {
        let row = chunk::Entity::find_by_id((x, y))
            .find_with_related(object::Entity)
            .all(self.db())
            .await?
            .into_iter()
            .next();
        row.map(Chunk::try_from).transpose()
    }

    pub async fn load_or_generate(&self, x: i32, y: i32, world_seed: u64) -> CoreResult<Chunk> {
        if let Some(existing) = self.load_existing(x, y).await? {
            return Ok(existing);
        }

        let generated_chunk = Chunk::generate(x, y, world_seed);
        let chunk_to_insert = chunk::ActiveModel::from(generated_chunk);
        let saved_chunk = self.insert(chunk_to_insert).await?;

        Chunk::try_from(saved_chunk)
    }

    pub async fn load_many(&self, coords: &[(i32, i32)]) -> CoreResult<Vec<Chunk>> {
        let mut condition = Condition::any();
        for &(x, y) in coords {
            condition = condition.add(chunk::Column::X.eq(x).and(chunk::Column::Y.eq(y)));
        }
        let rows = chunk::Entity::find()
            .filter(condition)
            .find_with_related(object::Entity)
            .all(self.db())
            .await?;
        rows.into_iter().map(Chunk::try_from).collect()
    }

    pub async fn load_or_generate_many(
        &self,
        coords: &[(i32, i32)],
        world_seed: u64,
    ) -> CoreResult<Vec<Chunk>> {
        let existing = self.load_many(coords).await?;
        let existing_set: std::collections::HashSet<(i32, i32)> =
            existing.iter().map(|c| (c.x, c.y)).collect();

        let mut to_insert = Vec::new();
        for &(x, y) in coords {
            if !existing_set.contains(&(x, y)) {
                to_insert.push(Chunk::generate(x, y, world_seed));
            }
        }

        if !to_insert.is_empty() {
            let models: Vec<chunk::ActiveModel> = to_insert
                .iter()
                .cloned()
                .map(chunk::ActiveModel::from)
                .collect();
            chunk::Entity::insert_many(models).exec(self.db()).await?;
        }

        let mut all = existing;
        all.extend(to_insert);
        Ok(all)
    }
}
