// Settlement controllers

use axum::{extract::{Json, State}, http::StatusCode, response::Json as JsonResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::services::settlement::SettlementService;

#[derive(Deserialize)]
pub struct CreateBatchRequest {
    pub issuer_id: Uuid,
    pub acquirer_id: Uuid,
    pub transaction_ids: Vec<Uuid>,
}

#[derive(Serialize)]
pub struct CreateBatchResponse {
    pub batch_id: Uuid,
}

#[derive(Deserialize)]
pub struct ProcessSettlementRequest {
    pub batch_id: Uuid,
}

pub async fn create_settlement_batch(
    State(settlement_service): State<Arc<Mutex<SettlementService>>>,
    Json(payload): Json<CreateBatchRequest>,
) -> Result<JsonResponse<CreateBatchResponse>, StatusCode> {
    // For now, create empty batch (in real system, fetch transactions)
    let mut service = settlement_service.lock().await;
    let batch_id = service.create_batch(payload.issuer_id, payload.acquirer_id, vec![]);

    Ok(Json(CreateBatchResponse { batch_id }))
}

pub async fn process_settlement(
    State(settlement_service): State<Arc<Mutex<SettlementService>>>,
    Json(payload): Json<ProcessSettlementRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut service = settlement_service.lock().await;
    match service.process_settlement(payload.batch_id) {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}