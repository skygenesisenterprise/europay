mod config;
mod middlewares;
mod controllers;
mod services;
mod models;
mod routes;
mod utils;
mod core;
mod queries;
mod tests;
mod scripts;

use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing;

use models::transactions::PaymentProcessor;
use routes::transactions;
use config::Config;
use middlewares::logging_middleware;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::default();

    // Create shared state
    let processor = Arc::new(Mutex::new(PaymentProcessor::new()));

    // Build the application
    let app = Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        .nest("/transactions", transactions::create_routes(processor.clone()))
        .layer(axum::middleware::from_fn(logging_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    // Run the server
    let addr = SocketAddr::from((config.server.host.parse::<std::net::IpAddr>().unwrap(), config.server.port));
    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
