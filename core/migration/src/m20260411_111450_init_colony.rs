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
                    .col(integer(Colony::ChunkX).not_null())
                    .col(integer(Colony::ChunkY).not_null())
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
                    .primary_key(Index::create().col(Colony::ChunkX).col(Colony::ChunkY))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Colony::Table, Colony::UserId)
                            .to(User::Table, User::Id)
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

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
enum Colony {
    Table,
    ChunkX,
    ChunkY,
    UserId,
    CreatedAt,
    UpdatedAt,
}
