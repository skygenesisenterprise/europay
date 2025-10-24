// Settlement routes

use axum::{routing::post, Router};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::controllers::settlement;
use crate::services::settlement::SettlementService;

pub fn create_routes(settlement_service: Arc<Mutex<SettlementService>>) -> Router<()> {
    Router::new()
        .route("/batch", post(settlement::create_settlement_batch))
        .route("/process", post(settlement::process_settlement))
        .with_state(settlement_service)
}