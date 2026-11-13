pub mod auth;
pub mod automation;
pub mod cli;
pub mod config;
pub mod db;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod ws;

use std::sync::Arc;
use sqlx::MySqlPool;
pub use config::Config;
pub use ws::WsHub;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    pub config: Config,
    pub ws_hub: Arc<WsHub>,
}
