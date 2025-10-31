# CryptoPayment: Cryptocurrency Payment via Blockchain Transfer

## Overview

Customer sends cryptocurrency directly from their wallet to a gateway-generated deposit address. Payment is settled on-chain through blockchain network confirmation. Transaction is irreversible once confirmed on the blockchain (typically 1-6 block confirmations depending on network and amount). Gateway monitors the blockchain for incoming transactions and confirms payment asynchronously via webhook.

## When to Use

- **Borderless payments**: No traditional banking infrastructure required
- **Crypto-native users**: Customers holding cryptocurrency who prefer to pay directly
- **Traditional rails unavailable**: Regions where card or bank transfer infrastructure is limited
- **High-value international transfers**: Lower fees than wire transfers for large amounts
- **Privacy-focused transactions**: Reduced KYC requirements compared to traditional payments
- **24/7 settlement**: No banking hours or weekend delays

## Authentication Model

Authentication occurs **on-chain through cryptographic signatures**, not in authorization request:

### Payment Flow Steps

1. **Merchant initiates**: Calls `authorize()` with crypto payment request
2. **Gateway generates address**: Creates unique deposit address for the transaction
3. **Customer receives address**: Via QR code, direct address, or payment link
4. **Customer initiates transfer**: Sends crypto from any wallet to the deposit address
5. **Blockchain confirmation**: Transaction propagates through network, miners/validators confirm
6. **Gateway monitors blockchain**: Watches for incoming transactions to the deposit address
7. **Webhook notification**: Gateway notifies merchant after sufficient confirmations (async, minutes to hours)
8. **Settlement complete**: Funds available to merchant (may be converted to fiat automatically)

### Authorization Request Content

The authorization request contains **only metadata for currency/network selection**, not authentication credentials. Authentication is inherent in blockchain — only the holder of private keys can sign valid transactions.

## Standards

- **Bitcoin (BTC)**: [Bitcoin Core reference implementation](https://bitcoin.org/en/bitcoin-core/), BIP standards
- **Ethereum (ETH)**: [Ethereum Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf), EIP standards
- **ERC-20 tokens**: [EIP-20 standard](https://eips.ethereum.org/EIPS/eip-20) for fungible tokens (USDT, USDC, DAI)
- **BIP-21**: [Bitcoin URI scheme](https://github.com/bitcoin/bips/blob/master/bip-0021.mediawiki) for payment requests
- **EIP-681**: [Ethereum URI scheme](https://eips.ethereum.org/EIPS/eip-681) for payment requests
- **Lightning Network**: [BOLT specifications](https://github.com/lightning/bolts) for instant Bitcoin payments

## Example Systems

### Payment Processors
- **BitPay**: BTC, BCH, ETH, stablecoins; invoice generation, auto-conversion
- **Coinbase Commerce**: BTC, ETH, LTC, BCH, USDC; merchant-hosted checkout
- **NOWPayments**: 200+ cryptocurrencies, auto-conversion, recurring payments
- **CoinGate**: 70+ cryptocurrencies, Lightning Network, fiat settlement
- **BTCPay Server**: Self-hosted, non-custodial, supports BTC and Lightning

### Supported Networks
- **Bitcoin (BTC)**: Most established, 10-60 minute confirmation
- **Ethereum (ETH)**: Smart contract platform, ~15 second blocks
- **Lightning Network**: Instant Bitcoin micropayments, off-chain scaling
- **Litecoin (LTC)**: Faster confirmation than Bitcoin (~2.5 minutes)
- **Bitcoin Cash (BCH)**: Low fees, ~10 minute confirmation
- **Binance Smart Chain (BSC)**: EVM-compatible, low fees
- **Polygon (MATIC)**: Ethereum sidechain, very low fees
- **Tron (TRX)**: High throughput, low fees

### Stablecoins
- **USDT (Tether)**: Multiple networks (Ethereum, Tron, BSC)
- **USDC (USD Coin)**: Ethereum, Polygon, other networks
- **DAI**: Decentralized stablecoin on Ethereum
- **BUSD (Binance USD)**: Binance ecosystem stablecoin

## Flow Diagram
```
┌─────────────┐
│  Customer   │
└──────┬──────┘
       │ 1. Initiates crypto payment
       ▼
┌─────────────┐
│  Merchant   │
└──────┬──────┘
       │ 2. Calls authorize() with CryptoPayment
       ▼
┌─────────────┐
│   Gateway   │
└──────┬──────┘
       │ 3. Generates unique deposit address
       │ 4. Returns address/QR code
       ▼
┌─────────────┐
│  Customer   │
└──────┬──────┘
       │ 5. Opens crypto wallet
       │ 6. Scans QR or pastes address
       │ 7. Signs and broadcasts transaction
       ▼
┌─────────────┐
│ Blockchain  │
└──────┬──────┘
       │ 8. Transaction propagates
       │ 9. Miners/validators confirm (1-6 blocks)
       ▼
┌─────────────┐
│   Gateway   │ (Monitoring blockchain)
└──────┬──────┘
       │ 10. Detects incoming transaction
       │ 11. Waits for confirmations
       │ 12. Webhook notification (async)
       ▼
┌─────────────┐
│  Merchant   │ ← Payment confirmed
└─────────────┘
```

## Security Considerations

### Blockchain Security
- Customer authenticates via private key cryptographic signature
- Gateway never handles customer's private keys
- Transactions are irreversible once confirmed on-chain
- No chargebacks or payment reversals

### Address Generation
- Use HD (Hierarchical Deterministic) wallets for address generation
- Each transaction should use a unique deposit address
- Never reuse addresses to prevent transaction correlation
- Validate address format before displaying to customer

### Confirmation Requirements
- Higher value transactions require more confirmations
- Bitcoin: 1 confirmation (~10 min) for small amounts, 6 confirmations (~60 min) for large
- Ethereum: 12 confirmations (~3 min) typical, 35+ for high value
- Stablecoins: Follow underlying network confirmation rules
- Lightning Network: Instant for small amounts (channel-based trust)

### Fraud Prevention
- Monitor for double-spend attempts on low-confirmation transactions
- Validate transaction amount matches requested amount
- Check for Replace-By-Fee (RBF) flags on Bitcoin transactions
- Implement rate limiting on deposit address generation
- Monitor for dust attacks and unusual patterns

### Price Volatility
- Cryptocurrency prices are highly volatile
- Lock exchange rate at time of address generation
- Set expiration time for payment requests (typically 15-30 minutes)
- Auto-convert to fiat immediately upon confirmation to reduce merchant risk
- Clearly communicate final fiat amount and exchange rate to customer

### Compliance
- **AML/KYC**: Requirements vary by jurisdiction and transaction size
- **Travel Rule**: Some jurisdictions require customer information for large transfers
- **Tax reporting**: Cryptocurrency transactions may trigger reporting obligations
- **Sanctions screening**: Check addresses against OFAC and other sanctions lists
- **GDPR**: Blockchain addresses are pseudonymous but may be considered PII
- **Securities regulations**: Some tokens may be classified as securities

### Network-Specific Considerations
- **Gas fees** (Ethereum): Can exceed payment amount for small transactions during high network usage
- **Mempool congestion**: Bitcoin transactions may be delayed during high activity
- **Smart contract risks**: Token contracts may have bugs or be malicious
- **Network forks**: Ensure correct chain after contentious forks
- **51% attacks**: Consider network hashrate for security assurance
