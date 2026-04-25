use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Colony::Table)
                    .if_not_exists()
                    .col(integer(Colony::OriginChunkX).not_null())
                    .col(integer(Colony::OriginChunkY).not_null())
                    .col(uuid(Colony::UserId).not_null())
                    .col(
                        timestamp(Colony::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(Colony::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(Colony::OriginChunkX)
                            .col(Colony::OriginChunkY),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Colony::Table, Colony::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Colony::Table)
                            .from_col(Colony::OriginChunkX)
                            .from_col(Colony::OriginChunkY)
                            .to_tbl(Chunk::Table)
                            .to_col(Chunk::X)
                            .to_col(Chunk::Y)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TRIGGER trigger_colony_updated_at
                     BEFORE UPDATE ON colony
                     FOR EACH ROW
                     EXECUTE FUNCTION set_updated_at();",
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ColonyChunk::Table)
                    .if_not_exists()
                    .col(integer(ColonyChunk::ChunkX).not_null())
                    .col(integer(ColonyChunk::ChunkY).not_null())
                    .col(integer(ColonyChunk::ColonyX).not_null())
                    .col(integer(ColonyChunk::ColonyY).not_null())
                    .col(
                        timestamp(ColonyChunk::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(ColonyChunk::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(ColonyChunk::ChunkX)
                            .col(ColonyChunk::ChunkY),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(ColonyChunk::Table)
                            .from_col(ColonyChunk::ColonyX)
                            .from_col(ColonyChunk::ColonyY)
                            .to_tbl(Colony::Table)
                            .to_col(Colony::OriginChunkX)
                            .to_col(Colony::OriginChunkY)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(ColonyChunk::Table)
                            .from_col(ColonyChunk::ChunkX)
                            .from_col(ColonyChunk::ChunkY)
                            .to_tbl(Chunk::Table)
                            .to_col(Chunk::X)
                            .to_col(Chunk::Y)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TRIGGER trigger_colony_chunk_updated_at
                     BEFORE UPDATE ON colony_chunk
                     FOR EACH ROW
                     EXECUTE FUNCTION set_updated_at();",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "DROP TRIGGER IF EXISTS trigger_colony_chunk_updated_at ON colony_chunk;",
            )
            .await?;

        manager
            .drop_table(Table::drop().table(ColonyChunk::Table).to_owned())
            .await?;

        manager
            .get_connection()
            .execute_unprepared("DROP TRIGGER IF EXISTS trigger_colony_updated_at ON colony;")
            .await?;

        manager
            .drop_table(Table::drop().table(Colony::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Chunk {
    Table,
    X,
    Y,
}

#[derive(DeriveIden)]
enum Colony {
    Table,
    OriginChunkX,
    OriginChunkY,
    UserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ColonyChunk {
    Table,
    ChunkX,
    ChunkY,
    ColonyX,
    ColonyY,
    CreatedAt,
    UpdatedAt,
}
