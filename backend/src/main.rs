use axum::{http::Method, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::{filter, fmt, prelude::*};

use fclc_backend::config::Config;
use fclc_backend::db::Database;
use fclc_backend::error::ApiError;
use fclc_backend::routes::app_router;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter::Targets::new().with_target("fclc_backend", Level::DEBUG))
        .init();

    info!("Starting FCLC backend v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = Config::load()?;
    info!("Loaded configuration for environment: {}", config.env);

    // Initialize database connection pool
    let db_pool = Database::connect(&config.database_url).await?;
    info!("Database connection established");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
    info!("Database migrations completed");

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any)
        .allow_origin(Any);

    // Create application router
    let app = Router::new()
        .nest("/api", app_router(db_pool))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| ApiError::ServerError(e.to_string()))?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    info!("Shutdown signal received");
}