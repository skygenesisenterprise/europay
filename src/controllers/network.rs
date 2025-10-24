// Network controllers

use axum::{extract::Json, http::StatusCode, response::Json as JsonResponse};
use serde::{Deserialize, Serialize};

use crate::core::network::NetworkMessage;

#[derive(Deserialize)]
pub struct NetworkMessageRequest {
    pub message: NetworkMessage,
}

#[derive(Serialize)]
pub struct NetworkMessageResponse {
    pub response: NetworkMessage,
}

pub async fn handle_network_message(
    Json(payload): Json<NetworkMessageRequest>,
) -> Result<JsonResponse<NetworkMessageResponse>, StatusCode> {
    // For now, just echo the message back
    // In real implementation, process the message
    let response = match payload.message {
        NetworkMessage::Heartbeat(_) => NetworkMessage::Heartbeat(crate::core::network::Heartbeat {
            node_id: uuid::Uuid::new_v4(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }),
        _ => payload.message, // Echo for other messages
    };

    Ok(Json(NetworkMessageResponse { response }))
}