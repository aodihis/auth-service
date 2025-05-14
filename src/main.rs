use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use axum::Router;
use sqlx::any::{install_default_drivers, AnyPoolOptions};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::app_state::{AppState, Services};
use crate::config::load_config;
use crate::routes::error::not_found_handler;
use crate::services::authentication::Authentication;
use crate::services::email::EmailService;
use crate::services::users::Users;

mod config;
mod routes;
mod services;
mod models;
mod error;
mod handlers;
mod utils;
mod extractors;
mod app_state;


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let config = match load_config() {
        Ok(cfg) => Arc::new(cfg),
        Err(e) => {
            eprintln!("Config load failed: {e}");
            return Err(anyhow::anyhow!("Config load failed"));
        },
    };


    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    install_default_drivers();
    let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(config.database.url.as_str())
            .await?;

    let email_service = EmailService::new(config.clone());
    let auth_service = Authentication::new(pool.clone(), config.clone());
    let user_service = Users::new(pool, config.clone());
    let state = Arc::new(AppState {
        services: Services {
            email_service,auth_service, user_service
        }
    });
    let app = Router::new()
        .nest("/health", routes::health::router())
        .nest("/user", routes::authentication::router(state))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .fallback(not_found_handler);

    // print!("hos : {}", config.server.host);
    let ip = IpAddr::from_str(&config.server.host)?;
    // println!("Trying to bind to {}", ip);
    let addr = SocketAddr::from((ip, config.server.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("listening on {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
