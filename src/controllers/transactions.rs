// Transaction controllers

use axum::{extract::{Json, State}, http::StatusCode, response::Json as JsonResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::transactions::PaymentProcessor;

#[derive(Deserialize)]
pub struct AuthorizeRequest {
    pub card_id: Uuid,
    pub merchant_id: Uuid,
    pub amount: f64,
    pub currency: String,
}

#[derive(Serialize)]
pub struct AuthorizeResponse {
    pub transaction_id: Uuid,
}

#[derive(Deserialize)]
pub struct TransactionActionRequest {
    pub transaction_id: Uuid,
}

pub async fn authorize_transaction(
    State(processor): State<Arc<Mutex<PaymentProcessor>>>,
    Json(payload): Json<AuthorizeRequest>,
) -> Result<JsonResponse<AuthorizeResponse>, StatusCode> {
    let mut proc = processor.lock().await;
    match proc.authorize_transaction(payload.card_id, payload.merchant_id, payload.amount, &payload.currency) {
        Ok(tx_id) => Ok(Json(AuthorizeResponse { transaction_id: tx_id })),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn capture_transaction(
    State(processor): State<Arc<Mutex<PaymentProcessor>>>,
    Json(payload): Json<TransactionActionRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut proc = processor.lock().await;
    match proc.capture_transaction(payload.transaction_id) {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn settle_transaction(
    State(processor): State<Arc<Mutex<PaymentProcessor>>>,
    Json(payload): Json<TransactionActionRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut proc = processor.lock().await;
    match proc.settle_transaction(payload.transaction_id) {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}