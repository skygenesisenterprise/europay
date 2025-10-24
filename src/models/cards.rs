// Payment card module

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentCard {
    pub id: Uuid,
    pub account_id: Uuid,
    pub pan: String, // Primary Account Number (masked for security)
    pub expiry_month: u8,
    pub expiry_year: u16,
    pub cvv: String, // In real system, this would be hashed/encrypted
    pub cardholder_name: String,
    pub status: CardStatus,
    pub issued_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardStatus {
    Active,
    Blocked,
    Expired,
}

impl PaymentCard {
    pub fn new(account_id: Uuid, pan: String, expiry_month: u8, expiry_year: u16, cvv: String, cardholder_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            account_id,
            pan,
            expiry_month,
            expiry_year,
            cvv,
            cardholder_name,
            status: CardStatus::Active,
            issued_at: Utc::now(),
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = Utc::now();
        let expiry_date = chrono::NaiveDate::from_ymd_opt(self.expiry_year as i32, self.expiry_month as u32, 1)
            .unwrap_or_default()
            .and_hms_opt(0, 0, 0)
            .unwrap_or_default();
        let expiry_datetime = DateTime::<Utc>::from_naive_utc_and_offset(expiry_date, Utc);
        now > expiry_datetime
    }
}