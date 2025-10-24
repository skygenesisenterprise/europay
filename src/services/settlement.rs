// Settlement service for fund transfers

use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::transactions::Transaction;
use crate::core::currency::Currency;

#[derive(Debug, Clone)]
pub struct SettlementBatch {
    pub id: Uuid,
    pub issuer_id: Uuid,
    pub acquirer_id: Uuid,
    pub transactions: Vec<Uuid>,
    pub total_amount: f64,
    pub currency: Currency,
    pub status: SettlementStatus,
    pub created_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SettlementStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

pub struct SettlementService {
    batches: HashMap<Uuid, SettlementBatch>,
}

impl SettlementService {
    pub fn new() -> Self {
        Self {
            batches: HashMap::new(),
        }
    }

    pub fn create_batch(&mut self, issuer_id: Uuid, acquirer_id: Uuid, transactions: Vec<&Transaction>) -> Uuid {
        let batch_id = Uuid::new_v4();
        let total_amount: f64 = transactions.iter().map(|t| t.amount).sum();
        let currency = transactions.first().map(|t| t.currency).unwrap_or(Currency::EUR);

        let batch = SettlementBatch {
            id: batch_id,
            issuer_id,
            acquirer_id,
            transactions: transactions.iter().map(|t| t.id).collect(),
            total_amount,
            currency,
            status: SettlementStatus::Pending,
            created_at: Utc::now(),
            settled_at: None,
        };

        self.batches.insert(batch_id, batch);
        batch_id
    }

    pub fn process_settlement(&mut self, batch_id: Uuid) -> Result<(), String> {
        let batch = self.batches.get_mut(&batch_id).ok_or("Batch not found")?;

        if batch.status != SettlementStatus::Pending {
            return Err("Batch not in pending status".to_string());
        }

        batch.status = SettlementStatus::Processing;

        // In real system, this would involve:
        // 1. Verify funds availability
        // 2. Transfer funds between issuer and acquirer accounts
        // 3. Update transaction statuses
        // 4. Send confirmations

        // For now, simulate successful settlement
        batch.status = SettlementStatus::Completed;
        batch.settled_at = Some(Utc::now());

        Ok(())
    }

    pub fn get_batch(&self, batch_id: &Uuid) -> Option<&SettlementBatch> {
        self.batches.get(batch_id)
    }

    pub fn get_pending_batches(&self) -> Vec<&SettlementBatch> {
        self.batches.values()
            .filter(|b| b.status == SettlementStatus::Pending)
            .collect()
    }

    pub fn calculate_net_settlement(&self, issuer_id: Uuid, acquirer_id: Uuid) -> f64 {
        // Calculate net amount to be settled between issuer and acquirer
        self.batches.values()
            .filter(|b| b.issuer_id == issuer_id && b.acquirer_id == acquirer_id && b.status == SettlementStatus::Completed)
            .map(|b| b.total_amount)
            .sum()
    }
}