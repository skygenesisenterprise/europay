mod accounts;
mod cards;
mod transactions;
mod messaging;
mod merchants;
mod security;

use accounts::Account;
use cards::PaymentCard;
use merchants::Merchant;
use transactions::PaymentProcessor;

fn main() {
    println!("Europay - Open Source Payment System");

    // Example usage
    let mut processor = PaymentProcessor::new();

    // Create account
    let mut account = Account::new("John Doe".to_string(), "EUR".to_string());
    account.credit(1000.0); // Add initial balance
    let account_id = account.id;
    processor.add_account(account);

    // Create card
    let card = PaymentCard::new(
        account_id,
        "4111111111111111".to_string(),
        12,
        2025,
        "123".to_string(),
        "John Doe".to_string(),
    );
    let card_id = card.id;
    processor.add_card(card);

    // Create merchant
    let merchant = Merchant::new(
        "Example Store".to_string(),
        "Retail".to_string(),
        uuid::Uuid::new_v4(), // Dummy acquirer
    );
    let merchant_id = merchant.id;
    processor.add_merchant(merchant);

    // Authorize transaction
    match processor.authorize_transaction(card_id, merchant_id, 100.0, "EUR") {
        Ok(tx_id) => {
            println!("Transaction authorized: {}", tx_id);

            // Capture
            if let Ok(()) = processor.capture_transaction(tx_id) {
                println!("Transaction captured");

                // Settle
                if let Ok(()) = processor.settle_transaction(tx_id) {
                    println!("Transaction settled");
                }
            }
        }
        Err(e) => println!("Authorization failed: {}", e),
    }
}
