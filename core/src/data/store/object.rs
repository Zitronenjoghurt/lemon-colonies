use crate::data::entity::object;
use crate::data::store::Store;
use sea_orm::DatabaseConnection;

pub struct ObjectStore {
    connection: DatabaseConnection,
}

impl Store for ObjectStore {
    type Entity = object::Entity;
    type ActiveModel = object::ActiveModel;

    fn db(&self) -> &DatabaseConnection {
        &self.connection
    }
}

impl ObjectStore {
    pub fn new(connection: &DatabaseConnection) -> Self {
        Self {
            connection: connection.clone(),
        }
    }
}
