// Network routes

use axum::{routing::post, Router};

use crate::controllers::network;

pub fn create_routes() -> Router {
    Router::new()
        .route("/message", post(network::handle_network_message))
}