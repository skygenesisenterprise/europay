// Network protocol for Europay nodes

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::core::currency::Currency;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    TransactionRequest(TransactionRequest),
    TransactionResponse(TransactionResponse),
    SettlementRequest(SettlementRequest),
    SettlementResponse(SettlementResponse),
    Heartbeat(Heartbeat),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRequest {
    pub transaction_id: Uuid,
    pub card_id: Uuid,
    pub merchant_id: Uuid,
    pub amount: f64,
    pub currency: Currency,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub transaction_id: Uuid,
    pub approved: bool,
    pub response_code: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementRequest {
    pub batch_id: Uuid,
    pub transactions: Vec<Uuid>,
    pub total_amount: f64,
    pub currency: Currency,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementResponse {
    pub batch_id: Uuid,
    pub success: bool,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    pub node_id: Uuid,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct NetworkNode {
    pub id: Uuid,
    pub address: String,
    pub role: NodeRole,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeRole {
    Issuer,
    Acquirer,
    Network,
}

impl NetworkNode {
    pub fn new(address: String, role: NodeRole) -> Self {
        Self {
            id: Uuid::new_v4(),
            address,
            role,
        }
    }
}

pub trait NetworkProtocol {
    async fn send_message(&self, to: &NetworkNode, message: NetworkMessage) -> Result<NetworkMessage, String>;
    async fn broadcast(&self, message: NetworkMessage) -> Result<(), String>;
}