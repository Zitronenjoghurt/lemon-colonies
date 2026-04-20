use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Object::Table)
                    .if_not_exists()
                    .col(pk_uuid(Object::Id).default(Expr::cust("gen_random_uuid()")))
                    .col(small_integer(Object::Kind))
                    .col(json_binary(Object::Data))
                    .col(integer(Object::ChunkX))
                    .col(integer(Object::ChunkY))
                    .col(small_integer(Object::X))
                    .col(small_integer(Object::Y))
                    .col(timestamp(Object::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Object::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Object::Table)
                            .from_col(Object::ChunkX)
                            .from_col(Object::ChunkY)
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
                "CREATE TRIGGER trigger_object_updated_at
                     BEFORE UPDATE ON object
                     FOR EACH ROW
                     EXECUTE FUNCTION set_updated_at();",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TRIGGER IF EXISTS trigger_object_updated_at ON object;")
            .await?;

        manager
            .drop_table(Table::drop().table(Object::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Chunk {
    Table,
    X,
    Y,
}

#[derive(DeriveIden)]
enum Object {
    Table,
    Id,
    Kind,
    Data,
    ChunkX,
    ChunkY,
    X,
    Y,
    CreatedAt,
    UpdatedAt,
}
