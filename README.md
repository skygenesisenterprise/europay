# Europay

Europay is an open-source payment system inspired by Visa/Mastercard, written in Rust. It provides a framework for processing electronic transactions securely and efficiently.

## Features

- **Account Management**: Create and manage user accounts with balances.
- **Payment Cards**: Issue and manage payment cards with expiration and status tracking.
- **Transaction Processing**: Handle authorization, capture, and settlement of transactions.
- **Security**: Basic encryption, tokenization, and fraud detection.
- **ISO 8583 Messaging**: Support for financial transaction messages (basic implementation).
- **Merchant Support**: Onboard merchants and process their transactions.

## Architecture

The system is built with modularity in mind:

- `accounts`: Account management
- `cards`: Payment card handling
- `transactions`: Transaction processing logic
- `messaging`: ISO 8583 message handling
- `merchants`: Merchant management
- `security`: Security features

## Getting Started

### Prerequisites

- Rust 1.70 or later

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

This starts the API server on `http://127.0.0.1:3000`.

### API Endpoints

- `GET /health` - Health check
- `POST /transactions/authorize` - Authorize a transaction
- `POST /transactions/capture` - Capture an authorized transaction
- `POST /transactions/settle` - Settle a captured transaction

Example request for authorization:

```json
{
  "card_id": "uuid-here",
  "merchant_id": "uuid-here",
  "amount": 100.0,
  "currency": "EUR"
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
