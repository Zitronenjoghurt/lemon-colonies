use crate::config::Config;
use crate::error::ServerResult;
use crate::integrations::Integrations;
use crate::websocket::Websocket;
use lemon_colonies_core::data::service::Services;
use lemon_colonies_core::data::Data;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerState {
    pub config: Arc<Config>,
    pub data: Arc<Data>,
    pub integrations: Arc<Integrations>,
    pub service: Arc<Services>,
    pub ws: Arc<Websocket>,
}

impl ServerState {
    pub async fn new(config: Config) -> ServerResult<Self> {
        let data = Arc::new(Data::initialize(&config.database_url).await?);
        let service = Services::new(&data);

        Ok(Self {
            data,
            integrations: Arc::new(Integrations::new(&config.integrations)?),
            config: Arc::new(config),
            service: Arc::new(service),
            ws: Arc::new(Websocket::default()),
        })
    }
}
