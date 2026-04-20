pub use sea_orm_migration::prelude::*;

mod m20260409_181359_init_user;
mod m20260411_110822_init_chunk;
mod m20260411_111450_init_colony;
mod m20260420_121559_init_object;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260409_181359_init_user::Migration),
            Box::new(m20260411_110822_init_chunk::Migration),
            Box::new(m20260411_111450_init_colony::Migration),
            Box::new(m20260420_121559_init_object::Migration),
        ]
    }
}
