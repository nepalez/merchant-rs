# BankAccount: Direct Bank Account Debit Payment

## Overview

Automated recurring debits from a customer's bank account based on a pre-authorized mandate. Funds are pulled by the merchant on a schedule (subscriptions, utility bills, loan payments). Settlement typically takes 1–3 business days. Customer must provide explicit authorization through mandate setup.

## When to Use

- **Subscription billing**: Monthly/annual recurring charges
- **Utility bills**: Electricity, water, internet, phone services
- **Loan payments**: Mortgage, car loans, student loans
- **Membership fees**: Gym memberships, professional associations
- **Insurance premiums**: Regular insurance payments

## Authentication Model

Authentication occurs **outside** the authorization flow, not during transaction:

### Pre-Authorization Methods

1. **Micro-deposits verification** (Stripe, Plaid):
    - Gateway sends two small deposits (e.g., $0.32 and $0.45) to the customer's account
    - Customer verifies amounts in their bank statement
    - Customer confirms amounts to gateway
    - Proves the customer controls the account

2. **Instant verification** (Plaid, Yodlee, Tink):
    - Customer redirected to bank aggregator
    - Customer logs into a bank through the aggregator's secure interface
    - Aggregator confirms account ownership and validity
    - Instant verification without waiting for deposits

3. **Mandate authorization**:
    - Customer signs direct debit mandate (electronic or physical)
    - Mandate authorizes merchant to initiate debits
    - Mandate stored by gateway and/or merchant
    - NACHA rules (US), Bacs rules (UK), etc. govern mandate requirements

### Authorization Flow

The authorization request contains **only account identification data**, not authentication credentials. Authentication already occurred during mandate setup. Gateway verifies:
- Mandate exists and is valid
- Account is active
- Sufficient funds available (optional pre-notification)

## Standards

- **NACHA Operating Rules**: National Automated Clearing House Association (United States ACH)
- **Bacs Payment Schemes**: Direct Debit scheme (United Kingdom)
- **Payments Canada Rule H1**: Pre-Authorized Debit (PAD) framework
- **[EFT Code of Conduct](https://www.asic.gov.au/regulatory-resources/financial-services/eft-code-of-conduct/)**: Electronic Funds Transfer (Australia)
- **GIRO**: Interbank GIRO system (Singapore)
- **CNP Standards**: China National Payment System
- **Zengin System**: Japanese bank clearing network

## Example Systems

### North America
- **ACH** (United States): NACHA rules, 1-2 day settlement
- **PAD** (Canada): Payments Canada Rule H1, 1-2 day settlement
- **AFT** (Mexico): Automated Funds Transfer

### Europe
- **BACS** (UK): 3-day settlement, Direct Debit Guarantee
- **Lastschrift** (Germany): SEPA Direct Debit variant
- **Incasso** (Netherlands): SEPA Direct Debit variant

### Asia Pacific
- **GIRO** (Singapore): Automated clearing house
- **Zengin** (Japan): Japanese bank clearing network
- **eNETS** (Singapore): Electronic Network for EFT Services

### Other
- **EFT** (Australia): ASIC regulated
- **Autogiro** (Sweden): Swedish direct debit
- **Betalingsservice** (Denmark): Danish direct debit

## Flow Diagram
```
┌─────────────┐
│   Merchant  │
└──────┬──────┘
       │ 1. Initiate mandate setup
       ▼
┌─────────────┐
│   Gateway   │
└──────┬──────┘
       │ 2. Send micro-deposits OR redirect to Plaid
       ▼
┌─────────────┐
│  Customer   │
└──────┬──────┘
       │ 3. Verify amounts OR login to bank
       ▼
┌─────────────┐
│   Gateway   │ ← Mandate established
└──────┬──────┘
       │
       │ [Time passes - subscription period]
       │
       │ 4. Merchant calls authorize()
       ▼
┌─────────────┐
│   Gateway   │
└──────┬──────┘
       │ 5. Initiate ACH/BACS debit (no customer interaction)
       ▼
┌─────────────┐
│     Bank    │
└──────┬──────┘
       │ 6. Settlement (1-3 business days)
       ▼
┌─────────────┐
│  Merchant   │ ← Funds received
└─────────────┘
```

## Security Considerations

### PCI DSS Compliance
Bank account numbers are **not** classified as Sensitive Authentication Data (SAD) under PCI DSS. However, they are critical PII and financial access data.

**Protection measures:**
- Use `AccountNumber` and `RoutingNumber` types which wrap `SecretString`
- Memory is zeroized on a drop
- Debug output is masked
- Access only via unsafe `with_exposed_secret()` method

### Fraud Prevention
- Verify account ownership through micro-deposits or instant verification
- Validate routing numbers against known bank databases
- Check account holder name matching
- Monitor for unusual patterns (multiple attempts, rapid changes)
- Implement velocity limits on failed attempts

### Compliance
- **NACHA**: Must comply with Operating Rules, including authorization requirements
- **Bacs**: Must be registered Service User, follow Direct Debit Guarantee
- **PSD2** (Europe): AML/KYC requirements for account verification
- **GDPR**: Bank account data is PII, must follow data protection regulations
