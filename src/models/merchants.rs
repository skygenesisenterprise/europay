// Merchant module

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Merchant {
    pub id: Uuid,
    pub name: String,
    pub category: String,
    pub acquirer_id: Uuid, // Bank that processes for merchant
    pub status: MerchantStatus,
    pub registered_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MerchantStatus {
    Active,
    Suspended,
    Closed,
}

impl Merchant {
    pub fn new(name: String, category: String, acquirer_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            category,
            acquirer_id,
            status: MerchantStatus::Active,
            registered_at: Utc::now(),
        }
    }
}