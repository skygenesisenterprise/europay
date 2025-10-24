// ISO 8583 messaging module

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso8583Message {
    pub mti: String, // Message Type Indicator, 4 digits
    pub bitmap: Vec<u8>, // Primary bitmap, 8 bytes
    pub fields: HashMap<u8, String>, // Data elements, key is field number
}

impl Iso8583Message {
    pub fn new(mti: String) -> Self {
        Self {
            mti,
            bitmap: vec![0; 8],
            fields: HashMap::new(),
        }
    }

    pub fn set_field(&mut self, field_num: u8, value: String) {
        self.fields.insert(field_num, value);
        // Update bitmap
        let byte_index = (field_num - 1) / 8;
        let bit_index = (field_num - 1) % 8;
        if byte_index < 8 {
            self.bitmap[byte_index as usize] |= 1 << (7 - bit_index);
        }
    }

    pub fn get_field(&self, field_num: u8) -> Option<&String> {
        self.fields.get(&field_num)
    }

    // Basic serialization (simplified, real ISO 8583 uses binary)
    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(self.mti.as_bytes());
        data.extend_from_slice(&self.bitmap);
        // For simplicity, serialize fields as JSON-like
        for (k, v) in &self.fields {
            data.push(*k);
            data.extend_from_slice(&(v.len() as u16).to_be_bytes());
            data.extend_from_slice(v.as_bytes());
        }
        data
    }

    // Basic deserialization
    pub fn deserialize(data: &[u8]) -> Result<Self, String> {
        if data.len() < 12 { // MTI 4 + bitmap 8
            return Err("Data too short".to_string());
        }
        let mti = String::from_utf8(data[0..4].to_vec()).map_err(|_| "Invalid MTI")?;
        let bitmap = data[4..12].to_vec();
        let mut fields = HashMap::new();
        let mut pos = 12;
        while pos < data.len() {
            if pos + 3 > data.len() {
                break;
            }
            let field_num = data[pos];
            let len = u16::from_be_bytes([data[pos + 1], data[pos + 2]]) as usize;
            pos += 3;
            if pos + len > data.len() {
                return Err("Invalid field length".to_string());
            }
            let value = String::from_utf8(data[pos..pos + len].to_vec()).map_err(|_| "Invalid field data")?;
            fields.insert(field_num, value);
            pos += len;
        }
        Ok(Self { mti, bitmap, fields })
    }
}

// Common MTIs
pub const MTI_AUTH_REQUEST: &str = "0100";
pub const MTI_AUTH_RESPONSE: &str = "0110";
pub const MTI_FINANCIAL_REQUEST: &str = "0200";
pub const MTI_FINANCIAL_RESPONSE: &str = "0210";