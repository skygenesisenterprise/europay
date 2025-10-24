// Utils module

use uuid::Uuid;

pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn validate_amount(amount: f64) -> Result<f64, String> {
    if amount <= 0.0 {
        Err("Amount must be positive".to_string())
    } else {
        Ok(amount)
    }
}