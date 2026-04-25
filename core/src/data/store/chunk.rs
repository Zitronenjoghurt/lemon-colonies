use crate::data::entity::{chunk, object};
use crate::data::store::Store;
use crate::error::CoreResult;
use crate::game::chunk::Chunk;
use crate::math::coords::ChunkCoords;
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

    pub async fn load_existing(&self, pos: ChunkCoords) -> CoreResult<Option<Chunk>> {
        let row = chunk::Entity::find_by_id((pos.x, pos.y))
            .find_with_related(object::Entity)
            .all(self.db())
            .await?
            .into_iter()
            .next();
        row.map(Chunk::try_from).transpose()
    }

    pub async fn load_or_generate(&self, pos: ChunkCoords, world_seed: u64) -> CoreResult<Chunk> {
        if let Some(existing) = self.load_existing(pos).await? {
            return Ok(existing);
        }

        let generated_chunk = Chunk::generate(pos, world_seed);
        let chunk_to_insert = chunk::ActiveModel::from(generated_chunk);
        let saved_chunk = self.insert(chunk_to_insert).await?;

        Chunk::try_from(saved_chunk)
    }

    pub async fn load_many(&self, coords: &[ChunkCoords]) -> CoreResult<Vec<Chunk>> {
        let mut condition = Condition::any();
        for &pos in coords {
            condition = condition.add(chunk::Column::X.eq(pos.x).and(chunk::Column::Y.eq(pos.y)));
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
        coords: &[ChunkCoords],
        world_seed: u64,
    ) -> CoreResult<Vec<Chunk>> {
        let existing = self.load_many(coords).await?;
        let existing_set: std::collections::HashSet<ChunkCoords> =
            existing.iter().map(|c| c.pos).collect();

        let mut to_insert = Vec::new();
        for &pos in coords {
            if !existing_set.contains(&pos) {
                to_insert.push(Chunk::generate(pos, world_seed));
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
