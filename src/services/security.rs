// Security module

use ring::rand::SecureRandom;
use ring::{aead, rand};
use std::collections::HashMap;
use uuid::Uuid;

pub struct SecurityManager {
    rng: rand::SystemRandom,
    key: aead::LessSafeKey,
    tokens: HashMap<String, String>, // token -> PAN
}

impl SecurityManager {
    pub fn new() -> Self {
        let rng = rand::SystemRandom::new();
        let mut key_bytes = [0u8; 32];
        rng.fill(&mut key_bytes).unwrap();
        let key = aead::LessSafeKey::new(aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes).unwrap());

        Self {
            rng,
            key,
            tokens: HashMap::new(),
        }
    }

    pub fn tokenize_pan(&mut self, pan: &str) -> String {
        let token = Uuid::new_v4().to_string();
        self.tokens.insert(token.clone(), pan.to_string());
        token
    }

    pub fn detokenize_pan(&self, token: &str) -> Option<&String> {
        self.tokens.get(token)
    }

    pub fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut in_out = data.to_vec();
        let nonce = aead::Nonce::assume_unique_for_key([0u8; 12]); // In real system, use unique nonce
        self.key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|_| "Encryption failed".to_string())?;
        Ok(in_out)
    }

    pub fn decrypt_data(&self, encrypted: &[u8]) -> Result<Vec<u8>, String> {
        let mut in_out = encrypted.to_vec();
        let nonce = aead::Nonce::assume_unique_for_key([0u8; 12]);
        self.key.open_in_place(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|_| "Decryption failed".to_string())?;
        in_out.truncate(in_out.len() - aead::AES_256_GCM.tag_len());
        Ok(in_out)
    }

    pub fn check_fraud(&self, amount: f64, card_pan: &str) -> bool {
        // Simple fraud detection: flag if amount > 1000
        amount > 1000.0
    }
}