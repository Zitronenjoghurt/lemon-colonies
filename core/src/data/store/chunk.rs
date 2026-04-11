use crate::data::entity::chunk;
use crate::data::store::Store;
use crate::error::CoreResult;
use crate::game::chunk::Chunk;
use sea_orm::DatabaseConnection;

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
        let Some(existing) = self.find_by_id((x, y)).await? else {
            return Ok(None);
        };
        Ok(Some(Chunk::try_from(existing)?))
    }

    pub async fn load_or_generate(&self, x: i32, y: i32, world_seed: u64) -> CoreResult<Chunk> {
        if let Some(existing) = self.load_existing(x, y).await? {
            return Ok(existing);
        }

        let generated_chunk = Chunk::generate(x, y, world_seed);
        let chunk_to_insert = chunk::ActiveModel::from(generated_chunk);
        let saved_chunk = self.insert(chunk_to_insert).await?;

        Ok(Chunk::try_from(saved_chunk)?)
    }
}
