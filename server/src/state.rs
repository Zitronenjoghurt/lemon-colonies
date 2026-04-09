use crate::config::Config;
use crate::error::ServerResult;
use crate::integrations::Integrations;
use lemon_colonies_core::data::Data;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerState {
    pub config: Arc<Config>,
    pub data: Arc<Data>,
    pub integrations: Arc<Integrations>,
}

impl ServerState {
    pub async fn new(config: Config) -> ServerResult<Self> {
        Ok(Self {
            data: Arc::new(Data::initialize(&config.database_url).await?),
            integrations: Arc::new(Integrations::new(&config.integrations)?),
            config: Arc::new(config),
        })
    }
}
