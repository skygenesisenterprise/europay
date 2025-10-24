// Transaction routes

use axum::{routing::post, Router};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::controllers::transactions;
use crate::models::transactions::PaymentProcessor;

pub fn create_routes(processor: Arc<Mutex<PaymentProcessor>>) -> Router<()> {
    Router::new()
        .route("/authorize", post(transactions::authorize_transaction))
        .route("/capture", post(transactions::capture_transaction))
        .route("/settle", post(transactions::settle_transaction))
        .with_state(processor)
}