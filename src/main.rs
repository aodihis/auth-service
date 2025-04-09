use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::config::load_config;
use crate::routes::error::not_found_handler;

mod config;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let config = match load_config() {
        Ok(cfg) => cfg,
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

    let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(config.database.url.as_str())
            .await?;

    let app = Router::new()
        .nest("/api/health", routes::health::router())
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
