# Europay

Europay is an open-source, sovereign European payment network alternative to Visa/Mastercard and Bancontact. Written in Rust, it provides a decentralized framework for processing electronic transactions across European currencies with strong privacy and security guarantees.

## Vision

Europay aims to create a European payment ecosystem that:
- Supports all major European currencies (EUR, GBP, CHF, SEK, NOK, DKK, PLN, CZK, HUF, RON, BGN, HRK)
- Ensures data sovereignty and privacy for European citizens
- Provides an open, interoperable alternative to proprietary payment networks
- Implements modern security standards and compliance (PSD2, GDPR)

## Features

- **Multi-Currency Support**: Native support for 12 European currencies with automatic conversion
- **Decentralized Network**: Peer-to-peer communication between issuers, acquirers, and network nodes
- **Settlement System**: Automated batch settlement processing between financial institutions
- **Transaction Processing**: Complete authorization, capture, and settlement workflow
- **Security**: Encryption, tokenization, and fraud detection
- **API-First Design**: RESTful APIs for integration with merchants and financial institutions
- **Network Protocol**: HTTP-based messaging for inter-node communication

## Architecture

The system is built with modularity in mind:

- `accounts`: Account management
- `cards`: Payment card handling
- `transactions`: Transaction processing logic
- `messaging`: ISO 8583 message handling
- `merchants`: Merchant management
- `security`: Security features

## Architecture

Europay follows a modular, service-oriented architecture:

- **Core**: Currency handling, network protocols
- **Models**: Data structures for accounts, cards, transactions
- **Services**: Business logic (security, settlement, networking)
- **Controllers**: API request handlers
- **Routes**: URL routing and middleware
- **Utils**: Helper functions

### Network Roles

- **Issuers**: Banks that issue payment cards
- **Acquirers**: Banks that process merchant payments
- **Network Nodes**: Routing and settlement infrastructure

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

This starts the Europay node on `http://127.0.0.1:3000`.

### API Endpoints

#### Transactions
- `POST /transactions/authorize` - Authorize a transaction
- `POST /transactions/capture` - Capture an authorized transaction
- `POST /transactions/settle` - Settle a captured transaction

#### Network
- `POST /network/message` - Send network messages between nodes

#### Settlement
- `POST /settlement/batch` - Create a settlement batch
- `POST /settlement/process` - Process settlement

#### Health
- `GET /health` - Health check

Example authorization request:

```json
{
  "card_id": "uuid-here",
  "merchant_id": "uuid-here",
  "amount": 100.0,
  "currency": "EUR"
}
```

Supported currencies: EUR, GBP, CHF, SEK, NOK, DKK, PLN, CZK, HUF, RON, BGN, HRK

## Roadmap

- [x] Multi-currency support for European currencies
- [x] Basic transaction processing (authorize/capture/settle)
- [x] Network protocol for inter-node communication
- [x] Settlement system for fund transfers
- [ ] Decentralized routing
- [ ] PSD2 compliance features
- [ ] Client SDKs for merchants/issuers
- [ ] Privacy enhancements (zero-knowledge proofs)
- [ ] Production deployment guides

## Contributing

Contributions are welcome! Areas of particular interest:
- European regulatory compliance
- Performance optimization
- Additional currency support
- Security audits
- Documentation

Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
