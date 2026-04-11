use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chunk::Table)
                    .if_not_exists()
                    .col(integer(Chunk::X).not_null())
                    .col(integer(Chunk::Y).not_null())
                    .col(binary_len(Chunk::Terrain, 2048).not_null())
                    .col(
                        timestamp(Chunk::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(Chunk::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(Index::create().col(Chunk::X).col(Chunk::Y))
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TRIGGER trigger_chunk_updated_at
                     BEFORE UPDATE ON chunk
                     FOR EACH ROW
                     EXECUTE FUNCTION set_updated_at();",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TRIGGER IF EXISTS trigger_chunk_updated_at ON chunk;")
            .await?;

        manager
            .drop_table(Table::drop().table(Chunk::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Chunk {
    Table,
    X,
    Y,
    Terrain,
    CreatedAt,
    UpdatedAt,
}
