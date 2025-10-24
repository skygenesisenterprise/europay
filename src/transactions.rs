// Transaction processing module

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::accounts::Account;
use crate::cards::PaymentCard;
use crate::merchants::Merchant;
use crate::security::SecurityManager;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub card_id: Uuid,
    pub merchant_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub status: TransactionStatus,
    pub transaction_type: TransactionType,
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Authorized,
    Captured,
    Settled,
    Declined,
    Reversed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionType {
    Purchase,
    Refund,
    Chargeback,
}

impl Transaction {
    pub fn new(card_id: Uuid, merchant_id: Uuid, amount: f64, currency: String, transaction_type: TransactionType) -> Self {
        Self {
            id: Uuid::new_v4(),
            card_id,
            merchant_id,
            amount,
            currency,
            status: TransactionStatus::Pending,
            transaction_type,
            created_at: Utc::now(),
            processed_at: None,
        }
    }
}

pub struct PaymentProcessor {
    accounts: HashMap<Uuid, Account>,
    cards: HashMap<Uuid, PaymentCard>,
    merchants: HashMap<Uuid, Merchant>,
    transactions: HashMap<Uuid, Transaction>,
    security: SecurityManager,
}

impl PaymentProcessor {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            cards: HashMap::new(),
            merchants: HashMap::new(),
            transactions: HashMap::new(),
            security: SecurityManager::new(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.insert(account.id, account);
    }

    pub fn add_card(&mut self, card: PaymentCard) {
        self.cards.insert(card.id, card);
    }

    pub fn add_merchant(&mut self, merchant: Merchant) {
        self.merchants.insert(merchant.id, merchant);
    }

    pub fn authorize_transaction(&mut self, card_id: Uuid, merchant_id: Uuid, amount: f64, currency: &str) -> Result<Uuid, String> {
        let card = self.cards.get(&card_id).ok_or("Card not found")?;
        let merchant = self.merchants.get(&merchant_id).ok_or("Merchant not found")?;
        let account = self.accounts.get(&card.account_id).ok_or("Account not found")?;

        if card.status != crate::cards::CardStatus::Active {
            return Err("Card not active".to_string());
        }
        if card.is_expired() {
            return Err("Card expired".to_string());
        }
        if account.balance < amount {
            return Err("Insufficient funds".to_string());
        }
        if self.security.check_fraud(amount, &card.pan) {
            return Err("Transaction flagged for fraud".to_string());
        }

        let mut transaction = Transaction::new(card_id, merchant_id, amount, currency.to_string(), TransactionType::Purchase);
        transaction.status = TransactionStatus::Authorized;
        transaction.processed_at = Some(Utc::now());

        let tx_id = transaction.id;
        self.transactions.insert(tx_id, transaction);
        Ok(tx_id)
    }

    pub fn capture_transaction(&mut self, tx_id: Uuid) -> Result<(), String> {
        let transaction = self.transactions.get_mut(&tx_id).ok_or("Transaction not found")?;
        if transaction.status != TransactionStatus::Authorized {
            return Err("Transaction not authorized".to_string());
        }

        let card = self.cards.get(&transaction.card_id).ok_or("Card not found")?;
        let account = self.accounts.get_mut(&card.account_id).ok_or("Account not found")?;

        account.debit(transaction.amount)?;
        transaction.status = TransactionStatus::Captured;
        transaction.processed_at = Some(Utc::now());
        Ok(())
    }

    pub fn settle_transaction(&mut self, tx_id: Uuid) -> Result<(), String> {
        let transaction = self.transactions.get_mut(&tx_id).ok_or("Transaction not found")?;
        if transaction.status != TransactionStatus::Captured {
            return Err("Transaction not captured".to_string());
        }

        // In real system, transfer funds to merchant's acquirer
        // For simplicity, just mark as settled
        transaction.status = TransactionStatus::Settled;
        transaction.processed_at = Some(Utc::now());
        Ok(())
    }

    pub fn get_transaction(&self, tx_id: Uuid) -> Option<&Transaction> {
        self.transactions.get(&tx_id)
    }
}