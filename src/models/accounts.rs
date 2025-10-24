// Account management module

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::core::currency::Currency;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub holder_name: String,
    pub balance: f64,
    pub currency: Currency,
    pub created_at: DateTime<Utc>,
    pub status: AccountStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccountStatus {
    Active,
    Frozen,
    Closed,
}

impl Account {
    pub fn new(holder_name: String, currency: Currency) -> Self {
        Self {
            id: Uuid::new_v4(),
            holder_name,
            balance: 0.0,
            currency,
            created_at: Utc::now(),
            status: AccountStatus::Active,
        }
    }

    pub fn debit(&mut self, amount: f64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Insufficient funds".to_string())
        }
    }

    pub fn credit(&mut self, amount: f64) {
        self.balance += amount;
    }
}