use std::{sync::Arc, time::Duration};
use sqlx::MySqlPool;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod automation;
mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod ws;

use automation::AutomationEngine;
use config::Config;
use ws::WsHub;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    pub config: Config,
    pub ws_hub: Arc<WsHub>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,epictask_backend=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    info!("Starting EpicTask Backend on {}:{}", config.host, config.port);

    let db_pool = match db::create_pool(&config.database_url).await {
        Ok(pool) => {
            info!("Connected to MariaDB successfully at {}", config.database_url);
            pool
        }
        Err(e) => {
            error!("Failed to connect to MariaDB: {}", e);
            return Err(e.into());
        }
    };

    // Auto-verify schema
    if let Err(e) = db::init_db(&db_pool).await {
        error!("Database initialization error: {}", e);
    }

    // Auto-seed if database is brand new
    if let Err(e) = handlers::seed::run_seed(&db_pool).await {
        error!("Database auto-seed error: {}", e);
    }

    let ws_hub = Arc::new(WsHub::new());

    let state = AppState {
        db: db_pool.clone(),
        config: config.clone(),
        ws_hub: ws_hub.clone(),
    };

    // Spawn background task for time-based automations (FR-5.2 Due date alerts)
    let bg_pool = db_pool.clone();
    let bg_ws = ws_hub.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            AutomationEngine::check_due_date_alerts(&bg_pool, &bg_ws).await;
        }
    });

    let app = routes::create_router(state);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("🚀 EpicTask Server is live at http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
