// Currency support for European payments

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    EUR, // Euro
    GBP, // British Pound
    CHF, // Swiss Franc
    SEK, // Swedish Krona
    NOK, // Norwegian Krone
    DKK, // Danish Krone
    PLN, // Polish Zloty
    CZK, // Czech Koruna
    HUF, // Hungarian Forint
    RON, // Romanian Leu
    BGN, // Bulgarian Lev
    HRK, // Croatian Kuna
}

impl Currency {
    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::EUR => "€",
            Currency::GBP => "£",
            Currency::CHF => "CHF",
            Currency::SEK => "kr",
            Currency::NOK => "kr",
            Currency::DKK => "kr",
            Currency::PLN => "zł",
            Currency::CZK => "Kč",
            Currency::HUF => "Ft",
            Currency::RON => "lei",
            Currency::BGN => "лв",
            Currency::HRK => "kn",
        }
    }

    pub fn decimal_places(&self) -> u8 {
        match self {
            Currency::EUR | Currency::GBP | Currency::CHF => 2,
            Currency::SEK | Currency::NOK | Currency::DKK => 2,
            Currency::PLN => 2,
            Currency::CZK => 2,
            Currency::HUF => 0, // Hungarian Forint has no decimal places
            Currency::RON => 2,
            Currency::BGN => 2,
            Currency::HRK => 2,
        }
    }

    pub fn is_eurozone(&self) -> bool {
        matches!(self, Currency::EUR)
    }
}

pub struct CurrencyConverter {
    rates: HashMap<(Currency, Currency), f64>,
}

impl CurrencyConverter {
    pub fn new() -> Self {
        let mut rates = HashMap::new();
        // Initialize with some example rates (in real system, fetch from API)
        rates.insert((Currency::EUR, Currency::GBP), 0.85);
        rates.insert((Currency::EUR, Currency::CHF), 0.95);
        rates.insert((Currency::EUR, Currency::SEK), 11.5);
        rates.insert((Currency::EUR, Currency::NOK), 11.8);
        rates.insert((Currency::EUR, Currency::DKK), 7.45);
        rates.insert((Currency::EUR, Currency::PLN), 4.3);
        rates.insert((Currency::EUR, Currency::CZK), 25.0);
        rates.insert((Currency::EUR, Currency::HUF), 380.0);
        rates.insert((Currency::EUR, Currency::RON), 4.95);
        rates.insert((Currency::EUR, Currency::BGN), 1.955);
        rates.insert((Currency::EUR, Currency::HRK), 7.5);

        // Add reverse rates
        let mut reverse_rates = HashMap::new();
        for ((from, to), rate) in &rates {
            reverse_rates.insert((*to, *from), 1.0 / rate);
        }
        rates.extend(reverse_rates);

        Self { rates }
    }

    pub fn convert(&self, amount: f64, from: &Currency, to: &Currency) -> f64 {
        if from == to {
            return amount;
        }

        if let Some(rate) = self.rates.get(&(*from, *to)) {
            amount * rate
        } else {
            // Fallback: convert through EUR
            let to_eur = self.rates.get(&(*from, Currency::EUR)).unwrap_or(&1.0);
            let from_eur = self.rates.get(&(Currency::EUR, *to)).unwrap_or(&1.0);
            amount * to_eur * from_eur
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_conversion() {
        let converter = CurrencyConverter::new();
        let amount = 100.0;
        let converted = converter.convert(amount, &Currency::EUR, &Currency::GBP);
        assert!(converted > 80.0 && converted < 90.0);
    }

    #[test]
    fn test_same_currency() {
        let converter = CurrencyConverter::new();
        let amount = 100.0;
        let converted = converter.convert(amount, &Currency::EUR, &Currency::EUR);
        assert_eq!(converted, 100.0);
    }
}